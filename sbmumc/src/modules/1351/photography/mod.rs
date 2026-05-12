//! # SBMUMC Module 1351: Photography
//!
//! Systems for photographic art and practice.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhotographyStyle {
    Portrait,
    Landscape,
    Street,
    Documentary,
    Abstract,
    Commercial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotographySystem {
    pub system_id: String,
    pub photography_style: PhotographyStyle,
    pub compositional_skill: f64,
    pub technical_mastery: f64,
    pub storytelling_ability: f64,
    pub artistic_vision: f64,
}

impl PhotographySystem {
    pub fn new(photography_style: PhotographyStyle) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            photography_style,
            compositional_skill: 0.0,
            technical_mastery: 0.0,
            storytelling_ability: 0.0,
            artistic_vision: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.photography_style {
            PhotographyStyle::Portrait => {
                self.storytelling_ability = 0.95 + rand_simple() * 0.05;
                self.artistic_vision = 0.90 + rand_simple() * 0.10;
                self.compositional_skill = 0.85 + rand_simple() * 0.14;
            },
            PhotographyStyle::Landscape => {
                self.compositional_skill = 0.95 + rand_simple() * 0.05;
                self.artistic_vision = 0.90 + rand_simple() * 0.10;
                self.technical_mastery = 0.85 + rand_simple() * 0.14;
            },
            PhotographyStyle::Street => {
                self.storytelling_ability = 0.95 + rand_simple() * 0.05;
                self.technical_mastery = 0.90 + rand_simple() * 0.10;
                self.compositional_skill = 0.85 + rand_simple() * 0.14;
            },
            PhotographyStyle::Documentary => {
                self.storytelling_ability = 0.95 + rand_simple() * 0.05;
                self.artistic_vision = 0.90 + rand_simple() * 0.10;
                self.technical_mastery = 0.85 + rand_simple() * 0.14;
            },
            PhotographyStyle::Abstract => {
                self.artistic_vision = 0.95 + rand_simple() * 0.05;
                self.technical_mastery = 0.90 + rand_simple() * 0.10;
                self.compositional_skill = 0.85 + rand_simple() * 0.14;
            },
            PhotographyStyle::Commercial => {
                self.technical_mastery = 0.95 + rand_simple() * 0.05;
                self.compositional_skill = 0.90 + rand_simple() * 0.10;
                self.storytelling_ability = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.technical_mastery == 0.0 {
            self.technical_mastery = (self.compositional_skill + self.storytelling_ability) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_street() {
        let mut system = PhotographySystem::new(PhotographyStyle::Street);
        system.analyze_system().unwrap();
        assert!(system.storytelling_ability > 0.8);
    }
}