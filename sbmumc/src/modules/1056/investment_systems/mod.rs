//! # SBMUMC Module 1056: Investment Systems
//!
//! Investment strategies, vehicles, and portfolio management.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvestmentStrategy {
    Growth,
    Value,
    Income,
    Index,
    Quantitative,
    SociallyResponsible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentPortfolio {
    pub portfolio_id: String,
    pub investor_profile: String,
    pub strategy: InvestmentStrategy,
    pub expected_return: f64,
    pub risk_level: f64,
    pub diversification_score: f64,
    pub sharpe_ratio: f64,
}

impl InvestmentPortfolio {
    pub fn new(profile: String, strategy: InvestmentStrategy) -> Self {
        Self {
            portfolio_id: crate::core::uuid_simple(),
            investor_profile: profile,
            strategy,
            expected_return: 0.0,
            risk_level: 0.0,
            diversification_score: 0.0,
            sharpe_ratio: 0.0,
        }
    }

    pub fn construct_portfolio(&mut self) -> Result<()> {
        match self.strategy {
            InvestmentStrategy::Growth => {
                self.expected_return = 0.10 + rand_simple() * 0.08;
                self.risk_level = 0.20 + rand_simple() * 0.15;
            },
            InvestmentStrategy::Value => {
                self.expected_return = 0.07 + rand_simple() * 0.05;
                self.risk_level = 0.15 + rand_simple() * 0.10;
            },
            InvestmentStrategy::Income => {
                self.expected_return = 0.04 + rand_simple() * 0.03;
                self.risk_level = 0.08 + rand_simple() * 0.07;
            },
            InvestmentStrategy::Index => {
                self.expected_return = 0.06 + rand_simple() * 0.04;
                self.risk_level = 0.15 + rand_simple() * 0.10;
            },
            InvestmentStrategy::SociallyResponsible => {
                self.expected_return = 0.07 + rand_simple() * 0.05;
                self.risk_level = 0.12 + rand_simple() * 0.08;
            },
            _ => {
                self.expected_return = 0.08 + rand_simple() * 0.06;
                self.risk_level = 0.15 + rand_simple() * 0.12;
            }
        }

        self.diversification_score = 0.6 + rand_simple() * 0.35;
        self.sharpe_ratio = (self.expected_return - 0.02) / self.risk_level;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentVehicle {
    pub vehicle_id: String,
    pub vehicle_type: String,
    pub min_investment: f64,
    pub management_fee_percent: f64,
    pub liquidity_score: f64,
    pub tax_efficiency: f64,
}

impl InvestmentVehicle {
    pub fn new(vehicle_type: String, min_investment: f64) -> Self {
        Self {
            vehicle_id: crate::core::uuid_simple(),
            vehicle_type,
            min_investment,
            management_fee_percent: 0.0,
            liquidity_score: 0.0,
            tax_efficiency: 0.0,
        }
    }

    pub fn evaluate_vehicle(&mut self) -> Result<()> {
        match self.vehicle_type.as_str() {
            "ETF" => {
                self.management_fee_percent = 0.03 + rand_simple() * 0.10;
                self.liquidity_score = 0.95 + rand_simple() * 0.05;
                self.tax_efficiency = 0.85 + rand_simple() * 0.15;
            },
            "Mutual_Fund" => {
                self.management_fee_percent = 0.50 + rand_simple() * 0.80;
                self.liquidity_score = 0.80 + rand_simple() * 0.15;
                self.tax_efficiency = 0.60 + rand_simple() * 0.20;
            },
            "Hedge_Fund" => {
                self.management_fee_percent = 1.50 + rand_simple() * 1.00;
                self.liquidity_score = 0.40 + rand_simple() * 0.30;
                self.tax_efficiency = 0.55 + rand_simple() * 0.25;
            },
            "Private_Equity" => {
                self.management_fee_percent = 1.00 + rand_simple() * 1.00;
                self.liquidity_score = 0.20 + rand_simple() * 0.25;
                self.tax_efficiency = 0.70 + rand_simple() * 0.20;
            },
            _ => {
                self.management_fee_percent = 0.20 + rand_simple() * 0.40;
                self.liquidity_score = 0.70 + rand_simple() * 0.25;
                self.tax_efficiency = 0.65 + rand_simple() * 0.25;
            }
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

pub fn optimize_portfolio_allocation(risk_tolerance: f64) -> Result<Vec<f64>> {
    let stocks = risk_tolerance * 0.6;
    let bonds = (1.0 - risk_tolerance) * 0.5;
    let alternatives = risk_tolerance * 0.3;
    let cash = 1.0 - stocks - bonds - alternatives;

    Ok(vec![stocks, bonds, alternatives.max(0.0), cash.max(0.0)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growth_portfolio() {
        let mut portfolio = InvestmentPortfolio::new(
            "Aggressive_Grower".to_string(),
            InvestmentStrategy::Growth,
        );
        portfolio.construct_portfolio().unwrap();
        assert!(portfolio.sharpe_ratio > 0.0);
    }

    #[test]
    fn test_etf_vehicle() {
        let mut vehicle = InvestmentVehicle::new("ETF".to_string(), 100.0);
        vehicle.evaluate_vehicle().unwrap();
        assert!(vehicle.liquidity_score > 0.9);
    }
}