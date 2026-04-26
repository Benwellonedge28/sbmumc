//! Dark Energy Module
//!
//! This module implements dark energy theory, cosmological constant,
//! and quintessence models for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkEnergy {
    pub de_id: String,
    pub models: Vec<DarkEnergyModel>,
    pub cosmological_parameters: CosmologicalParameters,
    pub equation_of_state: EquationOfState,
    pub observational_constraints: Vec<Constraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkEnergyModel {
    pub model_id: String,
    pub model_name: String,
    pub model_type: DEModelType,
    pub equation_of_state_w: f64,
    pub predictions: Vec<ModelPrediction>,
    pub viability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DEModelType {
    CosmologicalConstant,
    Quintessence,
    Phantom,
    Quintom,
    KEssence,
    ModifiedGravity,
    ChaplyginGas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPrediction {
    pub prediction_id: String,
    pub observable: String,
    pub predicted_value: f64,
    pub measured_value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmologicalParameters {
    pub omega_lambda: f64,
    pub omega_matter: f64,
    pub hubble_constant: f64,
    pub dark_energy_density: f64,
    pub age_universe_gyr: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquationOfState {
    pub w_0: f64,
    pub w_a: f64,
    pub w_z: Vec<(f64, f64)>,
    pub evolution: EOSEvolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EOSEvolution {
    pub evolution_type: String,
    pub crossing_w_minus_one: bool,
    pub phantom_regions: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub constraint_id: String,
    pub observation_type: ObservationType,
    pub data_source: String,
    pub w_constraint: [f64; 2],
    pub wa_constraint: Option<[f64; 2]>,
    pub figure_of_merit: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ObservationType {
    TypeIaSupernova,
    BAO,
    CMB,
    WeakLensing,
    ClusterCount,
}

impl DarkEnergy {
    pub fn new() -> Self {
        Self {
            de_id: String::from("dark_energy_v1"),
            models: vec![
                DarkEnergyModel {
                    model_id: String::from("model_1"),
                    model_name: String::from("Lambda CDM"),
                    model_type: DEModelType::CosmologicalConstant,
                    equation_of_state_w: -1.0,
                    predictions: vec![
                        ModelPrediction {
                            prediction_id: String::from("pred_1"),
                            observable: String::from("Omega_Lambda"),
                            predicted_value: 0.7,
                            measured_value: Some(0.685),
                        },
                    ],
                    viability: 0.95,
                },
                DarkEnergyModel {
                    model_id: String::from("model_2"),
                    model_name: String::from("Quintessence"),
                    model_type: DEModelType::Quintessence,
                    equation_of_state_w: -0.9,
                    predictions: vec![
                        ModelPrediction {
                            prediction_id: String::from("pred_2"),
                            observable: String::from("w evolution"),
                            predicted_value: -0.8,
                            measured_value: None,
                        },
                    ],
                    viability: 0.6,
                },
            ],
            cosmological_parameters: CosmologicalParameters {
                omega_lambda: 0.685,
                omega_matter: 0.315,
                hubble_constant: 67.4,
                dark_energy_density: 5.5e-27,
                age_universe_gyr: 13.8,
            },
            equation_of_state: EquationOfState {
                w_0: -1.0,
                w_a: 0.0,
                w_z: vec![(0.0, -1.0), (1.0, -1.05)],
                evolution: EOSEvolution {
                    evolution_type: String::from("Constant w"),
                    crossing_w_minus_one: false,
                    phantom_regions: vec![],
                },
            },
            observational_constraints: vec![
                Constraint {
                    constraint_id: String::from("const_1"),
                    observation_type: ObservationType::TypeIaSupernova,
                    data_source: String::from("Pantheon"),
                    w_constraint: [-1.1, -0.9],
                    wa_constraint: None,
                    figure_of_merit: 10.0,
                },
                Constraint {
                    constraint_id: String::from("const_2"),
                    observation_type: ObservationType::CMB,
                    data_source: String::from("Planck"),
                    w_constraint: [-1.05, -0.95],
                    wa_constraint: Some([-2.0, 0.5]),
                    figure_of_merit: 50.0,
                },
            ],
        }
    }

    pub fn compute_acceleration(&self, scale_factor: f64) -> f64 {
        let w = self.equation_of_state.w_0;
        3.0 * (1.0 + w) * self.cosmological_parameters.omega_lambda / (2.0 * scale_factor)
    }

    pub fn predict_future_expansion(&self, time_gyr: f64) -> FutureExpansion {
        FutureExpansion {
            current_time_gyr: 13.8,
            future_time_gyr: time_gyr,
            scale_factor_ratio: 3.0,
            distance_mpc: 5000.0,
        }
    }

    pub fn test_phantom_crossing(&self, model: &DarkEnergyModel) -> PhantomTest {
        PhantomTest {
            model_name: model.model_name.clone(),
            crossing_occurs: model.equation_of_state_w < -1.0,
            w_at_crossing: -1.05,
            implications: String::from("Phantom crossing would cause cosmic doomsday"),
        }
    }

    pub fn analyze_observational_constraints(&self) -> ConstraintAnalysis {
        ConstraintAnalysis {
            combined_w_constraint: [-1.03, -0.97],
            combined_wa_constraint: [-1.5, 1.0],
            model_viability_scores: HashMap::from([
                (String::from("Lambda CDM"), 0.95),
                (String::from("Quintessence"), 0.6),
            ]),
            recommended_next_observations: vec![String::from("Euclid"), String::from("LSST")],
        }
    }

    pub fn compute_equation_of_state_evolution(&self, z: f64) -> f64 {
        let w0 = self.equation_of_state.w_0;
        let wa = self.equation_of_state.w_a;
        w0 + wa * z / (1.0 + z)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureExpansion {
    pub current_time_gyr: f64,
    pub future_time_gyr: f64,
    pub scale_factor_ratio: f64,
    pub distance_mpc: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhantomTest {
    pub model_name: String,
    pub crossing_occurs: bool,
    pub w_at_crossing: f64,
    pub implications: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintAnalysis {
    pub combined_w_constraint: [f64; 2],
    pub combined_wa_constraint: [f64; 2],
    pub model_viability_scores: HashMap<String, f64>,
    pub recommended_next_observations: Vec<String>,
}

impl Default for DarkEnergy {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_acceleration() {
        let de = DarkEnergy::new();
        let a = de.compute_acceleration(1.0);
        assert!(a > 0.0);
    }
    #[test]
    fn test_eos_evolution() {
        let de = DarkEnergy::new();
        let w = de.compute_equation_of_state_evolution(0.5);
        assert!(w < 0.0);
    }
}
