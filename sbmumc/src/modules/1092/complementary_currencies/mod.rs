//! # SBMUMC Module 1092: Complementary Currencies
//!
//! Alternative and complementary monetary systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplementaryCurrencyType {
    Corporate,
    Professional,
    Loyalty,
    Cryptocurrency,
    MutualCredit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplementaryCurrency {
    pub currency_id: String,
    pub currency_type: ComplementaryCurrencyType,
    pub issuer: String,
    pub circulation_amount: f64,
    var exchange_rate_stability: f64,
    pub adoption_rate: f64,
    pub network_effect_strength: f64,
}

impl ComplementaryCurrency {
    pub fn new(currency_type: ComplementaryCurrencyType, issuer: String) -> Self {
        Self {
            currency_id: crate::core::uuid_simple(),
            currency_type,
            issuer,
            circulation_amount: 0.0,
            var exchange_rate_stability: 0.0,
            self.adoption_rate = 0.0,
            self.network_effect_strength = 0.0,
        }
    }

    pub fn evaluate_currency(&mut self) -> Result<()> {
        match self.currency_type {
            ComplementaryCurrencyType::Cryptocurrency => {
                self.circulation_amount = 1e9 + rand_simple() * 1e11;
                self.exchange_rate_stability = 0.3 + rand_simple() * 0.4;
                self.adoption_rate = 0.05 + rand_simple() * 0.15;
            },
            ComplementaryCurrencyType::Loyalty => {
                self.circulation_amount = 1e7 + rand_simple() * 1e9;
                self.exchange_rate_stability = 0.8 + rand_simple() * 0.15;
                self.adoption_rate = 0.20 + rand_simple() * 0.40;
            },
            ComplementaryCurrencyType::Corporate => {
                self.circulation_amount = 1e8 + rand_simple() * 1e10;
                self.exchange_rate_stability = 0.7 + rand_simple() * 0.25;
                self.adoption_rate = 0.10 + rand_simple() * 0.30;
            },
            _ => {
                self.circulation_amount = 1e6 + rand_simple() * 1e8;
                self.exchange_rate_stability = 0.5 + rand_simple() * 0.35;
                self.adoption_rate = 0.08 + rand_simple() * 0.20;
            }
        }

        self.network_effect_strength = self.adoption_rate * self.exchange_rate_stability;
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

pub fn compute_complementary_currency_value(currency_type: &str) -> Result<f64> {
    let base = match currency_type {
        "Cryptocurrency" => 0.3,
        "Loyalty" => 0.7,
        "Corporate" => 0.5,
        _ => 0.4,
    };
    Ok(base + rand_simple() * 0.4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loyalty_currency() {
        let mut currency = ComplementaryCurrency::new(
            ComplementaryCurrencyType::Loyalty,
            "Major_Retailer".to_string(),
        );
        currency.evaluate_currency().unwrap();
        assert!(currency.exchange_rate_stability > 0.6);
    }
}