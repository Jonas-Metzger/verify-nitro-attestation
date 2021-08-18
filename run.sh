#!/bin/bash

rust_project_name=verify_attestation
input_file="data/attestation_doc"
output_file="data/extracted.txt"

# You can add more input and output files by providing more `--file filename` args
node "../emscripten-module-wrapper/prepare.js" "target/wasm32-unknown-emscripten/release/$rust_project_name.js" --file "$input_file" --file "$output_file" --run --debug --out=truebit_run
