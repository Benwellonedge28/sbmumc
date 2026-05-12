//! # SBMUMC Module 1336: Hospitality Design
//!
//! Systems for hotel and hospitality design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HospitalityType {
    LuxuryHotel,
    BoutiqueHotel,
    BudgetAccommodation,
    Resort,
    ConferenceHotel,
    EcoLodge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HospitalityDesignSystem {
    pub system_id: String,
    pub hospitality_type: HospitalityType,
    pub guest_experience: f64,
    pub operational_efficiency: f64,
    pub brand_identity: f64,
    pub sustainability_practices: f64,
}

impl HospitalityDesignSystem {
    pub fn new(hospitality_type: HospitalityType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            hospitality_type,
            guest_experience: 0.0,
            operational_efficiency: 0.0,
            brand_identity: 0.0,
            sustainability_practices: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.hospitality_type {
            HospitalityType::LuxuryHotel => {
                self.guest_experience = 0.95 + rand_simple() * 0.05;
                self.brand_identity = 0.90 + rand_simple() * 0.10;
                self.operational_efficiency = 0.80 + rand_simple() * 0.18;
            },
            HospitalityType::BoutiqueHotel => {
                self.brand_identity = 0.95 + rand_simple() * 0.05;
                self.guest_experience = 0.90 + rand_simple() * 0.10;
                self.sustainability_practices = 0.80 + rand_simple() * 0.18;
            },
            HospitalityType::BudgetAccommodation => {
                self.operational_efficiency = 0.95 + rand_simple() * 0.05;
                self.guest_experience = 0.75 + rand_simple() * 0.22;
                self.sustainability_practices = 0.70 + rand_simple() * 0.25;
            },
            HospitalityType::Resort => {
                self.guest_experience = 0.95 + rand_simple() * 0.05;
                self.sustainability_practices = 0.90 + rand_simple() * 0.10;
                self.brand_identity = 0.85 + rand_simple() * 0.14;
            },
            HospitalityType::ConferenceHotel => {
                self.operational_efficiency = 0.90 + rand_simple() * 0.10;
                self.guest_experience = 0.85 + rand_simple() * 0.14;
                self.brand_identity = 0.80 + rand_simple() * 0.18;
            },
            HospitalityType::EcoLodge => {
                self.sustainability_practices = 0.95 + rand_simple() * 0.05;
                self.guest_experience = 0.85 + rand_simple() * 0.14;
                self.brand_identity = 0.90 + rand_simple() * 0.10;
            },
        }

        if self.sustainability_practices == 0.0 {
            self.sustainability_practices = (self.guest_experience + self.operational_efficiency) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_resort() {
        let mut system = HospitalityDesignSystem::new(HospitalityType::Resort);
        system.analyze_system().unwrap();
        assert!(system.guest_experience > 0.8);
    }
}