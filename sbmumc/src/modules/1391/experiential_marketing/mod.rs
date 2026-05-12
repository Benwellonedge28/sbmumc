//! # SBMUMC Module 1391: Experiential Marketing
//!
//! Systems for experiential marketing and brand activations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperienceScale {
    PopUp,
    BrandFestival,
    RetailActivation,
    ImmersiveBrandJourney,
    CorporateEvent,
    PublicInstallation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperientialMarketingSystem {
    pub system_id: String,
    pub experience_scale: ExperienceScale,
    pub brand_integration: f64,
    pub sensory_engagement: f64,
    pub social_shareability: f64,
    pub emotional_impact: f64,
}

impl ExperientialMarketingSystem {
    pub fn new(experience_scale: ExperienceScale) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            experience_scale,
            brand_integration: 0.0,
            sensory_engagement: 0.0,
            social_shareability: 0.0,
            emotional_impact: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.experience_scale {
            ExperienceScale::PopUp => {
                self.brand_integration = 0.95 + rand_simple() * 0.05;
                self.sensory_engagement = 0.90 + rand_simple() * 0.10;
                self.social_shareability = 0.85 + rand_simple() * 0.14;
            },
            ExperienceScale::BrandFestival => {
                self.emotional_impact = 0.95 + rand_simple() * 0.05;
                self.sensory_engagement = 0.90 + rand_simple() * 0.10;
                self.brand_integration = 0.85 + rand_simple() * 0.14;
            },
            ExperienceScale::RetailActivation => {
                self.sensory_engagement = 0.95 + rand_simple() * 0.05;
                self.social_shareability = 0.90 + rand_simple() * 0.10;
                self.emotional_impact = 0.85 + rand_simple() * 0.14;
            },
            ExperienceScale::ImmersiveBrandJourney => {
                self.emotional_impact = 0.95 + rand_simple() * 0.05;
                self.brand_integration = 0.90 + rand_simple() * 0.10;
                self.social_shareability = 0.85 + rand_simple() * 0.14;
            },
            ExperienceScale::CorporateEvent => {
                self.social_shareability = 0.95 + rand_simple() * 0.05;
                self.emotional_impact = 0.90 + rand_simple() * 0.10;
                self.sensory_engagement = 0.85 + rand_simple() * 0.14;
            },
            ExperienceScale::PublicInstallation => {
                self.sensory_engagement = 0.95 + rand_simple() * 0.05;
                self.brand_integration = 0.90 + rand_simple() * 0.10;
                self.emotional_impact = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.emotional_impact == 0.0 {
            self.emotional_impact = (self.brand_integration + self.sensory_engagement) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_popup() {
        let mut system = ExperientialMarketingSystem::new(ExperienceScale::PopUp);
        system.analyze_system().unwrap();
        assert!(system.brand_integration > 0.8);
    }
}
