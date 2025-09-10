.PHONY: clean
clean:
	rm -rf target
	rm -f ../pyo3_plugin*.wasm

.PHONY: build
build:
	cargo build --release --target=wasm32-wasip1
	rm -f ../pyo3_plugin*.wasm
	wasm-opt -Oz target/wasm32-wasip1/release/pyo3_plugin.wasm -o ../pyo3_plugin-opt.wasm
