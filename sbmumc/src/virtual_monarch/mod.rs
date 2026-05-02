//! Virtual Monarch Module (630)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMonarch {
    pub vm_id: String,
    pub dynasty: String,
    pub reign_duration_years: u32,
}

impl VirtualMonarch {
    pub fn new() -> Self {
        Self {
            vm_id: String::from("virtual_monarch_v1"),
            dynasty: String::from("Digital Dynasty"),
            reign_duration_years: 50,
        }
    }
}

impl Default for VirtualMonarch {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_monarch() {
        let m = VirtualMonarch::new();
        assert!(m.reign_duration_years > 0);
    }
}
