#!/bin/bash
set -euxo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

(
    cd $SCRIPT_DIR
    rm -f pyo3_example.wasm
    RUSTFLAGS="-Clink-arg=--import-memory -C link-arg=--initial-memory=4587520" cargo build --release --target wasm32-wasip1
    cp target/wasm32-wasip1/release/pyo3_example.wasm .
)
