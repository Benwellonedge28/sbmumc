//! # SBMUMC Module 1121: Healthcare Law
//!
//! Medical practice regulation, patient rights, and health policy.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthcareSystemModel {
    NationalHealth,
    SocialInsurance,
    Private,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthcareLawSystem {
    pub system_id: String,
    pub model: HealthcareSystemModel,
    pub access_equality: f64,
    var patient_rights_protection: f64,
    pub healthcare_quality: f64,
    pub cost_sustainability: f64,
}

impl HealthcareLawSystem {
    pub fn new(model: HealthcareSystemModel) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            model,
            access_equality: 0.0,
            var patient_rights_protection: 0.0,
            self.healthcare_quality = 0.0,
            self.cost_sustainability = 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.model {
            HealthcareSystemModel::NationalHealth => {
                self.access_equality = 0.90 + rand_simple() * 0.10;
                self.patient_rights_protection = 0.85 + rand_simple() * 0.15;
            },
            HealthcareSystemModel::SocialInsurance => {
                self.access_equality = 0.80 + rand_simple() * 0.15;
                self.patient_rights_protection = 0.80 + rand_simple() * 0.18;
            },
            HealthcareSystemModel::Private => {
                self.access_equality = 0.40 + rand_simple() * 0.30;
                self.patient_rights_protection = 0.70 + rand_simple() * 0.25;
            },
            _ => {
                self.access_equality = 0.60 + rand_simple() * 0.30;
                self.patient_rights_protection = 0.70 + rand_simple() * 0.25;
            }
        }

        self.healthcare_quality = 0.70 + rand_simple() * 0.25;
        self.cost_sustainability = 0.50 + rand_simple() * 0.40;
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
    fn test_national_health() {
        let mut system = HealthcareLawSystem::new(HealthcareSystemModel::NationalHealth);
        system.analyze_system().unwrap();
        assert!(system.access_equality > 0.8);
    }
}