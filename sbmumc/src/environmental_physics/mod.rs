//! Environmental Physics Module
//!
//! This module implements environmental physics, climate science,
//! and earth system physics for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalPhysics {
    pub env_id: String,
    pub climate_systems: ClimateSystem,
    pub atmospheric_physics: AtmosphericPhysics,
    pub ocean_physics: OceanPhysics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateSystem {
    pub energy_balance: EnergyBalance,
    pub feedback_mechanisms: Vec<Feedback>,
    pub climate_models: Vec<ClimateModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyBalance { pub incoming_w_m2: f64, pub outgoing_w_m2: f64, pub imbalance_w_m2: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback { pub feedback_id: String, pub feedback_name: String, pub strength: f64, pub sign: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateModel { pub model_id: String, pub model_name: String, pub resolution_deg: f64, pub predicted_change_k: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphericPhysics {
    pub atmospheric_layers: Vec<AtmosphericLayer>,
    pub radiation_balance: RadiationBalance,
    pub greenhouse_effect: GreenhouseEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphericLayer { pub layer_id: String, pub layer_name: String, pub altitude_km: f64, pub temperature_k: f64, pub pressure_pa: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationBalance { pub solar_input_w_m2: f64, pub terrestrial_output_w_m2: f64, pub net_radiation_w_m2: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreenhouseEffect { pub gases: Vec<GreenhouseGas>, pub forcing_w_m2: f64, pub feedback_factor: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreenhouseGas { pub gas_name: String, pub concentration_ppm: f64, pub radiative_forcing_w_m2: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OceanPhysics {
    pub ocean_layers: Vec<OceanLayer>,
    pub thermohaline_circulation: Thermohaline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OceanLayer { pub layer_id: String, pub layer_name: String, pub depth_m: f64, pub temperature_c: f64, pub salinity_psu: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thermohaline { pub circulation_rate_sv: f64, pub overturning_depth_m: f64, pub stability: f64 }

impl EnvironmentalPhysics {
    pub fn new() -> Self {
        Self {
            env_id: String::from("environmental_physics_v1"),
            climate_systems: ClimateSystem {
                energy_balance: EnergyBalance { incoming_w_m2: 341.0, outgoing_w_m2: 339.0, imbalance_w_m2: 2.0 },
                feedback_mechanisms: vec![Feedback { feedback_id: String::from("fb_water"), feedback_name: String::from("Water vapor feedback"), strength: 1.5, sign: String::from("Positive") }],
                climate_models: vec![ClimateModel { model_id: String::from("model_1"), model_name: String::from("GCM"), resolution_deg: 1.0, predicted_change_k: 3.0 }],
            },
            atmospheric_physics: AtmosphericPhysics {
                atmospheric_layers: vec![
                    AtmosphericLayer { layer_id: String::from("atm_1"), layer_name: String::from("Troposphere"), altitude_km: 12.0, temperature_k: 260.0, pressure_pa: 20000.0 },
                ],
                radiation_balance: RadiationBalance { solar_input_w_m2: 341.0, terrestrial_output_w_m2: 239.0, net_radiation_w_m2: 102.0 },
                greenhouse_effect: GreenhouseEffect { gases: vec![GreenhouseGas { gas_name: String::from("CO2"), concentration_ppm: 420.0, radiative_forcing_w_m2: 2.0 }], forcing_w_m2: 2.0, feedback_factor: 1.5 },
            },
            ocean_physics: OceanPhysics {
                ocean_layers: vec![OceanLayer { layer_id: String::from("ocean_1"), layer_name: String::from("Mixed layer"), depth_m: 100.0, temperature_c: 20.0, salinity_psu: 35.0 }],
                thermohaline_circulation: Thermohaline { circulation_rate_sv: 15.0, overturning_depth_m: 4000.0, stability: 0.8 },
            },
        }
    }

    pub fn compute_greenhouse_forcing(&self, concentration_ppm: f64) -> f64 { 5.35 * (concentration_ppm / 280.0).ln() }
    pub fn compute_climate_sensitivity(&self, forcing_w_m2: f64) -> f64 { forcing_w_m2 * 3.0 }
}

impl Default for EnvironmentalPhysics { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_forcing() { let ep = EnvironmentalPhysics::new(); assert!(ep.compute_greenhouse_forcing(400.0) > 0.0); } }
