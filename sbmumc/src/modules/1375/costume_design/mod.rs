//! # SBMUMC Module 1375: Costume Design
//!
//! Systems for costume design in entertainment.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CostumeDesignContext {
    PeriodDrama,
    Contemporary,
    Fantasy,
    SciFi,
    Dance,
    CharacterStudy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostumeDesignSystem {
    pub system_id: String,
    pub design_context: CostumeDesignContext,
    pub character_alignment: f64,
    pub visual_distinctiveness: f64,
    pub practical_functionality: f64,
    pub period_authenticity: f64,
}

impl CostumeDesignSystem {
    pub fn new(design_context: CostumeDesignContext) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            design_context,
            character_alignment: 0.0,
            visual_distinctiveness: 0.0,
            practical_functionality: 0.0,
            period_authenticity: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.design_context {
            CostumeDesignContext::PeriodDrama => {
                self.period_authenticity = 0.95 + rand_simple() * 0.05;
                self.character_alignment = 0.90 + rand_simple() * 0.10;
                self.visual_distinctiveness = 0.85 + rand_simple() * 0.14;
            },
            CostumeDesignContext::Contemporary => {
                self.character_alignment = 0.95 + rand_simple() * 0.05;
                self.practical_functionality = 0.90 + rand_simple() * 0.10;
                self.visual_distinctiveness = 0.85 + rand_simple() * 0.14;
            },
            CostumeDesignContext::Fantasy => {
                self.visual_distinctiveness = 0.95 + rand_simple() * 0.05;
                self.character_alignment = 0.90 + rand_simple() * 0.10;
                self.practical_functionality = 0.85 + rand_simple() * 0.14;
            },
            CostumeDesignContext::SciFi => {
                self.visual_distinctiveness = 0.95 + rand_simple() * 0.05;
                self.practical_functionality = 0.90 + rand_simple() * 0.10;
                self.character_alignment = 0.85 + rand_simple() * 0.14;
            },
            CostumeDesignContext::Dance => {
                self.practical_functionality = 0.95 + rand_simple() * 0.05;
                self.character_alignment = 0.90 + rand_simple() * 0.10;
                self.visual_distinctiveness = 0.85 + rand_simple() * 0.14;
            },
            CostumeDesignContext::CharacterStudy => {
                self.character_alignment = 0.95 + rand_simple() * 0.05;
                self.period_authenticity = 0.90 + rand_simple() * 0.10;
                self.visual_distinctiveness = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.period_authenticity == 0.0 {
            self.period_authenticity = (self.character_alignment + self.visual_distinctiveness) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_fantasy() {
        let mut system = CostumeDesignSystem::new(CostumeDesignContext::Fantasy);
        system.analyze_system().unwrap();
        assert!(system.visual_distinctiveness > 0.8);
    }
}