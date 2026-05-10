//! # SBMUMC Module 1210: Agricultural Research
//!
//! Scientific investigation in agricultural domains.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchFocus {
    CropImprovement,
    PestManagement,
    SoilHealth,
    WaterManagement,
    ClimateAdaptation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalResearchSystem {
    pub system_id: String,
    pub research_focus: ResearchFocus,
    pub innovation_potential: f64,
    pub practical_applicability: f64,
    pub funding_adequacy: f64,
    pub adoption_rate: f64,
}

impl AgriculturalResearchSystem {
    pub fn new(research_focus: ResearchFocus) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            research_focus,
            innovation_potential: 0.0,
            practical_applicability: 0.0,
            funding_adequacy: 0.0,
            adoption_rate: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.research_focus {
            ResearchFocus::CropImprovement => {
                self.innovation_potential = 0.85 + rand_simple() * 0.14;
                self.practical_applicability = 0.80 + rand_simple() * 0.18;
                self.adoption_rate = 0.70 + rand_simple() * 0.25;
            },
            ResearchFocus::PestManagement => {
                self.innovation_potential = 0.80 + rand_simple() * 0.18;
                self.practical_applicability = 0.75 + rand_simple() * 0.22;
            },
            ResearchFocus::SoilHealth => {
                self.practical_applicability = 0.85 + rand_simple() * 0.14;
                self.funding_adequacy = 0.70 + rand_simple() * 0.25;
            },
            ResearchFocus::WaterManagement => {
                self.innovation_potential = 0.75 + rand_simple() * 0.22;
                self.practical_applicability = 0.80 + rand_simple() * 0.18;
                self.adoption_rate = 0.65 + rand_simple() * 0.30;
            },
            ResearchFocus::ClimateAdaptation => {
                self.innovation_potential = 0.80 + rand_simple() * 0.18;
                self.funding_adequacy = 0.85 + rand_simple() * 0.14;
                self.adoption_rate = 0.60 + rand_simple() * 0.35;
            },
        }

        if self.funding_adequacy == 0.0 {
            self.funding_adequacy = 0.55 + rand_simple() * 0.40;
        }
        if self.adoption_rate == 0.0 {
            self.adoption_rate = (self.innovation_potential + self.practical_applicability) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_crop_improvement_research() {
        let mut system = AgriculturalResearchSystem::new(ResearchFocus::CropImprovement);
        system.analyze_system().unwrap();
        assert!(system.innovation_potential > 0.6);
    }
}