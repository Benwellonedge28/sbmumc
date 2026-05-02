//! Virtual Voting Module (610)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualVoting {
    pub vv_id: String,
    pub voting_system: String,
    pub anonymity_level: f64,
}

impl VirtualVoting {
    pub fn new() -> Self {
        Self {
            vv_id: String::from("virtual_voting_v1"),
            voting_system: String::from("quadratic"),
            anonymity_level: 0.99,
        }
    }
}

impl Default for VirtualVoting {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_voting() {
        let v = VirtualVoting::new();
        assert!(v.anonymity_level > 0.95);
    }
}
