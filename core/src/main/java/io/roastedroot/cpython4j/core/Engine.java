package io.roastedroot.cpython4j.core;

import static java.nio.charset.StandardCharsets.UTF_8;

import com.dylibso.chicory.annotations.WasmModuleInterface;
import com.dylibso.chicory.log.Logger;
import com.dylibso.chicory.log.SystemLogger;
import com.dylibso.chicory.runtime.ByteArrayMemory;
import com.dylibso.chicory.runtime.HostFunction;
import com.dylibso.chicory.runtime.ImportValues;
import com.dylibso.chicory.runtime.Instance;
import com.dylibso.chicory.runtime.Memory;
import com.dylibso.chicory.runtime.TrapException;
import com.dylibso.chicory.wasi.Files;
import com.dylibso.chicory.wasi.WasiOptions;
import com.dylibso.chicory.wasi.WasiPreview1;
import com.dylibso.chicory.wasm.types.MemoryLimits;
import com.dylibso.chicory.wasm.types.ValueType;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.roastedroot.zerofs.Configuration;
import io.roastedroot.zerofs.ZeroFs;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.nio.file.FileSystem;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Function;

@WasmModuleInterface(WasmResource.absoluteFile)
public final class Engine implements AutoCloseable {
    private static final int ALIGNMENT = 1;
    public static final ObjectMapper DEFAULT_OBJECT_MAPPER = new ObjectMapper();

    private final ByteArrayOutputStream stdout = new ByteArrayOutputStream();
    private final ByteArrayOutputStream stderr = new ByteArrayOutputStream();
    private final WasiOptions wasiOpts;

    private final WasiPreview1 wasi;
    private final FileSystem fs;
    private final Instance instance;
    private final Engine_ModuleExports exports;

    private final Map<String, Builtins> builtins;
    private final Map<String, Invokables> invokables;
    private final ObjectMapper mapper;

    private final List<Object> javaRefs = new ArrayList<>();

    private static final String ENGINE_MODULE_NAME = "cpython4j_engine";
    private static final String MODULE_NAME_FUNC = "module_name";
    private static final String FUNCTION_NAME_FUNC = "function_name";
    private static final String ARGS_FUNC = "args";

    private String invokeModuleName;
    private String invokeFunctionName;
    private String invokeArgs;

    public static Builder builder() {
        return new Builder();
    }

