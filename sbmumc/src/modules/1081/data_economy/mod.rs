//! # SBMUMC Module 1081: Data Economy
//!
//! Economics of data as a production factor.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataMarketType {
    Raw,
    Processed,
    Analytics,
    AI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEconomyModel {
    pub model_id: String,
    pub market_type: DataMarketType,
    pub data_volume_tb: f64,
    pub data_market_value_billion: f64,
    var data_pricing_efficiency: f64,
    pub privacy_impact_score: f64,
    pub market_concentration: f64,
}

impl DataEconomyModel {
    pub fn new(market_type: DataMarketType, volume_tb: f64) -> Self {
        Self {
            model_id: crate::core::uuid_simple(),
            market_type,
            data_volume_tb: volume_tb,
            data_market_value_billion: 0.0,
            var data_pricing_efficiency: 0.0,
            privacy_impact_score: 0.0,
            market_concentration: 0.0,
        }
    }

    pub fn analyze_data_economy(&mut self) -> Result<()> {
        match self.market_type {
            DataMarketType::Raw => {
                self.data_market_value_billion = self.data_volume_tb * 0.00001 * (0.3 + rand_simple() * 0.5);
                self.data_pricing_efficiency = 0.3 + rand_simple() * 0.3;
            },
            DataMarketType::Processed => {
                self.data_market_value_billion = self.data_volume_tb * 0.0001 * (0.5 + rand_simple() * 0.7);
                self.data_pricing_efficiency = 0.5 + rand_simple() * 0.3;
            },
            DataMarketType::Analytics => {
                self.data_market_value_billion = self.data_volume_tb * 0.001 * (0.7 + rand_simple() * 1.0);
                self.data_pricing_efficiency = 0.6 + rand_simple() * 0.3;
            },
            DataMarketType::AI => {
                self.data_market_value_billion = self.data_volume_tb * 0.01 * (1.0 + rand_simple() * 2.0);
                self.data_pricing_efficiency = 0.7 + rand_simple() * 0.25;
            }
        }

        self.privacy_impact_score = 0.1 + rand_simple() * 0.6;
        self.market_concentration = 0.4 + rand_simple() * 0.5;
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

pub fn compute_data_value(data_type: &str, volume_tb: f64) -> Result<f64> {
    let unit_value = match data_type {
        "AI_Training" => 0.01,
        "Analytics" => 0.001,
        "Raw" => 0.00001,
        _ => 0.0001,
    };
    Ok(unit_value * volume_tb * 1e9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_data_market() {
        let mut model = DataEconomyModel::new(DataMarketType::AI, 1_000_000.0);
        model.analyze_data_economy().unwrap();
        assert!(model.data_market_value_billion > 0.0);
    }
}