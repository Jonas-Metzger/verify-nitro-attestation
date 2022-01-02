use aws_nitro_enclaves_cose as cose;
use nsm_io::AttestationDoc;
use cose::CoseSign1;
use openssl::x509::store::X509StoreBuilder;
use openssl::x509::{X509, X509StoreContext};
use openssl::stack::Stack;
use openssl::x509::verify::X509VerifyFlags;

fn main() {
	let cose_doc = CoseSign1::from_bytes(&std::fs::read("data/attestation_doc").unwrap()).unwrap();
        let payload = cose_doc.get_payload(None).unwrap();
        let attestation_doc = AttestationDoc::from_binary(&payload).unwrap();
        let cert = X509::from_der(&attestation_doc.certificate).unwrap();
	println!("checking signature...");
	let signature_valid = cose_doc.verify_signature(&cert.public_key().unwrap()).unwrap();
	assert!(signature_valid);
	println!("signature valid.");
	let mut builder = X509StoreBuilder::new().unwrap();
	let root_cert = "-----BEGIN CERTIFICATE-----
MIICETCCAZagAwIBAgIRAPkxdWgbkK/hHUbMtOTn+FYwCgYIKoZIzj0EAwMwSTEL
MAkGA1UEBhMCVVMxDzANBgNVBAoMBkFtYXpvbjEMMAoGA1UECwwDQVdTMRswGQYD
VQQDDBJhd3Mubml0cm8tZW5jbGF2ZXMwHhcNMTkxMDI4MTMyODA1WhcNNDkxMDI4
MTQyODA1WjBJMQswCQYDVQQGEwJVUzEPMA0GA1UECgwGQW1hem9uMQwwCgYDVQQL
DANBV1MxGzAZBgNVBAMMEmF3cy5uaXRyby1lbmNsYXZlczB2MBAGByqGSM49AgEG
BSuBBAAiA2IABPwCVOumCMHzaHDimtqQvkY4MpJzbolL//Zy2YlES1BR5TSksfbb
48C8WBoyt7F2Bw7eEtaaP+ohG2bnUs990d0JX28TcPQXCEPZ3BABIeTPYwEoCWZE
h8l5YoQwTcU/9KNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUkCW1DdkF
R+eWw5b6cp3PmanfS5YwDgYDVR0PAQH/BAQDAgGGMAoGCCqGSM49BAMDA2kAMGYC
MQCjfy+Rocm9Xue4YnwWmNJVA44fA0P5W2OpYow9OYCVRaEevL8uO1XYru5xtMPW
rfMCMQCi85sWBbJwKKXdS6BptQFuZbT73o/gBh1qUxl/nNr12UO8Yfwr6wPLb+6N
IwLz3/Y=
-----END CERTIFICATE-----";
	let root_cert = X509::from_pem(root_cert.as_bytes()).unwrap();
	let _ = builder.add_cert(root_cert);
	let _ = builder.set_flags(X509VerifyFlags::NO_CHECK_TIME);
	let store = builder.build();
	let mut cabundle = Stack::new().unwrap();
	for c in attestation_doc.cabundle[1..].iter() {
		let _ = cabundle.push(X509::from_der(c).unwrap());
	}
	let mut ctx = X509StoreContext::new().unwrap();
        println!("checking certificate path...");
	let cert_path_valid = ctx.init(&store, &cert, &cabundle, |x| x.verify_cert()).unwrap();
        println!("valid: {:#?}",cert_path_valid);
	assert!(cert_path_valid);
	println!("certificate path valid (ignoring time).");
	let public_key = &attestation_doc.public_key.unwrap();
	let public_key = std::str::from_utf8(public_key).unwrap();
	println!("public key: {:#?}",public_key);
        let pcr0 = &attestation_doc.pcrs[&0];
        let pcr0 = hex::encode(pcr0);
	println!("PCR0: {:#?}", pcr0);
        let extracted = format!("{}|{}", public_key, pcr0);
        let _ = std::fs::write("data/extracted.txt", extracted);
}
