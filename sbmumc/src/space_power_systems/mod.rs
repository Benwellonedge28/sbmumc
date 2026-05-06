//! Space Power Systems Module (665)
//!
//! Spacecraft power generation, storage, and distribution systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PowerGenerationType {
    SolarPhotovoltaic,
    SolarConcentrator,
    RTG,
    NuclearFission,
    NuclearFusion,
    FuelCell,
    Battery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacePowerSystem {
    pub system_name: String,
    pub generation_type: PowerGenerationType,
    pub rated_power: f64,            // kW
    pub efficiency: f64,              // percent
    pub mass: f64,                   // kg
    pub surface_area: f64,            // m^2
    pub operational_lifetime: f64,    // years
    pub degradation_rate: f64,        // percent/year
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyStorage {
    pub storage_type: String,
    pub capacity: f64,               // kWh
    pub specific_energy: f64,        // Wh/kg
    pub charge_cycles: u32,
    pub state_of_charge: f64,        // percent
    pub temperature_range: (f64, f64), // C
}

impl SpacePowerSystem {
    pub fn new(system_name: String, generation_type: PowerGenerationType) -> Self {
        Self {
            system_name,
            generation_type,
            rated_power: 0.0,
            efficiency: 0.0,
            mass: 0.0,
            surface_area: 0.0,
            operational_lifetime: 0.0,
            degradation_rate: 0.0,
        }
    }

    pub fn power_at_end_of_life(&self) -> f64 {
        let degradation = self.degradation_rate * self.operational_lifetime;
        self.rated_power * (1.0 - degradation / 100.0)
    }

    pub fn power_per_mass(&self) -> f64 {
        self.rated_power / self.mass.max(1.0)
    }
}

impl EnergyStorage {
    pub fn new(storage_type: String, capacity: f64) -> Self {
        Self {
            storage_type,
            capacity,
            specific_energy: 0.0,
            charge_cycles: 0,
            state_of_charge: 100.0,
            temperature_range: (-20.0, 50.0),
        }
    }

    pub fn energy_density(&self) -> f64 {
        self.capacity / (self.specific_energy / 1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_power_system() {
        let system = SpacePowerSystem::new("ISS Solar Arrays".into(), PowerGenerationType::SolarPhotovoltaic);
        assert!(system.rated_power >= 0.0);
    }
}
