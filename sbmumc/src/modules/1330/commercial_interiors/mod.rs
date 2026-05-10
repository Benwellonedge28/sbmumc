//! # SBMUMC Module 1330: Commercial Interiors
//!
//! Systems for commercial interior design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommercialInteriorType {
    OpenPlan,
    CubicleOffice,
    ActivityBased,
    HotDesking,
    PrivateOffices,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommercialInteriorsSystem {
    pub system_id: String,
    pub interior_type: CommercialInteriorType,
    pub space_utilization: f64,
    pub employee_productivity: f64,
    pub brand_alignment: f64,
    pub cost_per_desk: f64,
}

impl CommercialInteriorsSystem {
    pub fn new(interior_type: CommercialInteriorType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            interior_type,
            space_utilization: 0.0,
            employee_productivity: 0.0,
            brand_alignment: 0.0,
            cost_per_desk: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.interior_type {
            CommercialInteriorType::OpenPlan => {
                self.space_utilization = 0.95 + rand_simple() * 0.05;
                self.employee_productivity = 0.75 + rand_simple() * 0.22;
                self.cost_per_desk = 0.85 + rand_simple() * 0.14;
            },
            CommercialInteriorType::CubicleOffice => {
                self.employee_productivity = 0.85 + rand_simple() * 0.14;
                self.space_utilization = 0.80 + rand_simple() * 0.18;
                self.brand_alignment = 0.70 + rand_simple() * 0.25;
            },
            CommercialInteriorType::ActivityBased => {
                self.space_utilization = 0.90 + rand_simple() * 0.10;
                self.employee_productivity = 0.90 + rand_simple() * 0.10;
                self.cost_per_desk = 0.80 + rand_simple() * 0.18;
            },
            CommercialInteriorType::HotDesking => {
                self.space_utilization = 0.95 + rand_simple() * 0.05;
                self.cost_per_desk = 0.90 + rand_simple() * 0.10;
                self.employee_productivity = 0.70 + rand_simple() * 0.25;
            },
            CommercialInteriorType::PrivateOffices => {
                self.employee_productivity = 0.90 + rand_simple() * 0.10;
                self.brand_alignment = 0.85 + rand_simple() * 0.14;
                self.space_utilization = 0.65 + rand_simple() * 0.30;
            },
            CommercialInteriorType::Hybrid => {
                self.space_utilization = 0.85 + rand_simple() * 0.14;
                self.employee_productivity = 0.85 + rand_simple() * 0.14;
                self.brand_alignment = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.brand_alignment == 0.0 {
            self.brand_alignment = (self.space_utilization + self.employee_productivity) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_activity_based() {
        let mut system = CommercialInteriorsSystem::new(CommercialInteriorType::ActivityBased);
        system.analyze_system().unwrap();
        assert!(system.space_utilization > 0.7);
    }
}
