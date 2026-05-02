//! Quantum Phase Transition Module (560)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumPhaseTransition {
    pub qpt_id: String,
    pub transition_type: TransitionType,
    pub critical_exponent: f64,
    pub order_parameter: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionType {
    SecondOrder,
    FirstOrder,
    Continuous,
    KosterlitzThouless,
}

impl QuantumPhaseTransition {
    pub fn new() -> Self {
        Self {
            qpt_id: String::from("quantum_phase_transition_v1"),
            transition_type: TransitionType::Continuous,
            critical_exponent: 0.5,
            order_parameter: 0.0,
        }
    }
}

impl Default for QuantumPhaseTransition {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_qpt() {
        let qpt = QuantumPhaseTransition::new();
        assert!(qpt.critical_exponent > 0.0);
    }
}
