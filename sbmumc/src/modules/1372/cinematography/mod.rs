//! # SBMUMC Module 1372: Cinematography
//!
//! Systems for cinematography and camera work.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CinematographyStyle {
    Naturalistic,
    Stylized,
    Documentary,
    Experimental,
    Classical,
    Contemporary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CinematographySystem {
    pub system_id: String,
    pub cinematographic_style: CinematographyStyle,
    pub visual_composition: f64,
    pub lighting_mastery: f64,
    pub camera_movement: f64,
    pub technical_excellence: f64,
}

impl CinematographySystem {
    pub fn new(cinematographic_style: CinematographyStyle) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            cinematographic_style,
            visual_composition: 0.0,
            lighting_mastery: 0.0,
            camera_movement: 0.0,
            technical_excellence: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.cinematographic_style {
            CinematographyStyle::Naturalistic => {
                self.visual_composition = 0.95 + rand_simple() * 0.05;
                self.lighting_mastery = 0.90 + rand_simple() * 0.10;
                self.technical_excellence = 0.85 + rand_simple() * 0.14;
            },
            CinematographyStyle::Stylized => {
                self.lighting_mastery = 0.95 + rand_simple() * 0.05;
                self.visual_composition = 0.90 + rand_simple() * 0.10;
                self.camera_movement = 0.85 + rand_simple() * 0.14;
            },
            CinematographyStyle::Documentary => {
                self.technical_excellence = 0.95 + rand_simple() * 0.05;
                self.camera_movement = 0.90 + rand_simple() * 0.10;
                self.visual_composition = 0.85 + rand_simple() * 0.14;
            },
            CinematographyStyle::Experimental => {
                self.camera_movement = 0.95 + rand_simple() * 0.05;
                self.lighting_mastery = 0.90 + rand_simple() * 0.10;
                self.visual_composition = 0.85 + rand_simple() * 0.14;
            },
            CinematographyStyle::Classical => {
                self.lighting_mastery = 0.95 + rand_simple() * 0.05;
                self.visual_composition = 0.95 + rand_simple() * 0.05;
                self.technical_excellence = 0.90 + rand_simple() * 0.10;
            },
            CinematographyStyle::Contemporary => {
                self.camera_movement = 0.95 + rand_simple() * 0.05;
                self.visual_composition = 0.90 + rand_simple() * 0.10;
                self.lighting_mastery = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.technical_excellence == 0.0 {
            self.technical_excellence = (self.visual_composition + self.lighting_mastery) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_stylized() {
        let mut system = CinematographySystem::new(CinematographyStyle::Stylized);
        system.analyze_system().unwrap();
        assert!(system.lighting_mastery > 0.8);
    }
}