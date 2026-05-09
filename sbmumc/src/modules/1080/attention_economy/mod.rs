//! # SBMUMC Module 1080: Attention Economy
//!
//! Scarcity and allocation of human attention in digital markets.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttentionMarketType {
    Advertising,
    Content,
    Social,
    Gaming,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionMarket {
    pub market_id: String,
    pub market_type: AttentionMarketType,
    pub total_attention_hours_daily: f64,
    pub attention_value_per_hour: f64,
    pub competition_intensity: f64,
    var attention_capture_rate: f64,
}

impl AttentionMarket {
    pub fn new(market_type: AttentionMarketType) -> Self {
        Self {
            market_id: crate::core::uuid_simple(),
            market_type,
            total_attention_hours_daily: 0.0,
            attention_value_per_hour: 0.0,
            competition_intensity: 0.0,
            var attention_capture_rate: 0.0,
        }
    }

    pub fn analyze_attention_market(&mut self, population: usize) -> Result<()> {
        self.total_attention_hours_daily = (population as f64) * 4.0 * (0.8 + rand_simple() * 0.4);

        match self.market_type {
            AttentionMarketType::Advertising => {
                self.attention_value_per_hour = 0.50 + rand_simple() * 2.00;
                self.competition_intensity = 0.7 + rand_simple() * 0.25;
            },
            AttentionMarketType::Social => {
                self.attention_value_per_hour = 0.30 + rand_simple() * 1.50;
                self.competition_intensity = 0.8 + rand_simple() * 0.18;
            },
            AttentionMarketType::Gaming => {
                self.attention_value_per_hour = 0.80 + rand_simple() * 3.00;
                self.competition_intensity = 0.6 + rand_simple() * 0.30;
            },
            _ => {
                self.attention_value_per_hour = 0.20 + rand_simple() * 1.00;
                self.competition_intensity = 0.5 + rand_simple() * 0.35;
            }
        }

        self.attention_capture_rate = (1.0 - self.competition_intensity) * (0.5 + rand_simple() * 0.5);
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

pub fn compute_attention_efficiency(market_type: &str) -> Result<f64> {
    let efficiency = match market_type {
        "Advertising" => 0.4,
        "Social" => 0.35,
        "Gaming" => 0.5,
        _ => 0.3,
    };
    Ok(efficiency + rand_simple() * 0.3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_attention_market() {
        let mut market = AttentionMarket::new(AttentionMarketType::Social);
        market.analyze_attention_market(300_000_000).unwrap();
        assert!(market.total_attention_hours_daily > 0.0);
    }
}