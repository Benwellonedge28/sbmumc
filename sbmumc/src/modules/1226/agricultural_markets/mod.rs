//! # SBMUMC Module 1226: Agricultural Markets
//!
//! Trading systems and market mechanisms for agricultural products.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketStructure {
    Spot,
    Forward,
    Cooperative,
    Contract,
    Electronic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalMarketsSystem {
    pub system_id: String,
    pub market_structure: MarketStructure,
    pub price_discovery: f64,
    pub market_efficiency: f64,
    pub risk_management: f64,
    pub participation_access: f64,
}

impl AgriculturalMarketsSystem {
    pub fn new(market_structure: MarketStructure) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            market_structure,
            price_discovery: 0.0,
            market_efficiency: 0.0,
            risk_management: 0.0,
            participation_access: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.market_structure {
            MarketStructure::Spot => {
                self.price_discovery = 0.85 + rand_simple() * 0.14;
                self.participation_access = 0.80 + rand_simple() * 0.18;
            },
            MarketStructure::Forward => {
                self.risk_management = 0.85 + rand_simple() * 0.14;
                self.market_efficiency = 0.75 + rand_simple() * 0.22;
            },
            MarketStructure::Cooperative => {
                self.participation_access = 0.85 + rand_simple() * 0.14;
                self.risk_management = 0.70 + rand_simple() * 0.25;
            },
            MarketStructure::Contract => {
                self.price_discovery = 0.70 + rand_simple() * 0.25;
                self.risk_management = 0.80 + rand_simple() * 0.18;
            },
            MarketStructure::Electronic => {
                self.price_discovery = 0.90 + rand_simple() * 0.10;
                self.market_efficiency = 0.85 + rand_simple() * 0.14;
                self.participation_access = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.market_efficiency == 0.0 {
            self.market_efficiency = (self.price_discovery + self.participation_access) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_electronic_market() {
        let mut system = AgriculturalMarketsSystem::new(MarketStructure::Electronic);
        system.analyze_system().unwrap();
        assert!(system.price_discovery > 0.7);
    }
}