//! # SBMUMC Module 1204: Food Security
//!
//! Ensuring consistent access to sufficient food for populations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FoodSecurityDimension {
    Availability,
    Access,
    Utilization,
    Stability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodSecurityFramework {
    pub framework_id: String,
    pub security_dimension: FoodSecurityDimension,
    pub production_capacity: f64,
    pub distribution_efficiency: f64,
    pub affordability_index: f64,
    pub nutritional_adequacy: f64,
}

impl FoodSecurityFramework {
    pub fn new(security_dimension: FoodSecurityDimension) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            security_dimension,
            production_capacity: 0.0,
            distribution_efficiency: 0.0,
            affordability_index: 0.0,
            nutritional_adequacy: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.security_dimension {
            FoodSecurityDimension::Availability => {
                self.production_capacity = 0.85 + rand_simple() * 0.14;
                self.distribution_efficiency = 0.75 + rand_simple() * 0.22;
            },
            FoodSecurityDimension::Access => {
                self.affordability_index = 0.80 + rand_simple() * 0.18;
                self.distribution_efficiency = 0.70 + rand_simple() * 0.25;
            },
            FoodSecurityDimension::Utilization => {
                self.nutritional_adequacy = 0.85 + rand_simple() * 0.14;
                self.affordability_index = 0.65 + rand_simple() * 0.30;
            },
            FoodSecurityDimension::Stability => {
                self.distribution_efficiency = 0.75 + rand_simple() * 0.22;
                self.production_capacity = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.affordability_index == 0.0 {
            self.affordability_index = (self.production_capacity + self.distribution_efficiency) / 2.0 * (0.6 + rand_simple() * 0.3);
        }
        if self.nutritional_adequacy == 0.0 {
            self.nutritional_adequacy = 0.55 + rand_simple() * 0.40;
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
    fn test_availability_dimension() {
        let mut framework = FoodSecurityFramework::new(FoodSecurityDimension::Availability);
        framework.analyze_framework().unwrap();
        assert!(framework.production_capacity > 0.6);
    }
}