//! Biophysics Module
//!
//! This module implements biophysics, biological physics,
//! and living systems physics for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biophysics {
    pub bio_id: String,
    pub molecular_biophysics: MolecularBiophysics,
    pub cellular_biophysics: CellularBiophysics,
    pub biomechanics: Biomechanics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularBiophysics { pub protein_structures: Vec<ProteinStructure>, pub dna_mechanics: DNAMechanics }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinStructure { pub structure_id: String, pub secondary_structure: Vec<String>, pub folding_energy_kcal_mol: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNAMechanics { pub persistence_length_nm: f64, pub torsional_stiffness: f64, pub elastic_modulus: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularBiophysics { pub membrane_properties: Vec<MembraneProperty>, pub ion_channels: Vec<IonChannel> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembraneProperty { pub property_id: String, pub fluidity: f64, pub permeability: f64, pub thickness_nm: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonChannel { pub channel_id: String, pub ion_type: String, pub conductance_ps: f64, pub gating: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biomechanics { pub muscle_properties: Vec<MuscleProperty>, pub skeletal_mechanics: SkeletalMechanics }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscleProperty { pub muscle_id: String, pub max_force_n: f64, pub contraction_speed: f64, pub efficiency: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkeletalMechanics { pub bone_strength_mpa: f64, pub joint_mechanics: Vec<JointMechanics> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointMechanics { pub joint_name: String, pub range_of_motion_deg: f64, pub torque_capacity: f64 }

impl Biophysics {
    pub fn new() -> Self {
        Self {
            bio_id: String::from("biophysics_v1"),
            molecular_biophysics: MolecularBiophysics {
                protein_structures: vec![ProteinStructure { structure_id: String::from("prot_1"), secondary_structure: vec![String::from("Alpha helix")], folding_energy_kcal_mol: 5.0 }],
                dna_mechanics: DNAMechanics { persistence_length_nm: 50.0, torsional_stiffness: 100.0, elastic_modulus: 1e9 },
            },
            cellular_biophysics: CellularBiophysics {
                membrane_properties: vec![MembraneProperty { property_id: String::from("mem_1"), fluidity: 0.8, permeability: 0.5, thickness_nm: 5.0 }],
                ion_channels: vec![IonChannel { channel_id: String::from("ch_1"), ion_type: String::from("Na+"), conductance_ps: 20.0, gating: String::from("Voltage-gated") }],
            },
            biomechanics: Biomechanics {
                muscle_properties: vec![MuscleProperty { muscle_id: String::from("mus_1"), max_force_n: 10000.0, contraction_speed: 5.0, efficiency: 0.3 }],
                skeletal_mechanics: SkeletalMechanics { bone_strength_mpa: 150.0, joint_mechanics: vec![JointMechanics { joint_name: String::from("Knee"), range_of_motion_deg: 140.0, torque_capacity: 200.0 }] },
            },
        }
    }

    pub fn compute_diffusion_time(&self, distance_um: f64, diffusion_coeff: f64) -> f64 { distance_um.powi(2) / (4.0 * diffusion_coeff) }
    pub fn compute_membrane_potential(&self, ion_concentrations: &[f64]) -> f64 { 60.0 * (ion_concentrations[0] / ion_concentrations[1]).log10() }
}

impl Default for Biophysics { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_diffusion_time() { let bio = Biophysics::new(); assert!(bio.compute_diffusion_time(1.0, 1e-9) > 0.0); } }
