//! # SBMUMC Module 1063: Development Economics
//!
//! Economic development, growth theories, and development strategies.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DevelopmentStage {
    LowIncome,
    LowerMiddleIncome,
    UpperMiddleIncome,
    HighIncome,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentEconomics {
    pub development_id: String,
    pub country: String,
    pub stage: DevelopmentStage,
    pub gdp_per_capita: f64,
    pub human_development_index: f64,
    pub poverty_rate: f64,
    pub growth_potential: f64,
}

impl DevelopmentEconomics {
    pub fn new(country: String) -> Self {
        Self {
            development_id: crate::core::uuid_simple(),
            country,
            stage: DevelopmentStage::LowIncome,
            gdp_per_capita: 0.0,
            human_development_index: 0.0,
            poverty_rate: 0.0,
            growth_potential: 0.0,
        }
    }

    pub fn classify_stage(&mut self) -> Result<()> {
        if self.gdp_per_capita < 1046.0 {
            self.stage = DevelopmentStage::LowIncome;
        } else if self.gdp_per_capita < 4096.0 {
            self.stage = DevelopmentStage::LowerMiddleIncome;
        } else if self.gdp_per_capita < 12696.0 {
            self.stage = DevelopmentStage::UpperMiddleIncome;
        } else {
            self.stage = DevelopmentStage::HighIncome;
        }

        match self.stage {
            DevelopmentStage::LowIncome => {
                self.human_development_index = 0.3 + rand_simple() * 0.2;
                self.poverty_rate = 30.0 + rand_simple() * 40.0;
                self.growth_potential = 5.0 + rand_simple() * 4.0;
            },
            DevelopmentStage::LowerMiddleIncome => {
                self.human_development_index = 0.5 + rand_simple() * 0.2;
                self.poverty_rate = 15.0 + rand_simple() * 25.0;
                self.growth_potential = 4.0 + rand_simple() * 3.0;
            },
            DevelopmentStage::UpperMiddleIncome => {
                self.human_development_index = 0.7 + rand_simple() * 0.15;
                self.poverty_rate = 5.0 + rand_simple() * 10.0;
                self.growth_potential = 3.0 + rand_simple() * 2.0;
            },
            DevelopmentStage::HighIncome => {
                self.human_development_index = 0.85 + rand_simple() * 0.12;
                self.poverty_rate = 1.0 + rand_simple() * 5.0;
                self.growth_potential = 1.5 + rand_simple() * 1.5;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentStrategy {
    pub strategy_id: String,
    pub country: String,
    pub strategy_type: String,
    pub industrialization_priority: f64,
    pub infrastructure_investment: f64,
    pub human_capital_investment: f64,
    pub expected_growth_rate: f64,
}

impl DevelopmentStrategy {
    pub fn new(country: String, strategy_type: String) -> Self {
        Self {
            strategy_id: crate::core::uuid_simple(),
            country,
            strategy_type,
            industrialization_priority: 0.0,
            infrastructure_investment: 0.0,
            human_capital_investment: 0.0,
            expected_growth_rate: 0.0,
        }
    }

    pub fn design_strategy(&mut self) -> Result<()> {
        match self.strategy_type.as_str() {
            "Export_Promotion" => {
                self.industrialization_priority = 0.6 + rand_simple() * 0.3;
                self.infrastructure_investment = 0.25 + rand_simple() * 0.15;
                self.human_capital_investment = 0.15 + rand_simple() * 0.10;
            },
            "Import_Substitution" => {
                self.industrialization_priority = 0.5 + rand_simple() * 0.25;
                self.infrastructure_investment = 0.30 + rand_simple() * 0.15;
                self.human_capital_investment = 0.20 + rand_simple() * 0.15;
            },
            "Balanced_Growth" => {
                self.industrialization_priority = 0.33 + rand_simple() * 0.10;
                self.infrastructure_investment = 0.33 + rand_simple() * 0.10;
                self.human_capital_investment = 0.34 + rand_simple() * 0.10;
            },
            _ => {
                self.industrialization_priority = 0.4 + rand_simple() * 0.3;
                self.infrastructure_investment = 0.3 + rand_simple() * 0.2;
                self.human_capital_investment = 0.3 + rand_simple() * 0.2;
            }
        }

        self.expected_growth_rate = (self.industrialization_priority * 4.0
            + self.infrastructure_investment * 3.0
            + self.human_capital_investment * 3.5) / 10.0;
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

pub fn estimate_convergence_rate(country_income: f64, frontier_income: f64) -> Result<f64> {
    let gap_ratio = country_income / frontier_income;
    Ok((1.0 - gap_ratio) * (0.02 + rand_simple() * 0.04))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_country_classification() {
        let mut development = DevelopmentEconomics::new("Vietnam".to_string());
        development.gdp_per_capita = 3500.0;
        development.classify_stage().unwrap();
        assert_eq!(development.stage, DevelopmentStage::LowerMiddleIncome);
    }

    #[test]
    fn test_export_strategy() {
        let mut strategy = DevelopmentStrategy::new(
            "Bangladesh".to_string(),
            "Export_Promotion".to_string(),
        );
        strategy.design_strategy().unwrap();
        assert!(strategy.expected_growth_rate > 2.0);
    }
}