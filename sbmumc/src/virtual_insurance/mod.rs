//! Virtual Insurance Module (608)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualInsurance {
    pub vi_id: String,
    pub coverage_types: Vec<String>,
    pub premium_rate: f64,
}

impl VirtualInsurance {
    pub fn new() -> Self {
        Self {
            vi_id: String::from("virtual_insurance_v1"),
            coverage_types: vec![String::from("life"), String::from("property")],
            premium_rate: 0.02,
        }
    }
}

impl Default for VirtualInsurance {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_insurance() {
        let i = VirtualInsurance::new();
        assert!(i.premium_rate < 1.0);
    }
}
