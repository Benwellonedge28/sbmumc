//! # SBMUMC Module 1308: Passive House
//!
//! Systems for passive house building standards.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassiveHouseComponent {
    ThermalEnvelope,
    VentilationSystem,
    WindowDesign,
    ThermalBridgeFree,
    AirTightness,
    SolarGain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassiveHouseSystem {
    pub system_id: String,
    pub passive_component: PassiveHouseComponent,
    pub energy_efficiency: f64,
    pub thermal_comfort: f64,
    pub air_quality: f64,
    pub construction_complexity: f64,
}

impl PassiveHouseSystem {
    pub fn new(passive_component: PassiveHouseComponent) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            passive_component,
            energy_efficiency: 0.0,
            thermal_comfort: 0.0,
            air_quality: 0.0,
            construction_complexity: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.passive_component {
            PassiveHouseComponent::ThermalEnvelope => {
                self.energy_efficiency = 0.95 + rand_simple() * 0.05;
                self.thermal_comfort = 0.90 + rand_simple() * 0.10;
                self.construction_complexity = 0.70 + rand_simple() * 0.25;
            },
            PassiveHouseComponent::VentilationSystem => {
                self.air_quality = 0.95 + rand_simple() * 0.05;
                self.energy_efficiency = 0.85 + rand_simple() * 0.14;
                self.construction_complexity = 0.75 + rand_simple() * 0.22;
            },
            PassiveHouseComponent::WindowDesign => {
                self.thermal_comfort = 0.90 + rand_simple() * 0.10;
                self.energy_efficiency = 0.85 + rand_simple() * 0.14;
                self.air_quality = 0.80 + rand_simple() * 0.18;
            },
            PassiveHouseComponent::ThermalBridgeFree => {
                self.energy_efficiency = 0.90 + rand_simple() * 0.10;
                self.thermal_comfort = 0.85 + rand_simple() * 0.14;
                self.construction_complexity = 0.85 + rand_simple() * 0.14;
            },
            PassiveHouseComponent::AirTightness => {
                self.energy_efficiency = 0.90 + rand_simple() * 0.10;
                self.thermal_comfort = 0.90 + rand_simple() * 0.10;
                self.air_quality = 0.85 + rand_simple() * 0.14;
            },
            PassiveHouseComponent::SolarGain => {
                self.energy_efficiency = 0.85 + rand_simple() * 0.14;
                self.thermal_comfort = 0.80 + rand_simple() * 0.18;
                self.construction_complexity = 0.60 + rand_simple() * 0.35;
            },
        }

        if self.construction_complexity == 0.0 {
            self.construction_complexity = (self.energy_efficiency + self.thermal_comfort) / 2.0 * (0.5 + rand_simple() * 0.4);
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
    fn test_ventilation_system() {
        let mut system = PassiveHouseSystem::new(PassiveHouseComponent::VentilationSystem);
        system.analyze_system().unwrap();
        assert!(system.air_quality > 0.8);
    }
}
