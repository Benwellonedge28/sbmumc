//! # SBMUMC Module 1060: Monetary Policy
//!
//! Central bank policy, interest rates, and money supply.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyRateType {
    TargetRate,
    DiscountRate,
    ReserveRequirement,
    QEProgram,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonetaryPolicy {
    pub policy_id: String,
    pub central_bank: String,
    pub policy_rate_type: PolicyRateType,
    pub policy_rate: f64,
    pub inflation_target: f64,
    pub actual_inflation: f64,
    pub policy_effectiveness: f64,
}

impl MonetaryPolicy {
    pub fn new(central_bank: String, rate_type: PolicyRateType) -> Self {
        Self {
            policy_id: crate::core::uuid_simple(),
            central_bank,
            policy_rate_type,
            policy_rate: 0.0,
            inflation_target: 0.0,
            actual_inflation: 0.0,
            policy_effectiveness: 0.0,
        }
    }

    pub fn set_policy_rate(&mut self, rate: f64) -> Result<()> {
        self.policy_rate = rate;
        self.inflation_target = 2.0;
        self.actual_inflation = rate * 0.5 + rand_simple() * 3.0;

        let inflation_gap = (self.actual_inflation - self.inflation_target).abs();
        self.policy_effectiveness = (1.0 - inflation_gap / 10.0).max(0.0);
        Ok(())
    }

    pub fn adjust_rate(&mut self, direction: f64) -> f64 {
        self.policy_rate += direction * 0.25;
        self.policy_rate = self.policy_rate.max(0.0).min(20.0);
        self.set_policy_rate(self.policy_rate).unwrap_or(self.policy_rate);
        self.policy_rate
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoneySupply {
    pub supply_id: String,
    pub m0_base: f64,
    pub m1_narrow: f64,
    pub m2_broad: f64,
    pub m3_broadest: f64,
    pub velocity_of_money: f64,
    pub money_multiplier: f64,
}

impl MoneySupply {
    pub fn new() -> Self {
        Self {
            supply_id: crate::core::uuid_simple(),
            m0_base: 0.0,
            m1_narrow: 0.0,
            m2_broad: 0.0,
            m3_broadest: 0.0,
            velocity_of_money: 0.0,
            money_multiplier: 0.0,
        }
    }

    pub fn analyze_supply(&mut self, base_money: f64, reserve_ratio: f64) -> Result<()> {
        self.m0_base = base_money;
        self.money_multiplier = 1.0 / reserve_ratio;

        self.m1_narrow = self.m0_base * self.money_multiplier * 0.8;
        self.m2_broad = self.m1_narrow * 1.5;
        self.m3_broadest = self.m2_broad * 2.0;

        self.velocity_of_money = 1.5 + rand_simple() * 1.5;
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

pub fn compute_optimal_rate(inflation_gap: f64, unemployment_gap: f64) -> Result<f64> {
    let natural_rate = 2.0;
    let phillips_weight = 0.5;
    let rate = natural_rate + inflation_gap * phillips_weight - unemployment_gap * (1.0 - phillips_weight);
    Ok(rate.max(0.0).min(15.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fed_policy() {
        let mut policy = MonetaryPolicy::new(
            "Federal_Reserve".to_string(),
            PolicyRateType::TargetRate,
        );
        policy.set_policy_rate(5.25).unwrap();
        assert!(policy.policy_effectiveness > 0.0);
    }

    #[test]
    fn test_money_supply() {
        let mut supply = MoneySupply::new();
        supply.analyze_supply(5e12, 0.10).unwrap();
        assert!(supply.m2_broad > supply.m1_narrow);
    }
}