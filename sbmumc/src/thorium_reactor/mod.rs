//! Thorium Reactor Module (746)
//!
//! Thorium-based nuclear fuel cycles and liquid fluoride reactors.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThoriumCycle {
    ThoriumUranium233,
    ThoriumPlutonium,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoriumReactor {
    pub reactor_id: String,
    pub thorium_cycle: ThoriumCycle,
    pub thorium_reserves_tonnes: f64,
    pub fuel_form: String,
    pub conversion_ratio: f64,
    pub waste_radioactivity: f64,
    pub proliferation_resistance: f64,
}

impl ThoriumReactor {
    pub fn new(reactor_id: String) -> Self {
        Self {
            reactor_id,
            thorium_cycle: ThoriumCycle::ThoriumUranium233,
            thorium_reserves_tonnes: 0.0,
            fuel_form: "FLiBe".into(),
            conversion_ratio: 0.0,
            waste_radioactivity: 0.0,
            proliferation_resistance: 0.0,
        }
    }

    pub fn sustainability_score(&self) -> f64 {
        (self.proliferation_resistance + 100.0 - self.waste_radioactivity) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_thorium() {
        let reactor = ThoriumReactor::new("TH-001".into());
        assert_eq!(reactor.reactor_id, "TH-001");
    }
}
