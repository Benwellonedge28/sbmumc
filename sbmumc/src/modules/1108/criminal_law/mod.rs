//! # SBMUMC Module 1108: Criminal Law
//!
//! Crimes, elements of offenses, and criminal liability.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CriminalLawModel {
    ActusReus,
    MensRea,
    StrictLiability,
    NegligenceBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriminalLawSystem {
    pub system_id: String,
    pub model: CriminalLawModel,
    pub culpability_requirement: f64,
    var harm_threshold: f64,
    pub proportionality_index: f64,
    pub legal_certainty: f64,
}

impl CriminalLawSystem {
    pub fn new(model: CriminalLawModel) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            model,
            culpability_requirement: 0.0,
            var harm_threshold: 0.0,
            self.proportionality_index = 0.0,
            self.legal_certainty = 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.model {
            CriminalLawModel::MensRea => {
                self.culpability_requirement = 0.85 + rand_simple() * 0.15;
                self.harm_threshold = 0.70 + rand_simple() * 0.25;
            },
            CriminalLawModel::StrictLiability => {
                self.culpability_requirement = 0.0;
                self.harm_threshold = 0.85 + rand_simple() * 0.15;
            },
            _ => {
                self.culpability_requirement = 0.60 + rand_simple() * 0.30;
                self.harm_threshold = 0.65 + rand_simple() * 0.30;
            }
        }

        self.proportionality_index = (self.culpability_requirement + self.harm_threshold) / 2.0;
        self.legal_certainty = 0.65 + rand_simple() * 0.30;
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
    fn test_mens_rea_model() {
        let mut system = CriminalLawSystem::new(CriminalLawModel::MensRea);
        system.analyze_system().unwrap();
        assert!(system.culpability_requirement > 0.7);
    }
}