.PHONY: clean
clean:
	rm -rf target
	rm -f ../pyo3_plugin*.wasm

.PHONY: build
build:
	# TODO: FIXME
	export WASI_SDK_PATH=/home/andreatp/workspace/quarkus-grpc-zero/buildtools/protoc-wrapper/tools/wasi-sdk-25.0-x86_64-linux && \
	cargo build --release --target=wasm32-wasip1
	rm -f ../pyo3_plugin*.wasm
	
	# First, disable reference-types for wasi-vfs compatibility
	wasm-opt --disable-reference-types target/wasm32-wasip1/release/pyo3_plugin.wasm -o output.wasm

	# Use wasi-vfs to pack the filesystem into the WASM binary
	# There is a problem, seems to be not working ... but wizer is correctly starting ...
	./wasi-vfs pack output.wasm \
		--dir target/wasm32-wasi/wasi-deps/usr::/usr \
		-o pyo3_plugin_with_fs.wasm

	# Use wizer to initialize the WASM
	wizer \
		--allow-wasi \
		--wasm-bulk-memory true \
		--inherit-stdio true \
		--inherit-env true \
		pyo3_plugin_with_fs.wasm -o pyo3_plugin_initialized.wasm

	wasm-opt -Oz pyo3_plugin_initialized.wasm -o ../pyo3_plugin-opt.wasm
