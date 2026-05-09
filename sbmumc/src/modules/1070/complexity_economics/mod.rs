//! # SBMUMC Module 1070: Complexity Economics
//!
//! Economic systems as complex adaptive systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergenceLevel {
    Micro,
    Meso,
    Macro,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexEconomicSystem {
    pub system_id: String,
    pub system_name: String,
    pub agent_count: usize,
    pub interaction_density: f64,
    pub emergence_level: EmergenceLevel,
    pub self_organization_score: f64,
    pub adaptation_rate: f64,
}

impl ComplexEconomicSystem {
    pub fn new(name: String, agents: usize) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            system_name: name,
            agent_count: agents,
            interaction_density: 0.0,
            emergence_level: EmergenceLevel::Meso,
            self_organization_score: 0.0,
            adaptation_rate: 0.0,
        }
    }

    pub fn analyze_complexity(&mut self) -> Result<()> {
        self.interaction_density = 0.1 + rand_simple() * 0.7;

        if self.agent_count > 1_000_000 {
            self.emergence_level = EmergenceLevel::Macro;
        } else if self.agent_count > 10_000 {
            self.emergence_level = EmergenceLevel::Meso;
        } else {
            self.emergence_level = EmergenceLevel::Micro;
        }

        match self.emergence_level {
            EmergenceLevel::Macro => {
                self.self_organization_score = 0.7 + rand_simple() * 0.25;
                self.adaptation_rate = 0.01 + rand_simple() * 0.03;
            },
            EmergenceLevel::Meso => {
                self.self_organization_score = 0.5 + rand_simple() * 0.35;
                self.adaptation_rate = 0.02 + rand_simple() * 0.05;
            },
            EmergenceLevel::Micro => {
                self.self_organization_score = 0.3 + rand_simple() * 0.40;
                self.adaptation_rate = 0.05 + rand_simple() * 0.10;
            }
        }
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

pub fn compute_network_effects(agent_count: usize) -> Result<f64> {
    Ok((agent_count as f64).ln() / 10.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_complexity() {
        let mut system = ComplexEconomicSystem::new("Stock_Market".to_string(), 10_000_000);
        system.analyze_complexity().unwrap();
        assert!(system.self_organization_score > 0.5);
    }
}