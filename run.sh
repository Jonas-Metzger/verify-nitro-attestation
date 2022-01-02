#!/bin/bash

# wasi-sdk-12.0-linux.tar.gz
# wasmer target/wasm32-wasi/debug/verify_attestation.wasm data/attestation_doc
wasmer run --enable-all --dir /dev --dir . target/wasm32-wasi/debug/verify_attestation.wasm
#rust_project_name=verify_attestation
#input_file="data/attestation_doc"
#output_file="data/extracted.txt"

# You can add more input and output files by providing more `--file filename` args
#node "../emscripten-module-wrapper/prepare.js" "target/wasm32-unknown-emscripten/release/$rust_project_name.js" --file "$input_file" --file "$output_file" --run --debug --out=truebit_run
