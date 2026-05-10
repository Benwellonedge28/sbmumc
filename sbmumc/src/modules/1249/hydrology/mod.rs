//! # SBMUMC Module 1249: Hydrology
//!
//! Study of water movement, distribution, and quality in the environment.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HydrologicalProcess {
    Precipitation,
    Infiltration,
    Runoff,
    Evapotranspiration,
    GroundwaterFlow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydrologySystem {
    pub system_id: String,
    pub hydrological_process: HydrologicalProcess,
    pub modeling_accuracy: f64,
    pub measurement_precision: f64,
    pub prediction_reliability: f64,
    pub water_balance: f64,
}

impl HydrologySystem {
    pub fn new(hydrological_process: HydrologicalProcess) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            hydrological_process,
            modeling_accuracy: 0.0,
            measurement_precision: 0.0,
            prediction_reliability: 0.0,
            water_balance: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.hydrological_process {
            HydrologicalProcess::Precipitation => {
                self.measurement_precision = 0.85 + rand_simple() * 0.14;
                self.prediction_reliability = 0.75 + rand_simple() * 0.22;
            },
            HydrologicalProcess::Infiltration => {
                self.modeling_accuracy = 0.70 + rand_simple() * 0.25;
                self.measurement_precision = 0.65 + rand_simple() * 0.30;
            },
            HydrologicalProcess::Runoff => {
                self.modeling_accuracy = 0.80 + rand_simple() * 0.18;
                self.prediction_reliability = 0.75 + rand_simple() * 0.22;
                self.water_balance = 0.70 + rand_simple() * 0.25;
            },
            HydrologicalProcess::Evapotranspiration => {
                self.measurement_precision = 0.70 + rand_simple() * 0.25;
                self.modeling_accuracy = 0.75 + rand_simple() * 0.22;
            },
            HydrologicalProcess::GroundwaterFlow => {
                self.modeling_accuracy = 0.65 + rand_simple() * 0.30;
                self.prediction_reliability = 0.60 + rand_simple() * 0.35;
            },
        }

        if self.water_balance == 0.0 {
            self.water_balance = (self.modeling_accuracy + self.measurement_precision) / 2.0 * (0.7 + rand_simple() * 0.3);
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
    fn test_precipitation_hydrology() {
        let mut system = HydrologySystem::new(HydrologicalProcess::Precipitation);
        system.analyze_system().unwrap();
        assert!(system.measurement_precision > 0.6);
    }
}