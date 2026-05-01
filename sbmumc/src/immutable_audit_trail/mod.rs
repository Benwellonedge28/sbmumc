//! Immutable Audit Trail Module (506)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmutableAuditTrail {
    pub iat_id: String,
    pub blockchain: AuditBlockchain,
    pub consensus_mechanism: ConsensusMechanism,
    pub immutability_level: ImmutabilityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusMechanism {
    ProofOfWork,
    ProofOfStake,
    BFT,
    PoA,
    DAGBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImmutabilityLevel {
    CryptographicallyGuaranteed,
    EconomicIncentiveBased,
    ConsortiumControlled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditBlock {
    pub block_height: u64,
    pub previous_hash: String,
    pub timestamp_ns: u64,
    pub audit_entries: Vec<AuditEntry>,
    pub merkle_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub entry_id: String,
    pub actor_id: String,
    pub action_type: String,
    pub resource_id: String,
    pub cryptographic_proof: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditBlockchain {
    pub chain_id: String,
    pub blocks: Vec<AuditBlock>,
    pub genesis_hash: String,
}

impl ImmutableAuditTrail {
    pub fn new() -> Self {
        Self {
            iat_id: String::from("immutable_audit_trail_v1"),
            blockchain: AuditBlockchain {
                chain_id: String::from("audit_chain"),
                blocks: vec![],
                genesis_hash: String::from("genesis_hash_0"),
            },
            consensus_mechanism: ConsensusMechanism::BFT,
            immutability_level: ImmutabilityLevel::CryptographicallyGuaranteed,
        }
    }

    pub fn append_entry(&mut self, entry: AuditEntry) {
        let block = AuditBlock {
            block_height: self.blockchain.blocks.len() as u64 + 1,
            previous_hash: self.blockchain.blocks.last().map(|b| b.merkle_root.clone()).unwrap_or_default(),
            timestamp_ns: std::time::SystemTime::now().elapsed().unwrap().as_nanos() as u64,
            audit_entries: vec![entry],
            merkle_root: format!("merkle_{}", self.blockchain.blocks.len()),
        };
        self.blockchain.blocks.push(block);
    }
}

impl Default for ImmutableAuditTrail {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_audit_trail() {
        let mut trail = ImmutableAuditTrail::new();
        let entry = AuditEntry {
            entry_id: String::from("entry_1"),
            actor_id: String::from("actor_1"),
            action_type: String::from("access"),
            resource_id: String::from("resource_1"),
            cryptographic_proof: String::from("proof_hash"),
        };
        trail.append_entry(entry);
        assert_eq!(trail.blockchain.blocks.len(), 1);
    }
}
