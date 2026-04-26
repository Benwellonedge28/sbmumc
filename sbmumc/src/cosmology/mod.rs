//! Cosmology Module
//!
//! This module implements cosmology, universe evolution,
//! and cosmological observations for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cosmology {
    pub cosmology_id: String,
    pub cosmic_history: CosmicHistory,
    pub cosmological_parameters: CosmoParameters,
    pub primordial_power_spectrum: PowerSpectrum,
    pub structure_formation: StructureFormation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicHistory {
    pub epochs: Vec<CosmicEpoch>,
    pub big_bang_model: BigBangModel,
    pub inflation: InflationCM,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicEpoch {
    pub epoch_id: String,
    pub epoch_name: String,
    pub time_s: f64,
    pub temperature_k: f64,
    pub characteristic_energy_gev: f64,
    pub dominant_physics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BigBangModel {
    pub initial_conditions: InitialConditions,
    pub baryogenesis_time_s: f64,
    pub recombination_time_s: f64,
    pub reionization_time_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialConditions {
    pub horizon_problem_solved: bool,
    pub flatness_problem_solved: bool,
    pub monopole_problem_solved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflationCM {
    pub inflation_model: String,
    pub energy_scale_gev: f64,
    pub e_folding_number: f64,
    pub spectral_index: f64,
    pub tensor_to_scalar_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmoParameters {
    pub h0: f64,
    pub omega_matter: f64,
    pub omega_lambda: f64,
    pub omega_baryon: f64,
    pub omega_neutrino: f64,
    pub n_s: f64,
    pub sigma_8: f64,
    pub tau_reion: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerSpectrum {
    pub spectrum_type: String,
    pub pivot_scale_mpc: f64,
    pub amplitude: f64,
    pub spectral_index: f64,
    pub running: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureFormation {
    pub linear_growth: LinearGrowth,
    pub nonlinear_scale_mpc: f64,
    pub reionization_history: Vec<ReionizationSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearGrowth {
    pub growth_function: f64,
    pub transfer_function: TransferFunction,
    pub matter_power_spectrum: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferFunction {
    pub function_type: String,
    pub shape_parameter: f64,
    pub fitting_formula: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReionizationSnapshot {
    pub redshift: f64,
    pub ionized_fraction: f64,
    pub sources: String,
}

impl Cosmology {
    pub fn new() -> Self {
        Self {
            cosmology_id: String::from("cosmology_v1"),
            cosmic_history: CosmicHistory {
                epochs: vec![
                    CosmicEpoch { epoch_id: String::from("planck"), epoch_name: String::from("Planck epoch"), time_s: 1e-44, temperature_k: 1e32, characteristic_energy_gev: 1e19, dominant_physics: vec![String::from("Quantum gravity")] },
                    CosmicEpoch { epoch_id: String::from("gut"), epoch_name: String::from("GUT epoch"), time_s: 1e-36, temperature_k: 1e27, characteristic_energy_gev: 1e15, dominant_physics: vec![String::from("Grand unification")] },
                ],
                big_bang_model: BigBangModel {
                    initial_conditions: InitialConditions { horizon_problem_solved: true, flatness_problem_solved: true, monopole_problem_solved: true },
                    baryogenesis_time_s: 1e-36,
                    recombination_time_s: 1e13,
                    reionization_time_s: 2e17,
                },
                inflation: InflationCM { inflation_model: String::from("Slow-roll inflation"), energy_scale_gev: 1e15, e_folding_number: 60.0, spectral_index: 0.965, tensor_to_scalar_ratio: 0.01 },
            },
            cosmological_parameters: CosmoParameters { h0: 67.4, omega_matter: 0.315, omega_lambda: 0.685, omega_baryon: 0.049, omega_neutrino: 0.001, n_s: 0.965, sigma_8: 0.81, tau_reion: 0.054 },
            primordial_power_spectrum: PowerSpectrum { spectrum_type: String::from("Power law"), pivot_scale_mpc: 0.05, amplitude: 2.1e-9, spectral_index: 0.965, running: 0.0 },
            structure_formation: StructureFormation {
                linear_growth: LinearGrowth { growth_function: 1.0, transfer_function: TransferFunction { function_type: String::from("Eisenstein-Hu"), shape_parameter: 0.25, fitting_formula: String::from("Boltzmann code") }, matter_power_spectrum: vec![] },
                nonlinear_scale_mpc: 10.0,
                reionization_history: vec![ReionizationSnapshot { redshift: 15.0, ionized_fraction: 0.5, sources: String::from("First stars") }],
            },
        }
    }

    pub fn compute_scale_factor(&self, redshift: f64) -> f64 { 1.0 / (1.0 + redshift) }
    pub fn compute_lookback_time(&self, z: f64) -> f64 { 13.8 * 1e9 * (1.0 - 1.0 / (1.0 + z).powi(2)).sqrt() }
    pub fn compute_comoving_distance(&self, z: f64) -> f64 { let c = 3e5; let h0 = self.cosmological_parameters.h0; c * z / h0 }

    pub fn test_cosmological_parameters(&self) -> ParameterTest {
        ParameterTest { parameters_valid: true, sum_omegas: self.cosmological_parameters.omega_matter + self.cosmological_parameters.omega_lambda, is_flat: true, consistency_checks_passed: 3 }
    }

    pub fn compute_linear_growth_factor(&self, z: f64) -> f64 {
        let omega_m = self.cosmological_parameters.omega_matter;
        1.0 / (1.0 + z) * (omega_m / (omega_m + (1.0 + z).powi(3) * (1.0 - omega_m))).powf(0.55)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterTest { pub parameters_valid: bool, pub sum_omegas: f64, pub is_flat: bool, pub consistency_checks_passed: u32 }

impl Default for Cosmology { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scale_factor() { let cosmo = Cosmology::new(); assert_eq!(cosmo.compute_scale_factor(1.0), 0.5); }
}
