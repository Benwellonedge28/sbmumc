//! Cryogenics Module
//!
//! This module implements cryogenics, low temperature physics,
//! and cryogenic systems for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cryogenics {
    pub cryo_id: String,
    pub temperature_regimes: Vec<TemperatureRegime>,
    pub cooling_methods: Vec<CoolingMethod>,
    pub cryogenic_fluids: Vec<CryogenicFluid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureRegime { pub regime_id: String, pub temp_range_k: [f64; 2], pub physics_regime: String, pub typical_applications: Vec<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoolingMethod { pub method_id: String, pub method_name: String, pub min_temp_k: f64, pub cooling_power_w: f64, pub mechanism: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryogenicFluid { pub fluid_id: String, pub fluid_name: String, pub boiling_point_k: f64, pub latent_heat_j_kg: f64, pub critical_temp_k: f64 }

impl Cryogenics {
    pub fn new() -> Self {
        Self {
            cryo_id: String::from("cryogenics_v1"),
            temperature_regimes: vec![
                TemperatureRegime { regime_id: String::from("reg_1"), temp_range_k: [0.0, 1.0], physics_regime: String::from("Quantum regime"), typical_applications: vec![String::from("Superconductivity")] },
            ],
            cooling_methods: vec![
                CoolingMethod { method_id: String::from("cool_1"), method_name: String::from("Joule-Thomson"), min_temp_k: 77.0, cooling_power_w: 1.0, mechanism: String::from("Isoenthalpic expansion") },
            ],
            cryogenic_fluids: vec![
                CryogenicFluid { fluid_id: String::from("fluid_n2"), fluid_name: String::from("Liquid nitrogen"), boiling_point_k: 77.0, latent_heat_j_kg: 199000.0, critical_temp_k: 126.0 },
            ],
        }
    }

    pub fn compute_thermal_conductivity(&self, temp_k: f64) -> f64 { 1.0 / temp_k }
    pub fn compute_vapor_pressure(&self, temp_k: f64, lvap: f64) -> f64 { (-lvap / (8.314 * temp_k)).exp() }
}

impl Default for Cryogenics { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_thermal_conductivity() { let cryo = Cryogenics::new(); assert!(cryo.compute_thermal_conductivity(100.0) > 0.0); } }
