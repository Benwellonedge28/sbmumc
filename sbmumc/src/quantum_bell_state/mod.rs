//! Quantum Bell State Module (582)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumBellState {
    pub qbs_id: String,
    pub bell_state_type: BellStateType,
    pub correlation_strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BellStateType {
    PhiPlus,
    PhiMinus,
    PsiPlus,
    PsiMinus,
}

impl QuantumBellState {
    pub fn new() -> Self {
        Self {
            qbs_id: String::from("quantum_bell_state_v1"),
            bell_state_type: BellStateType::PhiPlus,
            correlation_strength: 2.0,
        }
    }
}

impl Default for QuantumBellState {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_bell_state() {
        let b = QuantumBellState::new();
        assert!(b.correlation_strength > 1.0);
    }
}
