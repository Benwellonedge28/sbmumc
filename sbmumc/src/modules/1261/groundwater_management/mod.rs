//! # SBMUMC Module 1261: Groundwater Management
//!
//! Planning and sustainability of underground water resources.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroundwaterChallenge {
    OverExtraction,
    Contamination,
    SeawaterIntrusion,
    RechargeDecline,
    AquiferDepletion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundwaterManagementSystem {
    pub system_id: String,
    pub groundwater_challenge: GroundwaterChallenge,
    pub sustainability_index: f64,
    pub extraction_management: f64,
    pub quality_protection: f64,
    pub recharge_promotion: f64,
}

impl GroundwaterManagementSystem {
    pub fn new(groundwater_challenge: GroundwaterChallenge) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            groundwater_challenge,
            sustainability_index: 0.0,
            extraction_management: 0.0,
            quality_protection: 0.0,
            recharge_promotion: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.groundwater_challenge {
            GroundwaterChallenge::OverExtraction => {
                self.extraction_management = 0.70 + rand_simple() * 0.25;
                self.sustainability_index = 0.60 + rand_simple() * 0.35;
            },
            GroundwaterChallenge::Contamination => {
                self.quality_protection = 0.75 + rand_simple() * 0.22;
                self.sustainability_index = 0.65 + rand_simple() * 0.30;
            },
            GroundwaterChallenge::SeawaterIntrusion => {
                self.quality_protection = 0.80 + rand_simple() * 0.18;
                self.extraction_management = 0.70 + rand_simple() * 0.25;
            },
            GroundwaterChallenge::RechargeDecline => {
                self.recharge_promotion = 0.75 + rand_simple() * 0.22;
                self.sustainability_index = 0.65 + rand_simple() * 0.30;
            },
            GroundwaterChallenge::AquiferDepletion => {
                self.extraction_management = 0.65 + rand_simple() * 0.30;
                self.recharge_promotion = 0.70 + rand_simple() * 0.25;
                self.sustainability_index = 0.55 + rand_simple() * 0.40;
            },
        }

        if self.recharge_promotion == 0.0 {
            self.recharge_promotion = (self.extraction_management + self.quality_protection) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_seawater_intrusion() {
        let mut system = GroundwaterManagementSystem::new(GroundwaterChallenge::SeawaterIntrusion);
        system.analyze_system().unwrap();
        assert!(system.quality_protection > 0.6);
    }
}