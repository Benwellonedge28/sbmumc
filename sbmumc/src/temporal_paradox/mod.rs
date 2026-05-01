//! Temporal Paradox Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalParadox {
    pub tp_id: String,
    pub paradox_types: Vec<ParadoxType>,
    pub resolution_mechanisms: Vec<ResolutionMechanism>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParadoxType {
    GrandfatherParadox,
    BootstrapParadox,
    PolchinskiParadox,
    ConsistencyParadox,
    InformationParadox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionMechanism {
    pub mechanism_type: MechanismType,
    pub self_consistency_requirement: bool,
    pub physics_constraint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MechanismType {
    NovikovSelfConsistency,
    ManyWorldsInterpretation,
    ChronologyProtectionConjecture,
    EntanglementSelection,
    QuantumForbiddenRegions,
}

impl TemporalParadox {
    pub fn new() -> Self {
        Self {
            tp_id: String::from("temporal_paradox_v1"),
            paradox_types: vec![],
            resolution_mechanisms: vec![],
        }
    }
}

impl Default for TemporalParadox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_paradox_creation() {
        let paradox = TemporalParadox::new();
        assert_eq!(paradox.tp_id, "temporal_paradox_v1");
    }
}
