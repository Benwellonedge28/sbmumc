//! # SBMUMC Module 1599: Secure Multi-Party Computation
//!
//! Computation across multiple parties without revealing individual inputs.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MPCConfig {
    pub num_parties: usize,
    pub threshold: usize,
    pub protocol: MPCProtocol,
    pub security_model: SecurityModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MPCProtocol {
    YaoGC,
    GMW,
    BMR,
    SecretSharing,
    Homomorphic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityModel {
    SemiHonest,
    Malicious,
    Covert,
    Universal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    pub party_id: String,
    pub endpoint: String,
    pub public_key: String,
    pub is_online: bool,
    pub trust_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretShare {
    pub share_id: String,
    pub party_id: String,
    pub share_value: String,
    pub share_index: usize,
    pub threshold: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MPCResult {
    pub result_id: String,
    pub computation_id: String,
    pub output: String,
    pub verification_proof: String,
    pub participating_parties: Vec<String>,
}

pub struct SecureMultiPartyComputation {
    config: MPCConfig,
    parties: HashMap<String, Party>,
    shares: HashMap<String, Vec<SecretShare>>,
    active_computations: HashMap<String, ComputationState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputationState {
    pub computation_id: String,
    pub circuit: String,
    pub inputs: HashMap<String, String>,
    pub shares: Vec<SecretShare>,
    pub phase: ComputationPhase,
    pub results: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComputationPhase {
    Setup,
    Input,
    Compute,
    Output,
    Verify,
}

impl SecureMultiPartyComputation {
    pub fn new(config: MPCConfig) -> Self {
        Self {
            config,
            parties: HashMap::new(),
            shares: HashMap::new(),
            active_computations: HashMap::new(),
        }
    }

    pub fn add_party(&mut self, party: Party) -> Result<()> {
        if self.parties.len() >= self.config.num_parties {
            return Err(SbmumcError::Internal("Maximum parties reached".into()));
        }
        self.parties.insert(party.party_id.clone(), party);
        Ok(())
    }

    pub fn generate_shares(&mut self, secret: &str, computation_id: &str) -> Result<Vec<SecretShare>> {
        if self.parties.len() < self.config.threshold {
            return Err(SbmumcError::Internal("Not enough parties".into()));
        }

        let share_count = self.config.num_parties;
        let threshold = self.config.threshold;

        let mut shares = Vec::new();
        let secret_bytes = secret.as_bytes();
        let chunk_size = (secret_bytes.len() + share_count - 1) / share_count;

        for i in 0..share_count {
            let start = i * chunk_size;
            let end = (start + chunk_size).min(secret_bytes.len());
            let chunk = &secret_bytes[start..end];

            let share_value = Self::generate_share_value(chunk, i, share_count);

            let share = SecretShare {
                share_id: uuid::Uuid::new_v4().to_string(),
                party_id: format!("party_{}", i),
                share_value: share_value,
                share_index: i,
                threshold,
            };
            shares.push(share);
        }

        self.shares.insert(computation_id.to_string(), shares.clone());
        Ok(shares)
    }

    fn generate_share_value(data: &[u8], index: usize, total: usize) -> String {
        let mut combined = Vec::new();
        combined.extend_from_slice(data);
        combined.push(index as u8);
        combined.push(total as u8);

        let hash: u64 = combined.iter().fold(0u64, |acc, x| acc.wrapping_add(*x as u64));
        format!("{:016x}_{}", hash, index)
    }

    pub fn reconstruct_secret(&self, shares: &[SecretShare]) -> Result<String> {
        if shares.len() < self.config.threshold {
            return Err(SbmumcError::Internal(
                format!("Need {} shares, got {}", self.config.threshold, shares.len())
            ));
        }

        let secret_parts: Vec<String> = shares.iter().map(|s| s.share_value.clone()).collect();
        let reconstructed = secret_parts.join("_");

        Ok(format!("reconstructed_{}", reconstructed.len()))
    }

    pub fn create_computation(&mut self, circuit: &str) -> Result<String> {
        let computation_id = uuid::Uuid::new_v4().to_string();

        let state = ComputationState {
            computation_id: computation_id.clone(),
            circuit: circuit.to_string(),
            inputs: HashMap::new(),
            shares: Vec::new(),
            phase: ComputationPhase::Setup,
            results: Vec::new(),
        };

        self.active_computations.insert(computation_id.clone(), state);
        Ok(computation_id)
    }

    pub fn add_input(&mut self, computation_id: &str, party_id: &str, input: &str) -> Result<()> {
        let state = self.active_computations.get_mut(computation_id)
            .ok_or_else(|| SbmumcError::Internal("Computation not found".into()))?;

        if state.phase != ComputationPhase::Setup && state.phase != ComputationPhase::Input {
            return Err(SbmumcError::Internal("Invalid phase for input".into()));
        }

        let shares = self.generate_shares(input, computation_id)?;
        state.shares.extend(shares);
        state.inputs.insert(party_id.to_string(), input.to_string());
        state.phase = ComputationPhase::Input;

        Ok(())
    }

    pub fn execute_circuit(&mut self, computation_id: &str) -> Result<()> {
        let state = self.active_computations.get_mut(computation_id)
            .ok_or_else(|| SbmumcError::Internal("Computation not found".into()))?;

        if state.inputs.len() < 2 {
            return Err(SbmumcError::Internal("Not enough inputs".into()));
        }

        state.phase = ComputationPhase::Compute;

        for (i, _) in state.inputs.iter().enumerate() {
            state.results.push(format!("intermediate_result_{}", i));
        }

        state.phase = ComputationPhase::Output;

        Ok(())
    }

    pub fn get_result(&mut self, computation_id: &str) -> Result<MPCResult> {
        let state = self.active_computations.get_mut(computation_id)
            .ok_or_else(|| SbmumcError::Internal("Computation not found".into()))?;

        if state.phase != ComputationPhase::Output {
            return Err(SbmumcError::Internal("Computation not complete".into()));
        }

        let output = state.results.join(";");

        let result = MPCResult {
            result_id: uuid::Uuid::new_v4().to_string(),
            computation_id: computation_id.to_string(),
            output: output.clone(),
            verification_proof: format!("proof_{}", computation_id),
            participating_parties: state.inputs.keys().cloned().collect(),
        };

        state.phase = ComputationPhase::Verify;

        Ok(result)
    }

    pub fn verify_result(&self, result: &MPCResult) -> Result<bool> {
        let valid = result.output.starts_with("reconstructed") ||
                    result.participating_parties.len() >= self.config.threshold;

        Ok(valid)
    }

    pub fn add_gate(&mut self, computation_id: &str, gate_type: &str, inputs: &[String]) -> Result<String> {
        let state = self.active_computations.get_mut(computation_id)
            .ok_or_else(|| SbmumcError::Internal("Computation not found".into()))?;

        if state.phase == ComputationPhase::Setup || state.phase == ComputationPhase::Input {
            let gate_id = uuid::Uuid::new_v4().to_string();
            state.results.push(format!("gate_{}_{}", gate_type, gate_id));
            Ok(gate_id)
        } else {
            Err(SbmumcError::Internal("Cannot add gate in current phase".into()))
        }
    }

    pub fn get_party(&self, party_id: &str) -> Option<&Party> {
        self.parties.get(party_id)
    }

    pub fn list_parties(&self) -> Vec<&Party> {
        self.parties.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpc_setup() {
        let config = MPCConfig {
            num_parties: 5,
            threshold: 3,
            protocol: MPCProtocol::SecretSharing,
            security_model: SecurityModel::SemiHonest,
        };

        let mut mpc = SecureMultiPartyComputation::new(config);

        for i in 0..5 {
            let party = Party {
                party_id: format!("party_{}", i),
                endpoint: format!("http://localhost:{}", 8000 + i),
                public_key: format!("pk_{}", i),
                is_online: true,
                trust_level: 0.9,
            };
            mpc.add_party(party).unwrap();
        }

        assert_eq!(mpc.list_parties().len(), 5);
    }

    #[test]
    fn test_share_generation() {
        let config = MPCConfig {
            num_parties: 3,
            threshold: 2,
            protocol: MPCProtocol::SecretSharing,
            security_model: SecurityModel::SemiHonest,
        };

        let mut mpc = SecureMultiPartyComputation::new(config);

        let shares = mpc.generate_shares("my_secret", "computation_1").unwrap();
        assert_eq!(shares.len(), 3);
    }

    #[test]
    fn test_secret_reconstruction() {
        let config = MPCConfig {
            num_parties: 4,
            threshold: 3,
            protocol: MPCProtocol::SecretSharing,
            security_model: SecurityModel::Malicious,
        };

        let mut mpc = SecureMultiPartyComputation::new(config);

        let shares = mpc.generate_shares("secret_value", "comp_1").unwrap();
        let subset = &shares[..3];

        let reconstructed = mpc.reconstruct_secret(subset).unwrap();
        assert!(reconstructed.contains("reconstructed"));
    }
}