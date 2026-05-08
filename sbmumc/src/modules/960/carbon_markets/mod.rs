//! # SBMUMC Module 960: Carbon Markets
//! 
//! Frameworks for carbon trading and market mechanisms.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketType {
    CapAndTrade,
    Voluntary,
    BaselineAndCredit,
    InternalCarbon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonAllowance {
    pub allowance_id: String,
    pub vintage: u32,
    pub tons_co2: f64,
    pub price_per_ton: f64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonPortfolio {
    pub portfolio_id: String,
    pub market_type: MarketType,
    pub allowances: Vec<CarbonAllowance>,
    pub total_holdings: f64,
    pub market_value: f64,
}

impl CarbonAllowance {
    pub fn new(vintage: u32, tons: f64) -> Self {
        Self {
            allowance_id: format!("ca_{}", uuid_simple()),
            vintage,
            tons_co2: tons,
            price_per_ton: 0.0,
            status: "held".to_string(),
        }
    }

    pub fn set_price(&mut self, price: f64) {
        self.price_per_ton = price;
    }

    pub fn total_value(&self) -> f64 {
        self.tons_co2 * self.price_per_ton
    }
}

impl CarbonPortfolio {
    pub fn new(market_type: MarketType) -> Self {
        Self {
            portfolio_id: format!("cp_{}", uuid_simple()),
            market_type,
            allowances: Vec::new(),
            total_holdings: 0.0,
            market_value: 0.0,
        }
    }

    pub fn add_allowance(&mut self, allowance: CarbonAllowance) {
        self.allowances.push(allowance);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_holdings = self.allowances.iter()
            .map(|a| a.tons_co2)
            .sum();
        self.market_value = self.allowances.iter()
            .map(|a| a.total_value())
            .sum();
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{:x}{:x}", duration.as_secs(), duration.subsec_nanos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carbon_allowance() {
        let mut allowance = CarbonAllowance::new(2025, 1000.0);
        allowance.set_price(85.0);
        assert!(allowance.total_value() > 0.0);
    }

    #[test]
    fn test_carbon_portfolio() {
        let mut portfolio = CarbonPortfolio::new(MarketType::CapAndTrade);
        portfolio.add_allowance(CarbonAllowance::new(2025, 500.0));
        assert!(portfolio.total_holdings > 0.0);
    }
}
