//! Space Station Design Module (652)
//!
//! Orbital space station architecture, life support, and operational systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StationType {
    LEO,
    LunarOrbital,
    MarsOrbital,
    Lagrange,
    Interplanetary,
    DeepSpace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceStation {
    pub station_type: StationType,
    pub name: String,
    pub crew_capacity: u32,
    pub total_mass: f64,          // kg
    pub habitable_volume: f64,     // m^3
    pub power_generation: f64,     // kW
    pub solar_panel_area: f64,     // m^2
    pub orbit_altitude: f64,       // km
    pub orbital_period: f64,       // minutes
    pub construction_year: u32,
    pub operational_lifetime: f64, // years
    pub modules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeSupportSystem {
    pub oxygen_generation: f64,   // kg/day
    pub co2_scrubbing: f64,        // kg/day
    pub water_reclamation: f64,    // percent
    pub waste_management: f64,     // kg/day
    pub temperature_control: f64,  // kW
    pub humidity_control: f64,    // percent
    pub air_pressure: f64,         // kPa
    pub backup_systems: bool,
}

impl SpaceStation {
    pub fn new(name: String, station_type: StationType) -> Self {
        Self {
            station_type,
            name,
            crew_capacity: 0,
            total_mass: 0.0,
            habitable_volume: 0.0,
            power_generation: 0.0,
            solar_panel_area: 0.0,
            orbit_altitude: 0.0,
            orbital_period: 0.0,
            construction_year: 0,
            operational_lifetime: 0.0,
            modules: Vec::new(),
        }
    }

    pub fn calculate_orbital_period(&self) -> f64 {
        let mu = 398600.4418;
        let r = 6371.0 + self.orbit_altitude;
        2.0 * std::f64::consts::PI * ((r * r * r / mu).sqrt()) / 60.0
    }

    pub fn add_module(&mut self, module: String) {
        self.modules.push(module);
    }

    pub fn total_power_per_crew(&self) -> f64 {
        self.power_generation / self.crew_capacity.max(1) as f64
    }
}

impl LifeSupportSystem {
    pub fn new() -> Self {
        Self {
            oxygen_generation: 0.84,
            co2_scrubbing: 2.0,
            water_reclamation: 93.0,
            waste_management: 3.0,
            temperature_control: 5.0,
            humidity_control: 50.0,
            air_pressure: 101.3,
            backup_systems: true,
        }
    }

    pub fn calculate_consumables(&self, crew: u32, duration: f64) -> (f64, f64, f64) {
        let oxygen = self.oxygen_generation * crew as f64 * duration;
        let water = self.water_reclamation / 100.0 * crew as f64 * duration * 3.0;
        let food = 1.8 * crew as f64 * duration;
        (oxygen, water, food)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_station() {
        let station = SpaceStation::new("Orbital Base".into(), StationType::LEO);
        assert_eq!(station.name, "Orbital Base");
    }
}
