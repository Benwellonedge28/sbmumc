//! Decentralized Identity Module (530)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecentralizedIdentity {
    pub di_id: String,
    pub did_method: DidMethod,
    pub blockchain: String,
    pub key_management: KeyManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DidMethod {
    Web,
    Ethr,
    Sidetree,
    Key,
    Peer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyManagement {
    HierarchicalDeterministic,
    MultiSignature,
    ThresholdSignature,
    HardwareWallet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDDocument {
    pub did: String,
    pub public_keys: Vec<PublicKey>,
    pub authentication: Vec<AuthenticationMethod>,
    pub service_endpoints: Vec<ServiceEndpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKey {
    pub key_id: String,
    pub key_type: String,
    pub key_data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationMethod {
    pub method_type: String,
    pub key_reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub service_id: String,
    pub service_type: String,
    pub endpoint_url: String,
}

impl DecentralizedIdentity {
    pub fn new() -> Self {
        Self {
            di_id: String::from("decentralized_identity_v1"),
            did_method: DidMethod::Web,
            blockchain: String::from("ethereum"),
            key_management: KeyManagement::HierarchicalDeterministic,
        }
    }

    pub fn create_did(&self, id: &str) -> DIDDocument {
        DIDDocument {
            did: format!("did:web:{}", id),
            public_keys: vec![
                PublicKey {
                    key_id: String::from("key1"),
                    key_type: String::from("EcdsaSecp256k1VerificationKey2019"),
                    key_data: vec![0u8; 64],
                }
            ],
            authentication: vec![
                AuthenticationMethod {
                    method_type: String::from("authentication"),
                    key_reference: String::from("key1"),
                }
            ],
            service_endpoints: vec![],
        }
    }
}

impl Default for DecentralizedIdentity {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decentralized_identity() {
        let di = DecentralizedIdentity::new();
        let doc = di.create_did("user123");
        assert!(doc.did.starts_with("did:web:"));
    }
}
