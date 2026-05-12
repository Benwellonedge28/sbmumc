//! # SBMUMC Module 1374: Set Design
//!
//! Systems for theatrical and film set design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetDesignType {
    Theater,
    Film,
    Television,
    Exhibition,
    ThemePark,
    Virtual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDesignSystem {
    pub system_id: String,
    pub set_design_type: SetDesignType,
    pub spatial_design: f64,
    pub aesthetic_appeal: f64,
    pub functional_utility: f64,
    pub technical_feasibility: f64,
}

impl SetDesignSystem {
    pub fn new(set_design_type: SetDesignType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            set_design_type,
            spatial_design: 0.0,
            aesthetic_appeal: 0.0,
            functional_utility: 0.0,
            technical_feasibility: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.set_design_type {
            SetDesignType::Theater => {
                self.spatial_design = 0.95 + rand_simple() * 0.05;
                self.functional_utility = 0.90 + rand_simple() * 0.10;
                self.aesthetic_appeal = 0.85 + rand_simple() * 0.14;
            },
            SetDesignType::Film => {
                self.aesthetic_appeal = 0.95 + rand_simple() * 0.05;
                self.technical_feasibility = 0.90 + rand_simple() * 0.10;
                self.spatial_design = 0.85 + rand_simple() * 0.14;
            },
            SetDesignType::Television => {
                self.functional_utility = 0.95 + rand_simple() * 0.05;
                self.spatial_design = 0.90 + rand_simple() * 0.10;
                self.aesthetic_appeal = 0.85 + rand_simple() * 0.14;
            },
            SetDesignType::Exhibition => {
                self.spatial_design = 0.95 + rand_simple() * 0.05;
                self.aesthetic_appeal = 0.90 + rand_simple() * 0.10;
                self.functional_utility = 0.85 + rand_simple() * 0.14;
            },
            SetDesignType::ThemePark => {
                self.aesthetic_appeal = 0.95 + rand_simple() * 0.05;
                self.functional_utility = 0.90 + rand_simple() * 0.10;
                self.technical_feasibility = 0.85 + rand_simple() * 0.14;
            },
            SetDesignType::Virtual => {
                self.technical_feasibility = 0.95 + rand_simple() * 0.05;
                self.spatial_design = 0.90 + rand_simple() * 0.10;
                self.aesthetic_appeal = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.technical_feasibility == 0.0 {
            self.technical_feasibility = (self.spatial_design + self.functional_utility) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_film() {
        let mut system = SetDesignSystem::new(SetDesignType::Film);
        system.analyze_system().unwrap();
        assert!(system.aesthetic_appeal > 0.8);
    }
}