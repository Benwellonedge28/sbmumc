//! Antimatter Module
//!
//! This module implements antimatter theory, CP violation,
//! and baryogenesis for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Antimatter {
    pub anti_id: String,
    pub baryogenesis: Baryogenesis,
    pub cp_violation: CPViolation,
    pub matter_antimatter_asymmetry: Asymmetry,
    pub annihilation_phenomenology: Annihilation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Baryogenesis {
    pub baryogenesis_id: String,
    pub mechanisms: Vec<BaryogenesisMechanism>,
    pub sakharov_conditions: SakharovConditions,
    pub estimated_asymmetry: f64,
    pub viable_mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaryogenesisMechanism {
    pub mechanism_id: String,
    pub mechanism_name: String,
    pub baryon_to_photon: f64,
    pub energy_scale_gev: f64,
    pub status: MechanismStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MechanismStatus {
    Viable,
    RuledOut,
    RequiresNewPhysics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SakharovConditions {
    pub baryon_number_violation: bool,
    pub cp_violation: bool,
    pub out_of_equilibrium: bool,
    pub all_satisfied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPViolation {
    pub cpv_id: String,
    pub standard_model_contributions: Vec<SMCPViolation>,
    pub meson_systems: Vec<MesonSystem>,
    pub ckm_matrix: CKMMatrix,
    pub pmns_matrix: PMNSMatrix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMCPViolation {
    pub source_name: String,
    pub contribution_strength: f64,
    pub jarlskog_invariant: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MesonSystem {
    pub system_id: String,
    pub meson_type: MesonType,
    pub cpv_parameters: Vec<CPVParameter>,
    pub experimental_value: f64,
    pub theoretical_prediction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MesonType {
    Kaon,
    B0,
    Bs0,
    D0,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPVParameter {
    pub parameter_name: String,
    pub value: f64,
    pub uncertainty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CKMMatrix {
    pub matrix_elements: [[f64; 3]; 3],
    pub jarlskog: f64,
    pub unitarity_triangle: UnitarityTriangle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitarityTriangle {
    pub alpha_deg: f64,
    pub beta_deg: f64,
    pub gamma_deg: f64,
    pub area: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PMNSMatrix {
    pub matrix_elements: [[f64; 3]; 3],
    pub cp_phase_delta: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asymmetry {
    pub observed_asymmetry: f64,
    pub baryon_to_photon_ratio: f64,
    pub standard_model_prediction: f64,
    pub discrepancy_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annihilation {
    pub primary_channels: Vec<AnnihilationChannel>,
    pub cross_section: f64,
    pub residual_annihilation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnihilationChannel {
    pub channel_id: String,
    pub final_state: String,
    pub branching_fraction: f64,
    pub gamma_signature: String,
}

impl Antimatter {
    pub fn new() -> Self {
        Self {
            anti_id: String::from("antimatter_v1"),
            baryogenesis: Baryogenesis {
                baryogenesis_id: String::from("bg_1"),
                mechanisms: vec![
                    BaryogenesisMechanism {
                        mechanism_id: String::from("mech_1"),
                        mechanism_name: String::from("Electroweak baryogenesis"),
                        baryon_to_photon: 6e-10,
                        energy_scale_gev: 1e2,
                        status: MechanismStatus::RequiresNewPhysics,
                    },
                    BaryogenesisMechanism {
                        mechanism_id: String::from("mech_2"),
                        mechanism_name: String::from("Leptogenesis"),
                        baryon_to_photon: 6e-10,
                        energy_scale_gev: 1e9,
                        status: MechanismStatus::Viable,
                    },
                ],
                sakharov_conditions: SakharovConditions {
                    baryon_number_violation: true,
                    cp_violation: true,
                    out_of_equilibrium: true,
                    all_satisfied: true,
                },
                estimated_asymmetry: 6e-10,
                viable_mechanisms: vec![String::from("Leptogenesis")],
            },
            cp_violation: CPViolation {
                cpv_id: String::from("cpv_1"),
                standard_model_contributions: vec![
                    SMCPViolation {
                        source_name: String::from("CKM phase"),
                        contribution_strength: 3e-5,
                        jarlskog_invariant: 3e-5,
                    },
                ],
                meson_systems: vec![
                    MesonSystem {
                        system_id: String::from("kaon_1"),
                        meson_type: MesonType::Kaon,
                        cpv_parameters: vec![
                            CPVParameter { parameter_name: String::from("epsilon"), value: 2.2e-3, uncertainty: 0.001 },
                        ],
                        experimental_value: 2.2e-3,
                        theoretical_prediction: 2.1e-3,
                    },
                ],
                ckm_matrix: CKMMatrix {
                    matrix_elements: [
                        [0.974, 0.225, 0.003],
                        [0.225, 0.974, 0.041],
                        [0.009, 0.040, 0.999],
                    ],
                    jarlskog: 3e-5,
                    unitarity_triangle: UnitarityTriangle {
                        alpha_deg: 85.0,
                        beta_deg: 21.0,
                        gamma_deg: 74.0,
                        area: 3e-5,
                    },
                },
                pmns_matrix: PMNSMatrix {
                    matrix_elements: [
                        [0.8, 0.5, 0.2],
                        [0.4, 0.6, 0.7],
                        [0.3, 0.5, 0.8],
                    ],
                    cp_phase_delta: 234.0,
                },
            },
            matter_antimatter_asymmetry: Asymmetry {
                observed_asymmetry: 6e-10,
                baryon_to_photon_ratio: 6.1e-10,
                standard_model_prediction: 1e-20,
                discrepancy_factor: 1e10,
            },
            annihilation_phenomenology: Annihilation {
                primary_channels: vec![
                    AnnihilationChannel {
                        channel_id: String::from("ch_1"),
                        final_state: String::from("b b-bar"),
                        branching_fraction: 0.7,
                        gamma_signature: String::from("Line at 511 keV"),
                    },
                ],
                cross_section: 3e-26,
                residual_annihilation: true,
            },
        }
    }

    pub fn compute_baryon_asymmetry(&self) -> f64 {
        self.matter_antimatter_asymmetry.baryon_to_photon_ratio
    }

    pub fn analyze_sakharov_conditions(&self) -> SakharovAnalysis {
        SakharovAnalysis {
            conditions: self.baryogenesis.sakharov_conditions.clone(),
            all_satisfied: self.baryogenesis.sakharov_conditions.all_satisfied,
            missing_conditions: vec![],
            recommendations: vec![String::from("SM insufficient for observed asymmetry")],
        }
    }

    pub fn test_electroweak_baryogenesis(&self) -> EWBGTest {
        EWBGTest {
            test_id: String::from("ewbg_1"),
            sphaleron_rate_sufficient: false,
            phase_transition_strong_first_order: false,
            cp_violation_sufficient: false,
            overall_viable: false,
            required_new_physics: String::from("Additional CP violation sources"),
        }
    }

    pub fn predict_leptogenesis(&self) -> LeptogenesisPrediction {
        LeptogenesisPrediction {
            heavy_neutrino_mass_gev: 1e10,
            produced_asymmetry: 6e-10,
            washout_factor: 0.1,
            final_baryon_asymmetry: 6e-10,
        }
    }

    pub fn compute_ckm_phase(&self) -> f64 {
        let matrix = &self.cp_violation.ckm_matrix;
        matrix.jarlskog.acos() * 180.0 / 3.14159
    }

    pub fn analyze_meson_cp_violation(&self, meson: &MesonSystem) -> MesonCPVAnalysis {
        MesonCPVAnalysis {
            meson_type: meson.meson_type.clone(),
            experimental_vs_predicted: meson.experimental_value / meson.theoretical_prediction,
            agreement: 1.0 - (meson.experimental_value - meson.theoretical_prediction).abs() / meson.experimental_value,
            new_physics_needed: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SakharovAnalysis {
    pub conditions: SakharovConditions,
    pub all_satisfied: bool,
    pub missing_conditions: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EWBGTest {
    pub test_id: String,
    pub sphaleron_rate_sufficient: bool,
    pub phase_transition_strong_first_order: bool,
    pub cp_violation_sufficient: bool,
    pub overall_viable: bool,
    pub required_new_physics: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeptogenesisPrediction {
    pub heavy_neutrino_mass_gev: f64,
    pub produced_asymmetry: f64,
    pub washout_factor: f64,
    pub final_baryon_asymmetry: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MesonCPVAnalysis {
    pub meson_type: MesonType,
    pub experimental_vs_predicted: f64,
    pub agreement: f64,
    pub new_physics_needed: bool,
}

impl Default for Antimatter {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_baryon_asymmetry() {
        let anti = Antimatter::new();
        let asymmetry = anti.compute_baryon_asymmetry();
        assert!(asymmetry > 0.0);
    }
    #[test]
    fn test_sakharov_analysis() {
        let anti = Antimatter::new();
        let analysis = anti.analyze_sakharov_conditions();
        assert!(analysis.all_satisfied);
    }
}
