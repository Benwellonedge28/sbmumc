//! # SBMUMC Module 1045: Financial Systems
//!
//! Modern financial systems, markets, and institutions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FinancialMarketType {
    Stock,
    Bond,
    Currency,
    Commodity,
    Derivatives,
    Crypto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialMarket {
    pub market_id: String,
    pub market_type: FinancialMarketType,
    pub market_name: String,
    pub volume_traded: f64,
    pub volatility_index: f64,
    pub liquidity_score: f64,
    pub efficiency_score: f64,
}

impl FinancialMarket {
    pub fn new(market_type: FinancialMarketType, name: String) -> Self {
        Self {
            market_id: crate::core::uuid_simple(),
            market_type,
            market_name: name,
            volume_traded: 0.0,
            volatility_index: 0.0,
            liquidity_score: 0.0,
            efficiency_score: 0.0,
        }
    }

    pub fn analyze_market(&mut self) -> Result<()> {
        match self.market_type {
            FinancialMarketType::Stock => {
                self.volume_traded = 1e12 + rand_simple() * 5e12;
                self.volatility_index = 15.0 + rand_simple() * 30.0;
            },
            FinancialMarketType::Crypto => {
                self.volume_traded = 1e10 + rand_simple() * 1e11;
                self.volatility_index = 40.0 + rand_simple() * 60.0;
            },
            _ => {
                self.volume_traded = 1e11 + rand_simple() * 1e12;
                self.volatility_index = 10.0 + rand_simple() * 20.0;
            }
        }

        self.liquidity_score = 0.5 + rand_simple() * 0.5;
        self.efficiency_score = (1.0 / (1.0 + self.volatility_index / 100.0)) * self.liquidity_score;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialInstitution {
    pub institution_id: String,
    pub institution_type: String,
    pub assets_under_management: f64,
    pub risk_appetite: f64,
    pub regulatory_compliance_score: f64,
    pub stability_index: f64,
}

impl FinancialInstitution {
    pub fn new(institution_type: String) -> Self {
        Self {
            institution_id: crate::core::uuid_simple(),
            institution_type,
            assets_under_management: 0.0,
            risk_appetite: 0.0,
            regulatory_compliance_score: 0.0,
            stability_index: 0.0,
        }
    }

    pub fn assess_health(&mut self, assets: f64) -> Result<()> {
        self.assets_under_management = assets;
        self.risk_appetite = 0.3 + rand_simple() * 0.5;
        self.regulatory_compliance_score = 0.85 + rand_simple() * 0.15;
        self.stability_index = self.regulatory_compliance_score * (1.0 - self.risk_appetite * 0.3);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialProduct {
    pub product_id: String,
    pub product_name: String,
    pub product_type: String,
    pub expected_return: f64,
    pub risk_level: f64,
    pub complexity_score: u8,
}

impl FinancialProduct {
    pub fn new(name: String, product_type: String) -> Self {
        Self {
            product_id: crate::core::uuid_simple(),
            product_name: name,
            product_type,
            expected_return: 0.0,
            risk_level: 0.0,
            complexity_score: 0,
        }
    }

    pub fn price_product(&mut self) -> Result<()> {
        match self.product_type.as_str() {
            "Equity" => {
                self.expected_return = 0.08 + rand_simple() * 0.12;
                self.risk_level = 0.15 + rand_simple() * 0.20;
            },
            "Fixed_Income" => {
                self.expected_return = 0.03 + rand_simple() * 0.05;
                self.risk_level = 0.05 + rand_simple() * 0.10;
            },
            "Derivative" => {
                self.expected_return = rand_simple() * 0.30;
                self.risk_level = 0.30 + rand_simple() * 0.50;
            },
            _ => {
                self.expected_return = 0.05 + rand_simple() * 0.10;
                self.risk_level = 0.10 + rand_simple() * 0.15;
            }
        }
        self.complexity_score = ((self.risk_level * 10.0) as u8).min(10);
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

pub fn compute_market_risk(market_type: &str) -> Result<f64> {
    let base = match market_type {
        "Crypto" => 0.8,
        "Derivatives" => 0.6,
        "Stock" => 0.4,
        _ => 0.3,
    };
    Ok(base + rand_simple() * 0.2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_market() {
        let mut market = FinancialMarket::new(
            FinancialMarketType::Stock,
            "NYSE".to_string(),
        );
        market.analyze_market().unwrap();
        assert!(market.volume_traded > 0.0);
    }

    #[test]
    fn test_financial_institution() {
        let mut institution = FinancialInstitution::new("Commercial_Bank".to_string());
        institution.assess_health(1e9).unwrap();
        assert!(institution.stability_index > 0.0);
    }

    #[test]
    fn test_financial_product() {
        let mut product = FinancialProduct::new(
            "S&P500_ETF".to_string(),
            "Equity".to_string(),
        );
        product.price_product().unwrap();
        assert!(product.expected_return > 0.0);
    }
}