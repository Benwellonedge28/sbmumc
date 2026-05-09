//! # SBMUMC Module 1066: Global Finance
//!
//! International financial systems and global capital flows.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapitalFlowType {
    ForeignDirectInvestment,
    PortfolioInvestment,
    CommercialLoans,
    Remittances,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalFinance {
    pub finance_id: String,
    pub flow_type: CapitalFlowType,
    pub origin_region: String,
    pub destination_region: String,
    pub flow_volume_billion: f64,
    pub volatility: f64,
    pub systemic_risk_contribution: f64,
}

impl GlobalFinance {
    pub fn new(flow_type: CapitalFlowType, origin: String, destination: String) -> Self {
        Self {
            finance_id: crate::core::uuid_simple(),
            flow_type,
            origin_region: origin,
            destination_region: destination,
            flow_volume_billion: 0.0,
            volatility: 0.0,
            systemic_risk_contribution: 0.0,
        }
    }

    pub fn analyze_flow(&mut self) -> Result<()> {
        match self.flow_type {
            CapitalFlowType::ForeignDirectInvestment => {
                self.flow_volume_billion = 10.0 + rand_simple() * 500.0;
                self.volatility = 0.1 + rand_simple() * 0.2;
            },
            CapitalFlowType::PortfolioInvestment => {
                self.flow_volume_billion = 50.0 + rand_simple() * 1000.0;
                self.volatility = 0.3 + rand_simple() * 0.5;
            },
            CapitalFlowType::Remittances => {
                self.flow_volume_billion = 20.0 + rand_simple() * 300.0;
                self.volatility = 0.05 + rand_simple() * 0.15;
            },
            _ => {
                self.flow_volume_billion = 5.0 + rand_simple() * 200.0;
                self.volatility = 0.15 + rand_simple() * 0.35;
            }
        }

        self.systemic_risk_contribution = self.flow_volume_billion.log10() * self.volatility / 10.0;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalReserve {
    pub reserve_id: String,
    pub country: String,
    pub reserve_amount_billion: f64,
    pub currency_composition: Vec<(String, f64)>,
    pub adequacy_ratio: f64,
    pub liquidity_score: f64,
}

impl InternationalReserve {
    pub fn new(country: String, amount: f64) -> Self {
        Self {
            reserve_id: crate::core::uuid_simple(),
            country,
            reserve_amount_billion: amount,
            currency_composition: Vec::new(),
            adequacy_ratio: 0.0,
            liquidity_score: 0.0,
        }
    }

    pub fn assess_reserves(&mut self, import_coverage_months: f64) -> Result<()> {
        self.currency_composition = vec![
            ("USD".to_string(), 0.5 + rand_simple() * 0.3),
            ("EUR".to_string(), 0.15 + rand_simple() * 0.20),
            ("Gold".to_string(), 0.05 + rand_simple() * 0.15),
            ("Other".to_string(), 0.1 + rand_simple() * 0.15),
        ];

        self.adequacy_ratio = self.reserve_amount_billion / (import_coverage_months * 12.0);
        self.liquidity_score = if self.adequacy_ratio > 1.0 { 0.9 } else { 0.6 } + rand_simple() * 0.1;
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

pub fn compute_flow_stability(flow_type: &str) -> Result<f64> {
    let stability = match flow_type {
        "FDI" => 0.8,
        "Portfolio" => 0.4,
        "Loans" => 0.5,
        _ => 0.6,
    };
    Ok(stability + rand_simple() * 0.2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fdi_flow() {
        let mut flow = GlobalFinance::new(
            CapitalFlowType::ForeignDirectInvestment,
            "United_States".to_string(),
            "Southeast_Asia".to_string(),
        );
        flow.analyze_flow().unwrap();
        assert!(flow.flow_volume_billion > 0.0);
    }

    #[test]
    fn test_reserve_adequacy() {
        let mut reserves = InternationalReserve::new("China".to_string(), 3000.0);
        reserves.assess_reserves(18.0).unwrap();
        assert!(reserves.adequacy_ratio > 0.0);
    }
}