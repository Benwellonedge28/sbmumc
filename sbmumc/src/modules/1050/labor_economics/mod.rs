//! # SBMUMC Module 1050: Labor Economics
//!
//! Economic analysis of labor markets, wages, and employment.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaborMarketCondition {
    Tight,
    Loose,
    Equilibrium,
    Stagnant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborMarket {
    pub market_id: String,
    pub market_name: String,
    pub condition: LaborMarketCondition,
    pub unemployment_rate: f64,
    pub wage_growth_rate: f64,
    pub job_vacancy_rate: f64,
    pub labor_force_participation: f64,
}

impl LaborMarket {
    pub fn new(name: String) -> Self {
        Self {
            market_id: crate::core::uuid_simple(),
            market_name: name,
            condition: LaborMarketCondition::Equilibrium,
            unemployment_rate: 0.0,
            wage_growth_rate: 0.0,
            job_vacancy_rate: 0.0,
            labor_force_participation: 0.0,
        }
    }

    pub fn analyze_market(&mut self) -> Result<()> {
        self.unemployment_rate = 3.0 + rand_simple() * 8.0;
        self.wage_growth_rate = 1.0 + rand_simple() * 5.0;
        self.job_vacancy_rate = 2.0 + rand_simple() * 8.0;
        self.labor_force_participation = 60.0 + rand_simple() * 20.0;

        let uv_ratio = self.job_vacancy_rate / (self.unemployment_rate + 0.1);
        self.condition = if uv_ratio > 1.5 {
            LaborMarketCondition::Tight
        } else if uv_ratio < 0.5 {
            LaborMarketCondition::Loose
        } else {
            LaborMarketCondition::Equilibrium
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WageDetermination {
    pub wage_id: String,
    pub occupation: String,
    pub education_years: u8,
    pub experience_years: u8,
    pub base_wage: f64,
    pub wage_premium_factors: Vec<f64>,
}

impl WageDetermination {
    pub fn new(occupation: String, education: u8, experience: u8) -> Self {
        Self {
            wage_id: crate::core::uuid_simple(),
            occupation,
            education_years: education,
            experience_years: experience,
            base_wage: 0.0,
            wage_premium_factors: Vec::new(),
        }
    }

    pub fn calculate_wage(&mut self) -> Result<()> {
        self.base_wage = 20000.0 + (self.education_years as f64 * 3000.0)
            + (self.experience_years as f64 * 2000.0);
        self.base_wage += rand_simple() * self.base_wage * 0.2;

        self.wage_premium_factors = vec![
            1.0 + rand_simple() * 0.1,
            1.0 + rand_simple() * 0.05,
            1.0 + rand_simple() * 0.15,
        ];

        for premium in &mut self.wage_premium_factors {
            self.base_wage *= premium;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentContract {
    pub contract_id: String,
    pub contract_type: String,
    pub job_security_score: f64,
    pub benefits_level: f64,
    pub flexibility_score: f64,
    pub total_compensation: f64,
}

impl EmploymentContract {
    pub fn new(contract_type: String) -> Self {
        Self {
            contract_id: crate::core::uuid_simple(),
            contract_type,
            job_security_score: 0.0,
            benefits_level: 0.0,
            flexibility_score: 0.0,
            total_compensation: 0.0,
        }
    }

    pub fn evaluate_contract(&mut self) -> Result<()> {
        match self.contract_type.as_str() {
            "Permanent" => {
                self.job_security_score = 0.85 + rand_simple() * 0.15;
                self.benefits_level = 0.7 + rand_simple() * 0.3;
                self.flexibility_score = 0.4 + rand_simple() * 0.2;
            },
            "Contract" => {
                self.job_security_score = 0.4 + rand_simple() * 0.3;
                self.benefits_level = 0.5 + rand_simple() * 0.3;
                self.flexibility_score = 0.7 + rand_simple() * 0.2;
            },
            "Gig" => {
                self.job_security_score = 0.2 + rand_simple() * 0.2;
                self.benefits_level = 0.1 + rand_simple() * 0.2;
                self.flexibility_score = 0.9 + rand_simple() * 0.1;
            },
            _ => {
                self.job_security_score = 0.5 + rand_simple() * 0.3;
                self.benefits_level = 0.5 + rand_simple() * 0.3;
                self.flexibility_score = 0.5 + rand_simple() * 0.3;
            }
        }

        self.total_compensation = (self.job_security_score + self.benefits_level + self.flexibility_score) * 30000.0;
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

pub fn predict_wage_growth(market_id: &str) -> Result<f64> {
    Ok(1.5 + rand_simple() * 4.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_labor_market_analysis() {
        let mut market = LaborMarket::new("Tech_Industry".to_string());
        market.analyze_market().unwrap();
        assert!(market.unemployment_rate > 0.0);
    }

    #[test]
    fn test_wage_calculation() {
        let mut wage = WageDetermination::new("Software_Engineer".to_string(), 16, 5);
        wage.calculate_wage().unwrap();
        assert!(wage.base_wage > 50000.0);
    }

    #[test]
    fn test_contract_evaluation() {
        let mut contract = EmploymentContract::new("Permanent".to_string());
        contract.evaluate_contract().unwrap();
        assert!(contract.total_compensation > 0.0);
    }
}