//! # SBMUMC Module 1235: Agritech Startups
//!
//! Innovation-driven agricultural technology enterprises.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StartupFocus {
    PrecisionFarming,
    SupplyChain,
    MarketAccess,
    FarmManagement,
    Sustainability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgritechStartupsSystem {
    pub system_id: String,
    pub startup_focus: StartupFocus,
    pub innovation_index: f64,
    pub market_penetration: f64,
    pub adoption_rate: f64,
    pub investment_attraction: f64,
}

impl AgritechStartupsSystem {
    pub fn new(startup_focus: StartupFocus) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            startup_focus,
            innovation_index: 0.0,
            market_penetration: 0.0,
            adoption_rate: 0.0,
            investment_attraction: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.startup_focus {
            StartupFocus::PrecisionFarming => {
                self.innovation_index = 0.85 + rand_simple() * 0.14;
                self.adoption_rate = 0.65 + rand_simple() * 0.30;
                self.investment_attraction = 0.80 + rand_simple() * 0.18;
            },
            StartupFocus::SupplyChain => {
                self.innovation_index = 0.75 + rand_simple() * 0.22;
                self.market_penetration = 0.80 + rand_simple() * 0.18;
                self.adoption_rate = 0.70 + rand_simple() * 0.25;
            },
            StartupFocus::MarketAccess => {
                self.innovation_index = 0.70 + rand_simple() * 0.25;
                self.market_penetration = 0.85 + rand_simple() * 0.14;
                self.investment_attraction = 0.70 + rand_simple() * 0.25;
            },
            StartupFocus::FarmManagement => {
                self.innovation_index = 0.80 + rand_simple() * 0.18;
                self.adoption_rate = 0.75 + rand_simple() * 0.22;
                self.market_penetration = 0.65 + rand_simple() * 0.30;
            },
            StartupFocus::Sustainability => {
                self.innovation_index = 0.85 + rand_simple() * 0.14;
                self.investment_attraction = 0.85 + rand_simple() * 0.14;
                self.adoption_rate = 0.55 + rand_simple() * 0.35;
            },
        }

        if self.investment_attraction == 0.0 {
            self.investment_attraction = (self.innovation_index + self.market_penetration) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_precision_farming_startup() {
        let mut system = AgritechStartupsSystem::new(StartupFocus::PrecisionFarming);
        system.analyze_system().unwrap();
        assert!(system.innovation_index > 0.6);
    }
}