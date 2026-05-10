//! # SBMUMC Module 1263: Water Quality Monitoring
//!
//! Systems for monitoring and assessing water quality parameters.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WaterQualityParameter {
    Ph,
    DissolvedOxygen,
    Turbidity,
    Nutrients,
    HeavyMetals,
    OrganicMatter,
    Temperature,
    Conductivity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterQualityMonitoringSystem {
    pub system_id: String,
    pub parameter: WaterQualityParameter,
    pub detection_sensitivity: f64,
    pub monitoring_coverage: f64,
    pub data_accuracy: f64,
    pub response_time: f64,
}

impl WaterQualityMonitoringSystem {
    pub fn new(parameter: WaterQualityParameter) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            parameter,
            detection_sensitivity: 0.0,
            monitoring_coverage: 0.0,
            data_accuracy: 0.0,
            response_time: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.parameter {
            WaterQualityParameter::Ph => {
                self.detection_sensitivity = 0.95 + rand_simple() * 0.05;
                self.data_accuracy = 0.90 + rand_simple() * 0.10;
            },
            WaterQualityParameter::DissolvedOxygen => {
                self.detection_sensitivity = 0.85 + rand_simple() * 0.14;
                self.response_time = 0.80 + rand_simple() * 0.18;
            },
            WaterQualityParameter::Turbidity => {
                self.monitoring_coverage = 0.80 + rand_simple() * 0.18;
                self.data_accuracy = 0.85 + rand_simple() * 0.14;
            },
            WaterQualityParameter::Nutrients => {
                self.detection_sensitivity = 0.80 + rand_simple() * 0.18;
                self.monitoring_coverage = 0.75 + rand_simple() * 0.22;
            },
            WaterQualityParameter::HeavyMetals => {
                self.detection_sensitivity = 0.90 + rand_simple() * 0.10;
                self.data_accuracy = 0.85 + rand_simple() * 0.14;
            },
            WaterQualityParameter::OrganicMatter => {
                self.data_accuracy = 0.80 + rand_simple() * 0.18;
                self.response_time = 0.75 + rand_simple() * 0.22;
            },
            WaterQualityParameter::Temperature => {
                self.response_time = 0.95 + rand_simple() * 0.05;
                self.monitoring_coverage = 0.90 + rand_simple() * 0.10;
            },
            WaterQualityParameter::Conductivity => {
                self.detection_sensitivity = 0.85 + rand_simple() * 0.14;
                self.monitoring_coverage = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.response_time == 0.0 {
            self.response_time = (self.detection_sensitivity + self.data_accuracy) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_heavy_metals_detection() {
        let mut system = WaterQualityMonitoringSystem::new(WaterQualityParameter::HeavyMetals);
        system.analyze_system().unwrap();
        assert!(system.detection_sensitivity > 0.7);
    }
}
