//! # SBMUMC Module 1356: Theme Parks
//!
//! Systems for theme park design and operation.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemeParkAttraction {
    RollerCoaster,
    DarkRide,
    WaterRide,
    LiveShow,
    Simulation,
    MeetAndGreet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeParksSystem {
    pub system_id: String,
    pub attraction_type: ThemeParkAttraction,
    pub thrill_factor: f64,
    pub safety_rating: f64,
    pub immersion_level: f64,
    pub family_appeal: f64,
}

impl ThemeParksSystem {
    pub fn new(attraction_type: ThemeParkAttraction) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            attraction_type,
            thrill_factor: 0.0,
            safety_rating: 0.0,
            immersion_level: 0.0,
            family_appeal: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.attraction_type {
            ThemeParkAttraction::RollerCoaster => {
                self.thrill_factor = 0.95 + rand_simple() * 0.05;
                self.safety_rating = 0.95 + rand_simple() * 0.05;
                self.family_appeal = 0.75 + rand_simple() * 0.22;
            },
            ThemeParkAttraction::DarkRide => {
                self.immersion_level = 0.95 + rand_simple() * 0.05;
                self.family_appeal = 0.90 + rand_simple() * 0.10;
                self.thrill_factor = 0.80 + rand_simple() * 0.18;
            },
            ThemeParkAttraction::WaterRide => {
                self.thrill_factor = 0.90 + rand_simple() * 0.10;
                self.family_appeal = 0.90 + rand_simple() * 0.10;
                self.immersion_level = 0.80 + rand_simple() * 0.18;
            },
            ThemeParkAttraction::LiveShow => {
                self.immersion_level = 0.95 + rand_simple() * 0.05;
                self.family_appeal = 0.95 + rand_simple() * 0.05;
                self.safety_rating = 0.90 + rand_simple() * 0.10;
            },
            ThemeParkAttraction::Simulation => {
                self.immersion_level = 0.95 + rand_simple() * 0.05;
                self.thrill_factor = 0.90 + rand_simple() * 0.10;
                self.safety_rating = 0.85 + rand_simple() * 0.14;
            },
            ThemeParkAttraction::MeetAndGreet => {
                self.family_appeal = 0.95 + rand_simple() * 0.05;
                self.immersion_level = 0.85 + rand_simple() * 0.14;
                self.safety_rating = 0.90 + rand_simple() * 0.10;
            },
        }

        if self.safety_rating == 0.0 {
            self.safety_rating = (self.thrill_factor + self.family_appeal) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_live_show() {
        let mut system = ThemeParksSystem::new(ThemeParkAttraction::LiveShow);
        system.analyze_system().unwrap();
        assert!(system.immersion_level > 0.8);
    }
}