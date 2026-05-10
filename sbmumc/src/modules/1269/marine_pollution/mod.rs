//! # SBMUMC Module 1269: Marine Pollution
//!
//! Systems for monitoring and mitigating marine pollution.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PollutionSource {
    PlasticDebris,
    OilSpills,
    ChemicalRunoff,
    HeavyMetals,
    NutrientPollution,
    NoisePollution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarinePollutionSystem {
    pub system_id: String,
    pub pollution_source: PollutionSource,
    pub pollution_level: f64,
    pub cleanup_efficiency: f64,
    pub ecosystem_impact: f64,
    pub mitigation_effectiveness: f64,
}

impl MarinePollutionSystem {
    pub fn new(pollution_source: PollutionSource) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            pollution_source,
            pollution_level: 0.0,
            cleanup_efficiency: 0.0,
            ecosystem_impact: 0.0,
            mitigation_effectiveness: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.pollution_source {
            PollutionSource::PlasticDebris => {
                self.pollution_level = 0.70 + rand_simple() * 0.25;
                self.cleanup_efficiency = 0.50 + rand_simple() * 0.40;
                self.ecosystem_impact = 0.75 + rand_simple() * 0.22;
            },
            PollutionSource::OilSpills => {
                self.cleanup_efficiency = 0.60 + rand_simple() * 0.35;
                self.ecosystem_impact = 0.85 + rand_simple() * 0.14;
                self.mitigation_effectiveness = 0.70 + rand_simple() * 0.25;
            },
            PollutionSource::ChemicalRunoff => {
                self.pollution_level = 0.65 + rand_simple() * 0.30;
                self.ecosystem_impact = 0.80 + rand_simple() * 0.18;
                self.mitigation_effectiveness = 0.55 + rand_simple() * 0.40;
            },
            PollutionSource::HeavyMetals => {
                self.ecosystem_impact = 0.90 + rand_simple() * 0.10;
                self.mitigation_effectiveness = 0.45 + rand_simple() * 0.40;
                self.cleanup_efficiency = 0.40 + rand_simple() * 0.35;
            },
            PollutionSource::NutrientPollution => {
                self.pollution_level = 0.60 + rand_simple() * 0.35;
                self.mitigation_effectiveness = 0.65 + rand_simple() * 0.30;
                self.ecosystem_impact = 0.70 + rand_simple() * 0.25;
            },
            PollutionSource::NoisePollution => {
                self.ecosystem_impact = 0.65 + rand_simple() * 0.30;
                self.mitigation_effectiveness = 0.55 + rand_simple() * 0.40;
                self.pollution_level = 0.55 + rand_simple() * 0.40;
            },
        }

        if self.cleanup_efficiency == 0.0 {
            self.cleanup_efficiency = (self.mitigation_effectiveness + (1.0 - self.pollution_level)) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_oil_spills() {
        let mut system = MarinePollutionSystem::new(PollutionSource::OilSpills);
        system.analyze_system().unwrap();
        assert!(system.ecosystem_impact > 0.7);
    }
}
