//! # SBMUMC Module 1292: Water Climate Adaptation
//!
//! Systems for adapting water resources to climate change.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdaptationStrategy {
    InfrastructureUpgrades,
    DemandManagement,
    SourceDiversification,
    EcosystemBased,
    PolicyReform,
    CommunityEngagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterClimateAdaptationSystem {
    pub system_id: String,
    pub adaptation_strategy: AdaptationStrategy,
    pub adaptation_effectiveness: f64,
    pub cost_benefit_ratio: f64,
    pub implementation_readiness: f64,
    pub long_term_resilience: f64,
}

impl WaterClimateAdaptationSystem {
    pub fn new(adaptation_strategy: AdaptationStrategy) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            adaptation_strategy,
            adaptation_effectiveness: 0.0,
            cost_benefit_ratio: 0.0,
            implementation_readiness: 0.0,
            long_term_resilience: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.adaptation_strategy {
            AdaptationStrategy::InfrastructureUpgrades => {
                self.adaptation_effectiveness = 0.85 + rand_simple() * 0.14;
                self.implementation_readiness = 0.80 + rand_simple() * 0.18;
                self.cost_benefit_ratio = 0.65 + rand_simple() * 0.30;
            },
            AdaptationStrategy::DemandManagement => {
                self.cost_benefit_ratio = 0.90 + rand_simple() * 0.10;
                self.adaptation_effectiveness = 0.75 + rand_simple() * 0.22;
                self.long_term_resilience = 0.80 + rand_simple() * 0.18;
            },
            AdaptationStrategy::SourceDiversification => {
                self.long_term_resilience = 0.90 + rand_simple() * 0.10;
                self.adaptation_effectiveness = 0.80 + rand_simple() * 0.18;
                self.implementation_readiness = 0.70 + rand_simple() * 0.25;
            },
            AdaptationStrategy::EcosystemBased => {
                self.adaptation_effectiveness = 0.85 + rand_simple() * 0.14;
                self.long_term_resilience = 0.90 + rand_simple() * 0.10;
                self.cost_benefit_ratio = 0.75 + rand_simple() * 0.22;
            },
            AdaptationStrategy::PolicyReform => {
                self.implementation_readiness = 0.85 + rand_simple() * 0.14;
                self.adaptation_effectiveness = 0.75 + rand_simple() * 0.22;
                self.cost_benefit_ratio = 0.80 + rand_simple() * 0.18;
            },
            AdaptationStrategy::CommunityEngagement => {
                self.long_term_resilience = 0.85 + rand_simple() * 0.14;
                self.implementation_readiness = 0.90 + rand_simple() * 0.10;
                self.adaptation_effectiveness = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.cost_benefit_ratio == 0.0 {
            self.cost_benefit_ratio = (self.adaptation_effectiveness + self.long_term_resilience) / 2.0 * (0.6 + rand_simple() * 0.3);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecosystem_based() {
        let mut system = WaterClimateAdaptationSystem::new(AdaptationStrategy::EcosystemBased);
        system.analyze_system().unwrap();
        assert!(system.long_term_resilience > 0.7);
    }
}
