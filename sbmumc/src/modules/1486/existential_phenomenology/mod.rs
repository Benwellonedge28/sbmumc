//! # SBMUMC Module 1486: Existential Phenomenology
//!
//! Systems for existential phenomenology and existentialist thought.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExistentialPhenomenologyTopic {
    ExistencePrecedes,
    BeingInWorld,
    Authenticity,
    AngstFreedom,
    thrownness,
    SelfDeception,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistentialPhenomenologySystem {
    pub system_id: String,
    pub existential_phenomenology_topic: ExistentialPhenomenologyTopic,
    pub existential_analytics: f64,
    pub phenomenological_method: f64,
    pub authentic_existence: f64,
    pub thrown_projects: f64,
}

impl ExistentialPhenomenologySystem {
    pub fn new(existential_phenomenology_topic: ExistentialPhenomenologyTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            existential_phenomenology_topic,
            existential_analytics: 0.0,
            phenomenological_method: 0.0,
            authentic_existence: 0.0,
            thrown_projects: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.existential_phenomenology_topic {
            ExistentialPhenomenologyTopic::ExistencePrecedes => {
                self.existential_analytics = 0.95 + rand_simple() * 0.05;
                self.phenomenological_method = 0.90 + rand_simple() * 0.10;
                self.authentic_existence = 0.85 + rand_simple() * 0.14;
            },
            ExistentialPhenomenologyTopic::BeingInWorld => {
                self.thrown_projects = 0.95 + rand_simple() * 0.05;
                self.existential_analytics = 0.90 + rand_simple() * 0.10;
                self.phenomenological_method = 0.85 + rand_simple() * 0.14;
            },
            ExistentialPhenomenologyTopic::Authenticity => {
                self.authentic_existence = 0.95 + rand_simple() * 0.05;
                self.thrown_projects = 0.90 + rand_simple() * 0.10;
                self.existential_analytics = 0.85 + rand_simple() * 0.14;
            },
            ExistentialPhenomenologyTopic::AngstFreedom => {
                self.phenomenological_method = 0.95 + rand_simple() * 0.05;
                self.authentic_existence = 0.90 + rand_simple() * 0.10;
                self.thrown_projects = 0.85 + rand_simple() * 0.14;
            },
            ExistentialPhenomenologyTopic::thrownness => {
                self.existential_analytics = 0.95 + rand_simple() * 0.05;
                self.thrown_projects = 0.90 + rand_simple() * 0.10;
                self.authentic_existence = 0.85 + rand_simple() * 0.14;
            },
            ExistentialPhenomenologyTopic::SelfDeception => {
                self.phenomenological_method = 0.95 + rand_simple() * 0.05;
                self.existential_analytics = 0.90 + rand_simple() * 0.10;
                self.thrown_projects = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.thrown_projects == 0.0 {
            self.thrown_projects = (self.existential_analytics + self.phenomenological_method) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_existence_precedes() {
        let mut system = ExistentialPhenomenologySystem::new(ExistentialPhenomenologyTopic::ExistencePrecedes);
        system.analyze_system().unwrap();
        assert!(system.existential_analytics > 0.8);
    }
}