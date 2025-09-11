.PHONY: clean
clean:
	rm -rf target
	rm -f ../pyo3_plugin*.wasm

.PHONY: build
build:
	cargo build --release --target=wasm32-wasip1
	rm -f ../pyo3_plugin*.wasm

	wizer \
		--allow-wasi \
		--wasm-bulk-memory true \
		--inherit-stdio true \
		--inherit-env true \
		--mapdir="/usr::target/wasm32-wasi/wasi-deps/usr" \
		target/wasm32-wasip1/release/pyo3_plugin.wasm -o pyo3_plugin_initialized.wasm

	wasm-opt -Oz pyo3_plugin_initialized.wasm -o ../pyo3_plugin-opt.wasm
