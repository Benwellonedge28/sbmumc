//! # SBMUMC Module 1333: Worship Design
//!
//! Systems for religious and spiritual building design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorshipSpaceType {
    Church,
    Mosque,
    Temple,
    Synagogue,
    MeditationCenter,
    Chapel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorshipDesignSystem {
    pub system_id: String,
    pub worship_space_type: WorshipSpaceType,
    pub spiritual_atmosphere: f64,
    pub acoustic_quality: f64,
    pub ceremonial_functionality: f64,
    pub community_integration: f64,
}

impl WorshipDesignSystem {
    pub fn new(worship_space_type: WorshipSpaceType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            worship_space_type,
            spiritual_atmosphere: 0.0,
            acoustic_quality: 0.0,
            ceremonial_functionality: 0.0,
            community_integration: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.worship_space_type {
            WorshipSpaceType::Church => {
                self.spiritual_atmosphere = 0.95 + rand_simple() * 0.05;
                self.acoustic_quality = 0.90 + rand_simple() * 0.10;
                self.ceremonial_functionality = 0.85 + rand_simple() * 0.14;
            },
            WorshipSpaceType::Mosque => {
                self.spiritual_atmosphere = 0.95 + rand_simple() * 0.05;
                self.community_integration = 0.90 + rand_simple() * 0.10;
                self.ceremonial_functionality = 0.85 + rand_simple() * 0.14;
            },
            WorshipSpaceType::Temple => {
                self.ceremonial_functionality = 0.95 + rand_simple() * 0.05;
                self.spiritual_atmosphere = 0.90 + rand_simple() * 0.10;
                self.acoustic_quality = 0.85 + rand_simple() * 0.14;
            },
            WorshipSpaceType::Synagogue => {
                self.community_integration = 0.95 + rand_simple() * 0.05;
                self.spiritual_atmosphere = 0.90 + rand_simple() * 0.10;
                self.ceremonial_functionality = 0.85 + rand_simple() * 0.14;
            },
            WorshipSpaceType::MeditationCenter => {
                self.spiritual_atmosphere = 0.95 + rand_simple() * 0.05;
                self.acoustic_quality = 0.90 + rand_simple() * 0.10;
                self.community_integration = 0.80 + rand_simple() * 0.18;
            },
            WorshipSpaceType::Chapel => {
                self.spiritual_atmosphere = 0.90 + rand_simple() * 0.10;
                self.acoustic_quality = 0.85 + rand_simple() * 0.14;
                self.ceremonial_functionality = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.community_integration == 0.0 {
            self.community_integration = (self.spiritual_atmosphere + self.ceremonial_functionality) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_mosque() {
        let mut system = WorshipDesignSystem::new(WorshipSpaceType::Mosque);
        system.analyze_system().unwrap();
        assert!(system.spiritual_atmosphere > 0.8);
    }
}