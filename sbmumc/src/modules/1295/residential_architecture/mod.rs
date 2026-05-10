//! # SBMUMC Module 1295: Residential Architecture
//!
//! Systems for housing and residential building design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResidentialType {
    SingleFamily,
    MultiFamily,
    Cohousing,
    TinyHomes,
    ModularConstruction,
    AffordableHousing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResidentialArchitectureSystem {
    pub system_id: String,
    pub residential_type: ResidentialType,
    pub spatial_efficiency: f64,
    pub cost_efficiency: f64,
    pub occupant_satisfaction: f64,
    pub sustainability_rating: f64,
}

impl ResidentialArchitectureSystem {
    pub fn new(residential_type: ResidentialType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            residential_type,
            spatial_efficiency: 0.0,
            cost_efficiency: 0.0,
            occupant_satisfaction: 0.0,
            sustainability_rating: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.residential_type {
            ResidentialType::SingleFamily => {
                self.occupant_satisfaction = 0.90 + rand_simple() * 0.10;
                self.spatial_efficiency = 0.75 + rand_simple() * 0.22;
                self.cost_efficiency = 0.55 + rand_simple() * 0.40;
            },
            ResidentialType::MultiFamily => {
                self.spatial_efficiency = 0.85 + rand_simple() * 0.14;
                self.cost_efficiency = 0.80 + rand_simple() * 0.18;
                self.occupant_satisfaction = 0.70 + rand_simple() * 0.25;
            },
            ResidentialType::Cohousing => {
                self.occupant_satisfaction = 0.85 + rand_simple() * 0.14;
                self.cost_efficiency = 0.75 + rand_simple() * 0.22;
                self.spatial_efficiency = 0.70 + rand_simple() * 0.25;
            },
            ResidentialType::TinyHomes => {
                self.cost_efficiency = 0.95 + rand_simple() * 0.05;
                self.spatial_efficiency = 0.60 + rand_simple() * 0.35;
                self.occupant_satisfaction = 0.75 + rand_simple() * 0.22;
            },
            ResidentialType::ModularConstruction => {
                self.cost_efficiency = 0.85 + rand_simple() * 0.14;
                self.sustainability_rating = 0.80 + rand_simple() * 0.18;
                self.spatial_efficiency = 0.75 + rand_simple() * 0.22;
            },
            ResidentialType::AffordableHousing => {
                self.cost_efficiency = 0.90 + rand_simple() * 0.10;
                self.spatial_efficiency = 0.80 + rand_simple() * 0.18;
                self.occupant_satisfaction = 0.65 + rand_simple() * 0.30;
            },
        }

        if self.sustainability_rating == 0.0 {
            self.sustainability_rating = (self.cost_efficiency + self.occupant_satisfaction) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_multi_family() {
        let mut system = ResidentialArchitectureSystem::new(ResidentialType::MultiFamily);
        system.analyze_system().unwrap();
        assert!(system.spatial_efficiency > 0.7);
    }
}
