//! Quantum Topological Module (559)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumTopological {
    pub qt_id: String,
    pub topological_order: TopologicalOrder,
    pub anyon_type: AnyonType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopologicalOrder {
    FractionalQuantumHall,
    SpinLiquid,
    TopologicalInsulator,
    TopologicalSuperconductor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnyonType {
    Abelian,
    NonAbelian,
}

impl QuantumTopological {
    pub fn new() -> Self {
        Self {
            qt_id: String::from("quantum_topological_v1"),
            topological_order: TopologicalOrder::FractionalQuantumHall,
            anyon_type: AnyonType::NonAbelian,
        }
    }
}

impl Default for QuantumTopological {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_topological() {
        let t = QuantumTopological::new();
        assert!(matches!(t.anyon_type, AnyonType::NonAbelian));
    }
}
