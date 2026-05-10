//! # SBMUMC Module 1275: Marine Transportation
//!
//! Systems for ocean-based transportation and shipping.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarineTransportType {
    ContainerShips,
    TankerVessels,
    PassengerShips,
    FishingVessels,
    ResearchVessels,
    YachtsAndRecreational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarineTransportationSystem {
    pub system_id: String,
    pub transport_type: MarineTransportType,
    pub cargo_capacity: f64,
    pub fuel_efficiency: f64,
    pub safety_standard: f64,
    pub environmental_compliance: f64,
}

impl MarineTransportationSystem {
    pub fn new(transport_type: MarineTransportType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            transport_type,
            cargo_capacity: 0.0,
            fuel_efficiency: 0.0,
            safety_standard: 0.0,
            environmental_compliance: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.transport_type {
            MarineTransportType::ContainerShips => {
                self.cargo_capacity = 0.95 + rand_simple() * 0.05;
                self.fuel_efficiency = 0.70 + rand_simple() * 0.25;
                self.safety_standard = 0.85 + rand_simple() * 0.14;
            },
            MarineTransportType::TankerVessels => {
                self.cargo_capacity = 0.90 + rand_simple() * 0.10;
                self.safety_standard = 0.90 + rand_simple() * 0.10;
                self.environmental_compliance = 0.75 + rand_simple() * 0.22;
            },
            MarineTransportType::PassengerShips => {
                self.safety_standard = 0.95 + rand_simple() * 0.05;
                self.cargo_capacity = 0.60 + rand_simple() * 0.35;
                self.environmental_compliance = 0.80 + rand_simple() * 0.18;
            },
            MarineTransportType::FishingVessels => {
                self.fuel_efficiency = 0.60 + rand_simple() * 0.35;
                self.cargo_capacity = 0.55 + rand_simple() * 0.40;
                self.environmental_compliance = 0.50 + rand_simple() * 0.40;
            },
            MarineTransportType::ResearchVessels => {
                self.safety_standard = 0.90 + rand_simple() * 0.10;
                self.cargo_capacity = 0.70 + rand_simple() * 0.25;
                self.environmental_compliance = 0.85 + rand_simple() * 0.14;
            },
            MarineTransportType::YachtsAndRecreational => {
                self.fuel_efficiency = 0.55 + rand_simple() * 0.40;
                self.safety_standard = 0.75 + rand_simple() * 0.22;
                self.environmental_compliance = 0.65 + rand_simple() * 0.30;
            },
        }

        if self.environmental_compliance == 0.0 {
            self.environmental_compliance = (self.safety_standard + self.fuel_efficiency) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_container_ships() {
        let mut system = MarineTransportationSystem::new(MarineTransportType::ContainerShips);
        system.analyze_system().unwrap();
        assert!(system.cargo_capacity > 0.8);
    }
}
