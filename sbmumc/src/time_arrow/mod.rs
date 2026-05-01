//! Time Arrow Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeArrow {
    pub ta_id: String,
    pub arrow_types: Vec<ArrowType>,
    pub thermodynamic_arrow: ThermodynamicArrow,
    pub cosmological_arrow: CosmologicalArrow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrowType {
    Thermodynamic,
    Cosmological,
    Electromagnetic,
    Psychological,
    Quantum,
    Causal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermodynamicArrow {
    pub entropy_direction: EntropyDirection,
    pub second_law_valid: bool,
    pub initial_conditions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntropyDirection {
    Increasing,
    Decreasing,
    Stationary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmologicalArrow {
    pub expansion_direction: ExpansionDirection,
    pub universe_age: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpansionDirection {
    Accelerating,
    Decelerating,
    Constant,
}

impl TimeArrow {
    pub fn new() -> Self {
        Self {
            ta_id: String::from("time_arrow_v1"),
            arrow_types: vec![],
            thermodynamic_arrow: ThermodynamicArrow {
                entropy_direction: EntropyDirection::Increasing,
                second_law_valid: true,
                initial_conditions: String::new(),
            },
            cosmological_arrow: CosmologicalArrow {
                expansion_direction: ExpansionDirection::Accelerating,
                universe_age: 0.0,
            },
        }
    }
}

impl Default for TimeArrow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_arrow_creation() {
        let arrow = TimeArrow::new();
        assert!(arrow.thermodynamic_arrow.second_law_valid);
    }
}
