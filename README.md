# verify-nitro-attestation
To build:
1) install wasienv, rustup and run `rustup target add wasm32-wasi`
2) run `sh build.sh`

To run:
1) Put the binary `attestation_doc` file and `run.sh` into the same folder as `verify_attestaion.wasm`
2) Run `sh run.sh`
3) if the verification suceeds,  `public_key` and `image_hash` are extracted from the attestation doc and put into the same folder.
