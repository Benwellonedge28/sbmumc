//! # SBMUMC Module 1123: Humanitarian Law
//!
//! International humanitarian law, armed conflict, and protection of civilians.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    International,
    NonInternational,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanitarianLawSystem {
    pub system_id: String,
    pub conflict_type: ConflictType,
    pub civilian_protection: f64,
    var combatant_immunity: f64,
    pub proportionality_standard: f64,
    pub compliance_rate: f64,
}

impl HumanitarianLawSystem {
    pub fn new(conflict_type: ConflictType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            conflict_type,
            civilian_protection: 0.0,
            var combatant_immunity: 0.0,
            self.proportionality_standard = 0.0,
            self.compliance_rate = 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.conflict_type {
            ConflictType::International => {
                self.civilian_protection = 0.75 + rand_simple() * 0.20;
                self.combatant_immunity = 0.70 + rand_simple() * 0.25;
            },
            ConflictType::NonInternational => {
                self.civilian_protection = 0.55 + rand_simple() * 0.30;
                self.combatant_immunity = 0.50 + rand_simple() * 0.35;
            },
            _ => {
                self.civilian_protection = 0.60 + rand_simple() * 0.30;
                self.combatant_immunity = 0.55 + rand_simple() * 0.30;
            }
        }

        self.proportionality_standard = self.civilian_protection * (0.8 + rand_simple() * 0.2);
        self.compliance_rate = 0.40 + rand_simple() * 0.45;
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
    fn test_international_conflict() {
        let mut system = HumanitarianLawSystem::new(ConflictType::International);
        system.analyze_system().unwrap();
        assert!(system.civilian_protection > 0.5);
    }
}