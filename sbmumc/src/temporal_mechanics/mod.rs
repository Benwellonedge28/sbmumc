//! Temporal Mechanics Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalMechanics {
    pub tm_id: String,
    pub dynamic_systems: Vec<TemporalDynamicalSystem>,
    pub flow_characteristics: FlowCharacteristics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalDynamicalSystem {
    pub system_id: String,
    pub equations: Vec<String>,
    pub attractors: Vec<TemporalAttractor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalAttractor {
    pub attractor_type: AttractorType,
    pub basin_of_attraction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttractorType {
    FixedPoint,
    LimitCycle,
    Chaotic,
    Quasiperiodic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowCharacteristics {
    pub directionality: Directionality,
    pub measurability: Measurability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Directionality {
    Forward,
    Backward,
    Bidirectional,
    Acyclic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Measurability {
    Absolute,
    Relative,
    Conventional,
}

impl TemporalMechanics {
    pub fn new() -> Self {
        Self {
            tm_id: String::from("temporal_mechanics_v1"),
            dynamic_systems: vec![],
            flow_characteristics: FlowCharacteristics {
                directionality: Directionality::Forward,
                measurability: Measurability::Relative,
            },
        }
    }
}

impl Default for TemporalMechanics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_mechanics_creation() {
        let mechanics = TemporalMechanics::new();
        assert_eq!(mechanics.tm_id, "temporal_mechanics_v1");
    }
}
