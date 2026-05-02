//! Virtual Alliance Module (638)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualAlliance {
    pub va_id: String,
    pub alliance_members: u32,
    pub mutual_defense_pact: bool,
}

impl VirtualAlliance {
    pub fn new() -> Self {
        Self {
            va_id: String::from("virtual_alliance_v1"),
            alliance_members: 30,
            mutual_defense_pact: true,
        }
    }
}

impl Default for VirtualAlliance {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_alliance() {
        let a = VirtualAlliance::new();
        assert!(a.mutual_defense_pact);
    }
}
