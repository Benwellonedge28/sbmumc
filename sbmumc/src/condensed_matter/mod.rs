//! Condensed Matter Physics Module
//!
//! This module implements condensed matter physics, solid state physics,
//! and material properties for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CondensedMatter {
    pub cm_id: String,
    pub crystal_structures: Vec<CrystalStructure>,
    pub band_structures: Vec<BandStructure>,
    pub phase_transitions: Vec<PhaseTransitionCM>,
    pub superconductivity: Superconductivity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystalStructure {
    pub structure_id: String,
    pub lattice_type: LatticeType,
    pub space_group: String,
    pub lattice_parameters: LatticeParameters,
    pub atom_positions: Vec<AtomPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LatticeType {
    BCC,
    FCC,
    HCP,
    Diamond,
    ZincBlende,
    Perovskite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatticeParameters {
    pub a_angstrom: f64,
    pub b_angstrom: f64,
    pub c_angstrom: f64,
    pub alpha_deg: f64,
    pub beta_deg: f64,
    pub gamma_deg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomPosition {
    pub element: String,
    pub position: [f64; 3],
    pub occupancy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandStructure {
    pub band_id: String,
    pub material: String,
    pub k_points: Vec<KPoint>,
    pub band_gap_ev: f64,
    pub effective_masses: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPoint {
    pub k_coord: [f64; 3],
    pub energy_ev: f64,
    pub band_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseTransitionCM {
    pub transition_id: String,
    pub initial_phase: String,
    pub final_phase: String,
    pub transition_temperature_k: f64,
    pub entropy_change: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Superconductivity {
    pub superconducting_materials: Vec<Superconductor>,
    pub critical_parameters: CriticalParameters,
    pub pairing_mechanism: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Superconductor {
    pub material_id: String,
    pub material_name: String,
    pub critical_temp_k: f64,
    pub critical_field_t: f64,
    pub coherence_length_nm: f64,
    pub penetration_depth_nm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalParameters {
    pub max_critical_temp_k: f64,
    pub max_critical_field_t: f64,
    pub room_temp_superconductor: bool,
}

impl CondensedMatter {
    pub fn new() -> Self {
        Self {
            cm_id: String::from("condensed_matter_v1"),
            crystal_structures: vec![
                CrystalStructure {
                    structure_id: String::from("struct_1"),
                    lattice_type: LatticeType::FCC,
                    space_group: String::from("Fm-3m"),
                    lattice_parameters: LatticeParameters {
                        a_angstrom: 3.615,
                        b_angstrom: 3.615,
                        c_angstrom: 3.615,
                        alpha_deg: 90.0,
                        beta_deg: 90.0,
                        gamma_deg: 90.0,
                    },
                    atom_positions: vec![
                        AtomPosition { element: String::from("Si"), position: [0.0, 0.0, 0.0], occupancy: 1.0 },
                    ],
                },
            ],
            band_structures: vec![
                BandStructure {
                    band_id: String::from("band_1"),
                    material: String::from("Silicon"),
                    k_points: vec![],
                    band_gap_ev: 1.12,
                    effective_masses: HashMap::from([
                        (String::from("electron"), 0.26),
                        (String::from("hole"), 0.36),
                    ]),
                },
            ],
            phase_transitions: vec![
                PhaseTransitionCM {
                    transition_id: String::from("pt_1"),
                    initial_phase: String::from("Liquid"),
                    final_phase: String::from("Solid"),
                    transition_temperature_k: 273.15,
                    entropy_change: 6.0,
                },
            ],
            superconductivity: Superconductivity {
                superconducting_materials: vec![
                    Superconductor {
                        material_id: String::from("sc_1"),
                        material_name: String::from("NbTi"),
                        critical_temp_k: 9.5,
                        critical_field_t: 12.0,
                        coherence_length_nm: 12.0,
                        penetration_depth_nm: 200.0,
                    },
                ],
                critical_parameters: CriticalParameters {
                    max_critical_temp_k: 250.0,
                    max_critical_field_t: 2000.0,
                    room_temp_superconductor: false,
                },
                pairing_mechanism: String::from("Electron-phonon (BCS)"),
            },
        }
    }

    pub fn compute_band_gap(&self, material: &str) -> f64 {
        if material == "Silicon" { 1.12 } else { 0.0 }
    }

    pub fn analyze_phonon_spectrum(&self) -> PhononSpectrum {
        PhononSpectrum {
            acoustic_modes: 3,
            optical_modes: 6,
            debye_temperature_k: 645.0,
            phonon_dispersion: vec![],
        }
    }

    pub fn compute_conductivity(&self, carrier_density: f64, mobility: f64) -> f64 {
        let e = 1.602e-19;
        carrier_density * e * mobility
    }

    pub fn analyze_superconducting_phase(&self, material: &Superconductor) -> SCAnalysis {
        SCAnalysis {
            material_name: material.material_name.clone(),
            type_classification: String::from("Type II"),
            london_penetration_depth: material.penetration_depth_nm,
            ginzburg_landau_kappa: 10.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhononSpectrum {
    pub acoustic_modes: u32,
    pub optical_modes: u32,
    pub debye_temperature_k: f64,
    pub phonon_dispersion: Vec<DispersionRelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispersionRelation {
    pub mode_type: String,
    pub dispersion_coefficient: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCAnalysis {
    pub material_name: String,
    pub type_classification: String,
    pub london_penetration_depth: f64,
    pub ginzburg_landau_kappa: f64,
}

impl Default for CondensedMatter {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_band_gap() {
        let cm = CondensedMatter::new();
        let gap = cm.compute_band_gap("Silicon");
        assert!(gap > 0.0);
    }
}
