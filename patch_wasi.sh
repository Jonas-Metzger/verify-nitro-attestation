#!/bin/bash

# This should take the name of the Rust project name
rust_project_name=verify_attestation

# Transform to WAT
npx wasm2wat "run/$rust_project_name.wasm" -o "run/$rust_project_name.wat"

# The WASI functions are exported from `env` in the filesystem from `emscripten-module-wrapper`
sed -i 's/wasi_snapshot_preview1/env/g' "run/$rust_project_name.wat"
sed -i 's/wasi_unstable/env/g' "run/$rust_project_name.wat"
sed -i 's/wasi/env/g' "run/$rust_project_name.wat"

# Transform back to WASM
npx wat2wasm "run/$rust_project_name.wat" -o "run/$rust_project_name.wasm"
