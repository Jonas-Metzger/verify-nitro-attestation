#!/bin/bash
wasmer run --enable-all --dir /dev --dir . target/wasm32-wasi/debug/verify_attestation.wasm
# access to /dev/urandom is required because of https://stackoverflow.com/questions/61235285/why-does-ecc-signature-verification-need-random-numbers-sometimes-taking-a-long
# fake randomness is fine
