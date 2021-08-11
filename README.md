# verify-nitro-attestation

1) put the attestation doc into `data/attestation_doc` in binary format. 
2) `cargo run`
3) if verification suceeds, public key is extracted into `data/public_key.txt` and hash of the `.eif` is put into `data/eif_hash.txt`
