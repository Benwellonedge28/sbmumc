//! Causality Loops Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalityLoops {
    pub cl_id: String,
    pub loop_types: Vec<LoopType>,
    pub self_causing_events: Vec<SelfCausingEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoopType {
    CausalLoop,
    InformationLoop,
    ObjectLoop,
    ContradictoryLoop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfCausingEvent {
    pub event_id: String,
    pub loop_structure: LoopStructure,
    pub information_origin: InformationOrigin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopStructure {
    pub closed_timelike_curves: bool,
    pub self_consistent: bool,
    pub paradox_free: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InformationOrigin {
    FutureSelf,
    ExternalInfluence,
    Uncaused,
    QuantumDeterministic,
}

impl CausalityLoops {
    pub fn new() -> Self {
        Self {
            cl_id: String::from("causality_loops_v1"),
            loop_types: vec![],
            self_causing_events: vec![],
        }
    }
}

impl Default for CausalityLoops {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_causality_loops_creation() {
        let loops = CausalityLoops::new();
        assert_eq!(loops.cl_id, "causality_loops_v1");
    }
}
