//! # SBMUMC Module 1107: Property Law
//!
//! Ownership, possession, and property rights.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyRegime {
    PrivateProperty,
    CommonProperty,
    StateProperty,
    IndigenousLand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyLawSystem {
    pub system_id: String,
    pub regime: PropertyRegime,
    pub title_clarity: f64,
    var transfer_efficiency: f64,
    pub security_of_possession: f64,
    pub land_market_functionality: f64,
}

impl PropertyLawSystem {
    pub fn new(regime: PropertyRegime) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            regime,
            title_clarity: 0.0,
            var transfer_efficiency: 0.0,
            self.security_of_possession = 0.0,
            self.land_market_functionality = 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.regime {
            PropertyRegime::PrivateProperty => {
                self.title_clarity = 0.85 + rand_simple() * 0.15;
                self.transfer_efficiency = 0.80 + rand_simple() * 0.18;
            },
            PropertyRegime::CommonProperty => {
                self.title_clarity = 0.50 + rand_simple() * 0.35;
                self.transfer_efficiency = 0.40 + rand_simple() * 0.35;
            },
            PropertyRegime::IndigenousLand => {
                self.title_clarity = 0.45 + rand_simple() * 0.40;
                self.transfer_efficiency = 0.30 + rand_simple() * 0.35;
            },
            _ => {
                self.title_clarity = 0.65 + rand_simple() * 0.30;
                self.transfer_efficiency = 0.55 + rand_simple() * 0.30;
            }
        }

        self.security_of_possession = self.title_clarity * (0.8 + rand_simple() * 0.2);
        self.land_market_functionality = self.transfer_efficiency * self.title_clarity;
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
    fn test_private_property() {
        let mut system = PropertyLawSystem::new(PropertyRegime::PrivateProperty);
        system.analyze_system().unwrap();
        assert!(system.title_clarity > 0.7);
    }
}