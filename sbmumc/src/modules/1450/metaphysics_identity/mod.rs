//! # SBMUMC Module 1450: Metaphysics of Identity
//!
//! Systems for metaphysics of identity and persistence.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityTheory {
    haecceityism,
    Quiddities,
    CriteriaIdentity,
    StageTheory,
    ConstitutionView,
    BundleTheory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphysicsIdentitySystem {
    pub system_id: String,
    pub identity_theory: IdentityTheory,
    pub identity_conditions: f64,
    pub persistence_criteria: f64,
    pub numerical_identity: f64,
    pub qualitative_identity: f64,
}

impl MetaphysicsIdentitySystem {
    pub fn new(identity_theory: IdentityTheory) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            identity_theory,
            identity_conditions: 0.0,
            persistence_criteria: 0.0,
            numerical_identity: 0.0,
            qualitative_identity: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.identity_theory {
            IdentityTheory::haecceityism => {
                self.identity_conditions = 0.95 + rand_simple() * 0.05;
                self.persistence_criteria = 0.90 + rand_simple() * 0.10;
                self.numerical_identity = 0.85 + rand_simple() * 0.14;
            },
            IdentityTheory::Quiddities => {
                self.qualitative_identity = 0.95 + rand_simple() * 0.05;
                self.identity_conditions = 0.90 + rand_simple() * 0.10;
                self.persistence_criteria = 0.85 + rand_simple() * 0.14;
            },
            IdentityTheory::CriteriaIdentity => {
                self.numerical_identity = 0.95 + rand_simple() * 0.05;
                self.qualitative_identity = 0.90 + rand_simple() * 0.10;
                self.identity_conditions = 0.85 + rand_simple() * 0.14;
            },
            IdentityTheory::StageTheory => {
                self.persistence_criteria = 0.95 + rand_simple() * 0.05;
                self.numerical_identity = 0.90 + rand_simple() * 0.10;
                self.qualitative_identity = 0.85 + rand_simple() * 0.14;
            },
            IdentityTheory::ConstitutionView => {
                self.identity_conditions = 0.95 + rand_simple() * 0.05;
                self.qualitative_identity = 0.90 + rand_simple() * 0.10;
                self.numerical_identity = 0.85 + rand_simple() * 0.14;
            },
            IdentityTheory::BundleTheory => {
                self.persistence_criteria = 0.95 + rand_simple() * 0.05;
                self.identity_conditions = 0.90 + rand_simple() * 0.10;
                self.qualitative_identity = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.numerical_identity == 0.0 {
            self.numerical_identity = (self.identity_conditions + self.persistence_criteria) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_haecceityism() {
        let mut system = MetaphysicsIdentitySystem::new(IdentityTheory::haecceityism);
        system.analyze_system().unwrap();
        assert!(system.identity_conditions > 0.8);
    }
}