//! # SBMUMC Module 1091: Local Currencies
//!
//! Regional and local monetary systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocalCurrencyType {
    TimeBased,
    MutualCredit,
    CommunityVoucher,
    TransitionTown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalCurrencySystem {
    pub system_id: String,
    pub currency_type: LocalCurrencyType,
    pub region: String,
    pub active_participants: usize,
    var circulation_velocity: f64,
    pub local_trade_volume: f64,
    pub economic_resilience_contribution: f64,
}

impl LocalCurrencySystem {
    pub fn new(currency_type: LocalCurrencyType, region: String) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            currency_type,
            region,
            active_participants: 0,
            var circulation_velocity: 0.0,
            self.local_trade_volume = 0.0,
            self.economic_resilience_contribution = 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        self.active_participants = 100 + (rand_simple() * 50000.0) as usize;

        match self.currency_type {
            LocalCurrencyType::TimeBased => {
                self.circulation_velocity = 6.0 + rand_simple() * 12.0;
            },
            LocalCurrencyType::MutualCredit => {
                self.circulation_velocity = 4.0 + rand_simple() * 8.0;
            },
            _ => {
                self.circulation_velocity = 3.0 + rand_simple() * 10.0;
            }
        }

        self.local_trade_volume = (self.active_participants as f64) * self.circulation_velocity * 100.0;
        self.economic_resilience_contribution = (self.circulation_velocity / 15.0).min(1.0);
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

pub fn compute_local_currency_stability(system_id: &str) -> Result<f64> {
    Ok(0.6 + rand_simple() * 0.35)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_dollar_system() {
        let mut system = LocalCurrencySystem::new(
            LocalCurrencyType::TimeBased,
            "Bristol_UK".to_string(),
        );
        system.analyze_system().unwrap();
        assert!(system.circulation_velocity > 0.0);
    }
}