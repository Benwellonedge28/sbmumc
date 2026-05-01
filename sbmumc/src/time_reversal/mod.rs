//! Time Reversal Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeReversal {
    pub tr_id: String,
    pub reversal_symmetry: ReversalSymmetry,
    pub t_symmetric_processes: Vec<TSymmetricProcess>,
    pub t_violating_processes: Vec<TViolatingProcess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReversalSymmetry {
    pub exact_symmetry: bool,
    pub approximate_symmetry: bool,
    pub symmetry_breaking_scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSymmetricProcess {
    pub process_name: String,
    pub fundamental_laws: bool,
    pub macroscopic_analogs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TViolatingProcess {
    pub process_name: String,
    pub violation_source: ViolationSource,
    pub kaon_system: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationSource {
    WeakForce,
    NeutralMesons,
    CPViolation,
    CurvatureEffects,
}

impl TimeReversal {
    pub fn new() -> Self {
        Self {
            tr_id: String::from("time_reversal_v1"),
            reversal_symmetry: ReversalSymmetry {
                exact_symmetry: false,
                approximate_symmetry: true,
                symmetry_breaking_scale: 0.0,
            },
            t_symmetric_processes: vec![],
            t_violating_processes: vec![],
        }
    }
}

impl Default for TimeReversal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_reversal_creation() {
        let reversal = TimeReversal::new();
        assert!(!reversal.reversal_symmetry.exact_symmetry);
    }
}
