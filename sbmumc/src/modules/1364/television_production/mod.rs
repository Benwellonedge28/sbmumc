//! # SBMUMC Module 1364: Television Production
//!
//! Systems for television production and broadcasting.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TVProductionType {
    DramaSeries,
    ComedySeries,
    RealityTV,
    DocumentarySeries,
    News,
    LiveBroadcast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelevisionProductionSystem {
    pub system_id: String,
    pub production_type: TVProductionType,
    pub production_value: f64,
    pub audience_retention: f64,
    pub schedule_reliability: f64,
    pub creative_innovation: f64,
}

impl TelevisionProductionSystem {
    pub fn new(production_type: TVProductionType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            production_type,
            production_value: 0.0,
            audience_retention: 0.0,
            schedule_reliability: 0.0,
            creative_innovation: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.production_type {
            TVProductionType::DramaSeries => {
                self.creative_innovation = 0.95 + rand_simple() * 0.05;
                self.audience_retention = 0.90 + rand_simple() * 0.10;
                self.production_value = 0.85 + rand_simple() * 0.14;
            },
            TVProductionType::ComedySeries => {
                self.audience_retention = 0.95 + rand_simple() * 0.05;
                self.creative_innovation = 0.90 + rand_simple() * 0.10;
                self.schedule_reliability = 0.85 + rand_simple() * 0.14;
            },
            TVProductionType::RealityTV => {
                self.audience_retention = 0.95 + rand_simple() * 0.05;
                self.production_value = 0.90 + rand_simple() * 0.10;
                self.schedule_reliability = 0.85 + rand_simple() * 0.14;
            },
            TVProductionType::DocumentarySeries => {
                self.creative_innovation = 0.95 + rand_simple() * 0.05;
                self.production_value = 0.90 + rand_simple() * 0.10;
                self.audience_retention = 0.85 + rand_simple() * 0.14;
            },
            TVProductionType::News => {
                self.schedule_reliability = 0.95 + rand_simple() * 0.05;
                self.production_value = 0.90 + rand_simple() * 0.10;
                self.audience_retention = 0.85 + rand_simple() * 0.14;
            },
            TVProductionType::LiveBroadcast => {
                self.schedule_reliability = 0.95 + rand_simple() * 0.05;
                self.audience_retention = 0.90 + rand_simple() * 0.10;
                self.production_value = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.schedule_reliability == 0.0 {
            self.schedule_reliability = (self.audience_retention + self.production_value) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_drama_series() {
        let mut system = TelevisionProductionSystem::new(TVProductionType::DramaSeries);
        system.analyze_system().unwrap();
        assert!(system.creative_innovation > 0.8);
    }
}