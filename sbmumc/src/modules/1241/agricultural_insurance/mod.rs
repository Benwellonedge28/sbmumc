//! # SBMUMC Module 1241: Agricultural Insurance
//!
//! Risk protection mechanisms for agricultural producers.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsuranceProduct {
    CropInsurance,
    LivestockInsurance,
    RevenueProtection,
    IndexBased,
    MultiPeril,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalInsuranceSystem {
    pub system_id: String,
    pub insurance_product: InsuranceProduct,
    pub risk_coverage: f64,
    pub premium_affordability: f64,
    pub claim_efficiency: f64,
    pub uptake_rate: f64,
}

impl AgriculturalInsuranceSystem {
    pub fn new(insurance_product: InsuranceProduct) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            insurance_product,
            risk_coverage: 0.0,
            premium_affordability: 0.0,
            claim_efficiency: 0.0,
            uptake_rate: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.insurance_product {
            InsuranceProduct::CropInsurance => {
                self.risk_coverage = 0.85 + rand_simple() * 0.14;
                self.claim_efficiency = 0.70 + rand_simple() * 0.25;
            },
            InsuranceProduct::LivestockInsurance => {
                self.risk_coverage = 0.80 + rand_simple() * 0.18;
                self.premium_affordability = 0.65 + rand_simple() * 0.30;
            },
            InsuranceProduct::RevenueProtection => {
                self.risk_coverage = 0.90 + rand_simple() * 0.10;
                self.claim_efficiency = 0.80 + rand_simple() * 0.18;
                self.uptake_rate = 0.60 + rand_simple() * 0.35;
            },
            InsuranceProduct::IndexBased => {
                self.claim_efficiency = 0.90 + rand_simple() * 0.10;
                self.premium_affordability = 0.80 + rand_simple() * 0.18;
                self.uptake_rate = 0.70 + rand_simple() * 0.25;
            },
            InsuranceProduct::MultiPeril => {
                self.risk_coverage = 0.95 + rand_simple() * 0.05;
                self.premium_affordability = 0.55 + rand_simple() * 0.40;
            },
        }

        if self.uptake_rate == 0.0 {
            self.uptake_rate = (self.risk_coverage + self.premium_affordability) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_index_based_insurance() {
        let mut system = AgriculturalInsuranceSystem::new(InsuranceProduct::IndexBased);
        system.analyze_system().unwrap();
        assert!(system.claim_efficiency > 0.7);
    }
}