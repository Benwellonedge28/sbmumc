//! # SBMUMC Module 1288: Fisheries Ocean
//!
//! Systems for sustainable ocean fisheries management.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FisheryManagementType {
    QuotaManagement,
    SpatialClosures,
    GearRestrictions,
    CommunityManagement,
    EcosystemBased,
    ClimateAdaptive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FisheriesOceanSystem {
    pub system_id: String,
    pub management_type: FisheryManagementType,
    pub stock_health: f64,
    pub harvest_sustainability: f64,
    pub economic_viability: f64,
    pub ecosystem_impact: f64,
}

impl FisheriesOceanSystem {
    pub fn new(management_type: FisheryManagementType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            management_type,
            stock_health: 0.0,
            harvest_sustainability: 0.0,
            economic_viability: 0.0,
            ecosystem_impact: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.management_type {
            FisheryManagementType::QuotaManagement => {
                self.harvest_sustainability = 0.85 + rand_simple() * 0.14;
                self.stock_health = 0.80 + rand_simple() * 0.18;
                self.economic_viability = 0.75 + rand_simple() * 0.22;
            },
            FisheryManagementType::SpatialClosures => {
                self.stock_health = 0.90 + rand_simple() * 0.10;
                self.ecosystem_impact = 0.85 + rand_simple() * 0.14;
                self.harvest_sustainability = 0.80 + rand_simple() * 0.18;
            },
            FisheryManagementType::GearRestrictions => {
                self.ecosystem_impact = 0.80 + rand_simple() * 0.18;
                self.harvest_sustainability = 0.75 + rand_simple() * 0.22;
                self.economic_viability = 0.70 + rand_simple() * 0.25;
            },
            FisheryManagementType::CommunityManagement => {
                self.economic_viability = 0.85 + rand_simple() * 0.14;
                self.stock_health = 0.80 + rand_simple() * 0.18;
                self.harvest_sustainability = 0.75 + rand_simple() * 0.22;
            },
            FisheryManagementType::EcosystemBased => {
                self.ecosystem_impact = 0.90 + rand_simple() * 0.10;
                self.stock_health = 0.85 + rand_simple() * 0.14;
                self.harvest_sustainability = 0.80 + rand_simple() * 0.18;
            },
            FisheryManagementType::ClimateAdaptive => {
                self.stock_health = 0.75 + rand_simple() * 0.22;
                self.harvest_sustainability = 0.70 + rand_simple() * 0.25;
                self.economic_viability = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.ecosystem_impact == 0.0 {
            self.ecosystem_impact = (self.stock_health + self.harvest_sustainability) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_spatial_closures() {
        let mut system = FisheriesOceanSystem::new(FisheryManagementType::SpatialClosures);
        system.analyze_system().unwrap();
        assert!(system.stock_health > 0.7);
    }
}
