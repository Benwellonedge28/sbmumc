//! # SBMUMC Module 1480: Social Philosophy
//!
//! Systems for social philosophy and collective intentionality.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialPhilosophyTopic {
    CollectiveIntentionality,
    SocialOntology,
    SocialNorms,
    InstitutionalFacts,
    SocialConstruction,
    EmergentSocial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialPhilosophySystem {
    pub system_id: String,
    pub social_philosophy_topic: SocialPhilosophyTopic,
    pub collective_agency: f64,
    pub social_ontology: f64,
    pub normativity_social: f64,
    pub institutional_reality: f64,
}

impl SocialPhilosophySystem {
    pub fn new(social_philosophy_topic: SocialPhilosophyTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            social_philosophy_topic,
            collective_agency: 0.0,
            social_ontology: 0.0,
            normativity_social: 0.0,
            institutional_reality: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.social_philosophy_topic {
            SocialPhilosophyTopic::CollectiveIntentionality => {
                self.collective_agency = 0.95 + rand_simple() * 0.05;
                self.social_ontology = 0.90 + rand_simple() * 0.10;
                self.normativity_social = 0.85 + rand_simple() * 0.14;
            },
            SocialPhilosophyTopic::SocialOntology => {
                self.institutional_reality = 0.95 + rand_simple() * 0.05;
                self.collective_agency = 0.90 + rand_simple() * 0.10;
                self.social_ontology = 0.85 + rand_simple() * 0.14;
            },
            SocialPhilosophyTopic::SocialNorms => {
                self.normativity_social = 0.95 + rand_simple() * 0.05;
                self.institutional_reality = 0.90 + rand_simple() * 0.10;
                self.collective_agency = 0.85 + rand_simple() * 0.14;
            },
            SocialPhilosophyTopic::InstitutionalFacts => {
                self.social_ontology = 0.95 + rand_simple() * 0.05;
                self.normativity_social = 0.90 + rand_simple() * 0.10;
                self.institutional_reality = 0.85 + rand_simple() * 0.14;
            },
            SocialPhilosophyTopic::SocialConstruction => {
                self.collective_agency = 0.95 + rand_simple() * 0.05;
                self.social_ontology = 0.90 + rand_simple() * 0.10;
                self.institutional_reality = 0.85 + rand_simple() * 0.14;
            },
            SocialPhilosophyTopic::EmergentSocial => {
                self.normativity_social = 0.95 + rand_simple() * 0.05;
                self.collective_agency = 0.90 + rand_simple() * 0.10;
                self.social_ontology = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.normativity_social == 0.0 {
            self.normativity_social = (self.collective_agency + self.social_ontology) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_intentionality() {
        let mut system = SocialPhilosophySystem::new(SocialPhilosophyTopic::CollectiveIntentionality);
        system.analyze_system().unwrap();
        assert!(system.collective_agency > 0.8);
    }
}