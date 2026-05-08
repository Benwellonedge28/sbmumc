//! Breeder Reactor Module (745)
//!
//! Breeders, fast reactors, and fuel breeding technologies.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreederReactor {
    pub reactor_id: String,
    pub breeding_ratio: f64,
    pub conversion_ratio: f64,
    pub blanket_material: String,
    pub fuel_type: String,
    pub breeding_gain: f64,
    pub operational_status: String,
}

impl BreederReactor {
    pub fn new(reactor_id: String) -> Self {
        Self {
            reactor_id,
            breeding_ratio: 0.0,
            conversion_ratio: 0.0,
            blanket_material: "U-238".into(),
            fuel_type: "MOX".into(),
            breeding_gain: 0.0,
            operational_status: "Conceptual".into(),
        }
    }

    pub fn is_breeder(&self) -> bool {
        self.breeding_ratio > 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_breeder() {
        let reactor = BreederReactor::new("BR-001".into());
        assert_eq!(reactor.reactor_id, "BR-001");
    }
}
