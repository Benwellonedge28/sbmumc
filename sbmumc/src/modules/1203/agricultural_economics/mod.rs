//! # SBMUMC Module 1203: Agricultural Economics
//!
//! Economic analysis of agricultural production and markets.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketCondition {
    Stable,
    Volatile,
    Declining,
    Growing,
    Uncertain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalEconomicsSystem {
    pub system_id: String,
    pub market_condition: MarketCondition,
    pub price_volatility: f64,
    pub profit_margin: f64,
    pub market_access: f64,
    pub risk_resilience: f64,
}

impl AgriculturalEconomicsSystem {
    pub fn new(market_condition: MarketCondition) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            market_condition,
            price_volatility: 0.0,
            profit_margin: 0.0,
            market_access: 0.0,
            risk_resilience: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.market_condition {
            MarketCondition::Stable => {
                self.price_volatility = 0.20 + rand_simple() * 0.20;
                self.profit_margin = 0.70 + rand_simple() * 0.25;
            },
            MarketCondition::Volatile => {
                self.price_volatility = 0.80 + rand_simple() * 0.15;
                self.profit_margin = 0.50 + rand_simple() * 0.35;
                self.risk_resilience = 0.60 + rand_simple() * 0.30;
            },
            MarketCondition::Declining => {
                self.profit_margin = 0.35 + rand_simple() * 0.30;
                self.market_access = 0.65 + rand_simple() * 0.30;
            },
            MarketCondition::Growing => {
                self.profit_margin = 0.80 + rand_simple() * 0.18;
                self.price_volatility = 0.30 + rand_simple() * 0.25;
            },
            MarketCondition::Uncertain => {
                self.price_volatility = 0.60 + rand_simple() * 0.30;
                self.risk_resilience = 0.50 + rand_simple() * 0.35;
            },
        }

        if self.market_access == 0.0 {
            self.market_access = 0.55 + rand_simple() * 0.40;
        }
        if self.risk_resilience == 0.0 {
            self.risk_resilience = (self.profit_margin + self.market_access) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_growing_market() {
        let mut system = AgriculturalEconomicsSystem::new(MarketCondition::Growing);
        system.analyze_system().unwrap();
        assert!(system.profit_margin > 0.6);
    }
}