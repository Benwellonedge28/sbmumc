//! # SBMUMC Module 1340: Transit Architecture
//!
//! Systems for transportation hub design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitHubType {
    Airport,
    TrainStation,
    BusTerminal,
    MetroStation,
    FerryTerminal,
    MultiModal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitArchitectureSystem {
    pub system_id: String,
    pub transit_hub_type: TransitHubType,
    pub passenger_flow: f64,
    pub wayfinding_efficiency: f64,
    pub comfort_standards: f64,
    pub connectivity: f64,
}

impl TransitArchitectureSystem {
    pub fn new(transit_hub_type: TransitHubType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            transit_hub_type,
            passenger_flow: 0.0,
            wayfinding_efficiency: 0.0,
            comfort_standards: 0.0,
            connectivity: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.transit_hub_type {
            TransitHubType::Airport => {
                self.passenger_flow = 0.95 + rand_simple() * 0.05;
                self.connectivity = 0.90 + rand_simple() * 0.10;
                self.wayfinding_efficiency = 0.85 + rand_simple() * 0.14;
            },
            TransitHubType::TrainStation => {
                self.passenger_flow = 0.90 + rand_simple() * 0.10;
                self.comfort_standards = 0.90 + rand_simple() * 0.10;
                self.connectivity = 0.85 + rand_simple() * 0.14;
            },
            TransitHubType::BusTerminal => {
                self.passenger_flow = 0.85 + rand_simple() * 0.14;
                self.wayfinding_efficiency = 0.85 + rand_simple() * 0.14;
                self.comfort_standards = 0.80 + rand_simple() * 0.18;
            },
            TransitHubType::MetroStation => {
                self.wayfinding_efficiency = 0.95 + rand_simple() * 0.05;
                self.passenger_flow = 0.90 + rand_simple() * 0.10;
                self.connectivity = 0.85 + rand_simple() * 0.14;
            },
            TransitHubType::FerryTerminal => {
                self.comfort_standards = 0.90 + rand_simple() * 0.10;
                self.passenger_flow = 0.85 + rand_simple() * 0.14;
                self.connectivity = 0.85 + rand_simple() * 0.14;
            },
            TransitHubType::MultiModal => {
                self.connectivity = 0.95 + rand_simple() * 0.05;
                self.wayfinding_efficiency = 0.90 + rand_simple() * 0.10;
                self.passenger_flow = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.comfort_standards == 0.0 {
            self.comfort_standards = (self.passenger_flow + self.wayfinding_efficiency) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_airport() {
        let mut system = TransitArchitectureSystem::new(TransitHubType::Airport);
        system.analyze_system().unwrap();
        assert!(system.passenger_flow > 0.8);
    }
}