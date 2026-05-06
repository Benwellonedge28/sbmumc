//! Reentry Thermal Protection Module (685)
//!
//! Heat shield design, thermal protection systems, and atmospheric reentry engineering.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TPSMaterial {
    PICA,
    Avcoat,
    CFRP,
    CMC,
    Tile,
    Ablative,
    ActiveCooling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalProtectionSystem {
    pub tps_name: String,
    pub tps_material: TPSMaterial,
    pub heat_shield_diameter: f64,    // m
    pub areal_density: f64,             // kg/m^2
    pub peak_heat_flux: f64,           // kW/m^2
    pub total_heat_load: f64,          // MJ/m^2
    pub recession_rate: f64,            // mm
    pub surface_temperature_max: f64,   // C
    pub mass: f64,                     // kg
    pub reuseable: bool,
}

impl ThermalProtectionSystem {
    pub fn new(tps_name: String, tps_material: TPSMaterial) -> Self {
        Self {
            tps_name,
            tps_material,
            heat_shield_diameter: 0.0,
            areal_density: 0.0,
            peak_heat_flux: 0.0,
            total_heat_load: 0.0,
            recession_rate: 0.0,
            surface_temperature_max: 0.0,
            mass: 0.0,
            reuseable: false,
        }
    }

    pub fn thermal_efficiency(&self) -> f64 {
        self.total_heat_load / (self.areal_density * self.mass.max(1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_protection() {
        let tps = ThermalProtectionSystem::new("PICA-X".into(), TPSMaterial::PICA);
        assert!(matches!(tps.tps_material, TPSMaterial::PICA));
    }
}
