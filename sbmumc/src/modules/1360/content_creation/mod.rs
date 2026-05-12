//! # SBMUMC Module 1360: Content Creation
//!
//! Systems for digital content creation platforms.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Video,
    Text,
    Audio,
    Images,
    Interactive,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentCreationSystem {
    pub system_id: String,
    pub content_type: ContentType,
    pub creative_quality: f64,
    pub production_speed: f64,
    pub audience_reach: f64,
    pub monetization_potential: f64,
}

impl ContentCreationSystem {
    pub fn new(content_type: ContentType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            content_type,
            creative_quality: 0.0,
            production_speed: 0.0,
            audience_reach: 0.0,
            monetization_potential: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.content_type {
            ContentType::Video => {
                self.creative_quality = 0.95 + rand_simple() * 0.05;
                self.audience_reach = 0.90 + rand_simple() * 0.10;
                self.monetization_potential = 0.85 + rand_simple() * 0.14;
            },
            ContentType::Text => {
                self.creative_quality = 0.95 + rand_simple() * 0.05;
                self.production_speed = 0.90 + rand_simple() * 0.10;
                self.monetization_potential = 0.80 + rand_simple() * 0.18;
            },
            ContentType::Audio => {
                self.creative_quality = 0.95 + rand_simple() * 0.05;
                self.audience_reach = 0.85 + rand_simple() * 0.14;
                self.production_speed = 0.90 + rand_simple() * 0.10;
            },
            ContentType::Images => {
                self.creative_quality = 0.95 + rand_simple() * 0.05;
                self.production_speed = 0.90 + rand_simple() * 0.10;
                self.audience_reach = 0.85 + rand_simple() * 0.14;
            },
            ContentType::Interactive => {
                self.creative_quality = 0.90 + rand_simple() * 0.10;
                self.monetization_potential = 0.95 + rand_simple() * 0.05;
                self.audience_reach = 0.85 + rand_simple() * 0.14;
            },
            ContentType::Hybrid => {
                self.creative_quality = 0.90 + rand_simple() * 0.10;
                self.audience_reach = 0.90 + rand_simple() * 0.10;
                self.monetization_potential = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.monetization_potential == 0.0 {
            self.monetization_potential = (self.creative_quality + self.audience_reach) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_interactive() {
        let mut system = ContentCreationSystem::new(ContentType::Interactive);
        system.analyze_system().unwrap();
        assert!(system.monetization_potential > 0.8);
    }
}