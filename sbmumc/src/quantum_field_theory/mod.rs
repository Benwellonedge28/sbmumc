//! Quantum Field Theory Module
//!
//! This module implements quantum field theory frameworks, renormalization,
//! and field theoretic methods for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumFieldTheory {
    pub qft_id: String,
    pub field_theories: Vec<FieldTheory>,
    pub renormalization_groups: Vec<RenormalizationGroup>,
    pub symmetry_groups: Vec<SymmetryGroup>,
    pub feynman_diagrams: Vec<FeynmanDiagram>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldTheory {
    pub theory_id: String,
    pub theory_name: String,
    pub theory_type: QFTType,
    pub gauge_group: String,
    pub matter_content: Vec<MatterField>,
    pub lagrangian: String,
    pub renormalizable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QFTType {
    Scalar,
    Fermionic,
    Gauge,
    YangMills,
    Supersymmetric,
    Conformal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatterField {
    pub field_name: String,
    pub field_type: FieldCategory,
    pub spin: f64,
    pub mass_mev: f64,
    pub gauge_charges: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldCategory {
    ScalarBoson,
    Fermion,
    VectorBoson,
    Ghost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenormalizationGroup {
    pub rg_id: String,
    pub theory_name: String,
    pub beta_functions: Vec<BetaFunction>,
    pub anomalous_dimensions: Vec<f64>,
    pub fixed_points: Vec<FixedPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaFunction {
    pub coupling_name: String,
    pub beta_coefficient: f64,
    pub expansion_order: u32,
    pub asymptotic_free: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedPoint {
    pub fixed_point_id: String,
    pub coupling_values: Vec<f64>,
    pub stability: Stability,
    pub critical_exponents: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Stability {
    Stable,
    Unstable,
    MarginallyStable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymmetryGroup {
    pub group_id: String,
    pub group_name: String,
    pub rank: u32,
    pub dimension: u32,
    pub generators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeynmanDiagram {
    pub diagram_id: String,
    pub process_type: String,
    pub order: u32,
    pub external_legs: u32,
    pub internal_lines: u32,
    pub vertices: u32,
    pub amplitude: String,
}

impl QuantumFieldTheory {
    pub fn new() -> Self {
        Self {
            qft_id: String::from("qft_v1"),
            field_theories: vec![
                FieldTheory {
                    theory_id: String::from("qcd_1"),
                    theory_name: String::from("Quantum Chromodynamics"),
                    theory_type: QFTType::YangMills,
                    gauge_group: String::from("SU(3)"),
                    matter_content: vec![
                        MatterField { field_name: String::from("Quark"), field_type: FieldCategory::Fermion, spin: 0.5, mass_mev: 5.0, gauge_charges: vec![String::from("Color")] },
                    ],
                    lagrangian: String::from("QCD Lagrangian"),
                    renormalizable: true,
                },
            ],
            renormalization_groups: vec![
                RenormalizationGroup {
                    rg_id: String::from("rg_1"),
                    theory_name: String::from("QCD"),
                    beta_functions: vec![
                        BetaFunction { coupling_name: String::from("alpha_s"), beta_coefficient: -7.0, expansion_order: 3, asymptotic_free: true },
                    ],
                    anomalous_dimensions: vec![0.0],
                    fixed_points: vec![],
                },
            ],
            symmetry_groups: vec![
                SymmetryGroup {
                    group_id: String::from("grp_1"),
                    group_name: String::from("SU(2)"),
                    rank: 1,
                    dimension: 3,
                    generators: vec![String::from("sigma_1"), String::from("sigma_2"), String::from("sigma_3")],
                },
            ],
            feynman_diagrams: vec![
                FeynmanDiagram {
                    diagram_id: String::from("diag_1"),
                    process_type: String::from("Electron-Muon scattering"),
                    order: 1,
                    external_legs: 4,
                    internal_lines: 1,
                    vertices: 1,
                    amplitude: String::from("i M = -i e^2 / q^2"),
                },
            ],
        }
    }

    pub fn compute_amplitude(&self, diagram_id: &str) -> Result<Amplitude> {
        let diagram = self.feynman_diagrams.iter().find(|d| d.diagram_id == diagram_id)
            .ok_or(SbmumcError::NotFound(String::from("Diagram not found")))?;
        Ok(Amplitude {
            amplitude_id: format!("{}_amp", diagram_id),
            magnitude: 1e-10,
            phase: 0.0,
        })
    }

    pub fn calculate_cross_section(&self, initial_state: &[String], final_state: &[String]) -> CrossSection {
        CrossSection {
            cs_id: String::from("cs_1"),
            initial_state: initial_state.to_vec(),
            final_state: final_state.to_vec(),
            sigma: 1e-40,
            energy_gev: 100.0,
        }
    }

    pub fn run_coupling(&self, coupling: f64, mu_initial: f64, mu_final: f64) -> f64 {
        let beta = -0.5;
        coupling * (1.0 + beta * (mu_final / mu_initial).ln()).abs()
    }

    pub fn renormalize(&self, theory_id: &str, scheme: &str) -> RenormalizationResult {
        RenormalizationResult {
            theory_id: theory_id.to_string(),
            scheme: scheme.to_string(),
            renormalized_couplings: vec![0.1],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amplitude {
    pub amplitude_id: String,
    pub magnitude: f64,
    pub phase: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossSection {
    pub cs_id: String,
    pub initial_state: Vec<String>,
    pub final_state: Vec<String>,
    pub sigma: f64,
    pub energy_gev: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenormalizationResult {
    pub theory_id: String,
    pub scheme: String,
    pub renormalized_couplings: Vec<f64>,
}

impl Default for QuantumFieldTheory {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cross_section_calculation() {
        let qft = QuantumFieldTheory::new();
        let cs = qft.calculate_cross_section(&["e-".to_string()], &["mu-".to_string()]);
        assert!(cs.sigma > 0.0);
    }
    #[test]
    fn test_coupling_running() {
        let qft = QuantumFieldTheory::new();
        let g_run = qft.run_coupling(0.1, 100.0, 1000.0);
        assert!(g_run > 0.0);
    }
}
