//! Time Travel Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeTravel {
    pub tt_id: String,
    pub travel_mechanisms: Vec<TimeTravelMechanism>,
    pub paradox_constraints: Vec<ParadoxConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeTravelMechanism {
    Wormhole { stable: bool },
    CosmicString { tension: f64 },
    AlcubierreDrive { expansion_rate: f64 },
    TiplerCylinder { rotation_rate: f64 },
    QuantumTunneling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParadoxConstraint {
    pub constraint_type: ConstraintType,
    pub self_consistency: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    GrandfatherParadox,
    BootstrapParadox,
    ClosedTimelikeCurve,
    NovikovSelfConsistency,
}

impl TimeTravel {
    pub fn new() -> Self {
        Self {
            tt_id: String::from("time_travel_v1"),
            travel_mechanisms: vec![],
            paradox_constraints: vec![],
        }
    }
}

impl Default for TimeTravel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_travel_creation() {
        let travel = TimeTravel::new();
        assert_eq!(travel.tt_id, "time_travel_v1");
    }
}
