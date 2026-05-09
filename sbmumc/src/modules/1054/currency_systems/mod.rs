//! # SBMUMC Module 1054: Currency Systems
//!
//! National and international currency systems and monetary instruments.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurrencyType {
    Fiat,
    Commodity,
    Digital,
    Crypto,
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencySystem {
    pub currency_id: String,
    pub currency_code: String,
    pub currency_type: CurrencyType,
    pub stability_score: f64,
    pub adoption_rate: f64,
    pub international_reserve_share: f64,
    pub transaction_volume_daily: f64,
}

impl CurrencySystem {
    pub fn new(code: String, currency_type: CurrencyType) -> Self {
        Self {
            currency_id: crate::core::uuid_simple(),
            currency_code: code,
            currency_type,
            stability_score: 0.0,
            adoption_rate: 0.0,
            international_reserve_share: 0.0,
            transaction_volume_daily: 0.0,
        }
    }

    pub fn analyze_currency(&mut self) -> Result<()> {
        match self.currency_type {
            CurrencyType::Fiat => {
                self.stability_score = 0.7 + rand_simple() * 0.3;
                self.adoption_rate = 0.8 + rand_simple() * 0.2;
                self.transaction_volume_daily = 1e12 + rand_simple() * 5e12;
            },
            CurrencyType::Crypto => {
                self.stability_score = 0.2 + rand_simple() * 0.5;
                self.adoption_rate = 0.01 + rand_simple() * 0.1;
                self.transaction_volume_daily = 1e10 + rand_simple() * 1e11;
            },
            CurrencyType::Stable => {
                self.stability_score = 0.75 + rand_simple() * 0.25;
                self.adoption_rate = 0.05 + rand_simple() * 0.15;
                self.transaction_volume_daily = 1e8 + rand_simple() * 1e10;
            },
            _ => {
                self.stability_score = 0.5 + rand_simple() * 0.4;
                self.adoption_rate = 0.2 + rand_simple() * 0.3;
                self.transaction_volume_daily = 1e9 + rand_simple() * 1e11;
            }
        }

        if self.currency_code == "USD" || self.currency_code == "EUR" || self.currency_code == "GBP" {
            self.international_reserve_share = 0.5 + rand_simple() * 0.4;
        } else {
            self.international_reserve_share = 0.01 + rand_simple() * 0.2;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub rate_id: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub exchange_rate: f64,
    pub volatility_30d: f64,
    pub forecast_90d: f64,
}

impl ExchangeRate {
    pub fn new(base: String, quote: String, rate: f64) -> Self {
        Self {
            rate_id: crate::core::uuid_simple(),
            base_currency: base,
            quote_currency: quote,
            exchange_rate: rate,
            volatility_30d: 0.0,
            forecast_90d: 0.0,
        }
    }

    pub fn analyze_rate(&mut self) -> Result<()> {
        self.volatility_30d = 0.01 + rand_simple() * 0.1;
        self.forecast_90d = self.exchange_rate * (1.0 + (rand_simple() - 0.5) * 0.05);
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

pub fn compute_currency_strength(currency_code: &str) -> Result<f64> {
    let base = match currency_code {
        "USD" | "EUR" | "GBP" | "JPY" => 0.85,
        "CNY" => 0.75,
        _ => 0.5,
    };
    Ok(base + rand_simple() * 0.15)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usd_system() {
        let mut currency = CurrencySystem::new("USD".to_string(), CurrencyType::Fiat);
        currency.analyze_currency().unwrap();
        assert!(currency.international_reserve_share > 0.3);
    }

    #[test]
    fn test_exchange_rate() {
        let mut rate = ExchangeRate::new("EUR".to_string(), "USD".to_string(), 1.08);
        rate.analyze_rate().unwrap();
        assert!(rate.forecast_90d > 0.0);
    }
}