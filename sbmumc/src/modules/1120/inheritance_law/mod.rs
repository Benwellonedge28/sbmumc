//! # SBMUMC Module 1120: Inheritance Law
//!
//! Succession, wills, estates, and wealth transfer.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InheritanceRegime {
    FreedomOfTestation,
    ForcedHeirship,
    CommunityProperty,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritanceLawSystem {
    pub system_id: String,
    pub regime: InheritanceRegime,
    pub testamentary_freedom: f64,
    var heir_protection: f64,
    pub estate_administration_efficiency: f64,
    pub wealth_distribution_equality: f64,
}

impl InheritanceLawSystem {
    pub fn new(regime: InheritanceRegime) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            regime,
            testamentary_freedom: 0.0,
            var heir_protection: 0.0,
            self.estate_administration_efficiency = 0.0,
            self.wealth_distribution_equality = 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.regime {
            InheritanceRegime::FreedomOfTestation => {
                self.testamentary_freedom = 0.90 + rand_simple() * 0.10;
                self.heir_protection = 0.40 + rand_simple() * 0.30;
            },
            InheritanceRegime::ForcedHeirship => {
                self.testamentary_freedom = 0.30 + rand_simple() * 0.25;
                self.heir_protection = 0.85 + rand_simple() * 0.15;
            },
            _ => {
                self.testamentary_freedom = 0.50 + rand_simple() * 0.40;
                self.heir_protection = 0.55 + rand_simple() * 0.35;
            }
        }

        self.estate_administration_efficiency = 0.60 + rand_simple() * 0.35;
        self.wealth_distribution_equality = self.heir_protection * (0.7 + rand_simple() * 0.3);
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
    fn test_forced_heirship() {
        let mut system = InheritanceLawSystem::new(InheritanceRegime::ForcedHeirship);
        system.analyze_system().unwrap();
        assert!(system.heir_protection > 0.7);
    }
}