//use aws_nitro_enclaves_cose as cose;
//use cose::CoseSign1;
//use openssl::{x509, stack};


use std::collections::{BTreeMap, BTreeSet};
use std::io::Error as IoError;
use std::result;
use serde;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use serde_cbor::error::Error as CborError;
use serde_cbor::{from_slice, to_vec};

#[derive(Debug)]
/// Possible error types return from this library.
pub enum Error {
    /// An IO error of type `std::io::Error`
    Io(IoError),
    /// A CBOR ser/de error of type `serde_cbor::error::Error`.
    Cbor(CborError),
}

/// Result type return nsm-io::Error on failure.
pub type Result<T> = result::Result<T, Error>;

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::Io(error)
    }
}

impl From<CborError> for Error {
    fn from(error: CborError) -> Self {
        Error::Cbor(error)
    }
}


/// The digest implementation used by a NitroSecureModule
#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum Digest {
    /// SHA256
    SHA256,
    /// SHA384
    SHA384,
    /// SHA512
    SHA512,
}

/// An attestation response.  This is also used for sealing
/// data.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AttestationDoc {
    /// Issuing NSM ID
    pub module_id: String,

    /// The digest function used for calculating the register values
    /// Can be: "SHA256" | "SHA512"
    pub digest: Digest,

    /// UTC time when document was created expressed as seconds since Unix Epoch
    pub timestamp: u64,

    /// Map of all locked PCRs at the moment the attestation document was generated
    pub pcrs: BTreeMap<usize, ByteBuf>,

    /// The infrastucture certificate used to sign the document, DER encoded
    pub certificate: ByteBuf,
    /// Issuing CA bundle for infrastructure certificate
    pub cabundle: Vec<ByteBuf>,

    /// An optional DER-encoded key the attestation consumer can use to encrypt data with
    pub public_key: Option<ByteBuf>,

    /// Additional signed user data, as defined by protocol.
    pub user_data: Option<ByteBuf>,

    /// An optional cryptographic nonce provided by the attestation consumer as a proof of
    /// authenticity.
    pub nonce: Option<ByteBuf>,
}

impl AttestationDoc {
    /// Creates a new AttestationDoc.
    ///
    /// # Arguments
    ///
    /// * module_id: a String representing the name of the NitroSecureModule
    /// * digest: nsm_io::Digest that describes what the PlatformConfigurationRegisters
    ///           contain
    /// * pcrs: BTreeMap containing the index to PCR value
    /// * certificate: the serialized certificate that will be used to sign this AttestationDoc
    /// * cabundle: the serialized set of certificates up to the root of trust certificate that
    ///             emitted `certificate`
    /// * user_data: optional user definted data included in the AttestationDoc
    /// * nonce: optional cryptographic nonce that will be included in the AttestationDoc
    /// * public_key: optional DER-encoded public key that will be included in the AttestationDoc
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        module_id: String,
        digest: Digest,
        timestamp: u64,
        pcrs: BTreeMap<usize, Vec<u8>>,
        certificate: Vec<u8>,
        cabundle: Vec<Vec<u8>>,
        user_data: Option<Vec<u8>>,
        nonce: Option<Vec<u8>>,
        public_key: Option<Vec<u8>>,
    ) -> Self {
        let mut pcrs_serialized = BTreeMap::new();

        for (i, pcr) in pcrs.into_iter() {
            let pcr = ByteBuf::from(pcr);
            pcrs_serialized.insert(i, pcr);
        }

        let cabundle_serialized = cabundle.into_iter().map(ByteBuf::from).collect();

        AttestationDoc {
            module_id,
            digest,
            timestamp,
            pcrs: pcrs_serialized,
            cabundle: cabundle_serialized,
            certificate: ByteBuf::from(certificate),
            user_data: user_data.map(ByteBuf::from),
            nonce: nonce.map(ByteBuf::from),
            public_key: public_key.map(ByteBuf::from),
        }
    }

    /// Helper function that converts an AttestationDoc structure to its CBOR representation
    pub fn to_binary(&self) -> Vec<u8> {
        // This should not fail
        to_vec(self).unwrap()
    }

    /// Helper function that parses a CBOR representation of an AttestationDoc and creates the
    /// structure from it, if possible.
    pub fn from_binary(bin: &[u8]) -> Result<Self> {
        from_slice(bin).map_err(Error::Cbor)
    }
}


fn main() {
	//let cose_doc = CoseSign1::from_bytes(&std::fs::read("data/attestation_doc").unwrap()).unwrap();
        //let payload = cose_doc.get_payload(None).unwrap();
        //let attestation_doc = AttestationDoc::from_binary(&payload).unwrap();
	/*let cert = x509::X509::from_der(&attestation_doc.certificate).unwrap();
	println!("checking signature...");
	let signature_valid = cose_doc.verify_signature(&cert.public_key().unwrap()).unwrap();
	assert!(signature_valid);
	println!("signature valid.");
	let mut builder = x509::store::X509StoreBuilder::new().unwrap();
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
	let root_cert = x509::X509::from_pem(root_cert.as_bytes()).unwrap();
	let _ = builder.add_cert(root_cert);
	let _ = builder.set_flags(x509::verify::X509VerifyFlags::NO_CHECK_TIME);
	let store = builder.build();
	let mut cabundle = stack::Stack::new().unwrap();
	for c in attestation_doc.cabundle[1..].iter() {
    let _ = cabundle.push(x509::X509::from_der(c).unwrap());
	}
	let mut ctx = x509::X509StoreContext::new().unwrap();
        println!("checking certificate path...");
	let cert_path_valid = ctx.init(&store, &cert, &cabundle, |x| x.verify_cert()).unwrap();
	assert!(cert_path_valid);
	println!("certificate path valid (ignoring time).");
	let public_key = &attestation_doc.public_key.unwrap();
	let public_key = std::str::from_utf8(public_key).unwrap();
	println!("public key: {:#?}",public_key);
        let pcr0 = &attestation_doc.pcrs[&0];
        let pcr0 = hex::encode(pcr0);
	println!("PCR0: {:#?}", pcr0);
        let extracted = format!("{}|{}", public_key, pcr0);
        let _ = std::fs::write("data/extracted.txt", extracted);*/
}
