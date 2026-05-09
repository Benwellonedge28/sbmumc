//! # SBMUMC Module 1104: Intellectual Property Law
//!
//! Patents, copyrights, trademarks, and trade secrets.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IPType {
    Patent,
    Copyright,
    Trademark,
    TradeSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntellectualPropertySystem {
    pub system_id: String,
    pub ip_type: IPType,
    pub registration_efficiency: f64,
    var enforcement_strength: f64,
    pub innovation_incentive_effect: f64,
    pub public_domain_balance: f64,
}

impl IntellectualPropertySystem {
    pub fn new(ip_type: IPType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            ip_type,
            registration_efficiency: 0.0,
            var enforcement_strength: 0.0,
            self.innovation_incentive_effect = 0.0,
            self.public_domain_balance = 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.ip_type {
            IPType::Patent => {
                self.registration_efficiency = 0.60 + rand_simple() * 0.30;
                self.enforcement_strength = 0.70 + rand_simple() * 0.25;
            },
            IPType::Copyright => {
                self.registration_efficiency = 0.85 + rand_simple() * 0.15;
                self.enforcement_strength = 0.65 + rand_simple() * 0.30;
            },
            IPType::Trademark => {
                self.registration_efficiency = 0.75 + rand_simple() * 0.20;
                self.enforcement_strength = 0.70 + rand_simple() * 0.25;
            },
            IPType::TradeSecret => {
                self.registration_efficiency = 0.0;
                self.enforcement_strength = 0.55 + rand_simple() * 0.35;
            }
        }

        self.innovation_incentive_effect = self.enforcement_strength * (0.7 + rand_simple() * 0.3);
        self.public_domain_balance = (1.0 - self.enforcement_strength) * 0.6 + self.registration_efficiency * 0.4;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patent_system() {
        let mut system = IntellectualPropertySystem::new(IPType::Patent);
        system.analyze_system().unwrap();
        assert!(system.enforcement_strength > 0.5);
    }
}