//! # SBMUMC Module 1238: Remote Sensing Agriculture
//!
//! Satellite and aerial sensing for agricultural monitoring.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensingPlatform {
    Satellite,
    Aircraft,
    UAV,
    GroundBased,
    MultiSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSensingAgricultureSystem {
    pub system_id: String,
    pub sensing_platform: SensingPlatform,
    pub spatial_resolution: f64,
    pub temporal_resolution: f64,
    pub data_accuracy: f64,
    pub coverage_extent: f64,
}

impl RemoteSensingAgricultureSystem {
    pub fn new(sensing_platform: SensingPlatform) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            sensing_platform,
            spatial_resolution: 0.0,
            temporal_resolution: 0.0,
            data_accuracy: 0.0,
            coverage_extent: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.sensing_platform {
            SensingPlatform::Satellite => {
                self.coverage_extent = 0.95 + rand_simple() * 0.05;
                self.temporal_resolution = 0.60 + rand_simple() * 0.35;
                self.spatial_resolution = 0.70 + rand_simple() * 0.25;
            },
            SensingPlatform::Aircraft => {
                self.spatial_resolution = 0.85 + rand_simple() * 0.14;
                self.data_accuracy = 0.80 + rand_simple() * 0.18;
                self.temporal_resolution = 0.50 + rand_simple() * 0.40;
            },
            SensingPlatform::UAV => {
                self.spatial_resolution = 0.90 + rand_simple() * 0.10;
                self.data_accuracy = 0.85 + rand_simple() * 0.14;
                self.temporal_resolution = 0.80 + rand_simple() * 0.18;
            },
            SensingPlatform::GroundBased => {
                self.data_accuracy = 0.90 + rand_simple() * 0.10;
                self.spatial_resolution = 0.85 + rand_simple() * 0.14;
                self.coverage_extent = 0.40 + rand_simple() * 0.40;
            },
            SensingPlatform::MultiSource => {
                self.data_accuracy = 0.85 + rand_simple() * 0.14;
                self.coverage_extent = 0.85 + rand_simple() * 0.14;
                self.temporal_resolution = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.coverage_extent == 0.0 {
            self.coverage_extent = (self.spatial_resolution + self.temporal_resolution) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_uav_sensing() {
        let mut system = RemoteSensingAgricultureSystem::new(SensingPlatform::UAV);
        system.analyze_system().unwrap();
        assert!(system.spatial_resolution > 0.7);
    }
}