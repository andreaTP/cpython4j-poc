
I should be able to:

- compile a minimal python interpreter statically, exposing the eval and invoke functions to wasm
- compile to Java bytecode and initialize it
- compile a "guest module code" to wasm using the same toolchain - e.g. using `extension-module`
- now I have 2 wasm payloads, an heavy one with the Python interpreter and a little one with just the user code
