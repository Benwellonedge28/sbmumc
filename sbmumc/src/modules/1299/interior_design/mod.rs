//! # SBMUMC Module 1299: Interior Design
//!
//! Systems for interior space planning and decoration.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteriorDesignStyle {
    Minimalist,
    Biophilic,
    Industrial,
    Scandinavian,
    ArtDeco,
    Contemporary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteriorDesignSystem {
    pub system_id: String,
    pub design_style: InteriorDesignStyle,
    pub spatial_optimization: f64,
    pub aesthetic_appeal: f64,
    pub functionality: f64,
    pub budget_efficiency: f64,
}

impl InteriorDesignSystem {
    pub fn new(design_style: InteriorDesignStyle) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            design_style,
            spatial_optimization: 0.0,
            aesthetic_appeal: 0.0,
            functionality: 0.0,
            budget_efficiency: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.design_style {
            InteriorDesignStyle::Minimalist => {
                self.spatial_optimization = 0.95 + rand_simple() * 0.05;
                self.aesthetic_appeal = 0.85 + rand_simple() * 0.14;
                self.budget_efficiency = 0.70 + rand_simple() * 0.25;
            },
            InteriorDesignStyle::Biophilic => {
                self.aesthetic_appeal = 0.90 + rand_simple() * 0.10;
                self.functionality = 0.80 + rand_simple() * 0.18;
                self.budget_efficiency = 0.65 + rand_simple() * 0.30;
            },
            InteriorDesignStyle::Industrial => {
                self.aesthetic_appeal = 0.80 + rand_simple() * 0.18;
                self.budget_efficiency = 0.75 + rand_simple() * 0.22;
                self.spatial_optimization = 0.70 + rand_simple() * 0.25;
            },
            InteriorDesignStyle::Scandinavian => {
                self.functionality = 0.90 + rand_simple() * 0.10;
                self.spatial_optimization = 0.85 + rand_simple() * 0.14;
                self.aesthetic_appeal = 0.80 + rand_simple() * 0.18;
            },
            InteriorDesignStyle::ArtDeco => {
                self.aesthetic_appeal = 0.95 + rand_simple() * 0.05;
                self.functionality = 0.70 + rand_simple() * 0.25;
                self.budget_efficiency = 0.55 + rand_simple() * 0.40;
            },
            InteriorDesignStyle::Contemporary => {
                self.functionality = 0.85 + rand_simple() * 0.14;
                self.aesthetic_appeal = 0.80 + rand_simple() * 0.18;
                self.budget_efficiency = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.budget_efficiency == 0.0 {
            self.budget_efficiency = (self.spatial_optimization + self.functionality) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_scandinavian() {
        let mut system = InteriorDesignSystem::new(InteriorDesignStyle::Scandinavian);
        system.analyze_system().unwrap();
        assert!(system.functionality > 0.7);
    }
}
