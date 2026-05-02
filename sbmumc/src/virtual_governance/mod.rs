//! Virtual Governance Module (602)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualGovernance {
    pub vg_id: String,
    pub governance_model: GovernanceModel,
    pub voter_turnout: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceModel {
    DAODemocracy,
    Republic,
    Monarchy,
    Technocracy,
}

impl VirtualGovernance {
    pub fn new() -> Self {
        Self {
            vg_id: String::from("virtual_governance_v1"),
            governance_model: GovernanceModel::DAODemocracy,
            voter_turnout: 0.75,
        }
    }
}

impl Default for VirtualGovernance {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_governance() {
        let g = VirtualGovernance::new();
        assert!(g.voter_turnout > 0.5);
    }
}
