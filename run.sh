#!/bin/bash
wasmer run --enable-all --mapdir ./dev:/dev --mapdir ./data:./data target/wasm32-wasi/debug/verify_attestation.wasm
