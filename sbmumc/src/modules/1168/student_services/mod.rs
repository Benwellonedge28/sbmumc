//! # SBMUMC Module 1168: Student Services
//!
//! Support systems for student wellbeing and success.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StudentServiceType {
    AcademicAdvising,
    Counseling,
    HealthServices,
    CareerServices,
    FinancialAid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentServicesSystem {
    pub system_id: String,
    pub service_type: StudentServiceType,
    pub service_availability: f64,
    pub service_quality: f64,
    pub utilization_rate: f64,
    pub outcome_impact: f64,
}

impl StudentServicesSystem {
    pub fn new(service_type: StudentServiceType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            service_type,
            service_availability: 0.0,
            service_quality: 0.0,
            utilization_rate: 0.0,
            outcome_impact: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.service_type {
            StudentServiceType::AcademicAdvising => {
                self.service_availability = 0.85 + rand_simple() * 0.14;
                self.service_quality = 0.75 + rand_simple() * 0.22;
                self.utilization_rate = 0.65 + rand_simple() * 0.30;
            },
            StudentServiceType::Counseling => {
                self.service_availability = 0.60 + rand_simple() * 0.35;
                self.service_quality = 0.80 + rand_simple() * 0.18;
                self.outcome_impact = 0.85 + rand_simple() * 0.14;
            },
            StudentServiceType::HealthServices => {
                self.service_availability = 0.70 + rand_simple() * 0.25;
                self.service_quality = 0.80 + rand_simple() * 0.18;
                self.utilization_rate = 0.55 + rand_simple() * 0.40;
            },
            StudentServiceType::CareerServices => {
                self.service_availability = 0.75 + rand_simple() * 0.22;
                self.service_quality = 0.70 + rand_simple() * 0.25;
                self.outcome_impact = 0.75 + rand_simple() * 0.22;
            },
            StudentServiceType::FinancialAid => {
                self.service_availability = 0.80 + rand_simple() * 0.18;
                self.service_quality = 0.70 + rand_simple() * 0.25;
                self.utilization_rate = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.outcome_impact == 0.0 {
            self.outcome_impact = (self.service_quality + self.utilization_rate) / 2.0;
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
    fn test_counseling_services() {
        let mut system = StudentServicesSystem::new(StudentServiceType::Counseling);
        system.analyze_system().unwrap();
        assert!(system.outcome_impact > 0.7);
    }
}