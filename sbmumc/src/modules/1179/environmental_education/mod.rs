//! # SBMUMC Module 1179: Environmental Education
//!
//! Education about ecological systems and environmental stewardship.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentalEducationApproach {
    PlaceBased,
    Inquiry,
    ActionOriented,
    SystemsThinking,
    Conservation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalEducationFramework {
    pub framework_id: String,
    pub approach: EnvironmentalEducationApproach,
    pub ecological_knowledge: f64,
    pub environmental_awareness: f64,
    pub sustainable_behaviors: f64,
    pub stewardship_commitment: f64,
}

impl EnvironmentalEducationFramework {
    pub fn new(approach: EnvironmentalEducationApproach) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            approach,
            ecological_knowledge: 0.0,
            environmental_awareness: 0.0,
            sustainable_behaviors: 0.0,
            stewardship_commitment: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.approach {
            EnvironmentalEducationApproach::PlaceBased => {
                self.ecological_knowledge = 0.80 + rand_simple() * 0.18;
                self.environmental_awareness = 0.85 + rand_simple() * 0.14;
            },
            EnvironmentalEducationApproach::Inquiry => {
                self.ecological_knowledge = 0.85 + rand_simple() * 0.14;
                self.sustainable_behaviors = 0.70 + rand_simple() * 0.25;
            },
            EnvironmentalEducationApproach::ActionOriented => {
                self.sustainable_behaviors = 0.85 + rand_simple() * 0.14;
                self.stewardship_commitment = 0.80 + rand_simple() * 0.18;
            },
            EnvironmentalEducationApproach::SystemsThinking => {
                self.ecological_knowledge = 0.90 + rand_simple() * 0.10;
                self.environmental_awareness = 0.80 + rand_simple() * 0.18;
            },
            EnvironmentalEducationApproach::Conservation => {
                self.stewardship_commitment = 0.85 + rand_simple() * 0.14;
                self.sustainable_behaviors = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.environmental_awareness == 0.0 {
            self.environmental_awareness = 0.65 + rand_simple() * 0.30;
        }
        if self.sustainable_behaviors == 0.0 {
            self.sustainable_behaviors = (self.ecological_knowledge + self.stewardship_commitment) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_action_oriented_education() {
        let mut framework = EnvironmentalEducationFramework::new(EnvironmentalEducationApproach::ActionOriented);
        framework.analyze_framework().unwrap();
        assert!(framework.sustainable_behaviors > 0.7);
    }
}