    private Engine(
            Map<String, Builtins> builtins,
            Map<String, Invokables> invokables,
            ObjectMapper mapper,
            Function<MemoryLimits, Memory> memoryFactory,
            Logger logger) {
        this.mapper = mapper;
        this.builtins = builtins;

        // builtins to make invoke dynamic javascript functions
        // TODO: we should be able to use Py03 functions for doing this
        //        builtins.put(
        //                ENGINE_MODULE_NAME,
        //                Builtins.builder(ENGINE_MODULE_NAME)
        //                        .addVoidToString(MODULE_NAME_FUNC, () -> invokeModuleName)
        //                        .addVoidToString(FUNCTION_NAME_FUNC, () -> invokeFunctionName)
        //                        .addVoidToString(ARGS_FUNC, () -> invokeArgs)
        //                        .build());

        var wasiOptsBuilder = WasiOptions.builder().withStdout(stdout).withStderr(stderr);
        this.fs =
                ZeroFs.newFileSystem(
                        Configuration.unix().toBuilder().setAttributeViews("unix").build());

        // TODO: FIXME - we should bake the FS into wasm - but it doesn't seem to work ...
        // this should not be needed ... try again on Python 3.13 maybe
        // check again
        // pyo3-plugin/webassembly-language-runtimes/python/examples/embedding/wasi-py-rs-pyo3/README.md
        // and https://github.com/trinodb/trino-wasm-python
        Path inputFolder = fs.getPath("/usr");
        Path copyFrom = Path.of("../pyo3-plugin/target/wasm32-wasi/wasi-deps/usr");
        try {
            Files.copyDirectory(copyFrom, inputFolder);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
        wasiOptsBuilder.withDirectory(inputFolder.toString(), inputFolder);

        wasiOptsBuilder.withEnvironment("PYTHONDONTWRITEBYTECODE", "1");

        this.wasiOpts = wasiOptsBuilder.build();
        this.wasi = WasiPreview1.builder().withOptions(this.wasiOpts).withLogger(logger).build();

        // set_result builtins
        invokables.entrySet().stream()
                .forEach(
                        e -> {
                            var builder = Builtins.builder(e.getKey());
                            e.getValue()
                                    .functions()
                                    .forEach(entry -> builder.add(entry.setResultHostFunction()));
                            this.builtins.put(e.getKey(), builder.build());
                        });
        this.invokables = invokables;
        instance =
                Instance.builder(PythonPlugin.load())
                        .withMemoryFactory(memoryFactory)
                        .withMachineFactory(PythonPlugin::create)
                        .withImportValues(
                                ImportValues.builder()
                                        .addFunction(wasi.toHostFunctions())
                                        .addFunction(invokeFn)
                                        .build())
                        .build();
        exports = new Engine_ModuleExports(instance);
        exports.pluginInit();
    }

    private String readpy03String(int ptr, int len) {
        var bytes = instance.memory().readBytes(ptr, len);
        return new String(bytes, UTF_8);
    }

    private String computeArgs(String moduleName, String name, List<Object> args) {
        GuestFunction guestFunction = invokables.get(moduleName).byName(name);
        if (guestFunction.paramTypes().size() != args.size()) {
            throw new IllegalArgumentException(
                    "Guest function should be invoked with the expected "
                            + guestFunction.paramTypes().size()
                            + " params, but got: "
                            + args.size());
        }
        StringBuilder paramsStr = new StringBuilder();
        try {
            for (int i = 0; i < args.size(); i++) {
                if (i > 0) {
                    paramsStr.append(", ");
                }
                var clazz = guestFunction.paramTypes().get(i);
                if (clazz == HostRef.class) {
                    javaRefs.add(args.get(i));
                    var ptr = javaRefs.size() - 1;
                    paramsStr.append(mapper.writeValueAsString(ptr));
                } else {
                    paramsStr.append(mapper.writeValueAsString(args.get(i)));
                }
            }
        } catch (JsonProcessingException e) {
            throw new RuntimeException(e);
        }

        return "[" + paramsStr + "]";
    }

    private long[] invokeBuiltin(Instance instance, long[] args) {
        String moduleName = readpy03String((int) args[0], (int) args[1]);
        String funcName = readpy03String((int) args[2], (int) args[3]);
        String argsString = readpy03String((int) args[4], (int) args[5]);

        if (!builtins.containsKey(moduleName)) {
            throw new IllegalArgumentException("Failed to find builtin module name " + moduleName);
        }
        if (builtins.get(moduleName).byName(funcName) == null) {
            throw new IllegalArgumentException(
                    "Failed to find function with name " + funcName + " in module " + moduleName);
        }
        var receiver = builtins.get(moduleName).byName(funcName);

        var argsList = new ArrayList<>();
        try {
            JsonNode tree = mapper.readTree(argsString);

            for (int i = 0; i < receiver.paramTypes().size(); i++) {
                var clazz = receiver.paramTypes().get(i);
                JsonNode value = null;
                if (tree.size() > i) {
                    value = tree.get(i);
                }

                if (clazz == HostRef.class) {
                    argsList.add(javaRefs.get(value.intValue()));
                } else {
                    argsList.add(mapper.treeToValue(value, clazz));
                }
            }

            var res = receiver.invoke(argsList);

            // Converting Java references into pointers for JS
            var returnType = receiver.returnType();
            if (returnType == HostRef.class) {
                returnType = Integer.class;
                if (res instanceof HostRef) {
                    res = ((HostRef) res).pointer();
                } else {
                    javaRefs.add(res);
                    res = javaRefs.size() - 1;
                }
            }

            var returnStr =
                    (returnType == Void.class)
                            ? "null"
                            : mapper.writerFor(returnType).writeValueAsString(res);
            var returnBytes = returnStr.getBytes();

            var returnPtr = exports.pluginMalloc(returnBytes.length);
            exports.memory().write(returnPtr, returnBytes);

            var LEN = 8;
            var widePtr = exports.pluginMalloc(LEN);

            instance.memory().writeI32(widePtr, returnPtr);
            instance.memory().writeI32(widePtr + 4, returnBytes.length);

            return new long[] {widePtr};
        } catch (JsonProcessingException e) {
            throw new RuntimeException(e);
        }
    }

    private final HostFunction invokeFn =
            new HostFunction(
                    "chicory",
                    "wasm_invoke",
                    List.of(
                            ValueType.I32,
                            ValueType.I32,
                            ValueType.I32,
                            ValueType.I32,
                            ValueType.I32,
                            ValueType.I32),
                    List.of(ValueType.I32),
                    this::invokeBuiltin);

    // This function dynamically generates the global functions defined by the Builtins
    private byte[] jsPrelude() {
        var preludeBuilder = new StringBuilder();
        preludeBuilder.append("import builtins, json, pyo3_plugin\n");
        preludeBuilder.append("from types import SimpleNamespace\n");
        for (Map.Entry<String, Builtins> builtin : builtins.entrySet()) {
            preludeBuilder.append("builtins." + builtin.getKey() + " = SimpleNamespace()\n");
            for (var func : builtins.get(builtin.getKey()).functions()) {
                var functionBuiltin =
                        "builtins."
                                + builtin.getKey()
                                + "."
                                + func.name()
                                + " = lambda *args: json.loads(pyo3_plugin.invoke(\""
                                + builtin.getKey()
                                + "\", \""
                                + func.name()
                                + "\", json.dumps(args)))\n";
                preludeBuilder.append(functionBuiltin);
            }
        }
        return preludeBuilder.toString().getBytes();
    }

    public void exec(String py) {
        exec(py.getBytes(UTF_8));
    }

    public void exec(byte[] py) {
        byte[] prelude = jsPrelude();
        byte[] pyCode = new byte[prelude.length + py.length];
        System.arraycopy(prelude, 0, pyCode, 0, prelude.length);
        System.arraycopy(py, 0, pyCode, prelude.length, py.length);

        var codePtr = exports.pluginMalloc(pyCode.length);
        instance.memory().write(codePtr, pyCode);

        try {
            exports.pluginEval(codePtr, pyCode.length);
        } catch (TrapException e) {
            try {
                stderr.flush();
                stdout.flush();
            } catch (IOException ex) {
                throw new RuntimeException("Failed to flush stdout/stderr");
            }

            throw new GuestException(
                    "An exception occurred during the execution.\nstderr: "
                            + stderr.toString(UTF_8)
                            + "\nstdout: "
                            + stdout.toString(UTF_8));
        }
    }

    public String stdout() {
        try {
            stdout.flush();
        } catch (IOException ex) {
            throw new RuntimeException("Failed to flush stdout");
        }

        return stdout.toString(UTF_8);
    }

    public String stderr() {
        try {
            stderr.flush();
        } catch (IOException ex) {
            throw new RuntimeException("Failed to flush stdout");
        }

        return stderr.toString(UTF_8);
    }

    //    public void free(int codePtr) {
    //        var ptr = exports.memory().readInt(codePtr);
    //        var codeLength = exports.memory().readInt(codePtr + 4);
    //
    //        exports.pluginFree(codePtr);
    //    }

    @Override
    public void close() {
        if (wasi != null) {
            wasi.close();
        }
        if (stdout != null) {
            try {
                stdout.flush();
                stdout.close();
            } catch (IOException e) {
                throw new RuntimeException("Failed to close stdout", e);
            }
        }
        if (stderr != null) {
            try {
                stderr.flush();
                stderr.close();
            } catch (IOException e) {
                throw new RuntimeException("Failed to close stderr", e);
            }
        }
        if (fs != null) {
            try {
                fs.close();
            } catch (IOException e) {
                throw new RuntimeException("Failed to close fs", e);
            }
        }
    }

    public static final class Builder {
        private List<Builtins> builtins = new ArrayList<>();
        private List<Invokables> invokables = new ArrayList<>();
        private ObjectMapper mapper;
        private Function<MemoryLimits, Memory> memoryFactory;
        private Logger logger;

        private Builder() {}

        public Builder addBuiltins(Builtins builtins) {
            this.builtins.add(builtins);
            return this;
        }

        public Builder addInvokables(Invokables invokables) {
            this.invokables.add(invokables);
            return this;
        }

        public Builder withObjectMapper(ObjectMapper mapper) {
            this.mapper = mapper;
            return this;
        }

        public Builder withMemoryFactory(Function<MemoryLimits, Memory> memoryFactory) {
            this.memoryFactory = memoryFactory;
            return this;
        }

        public Builder withLogger(Logger logger) {
            this.logger = logger;
            return this;
        }

        public Engine build() {
            if (mapper == null) {
                mapper = DEFAULT_OBJECT_MAPPER;
            }
            if (memoryFactory == null) {
                memoryFactory = ByteArrayMemory::new;
            }
            Map<String, Builtins> finalBuiltins = new HashMap<>();
            // TODO: any validation to be done here?
            for (var builtin : builtins) {
                finalBuiltins.put(builtin.moduleName(), builtin);
            }
            Map<String, Invokables> finalInvokables = new HashMap<>();
            // TODO: any validation to be done here?
            for (var invokable : invokables) {
                finalInvokables.put(invokable.moduleName(), invokable);
            }
            if (logger == null) {
                logger = new SystemLogger();
            }
            return new Engine(finalBuiltins, finalInvokables, mapper, memoryFactory, logger);
        }
    }
}
