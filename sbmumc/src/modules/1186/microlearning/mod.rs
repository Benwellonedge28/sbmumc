//! # SBMUMC Module 1186: Microlearning
//!
//! Short, focused learning units delivered in digestible segments.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MicrolearningFormat {
    Video,
    Quiz,
    Infographic,
    Podcast,
    InteractiveCard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrolearningSystem {
    pub system_id: String,
    pub microlearning_format: MicrolearningFormat,
    pub attention_retention: f64,
    pub knowledge_absorption: f64,
    pub accessibility_convenience: f64,
    pub engagement_rate: f64,
}

impl MicrolearningSystem {
    pub fn new(microlearning_format: MicrolearningFormat) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            microlearning_format,
            attention_retention: 0.0,
            knowledge_absorption: 0.0,
            accessibility_convenience: 0.0,
            engagement_rate: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.microlearning_format {
            MicrolearningFormat::Video => {
                self.attention_retention = 0.75 + rand_simple() * 0.22;
                self.knowledge_absorption = 0.70 + rand_simple() * 0.25;
                self.engagement_rate = 0.80 + rand_simple() * 0.18;
            },
            MicrolearningFormat::Quiz => {
                self.knowledge_absorption = 0.85 + rand_simple() * 0.14;
                self.attention_retention = 0.70 + rand_simple() * 0.25;
            },
            MicrolearningFormat::Infographic => {
                self.attention_retention = 0.85 + rand_simple() * 0.14;
                self.accessibility_convenience = 0.80 + rand_simple() * 0.18;
            },
            MicrolearningFormat::Podcast => {
                self.accessibility_convenience = 0.90 + rand_simple() * 0.10;
                self.engagement_rate = 0.70 + rand_simple() * 0.25;
                self.knowledge_absorption = 0.60 + rand_simple() * 0.35;
            },
            MicrolearningFormat::InteractiveCard => {
                self.attention_retention = 0.80 + rand_simple() * 0.18;
                self.knowledge_absorption = 0.80 + rand_simple() * 0.18;
                self.engagement_rate = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.accessibility_convenience == 0.0 {
            self.accessibility_convenience = (self.attention_retention + self.engagement_rate) / 2.0 * (0.7 + rand_simple() * 0.3);
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
    fn test_interactive_microlearning() {
        let mut system = MicrolearningSystem::new(MicrolearningFormat::InteractiveCard);
        system.analyze_system().unwrap();
        assert!(system.engagement_rate > 0.7);
    }
}