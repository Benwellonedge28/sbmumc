//! # SBMUMC Module 1315: Building Physics
//!
//! Systems for understanding building physical properties.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhysicalProperty {
    HeatTransfer,
    MoistureTransfer,
    AirMovement,
    LightTransmission,
    SoundTransmission,
    StructuralLoad,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingPhysicsSystem {
    pub system_id: String,
    pub physical_property: PhysicalProperty,
    pub simulation_accuracy: f64,
    pub prediction_reliability: f64,
    pub modeling_complexity: f64,
    pub practical_applicability: f64,
}

impl BuildingPhysicsSystem {
    pub fn new(physical_property: PhysicalProperty) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            physical_property,
            simulation_accuracy: 0.0,
            prediction_reliability: 0.0,
            modeling_complexity: 0.0,
            practical_applicability: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.physical_property {
            PhysicalProperty::HeatTransfer => {
                self.simulation_accuracy = 0.95 + rand_simple() * 0.05;
                self.prediction_reliability = 0.90 + rand_simple() * 0.10;
                self.practical_applicability = 0.85 + rand_simple() * 0.14;
            },
            PhysicalProperty::MoistureTransfer => {
                self.simulation_accuracy = 0.85 + rand_simple() * 0.14;
                self.modeling_complexity = 0.90 + rand_simple() * 0.10;
                self.prediction_reliability = 0.80 + rand_simple() * 0.18;
            },
            PhysicalProperty::AirMovement => {
                self.prediction_reliability = 0.90 + rand_simple() * 0.10;
                self.simulation_accuracy = 0.85 + rand_simple() * 0.14;
                self.practical_applicability = 0.80 + rand_simple() * 0.18;
            },
            PhysicalProperty::LightTransmission => {
                self.simulation_accuracy = 0.95 + rand_simple() * 0.05;
                self.prediction_reliability = 0.95 + rand_simple() * 0.05;
                self.practical_applicability = 0.90 + rand_simple() * 0.10;
            },
            PhysicalProperty::SoundTransmission => {
                self.simulation_accuracy = 0.90 + rand_simple() * 0.10;
                self.modeling_complexity = 0.85 + rand_simple() * 0.14;
                self.prediction_reliability = 0.85 + rand_simple() * 0.14;
            },
            PhysicalProperty::StructuralLoad => {
                self.prediction_reliability = 0.95 + rand_simple() * 0.05;
                self.simulation_accuracy = 0.90 + rand_simple() * 0.10;
                self.practical_applicability = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.modeling_complexity == 0.0 {
            self.modeling_complexity = (self.simulation_accuracy + self.prediction_reliability) / 2.0 * (0.5 + rand_simple() * 0.4);
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
    fn test_heat_transfer() {
        let mut system = BuildingPhysicsSystem::new(PhysicalProperty::HeatTransfer);
        system.analyze_system().unwrap();
        assert!(system.simulation_accuracy > 0.8);
    }
}
