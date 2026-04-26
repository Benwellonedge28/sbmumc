//! Astrophysics Module
//!
//! This module implements astrophysics, stellar evolution,
//! and astronomical phenomena for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Astrophysics {
    pub astro_id: String,
    pub stellar_models: Vec<StellarModel>,
    pub astronomical_objects: Vec<AstronomicalObject>,
    pub observation_methods: Vec<ObservationMethod>,
    pub astrophysical_processes: Vec<AstrophysicalProcess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StellarModel {
    pub model_id: String,
    pub stellar_type: StellarType,
    pub mass_msun: f64,
    pub radius_rsun: f64,
    pub luminosity_lsun: f64,
    pub effective_temperature_k: f64,
    pub evolutionary_stage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StellarType {
    O, B, A, F, G, K, M,
    WhiteDwarf, NeutronStar, BlackHole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstronomicalObject {
    pub object_id: String,
    pub object_type: ObjectType,
    pub mass_kg: f64,
    pub distance_pc: f64,
    pub luminosity_w: f64,
    pub catalog_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ObjectType {
    Star, Planet, Galaxy, Nebula, Cluster, AGN, Quasar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationMethod {
    pub method_id: String,
    pub wavelength_band: WavelengthBand,
    pub telescope_type: String,
    pub angular_resolution_arcsec: f64,
    pub sensitivity_limit: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WavelengthBand {
    Radio, Microwave, Infrared, Optical, UV, XRay, GammaRay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstrophysicalProcess {
    pub process_id: String,
    pub process_name: String,
    pub physical_mechanism: String,
    pub energy_scale_ev: f64,
    pub observational_signature: String,
}

impl Astrophysics {
    pub fn new() -> Self {
        Self {
            astro_id: String::from("astrophysics_v1"),
            stellar_models: vec![
                StellarModel {
                    model_id: String::from("sun_1"),
                    stellar_type: StellarType::G,
                    mass_msun: 1.0, radius_rsun: 1.0, luminosity_lsun: 1.0,
                    effective_temperature_k: 5778.0,
                    evolutionary_stage: String::from("Main sequence"),
                },
            ],
            astronomical_objects: vec![
                AstronomicalObject {
                    object_id: String::from("sirius"),
                    object_type: ObjectType::Star,
                    mass_kg: 4.0 * 1.989e30,
                    distance_pc: 2.6,
                    luminosity_w: 25.0 * 3.846e26,
                    catalog_names: vec![String::from("HD 48915")],
                },
            ],
            observation_methods: vec![
                ObservationMethod {
                    method_id: String::from("obs_1"),
                    wavelength_band: WavelengthBand::Optical,
                    telescope_type: String::from("Optical reflector"),
                    angular_resolution_arcsec: 0.5,
                    sensitivity_limit: 1e-11,
                },
            ],
            astrophysical_processes: vec![
                AstrophysicalProcess {
                    process_id: String::from("proc_1"),
                    process_name: String::from("Core hydrogen burning"),
                    physical_mechanism: String::from("PP chain and CNO cycle"),
                    energy_scale_ev: 1e7,
                    observational_signature: String::from("Solar luminosity"),
                },
            ],
        }
    }

    pub fn compute_hr_diagram_position(&self, model: &StellarModel) -> HRDiagramPoint {
        HRDiagramPoint {
            temperature_k: model.effective_temperature_k,
            luminosity_lsun: model.luminosity_lsun,
            stellar_type: model.stellar_type.clone(),
        }
    }

    pub fn calculate_luminosity(&self, mass_msun: f64) -> f64 {
        if mass_msun < 2.0 { mass_msun.powi(4) } else { mass_msun.powi(3) }
    }

    pub fn analyze_stellar_evolution(&self, initial_mass_msun: f64) -> StellarEvolution {
        StellarEvolution {
            initial_mass_msun,
            main_sequence_lifetime_gyr: 10.0 / (initial_mass_msun.powi(2)),
            final_state: if initial_mass_msun < 8.0 { String::from("White dwarf") } else { String::from("Supernova") },
            remnant_mass_msun: if initial_mass_msun < 8.0 { 0.6 } else { 1.4 },
        }
    }

    pub fn compute_eddington_luminosity(&self, mass_msun: f64) -> f64 {
        3.2e4 * mass_msun
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HRDiagramPoint {
    pub temperature_k: f64,
    pub luminosity_lsun: f64,
    pub stellar_type: StellarType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StellarEvolution {
    pub initial_mass_msun: f64,
    pub main_sequence_lifetime_gyr: f64,
    pub final_state: String,
    pub remnant_mass_msun: f64,
}

impl Default for Astrophysics {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_luminosity() {
        let astro = Astrophysics::new();
        let lum = astro.calculate_luminosity(1.0);
        assert!(lum > 0.0);
    }
}
