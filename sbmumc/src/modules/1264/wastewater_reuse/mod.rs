//! # SBMUMC Module 1264: Wastewater Reuse
//!
//! Systems for treating and reusing wastewater resources.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WastewaterReuseMethod {
    DirectPotableReuse,
    IndirectPotableReuse,
    AgriculturalReuse,
    IndustrialReuse,
    EnvironmentalReuse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WastewaterReuseSystem {
    pub system_id: String,
    pub reuse_method: WastewaterReuseMethod,
    pub treatment_efficiency: f64,
    pub safety_standard: f64,
    pub resource_recovery: f64,
    pub public_acceptance: f64,
}

impl WastewaterReuseSystem {
    pub fn new(reuse_method: WastewaterReuseMethod) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            reuse_method,
            treatment_efficiency: 0.0,
            safety_standard: 0.0,
            resource_recovery: 0.0,
            public_acceptance: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.reuse_method {
            WastewaterReuseMethod::DirectPotableReuse => {
                self.treatment_efficiency = 0.95 + rand_simple() * 0.05;
                self.safety_standard = 0.98 + rand_simple() * 0.02;
                self.public_acceptance = 0.55 + rand_simple() * 0.40;
            },
            WastewaterReuseMethod::IndirectPotableReuse => {
                self.treatment_efficiency = 0.90 + rand_simple() * 0.10;
                self.safety_standard = 0.95 + rand_simple() * 0.05;
                self.public_acceptance = 0.75 + rand_simple() * 0.22;
            },
            WastewaterReuseMethod::AgriculturalReuse => {
                self.resource_recovery = 0.85 + rand_simple() * 0.14;
                self.treatment_efficiency = 0.75 + rand_simple() * 0.22;
                self.public_acceptance = 0.80 + rand_simple() * 0.18;
            },
            WastewaterReuseMethod::IndustrialReuse => {
                self.resource_recovery = 0.90 + rand_simple() * 0.10;
                self.treatment_efficiency = 0.80 + rand_simple() * 0.18;
            },
            WastewaterReuseMethod::EnvironmentalReuse => {
                self.public_acceptance = 0.90 + rand_simple() * 0.10;
                self.safety_standard = 0.80 + rand_simple() * 0.18;
                self.resource_recovery = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.resource_recovery == 0.0 {
            self.resource_recovery = (self.treatment_efficiency + self.safety_standard) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_industrial_reuse() {
        let mut system = WastewaterReuseSystem::new(WastewaterReuseMethod::IndustrialReuse);
        system.analyze_system().unwrap();
        assert!(system.resource_recovery > 0.7);
    }
}
