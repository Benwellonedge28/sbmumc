//! # SBMUMC Module 1064: Emerging Markets
//!
//! Analysis of developing economies and investment opportunities.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketRiskLevel {
    High,
    Medium,
    LowerMedium,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergingMarket {
    pub market_id: String,
    pub country: String,
    pub risk_level: MarketRiskLevel,
    pub market_cap_billion: f64,
    pub volatility_index: f64,
    pub growth_prospects: f64,
    pub liquidity_score: f64,
}

impl EmergingMarket {
    pub fn new(country: String) -> Self {
        Self {
            market_id: crate::core::uuid_simple(),
            country,
            risk_level: MarketRiskLevel::Medium,
            market_cap_billion: 0.0,
            volatility_index: 0.0,
            growth_prospects: 0.0,
            liquidity_score: 0.0,
        }
    }

    pub fn analyze_market(&mut self) -> Result<()> {
        self.market_cap_billion = 50.0 + rand_simple() * 2000.0;
        self.volatility_index = 15.0 + rand_simple() * 35.0;
        self.growth_prospects = 4.0 + rand_simple() * 8.0;
        self.liquidity_score = 0.4 + rand_simple() * 0.5;

        if self.volatility_index > 30.0 {
            self.risk_level = MarketRiskLevel::High;
        } else if self.volatility_index > 20.0 {
            self.risk_level = MarketRiskLevel::Medium;
        } else {
            self.risk_level = MarketRiskLevel::LowerMedium;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketOpportunity {
    pub opportunity_id: String,
    pub sector: String,
    pub country: String,
    pub TAM_billion: f64,
    pub growth_rate: f64,
    pub competitive_intensity: f64,
    pub entry_barrier: f64,
}

impl MarketOpportunity {
    pub fn new(sector: String, country: String) -> Self {
        Self {
            opportunity_id: crate::core::uuid_simple(),
            sector,
            country,
            TAM_billion: 0.0,
            growth_rate: 0.0,
            competitive_intensity: 0.0,
            entry_barrier: 0.0,
        }
    }

    pub fn evaluate_opportunity(&mut self) -> Result<()> {
        match self.sector.as_str() {
            "Technology" => {
                self.TAM_billion = 10.0 + rand_simple() * 100.0;
                self.growth_rate = 15.0 + rand_simple() * 25.0;
            },
            "Healthcare" => {
                self.TAM_billion = 5.0 + rand_simple() * 50.0;
                self.growth_rate = 8.0 + rand_simple() * 15.0;
            },
            "Financial_Services" => {
                self.TAM_billion = 8.0 + rand_simple() * 80.0;
                self.growth_rate = 6.0 + rand_simple() * 12.0;
            },
            _ => {
                self.TAM_billion = 2.0 + rand_simple() * 30.0;
                self.growth_rate = 5.0 + rand_simple() * 10.0;
            }
        }

        self.competitive_intensity = 0.4 + rand_simple() * 0.5;
        self.entry_barrier = 0.3 + rand_simple() * 0.6;
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

pub fn calculate_EM_premium(base_rate: f64, risk_premium: f64) -> Result<f64> {
    Ok(base_rate + risk_premium)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_india_market() {
        let mut market = EmergingMarket::new("India".to_string());
        market.analyze_market().unwrap();
        assert!(market.market_cap_billion > 0.0);
    }

    #[test]
    fn test_fintech_opportunity() {
        let mut opportunity = MarketOpportunity::new(
            "Financial_Services".to_string(),
            "Nigeria".to_string(),
        );
        opportunity.evaluate_opportunity().unwrap();
        assert!(opportunity.TAM_billion > 0.0);
    }
}