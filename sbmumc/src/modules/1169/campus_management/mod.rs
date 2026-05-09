//! # SBMUMC Module 1169: Campus Management
//!
//! Administration and operations of educational campuses.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CampusFacilityType {
    Academic,
    Residential,
    Recreational,
    Administrative,
    Support,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampusManagementSystem {
    pub system_id: String,
    pub facility_type: CampusFacilityType,
    pub maintenance_quality: f64,
    pub safety_score: f64,
    pub sustainability_index: f64,
    pub user_satisfaction: f64,
}

impl CampusManagementSystem {
    pub fn new(facility_type: CampusFacilityType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            facility_type,
            maintenance_quality: 0.0,
            safety_score: 0.0,
            sustainability_index: 0.0,
            user_satisfaction: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.facility_type {
            CampusFacilityType::Academic => {
                self.maintenance_quality = 0.80 + rand_simple() * 0.18;
                self.safety_score = 0.85 + rand_simple() * 0.14;
            },
            CampusFacilityType::Residential => {
                self.maintenance_quality = 0.70 + rand_simple() * 0.25;
                self.safety_score = 0.80 + rand_simple() * 0.18;
                self.user_satisfaction = 0.75 + rand_simple() * 0.22;
            },
            CampusFacilityType::Recreational => {
                self.maintenance_quality = 0.75 + rand_simple() * 0.22;
                self.safety_score = 0.90 + rand_simple() * 0.10;
                self.sustainability_index = 0.65 + rand_simple() * 0.30;
            },
            CampusFacilityType::Administrative => {
                self.maintenance_quality = 0.85 + rand_simple() * 0.14;
                self.safety_score = 0.75 + rand_simple() * 0.22;
            },
            CampusFacilityType::Support => {
                self.maintenance_quality = 0.70 + rand_simple() * 0.25;
                self.safety_score = 0.80 + rand_simple() * 0.18;
                self.sustainability_index = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.sustainability_index == 0.0 {
            self.sustainability_index = 0.55 + rand_simple() * 0.40;
        }
        if self.user_satisfaction == 0.0 {
            self.user_satisfaction = (self.maintenance_quality + self.safety_score) / 2.0 * (0.7 + rand_simple() * 0.3);
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
    fn test_residential_campus() {
        let mut system = CampusManagementSystem::new(CampusFacilityType::Residential);
        system.analyze_system().unwrap();
        assert!(system.user_satisfaction > 0.5);
    }
}