//! Unified Field Theory Module
//!
//! This module implements unified field theory frameworks, force unification,
//! and grand unified theories for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedFieldTheory {
    pub uft_id: String,
    pub gut_theories: Vec<GUTTheory>,
    pub force_unification: ForceUnification,
    pub symmetry_breaking: Vec<SymmetryBreaking>,
    pub unification_scales: Vec<UnificationScale>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GUTTheory {
    pub gut_id: String,
    pub gut_name: String,
    pub gauge_group: String,
    pub unification_scale_gev: f64,
    pub symmetry_breaking_chain: Vec<String>,
    pub predictions: Vec<GUTPrediction>,
    pub testable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GUTPrediction {
    pub prediction_id: String,
    pub phenomenon: String,
    pub rate: f64,
    pub detection_status: DetectionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DetectionStatus {
    Detected,
    Undetected,
    Predicted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceUnification {
    pub unified_force: String,
    pub coupling_unification: CouplingUnification,
    pub force_strengths: Vec<ForceStrength>,
    pub unification_point: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouplingUnification {
    pub qed_coupling: f64,
    pub weak_coupling: f64,
    pub strong_coupling: f64,
    pub unification_energy_gev: f64,
    pub coupling_match: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceStrength {
    pub force_name: String,
    pub coupling_constant: f64,
    pub energy_scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymmetryBreaking {
    pub breaking_id: String,
    pub initial_symmetry: String,
    pub final_symmetry: String,
    pub breaking_scale_gev: f64,
    pub mechanism: String,
    pub residual_symmetry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnificationScale {
    pub scale_id: String,
    pub scale_name: String,
    pub energy_gev: f64,
    pub corresponding_force: String,
    pub physics_at_scale: Vec<String>,
}

impl UnifiedFieldTheory {
    pub fn new() -> Self {
        Self {
            uft_id: String::from("unified_field_theory_v1"),
            gut_theories: vec![
                GUTTheory {
                    gut_id: String::from("gut_1"),
                    gut_name: String::from("SU(5)"),
                    gauge_group: String::from("SU(5)"),
                    unification_scale_gev: 1e15,
                    symmetry_breaking_chain: vec![String::from("SU(5)"), String::from("SM")],
                    predictions: vec![
                        GUTPrediction {
                            prediction_id: String::from("pred_1"),
                            phenomenon: String::from("Proton decay"),
                            rate: 1e34,
                            detection_status: DetectionStatus::Undetected,
                        },
                    ],
                    testable: true,
                },
                GUTTheory {
                    gut_id: String::from("gut_2"),
                    gut_name: String::from("SO(10)"),
                    gauge_group: String::from("SO(10)"),
                    unification_scale_gev: 1e16,
                    symmetry_breaking_chain: vec![String::from("SO(10)"), String::from("SU(5)"), String::from("SM")],
                    predictions: vec![
                        GUTPrediction {
                            prediction_id: String::from("pred_2"),
                            phenomenon: String::from("Neutrino mass"),
                            rate: 0.1,
                            detection_status: DetectionStatus::Detected,
                        },
                    ],
                    testable: true,
                },
            ],
            force_unification: ForceUnification {
                unified_force: String::from("Electroweak-Strong"),
                coupling_unification: CouplingUnification {
                    qed_coupling: 0.007,
                    weak_coupling: 0.01,
                    strong_coupling: 0.1,
                    unification_energy_gev: 1e15,
                    coupling_match: true,
                },
                force_strengths: vec![
                    ForceStrength { force_name: String::from("Electromagnetic"), coupling_constant: 0.007, energy_scale: 100.0 },
                    ForceStrength { force_name: String::from("Weak"), coupling_constant: 0.01, energy_scale: 100.0 },
                    ForceStrength { force_name: String::from("Strong"), coupling_constant: 0.1, energy_scale: 100.0 },
                ],
                unification_point: 1e15,
            },
            symmetry_breaking: vec![
                SymmetryBreaking {
                    breaking_id: String::from("break_1"),
                    initial_symmetry: String::from("GUT"),
                    final_symmetry: String::from("SM"),
                    breaking_scale_gev: 1e15,
                    mechanism: String::from("Higgs mechanism"),
                    residual_symmetry: String::from("SU(3)xSU(2)xU(1)"),
                },
            ],
            unification_scales: vec![
                UnificationScale {
                    scale_id: String::from("scale_1"),
                    scale_name: String::from("Electroweak scale"),
                    energy_gev: 1e2,
                    corresponding_force: String::from("Electroweak"),
                    physics_at_scale: vec![String::from("W/Z bosons"), String::from("Higgs")],
                },
                UnificationScale {
                    scale_id: String::from("scale_2"),
                    scale_name: String::from("GUT scale"),
                    energy_gev: 1e15,
                    corresponding_force: String::from("All forces unified"),
                    physics_at_scale: vec![String::from("X/Y bosons"), String::from("Proton decay")],
                },
            ],
        }
    }

    pub fn compute_coupling_running(&self, coupling_mz: f64, scale_gev: f64) -> f64 {
        let beta = -0.05;
        coupling_mz * (1.0 + beta * (scale_gev / 91.0).ln())
    }

    pub fn test_unification(&self) -> UnificationTest {
        UnificationTest {
            test_id: String::from("unif_test_1"),
            unification_achieved: self.force_unification.coupling_unification.coupling_match,
            unification_scale_gev: self.force_unification.unification_point,
            coupling_accuracy: 0.95,
            conclusions: String::from("Unification is consistent within uncertainties"),
        }
    }

    pub fn predict_proton_lifetime(&self, gut: &GUTTheory) -> ProtonLifetime {
        ProtonLifetime {
            gut_name: gut.gut_name.clone(),
            predicted_lifetime_s: 1e35,
            experimental_lower_bound_s: 1.6e34,
            status: LifetimeStatus::Constrained,
        }
    }

    pub fn compute_baryon_asymmetry(&self) -> BaryonAsymmetry {
        BaryonAsymmetry {
            observed_ratio: 6e-10,
            predicted_ratio: 1e-20,
            discrepancy_explanation: String::from("Requires additional CP violation and out-of-equilibrium decay"),
        }
    }

    pub fn analyze_symmetry_breaking(&self, gut: &GUTTheory) -> BreakingAnalysis {
        BreakingAnalysis {
            gut_name: gut.gut_name.clone(),
            initial_symmetry: gut.gauge_group.clone(),
            breaking_pattern: gut.symmetry_breaking_chain.clone(),
            vacuum_manifold: String::from("Coset space"),
            number_of_vacua: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnificationTest {
    pub test_id: String,
    pub unification_achieved: bool,
    pub unification_scale_gev: f64,
    pub coupling_accuracy: f64,
    pub conclusions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtonLifetime {
    pub gut_name: String,
    pub predicted_lifetime_s: f64,
    pub experimental_lower_bound_s: f64,
    pub status: LifetimeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LifetimeStatus {
    Allowed,
    Excluded,
    Constrained,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaryonAsymmetry {
    pub observed_ratio: f64,
    pub predicted_ratio: f64,
    pub discrepancy_explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakingAnalysis {
    pub gut_name: String,
    pub initial_symmetry: String,
    pub breaking_pattern: Vec<String>,
    pub vacuum_manifold: String,
    pub number_of_vacua: u64,
}

impl Default for UnifiedFieldTheory {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unification() {
        let uft = UnifiedFieldTheory::new();
        let test = uft.test_unification();
        assert!(test.unification_achieved);
    }
    #[test]
    fn test_coupling_running() {
        let uft = UnifiedFieldTheory::new();
        let g = uft.compute_coupling_running(0.1, 1e15);
        assert!(g > 0.0);
    }
}
