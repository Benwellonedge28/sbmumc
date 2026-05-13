//! # SBMUMC Module 1468: Human Rights
//!
//! Systems for human rights theory and practice.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HumanRightsTopic {
    RightsTheories,
    Universalism,
    CulturalRelativism,
    RightsImplementation,
    RightsRestrictions,
    NewRights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanRightsSystem {
    pub system_id: String,
    pub human_rights_topic: HumanRightsTopic,
    pub rights_grounding: f64,
    pub rights_justification: f64,
    pub rights_protection: f64,
    pub rights_fulfillment: f64,
}

impl HumanRightsSystem {
    pub fn new(human_rights_topic: HumanRightsTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            human_rights_topic,
            rights_grounding: 0.0,
            rights_justification: 0.0,
            rights_protection: 0.0,
            rights_fulfillment: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.human_rights_topic {
            HumanRightsTopic::RightsTheories => {
                self.rights_grounding = 0.95 + rand_simple() * 0.05;
                self.rights_justification = 0.90 + rand_simple() * 0.10;
                self.rights_protection = 0.85 + rand_simple() * 0.14;
            },
            HumanRightsTopic::Universalism => {
                self.rights_fulfillment = 0.95 + rand_simple() * 0.05;
                self.rights_grounding = 0.90 + rand_simple() * 0.10;
                self.rights_justification = 0.85 + rand_simple() * 0.14;
            },
            HumanRightsTopic::CulturalRelativism => {
                self.rights_protection = 0.95 + rand_simple() * 0.05;
                self.rights_fulfillment = 0.90 + rand_simple() * 0.10;
                self.rights_grounding = 0.85 + rand_simple() * 0.14;
            },
            HumanRightsTopic::RightsImplementation => {
                self.rights_justification = 0.95 + rand_simple() * 0.05;
                self.rights_protection = 0.90 + rand_simple() * 0.10;
                self.rights_fulfillment = 0.85 + rand_simple() * 0.14;
            },
            HumanRightsTopic::RightsRestrictions => {
                self.rights_grounding = 0.95 + rand_simple() * 0.05;
                self.rights_fulfillment = 0.90 + rand_simple() * 0.10;
                self.rights_justification = 0.85 + rand_simple() * 0.14;
            },
            HumanRightsTopic::NewRights => {
                self.rights_protection = 0.95 + rand_simple() * 0.05;
                self.rights_grounding = 0.90 + rand_simple() * 0.10;
                self.rights_justification = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.rights_justification == 0.0 {
            self.rights_justification = (self.rights_grounding + self.rights_protection) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_theories() {
        let mut system = HumanRightsSystem::new(HumanRightsTopic::RightsTheories);
        system.analyze_system().unwrap();
        assert!(system.rights_grounding > 0.8);
    }
}