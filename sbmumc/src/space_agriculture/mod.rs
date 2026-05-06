//! Space Agriculture Module (664)
//!
//! Agricultural systems for space habitats, including hydroponics, aeroponics, and bio-regenerative life support.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgricultureType {
    Hydroponics,
    Aeroponics,
    Aquaponics,
    BioRegenerative,
    VerticalFarming,
    CellCultivation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceAgriculture {
    pub farm_name: String,
    pub agriculture_type: AgricultureType,
    pub growing_area: f64,           // m^2
    pub crops_grown: Vec<String>,
    pub harvest_cycle: f64,           // days
    pub yield_per_cycle: f64,         // kg
    pub water_usage: f64,             // L/day
    pub power_consumption: f64,       // kW
    pub co2_consumption: f64,         // kg/day
    pub oxygen_production: f64,       // kg/day
    pub automation_level: f64,        // percent
}

impl SpaceAgriculture {
    pub fn new(farm_name: String, agriculture_type: AgricultureType) -> Self {
        Self {
            farm_name,
            agriculture_type,
            growing_area: 0.0,
            crops_grown: Vec::new(),
            harvest_cycle: 60.0,
            yield_per_cycle: 0.0,
            water_usage: 0.0,
            power_consumption: 0.0,
            co2_consumption: 0.0,
            oxygen_production: 0.0,
            automation_level: 0.0,
        }
    }

    pub fn add_crop(&mut self, crop: String) {
        self.crops_grown.push(crop);
    }

    pub fn annual_yield(&self) -> f64 {
        (365.0 / self.harvest_cycle) * self.yield_per_cycle
    }

    pub fn caloric_output(&self) -> f64 {
        self.annual_yield() * 2500.0 // rough estimate kcal/kg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_agriculture() {
        let farm = SpaceAgriculture::new("Veggie ISS".into(), AgricultureType::Hydroponics);
        assert_eq!(farm.farm_name, "Veggie ISS");
    }
}
