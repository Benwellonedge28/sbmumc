//! # SBMUMC Module 1283: Ice Science
//!
//! Systems for studying ice in aquatic and polar environments.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IceScienceFocus {
    SeaIceDynamics,
    GlacierBehavior,
    IceShelfStability,
    IcebergTracking,
    IceChemistry,
    ClimateIceInteraction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceScienceSystem {
    pub system_id: String,
    pub science_focus: IceScienceFocus,
    pub measurement_accuracy: f64,
    pub modeling_capability: f64,
    pub climate_relevance: f64,
    pub monitoring_coverage: f64,
}

impl IceScienceSystem {
    pub fn new(science_focus: IceScienceFocus) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            science_focus,
            measurement_accuracy: 0.0,
            modeling_capability: 0.0,
            climate_relevance: 0.0,
            monitoring_coverage: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.science_focus {
            IceScienceFocus::SeaIceDynamics => {
                self.measurement_accuracy = 0.85 + rand_simple() * 0.14;
                self.monitoring_coverage = 0.90 + rand_simple() * 0.10;
                self.climate_relevance = 0.85 + rand_simple() * 0.14;
            },
            IceScienceFocus::GlacierBehavior => {
                self.modeling_capability = 0.80 + rand_simple() * 0.18;
                self.measurement_accuracy = 0.75 + rand_simple() * 0.22;
                self.climate_relevance = 0.90 + rand_simple() * 0.10;
            },
            IceScienceFocus::IceShelfStability => {
                self.climate_relevance = 0.95 + rand_simple() * 0.05;
                self.modeling_capability = 0.85 + rand_simple() * 0.14;
                self.monitoring_coverage = 0.80 + rand_simple() * 0.18;
            },
            IceScienceFocus::IcebergTracking => {
                self.monitoring_coverage = 0.90 + rand_simple() * 0.10;
                self.measurement_accuracy = 0.85 + rand_simple() * 0.14;
                self.modeling_capability = 0.75 + rand_simple() * 0.22;
            },
            IceScienceFocus::IceChemistry => {
                self.measurement_accuracy = 0.80 + rand_simple() * 0.18;
                self.climate_relevance = 0.75 + rand_simple() * 0.22;
                self.modeling_capability = 0.70 + rand_simple() * 0.25;
            },
            IceScienceFocus::ClimateIceInteraction => {
                self.climate_relevance = 0.95 + rand_simple() * 0.05;
                self.modeling_capability = 0.90 + rand_simple() * 0.10;
                self.monitoring_coverage = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.modeling_capability == 0.0 {
            self.modeling_capability = (self.measurement_accuracy + self.monitoring_coverage) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_glacier_behavior() {
        let mut system = IceScienceSystem::new(IceScienceFocus::GlacierBehavior);
        system.analyze_system().unwrap();
        assert!(system.climate_relevance > 0.7);
    }
}
