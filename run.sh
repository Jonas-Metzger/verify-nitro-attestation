#!/bin/bash
wasmtime run --mapdir ./dev::/dev --mapdir ./data::./data ./verify_attestation.wasm
