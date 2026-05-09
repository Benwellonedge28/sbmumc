//! # SBMUMC Module 1071: Ecological Economics
//!
//! Integration of ecological and economic systems thinking.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EcosystemServiceType {
    Provisioning,
    Regulating,
    Cultural,
    Supporting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemValuation {
    pub valuation_id: String,
    pub ecosystem_type: String,
    pub service_type: EcosystemServiceType,
    pub annual_value_billion: f64,
    pub threat_impact_percent: f64,
    pub conservation_cost_billion: f64,
    pub value_per_hectare: f64,
}

impl EcosystemValuation {
    pub fn new(ecosystem: String, service_type: EcosystemServiceType) -> Self {
        Self {
            valuation_id: crate::core::uuid_simple(),
            ecosystem_type: ecosystem,
            service_type,
            annual_value_billion: 0.0,
            threat_impact_percent: 0.0,
            conservation_cost_billion: 0.0,
            value_per_hectare: 0.0,
        }
    }

    pub fn value_ecosystem(&mut self, area_hectares: f64) -> Result<()> {
        match self.service_type {
            EcosystemServiceType::Provisioning => {
                self.annual_value_billion = 50.0 + rand_simple() * 200.0;
            },
            EcosystemServiceType::Regulating => {
                self.annual_value_billion = 100.0 + rand_simple() * 400.0;
            },
            EcosystemServiceType::Cultural => {
                self.annual_value_billion = 20.0 + rand_simple() * 100.0;
            },
            EcosystemServiceType::Supporting => {
                self.annual_value_billion = 200.0 + rand_simple() * 500.0;
            }
        }

        self.threat_impact_percent = 5.0 + rand_simple() * 25.0;
        self.conservation_cost_billion = self.annual_value_billion * 0.05 + rand_simple() * self.annual_value_billion * 0.10;
        self.value_per_hectare = self.annual_value_billion * 1e9 / area_hectares;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalCapitalAccount {
    pub account_id: String,
    pub country: String,
    pub total_natural_capital_trillion: f64,
    pub renewable_resources_share: f64,
    pub non_renewable_share: f64,
    pub degradation_rate_percent: f64,
    pub sustainability_index: f64,
}

impl NaturalCapitalAccount {
    pub fn new(country: String) -> Self {
        Self {
            account_id: crate::core::uuid_simple(),
            country,
            total_natural_capital_trillion: 0.0,
            renewable_resources_share: 0.0,
            non_renewable_share: 0.0,
            degradation_rate_percent: 0.0,
            sustainability_index: 0.0,
        }
    }

    pub fn compile_account(&mut self) -> Result<()> {
        self.total_natural_capital_trillion = 50.0 + rand_simple() * 950.0;
        self.renewable_resources_share = 0.3 + rand_simple() * 0.4;
        self.non_renewable_share = 1.0 - self.renewable_resources_share;
        self.degradation_rate_percent = 0.5 + rand_simple() * 3.0;
        self.sustainability_index = (self.renewable_resources_share * 0.7) / (1.0 + self.degradation_rate_percent / 10.0);
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

pub fn compute_carbon_price(emissions_tons: f64) -> Result<f64> {
    Ok(emissions_tons * (20.0 + rand_simple() * 60.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forest_valuation() {
        let mut valuation = EcosystemValuation::new(
            "Tropical_Rainforest".to_string(),
            EcosystemServiceType::Regulating,
        );
        valuation.value_ecosystem(100_000_000.0).unwrap();
        assert!(valuation.annual_value_billion > 0.0);
    }

    #[test]
    fn test_natural_capital_account() {
        let mut account = NaturalCapitalAccount::new("Brazil".to_string());
        account.compile_account().unwrap();
        assert!(account.sustainability_index > 0.0);
    }
}