//! # SBMUMC Module 1252: Wetland Conservation
//!
//! Protection and restoration of wetland ecosystems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WetlandType {
    Marsh,
    Swamp,
    Bog,
    Fen,
    Mangrove,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WetlandConservationFramework {
    pub framework_id: String,
    pub wetland_type: WetlandType,
    pub biodiversity_value: f64,
    pub carbon_storage: f64,
    pub water_regulation: f64,
    pub restoration_potential: f64,
}

impl WetlandConservationFramework {
    pub fn new(wetland_type: WetlandType) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            wetland_type,
            biodiversity_value: 0.0,
            carbon_storage: 0.0,
            water_regulation: 0.0,
            restoration_potential: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.wetland_type {
            WetlandType::Marsh => {
                self.biodiversity_value = 0.85 + rand_simple() * 0.14;
                self.water_regulation = 0.80 + rand_simple() * 0.18;
            },
            WetlandType::Swamp => {
                self.biodiversity_value = 0.80 + rand_simple() * 0.18;
                self.carbon_storage = 0.75 + rand_simple() * 0.22;
            },
            WetlandType::Bog => {
                self.carbon_storage = 0.90 + rand_simple() * 0.10;
                self.water_regulation = 0.70 + rand_simple() * 0.25;
                self.biodiversity_value = 0.70 + rand_simple() * 0.25;
            },
            WetlandType::Fen => {
                self.carbon_storage = 0.85 + rand_simple() * 0.14;
                self.biodiversity_value = 0.75 + rand_simple() * 0.22;
            },
            WetlandType::Mangrove => {
                self.carbon_storage = 0.85 + rand_simple() * 0.14;
                self.biodiversity_value = 0.80 + rand_simple() * 0.18;
                self.water_regulation = 0.75 + rand_simple() * 0.22;
                self.restoration_potential = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.restoration_potential == 0.0 {
            self.restoration_potential = (self.biodiversity_value + self.carbon_storage) / 2.0 * (0.6 + rand_simple() * 0.3);
        }
        Ok(())
    }
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mangrove_conservation() {
        let mut framework = WetlandConservationFramework::new(WetlandType::Mangrove);
        framework.analyze_framework().unwrap();
        assert!(framework.carbon_storage > 0.6);
    }
}