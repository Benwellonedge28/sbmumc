//! # SBMUMC Module 1053: Trade Systems
//!
//! International trade systems and commercial exchange frameworks.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeAgreementType {
    Bilateral,
    Multilateral,
    Regional,
    Sectoral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeAgreement {
    pub agreement_id: String,
    pub agreement_name: String,
    pub agreement_type: TradeAgreementType,
    pub member_count: usize,
    pub trade_volume_billion: f64,
    pub tariff_reduction_percent: f64,
    pub trade_enhancement_factor: f64,
}

impl TradeAgreement {
    pub fn new(name: String, agreement_type: TradeAgreementType) -> Self {
        Self {
            agreement_id: crate::core::uuid_simple(),
            agreement_name: name,
            agreement_type,
            member_count: 0,
            trade_volume_billion: 0.0,
            tariff_reduction_percent: 0.0,
            trade_enhancement_factor: 0.0,
        }
    }

    pub fn evaluate_agreement(&mut self, members: usize) -> Result<()> {
        self.member_count = members;
        self.tariff_reduction_percent = 30.0 + rand_simple() * 60.0;

        match self.agreement_type {
            TradeAgreementType::Bilateral => {
                self.trade_volume_billion = 50.0 + rand_simple() * 200.0;
                self.trade_enhancement_factor = 1.2 + rand_simple() * 0.4;
            },
            TradeAgreementType::Multilateral => {
                self.trade_volume_billion = 500.0 + rand_simple() * 2000.0;
                self.trade_enhancement_factor = 1.4 + rand_simple() * 0.6;
            },
            TradeAgreementType::Regional => {
                self.trade_volume_billion = 200.0 + rand_simple() * 800.0;
                self.trade_enhancement_factor = 1.3 + rand_simple() * 0.5;
            },
            TradeAgreementType::Sectoral => {
                self.trade_volume_billion = 20.0 + rand_simple() * 100.0;
                self.trade_enhancement_factor = 1.15 + rand_simple() * 0.35;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRoute {
    pub route_id: String,
    pub origin_region: String,
    pub destination_region: String,
    pub distance_km: f64,
    pub throughput_tons: f64,
    pub efficiency_score: f64,
    pub risk_factor: f64,
}

impl TradeRoute {
    pub fn new(origin: String, destination: String, distance: f64) -> Self {
        Self {
            route_id: crate::core::uuid_simple(),
            origin_region: origin,
            destination_region: destination,
            distance_km: distance,
            throughput_tons: 0.0,
            efficiency_score: 0.0,
            risk_factor: 0.0,
        }
    }

    pub fn assess_route(&mut self) -> Result<()> {
        self.throughput_tons = (1e6 / (1.0 + self.distance_km / 10000.0)) * (0.5 + rand_simple() * 1.0);
        self.efficiency_score = 0.6 + rand_simple() * 0.35;
        self.risk_factor = 0.05 + rand_simple() * 0.25;
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

pub fn compute_trade_benefit(route_id: &str) -> Result<f64> {
    Ok(0.1 + rand_simple() * 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regional_agreement() {
        let mut agreement = TradeAgreement::new(
            "ASEAN_Free_Trade_Area".to_string(),
            TradeAgreementType::Regional,
        );
        agreement.evaluate_agreement(10).unwrap();
        assert!(agreement.trade_enhancement_factor > 1.0);
    }

    #[test]
    fn test_trade_route() {
        let mut route = TradeRoute::new("Asia".to_string(), "Europe".to_string(), 10000.0);
        route.assess_route().unwrap();
        assert!(route.throughput_tons > 0.0);
    }
}