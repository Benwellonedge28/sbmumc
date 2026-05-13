//! # SBMUMC Module 1457: Political Philosophy
//!
//! Systems for political philosophy and social justice.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoliticalTheory {
    Liberalism,
    Conservatism,
    Libertarianism,
    Communitarianism,
    Republicanism,
    Cosmopolitanism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalPhilosophySystem {
    pub system_id: String,
    pub political_theory: PoliticalTheory,
    pub political_legitimacy: f64,
    pub justice_theories: f64,
    pub democratic_theory: f64,
    pub authority_grounds: f64,
}

impl PoliticalPhilosophySystem {
    pub fn new(political_theory: PoliticalTheory) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            political_theory,
            political_legitimacy: 0.0,
            justice_theories: 0.0,
            democratic_theory: 0.0,
            authority_grounds: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.political_theory {
            PoliticalTheory::Liberalism => {
                self.political_legitimacy = 0.95 + rand_simple() * 0.05;
                self.justice_theories = 0.90 + rand_simple() * 0.10;
                self.democratic_theory = 0.85 + rand_simple() * 0.14;
            },
            PoliticalTheory::Conservatism => {
                self.authority_grounds = 0.95 + rand_simple() * 0.05;
                self.political_legitimacy = 0.90 + rand_simple() * 0.10;
                self.justice_theories = 0.85 + rand_simple() * 0.14;
            },
            PoliticalTheory::Libertarianism => {
                self.democratic_theory = 0.95 + rand_simple() * 0.05;
                self.authority_grounds = 0.90 + rand_simple() * 0.10;
                self.political_legitimacy = 0.85 + rand_simple() * 0.14;
            },
            PoliticalTheory::Communitarianism => {
                self.justice_theories = 0.95 + rand_simple() * 0.05;
                self.democratic_theory = 0.90 + rand_simple() * 0.10;
                self.authority_grounds = 0.85 + rand_simple() * 0.14;
            },
            PoliticalTheory::Republicanism => {
                self.political_legitimacy = 0.95 + rand_simple() * 0.05;
                self.justice_theories = 0.90 + rand_simple() * 0.10;
                self.authority_grounds = 0.85 + rand_simple() * 0.14;
            },
            PoliticalTheory::Cosmopolitanism => {
                self.democratic_theory = 0.95 + rand_simple() * 0.05;
                self.political_legitimacy = 0.90 + rand_simple() * 0.10;
                self.justice_theories = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.authority_grounds == 0.0 {
            self.authority_grounds = (self.political_legitimacy + self.justice_theories) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_liberalism() {
        let mut system = PoliticalPhilosophySystem::new(PoliticalTheory::Liberalism);
        system.analyze_system().unwrap();
        assert!(system.political_legitimacy > 0.8);
    }
}