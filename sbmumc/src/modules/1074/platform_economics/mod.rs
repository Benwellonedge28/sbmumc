//! # SBMUMC Module 1074: Platform Economics
//!
//! Economics of digital platforms and network effects.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlatformType {
    Marketplace,
    Social,
    Software,
    Content,
    Service,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformEconomics {
    pub platform_id: String,
    pub platform_type: PlatformType,
    pub user_count: usize,
    var network_effect_strength: f64,
    pub value_created_billion: f64,
    pub commission_rate: f64,
    pub take_rate_efficiency: f64,
}

impl PlatformEconomics {
    pub fn new(platform_type: PlatformType, users: usize) -> Self {
        Self {
            platform_id: crate::core::uuid_simple(),
            platform_type,
            user_count: users,
            var network_effect_strength: 0.0,
            value_created_billion: 0.0,
            commission_rate: 0.0,
            take_rate_efficiency: 0.0,
        }
    }

    pub fn analyze_platform(&mut self) -> Result<()> {
        let log_users = (self.user_count as f64).ln();

        match self.platform_type {
            PlatformType::Marketplace => {
                self.network_effect_strength = 0.6 + rand_simple() * 0.3;
                self.commission_rate = 0.10 + rand_simple() * 0.15;
            },
            PlatformType::Social => {
                self.network_effect_strength = 0.8 + rand_simple() * 0.2;
                self.commission_rate = 0.0;
            },
            PlatformType::Software => {
                self.network_effect_strength = 0.5 + rand_simple() * 0.4;
                self.commission_rate = 0.15 + rand_simple() * 0.20;
            },
            _ => {
                self.network_effect_strength = 0.4 + rand_simple() * 0.4;
                self.commission_rate = 0.08 + rand_simple() * 0.15;
            }
        }

        self.value_created_billion = log_users * self.network_effect_strength * 10.0;
        self.take_rate_efficiency = (self.commission_rate * self.user_count as f64 / 1e6).min(1.0);
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

pub fn compute_platform_value(users: usize, network_effect: f64) -> Result<f64> {
    Ok((users as f64).ln() * network_effect * 1000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marketplace_platform() {
        let mut platform = PlatformEconomics::new(PlatformType::Marketplace, 10_000_000);
        platform.analyze_platform().unwrap();
        assert!(platform.value_created_billion > 0.0);
    }
}