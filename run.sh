#!/bin/bash
wasmtime run --mapdir ./dev::/dev --mapdir .::. ./verify_attestation.wasm
