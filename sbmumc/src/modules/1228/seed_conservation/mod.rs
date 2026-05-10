//! # SBMUMC Module 1228: Seed Conservation
//!
//! Preservation of plant genetic diversity through seed banking.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConservationMethod {
    SeedBank,
    FieldGeneBank,
    Cryopreservation,
    in situ,
    CommunitySeed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedConservationSystem {
    pub system_id: String,
    pub conservation_method: ConservationMethod,
    pub genetic_diversity: f64,
    pub viability_maintenance: f64,
    pub accessibility_score: f64,
    pub longTerm_security: f64,
}

impl SeedConservationSystem {
    pub fn new(conservation_method: ConservationMethod) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            conservation_method,
            genetic_diversity: 0.0,
            viability_maintenance: 0.0,
            accessibility_score: 0.0,
            longTerm_security: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.conservation_method {
            ConservationMethod::SeedBank => {
                self.genetic_diversity = 0.85 + rand_simple() * 0.14;
                self.longTerm_security = 0.80 + rand_simple() * 0.18;
            },
            ConservationMethod::FieldGeneBank => {
                self.genetic_diversity = 0.80 + rand_simple() * 0.18;
                self.accessibility_score = 0.75 + rand_simple() * 0.22;
            },
            ConservationMethod::Cryopreservation => {
                self.longTerm_security = 0.90 + rand_simple() * 0.10;
                self.viability_maintenance = 0.90 + rand_simple() * 0.10;
            },
            ConservationMethod::in_situ => {
                self.genetic_diversity = 0.75 + rand_simple() * 0.22;
                self.accessibility_score = 0.80 + rand_simple() * 0.18;
            },
            ConservationMethod::CommunitySeed => {
                self.accessibility_score = 0.85 + rand_simple() * 0.14;
                self.viability_maintenance = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.viability_maintenance == 0.0 {
            self.viability_maintenance = (self.genetic_diversity + self.longTerm_security) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_cryopreservation() {
        let mut system = SeedConservationSystem::new(ConservationMethod::Cryopreservation);
        system.analyze_system().unwrap();
        assert!(system.longTerm_security > 0.7);
    }
}