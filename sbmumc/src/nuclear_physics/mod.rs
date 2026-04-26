//! Nuclear Physics Module
//!
//! This module implements nuclear physics, nuclear reactions,
//! and nuclear structure for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearPhysics {
    pub nuclear_id: String,
    pub nuclei: Vec<Nucleus>,
    pub reactions: Vec<NuclearReaction>,
    pub decay_schemes: Vec<DecayScheme>,
    pub nuclear_structure: NuclearStructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nucleus {
    pub nucleus_id: String,
    pub atomic_number: u32,
    pub mass_number: u32,
    pub mass_amu: f64,
    pub binding_energy_mev: f64,
    pub spin: f64,
    pub parity: String,
    pub half_life_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearReaction {
    pub reaction_id: String,
    pub target: String,
    pub projectile: String,
    pub products: Vec<String>,
    pub q_value_mev: f64,
    pub threshold_energy_mev: f64,
    pub cross_section_mb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayScheme {
    pub scheme_id: String,
    pub parent_nucleus: String,
    pub decay_type: DecayType,
    pub daughter_nucleus: String,
    pub branching_ratio: f64,
    pub energy_release_mev: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DecayType {
    Alpha,
    BetaMinus,
    BetaPlus,
    Gamma,
    Fission,
    ProtonEmission,
    NeutronEmission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearStructure {
    pub shell_model: ShellModel,
    pub collective_model: CollectiveModel,
    pub magic_numbers: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellModel {
    pub shells: Vec<Shell>,
    pub level_energies: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shell {
    pub shell_number: u32,
    pub degeneracy: u32,
    pub occupied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveModel {
    pub rotational_bands: Vec<RotationalBand>,
    pub vibrational_states: Vec<VibrationalState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationalBand {
    pub band_id: String,
    pub band_head_spin: u32,
    pub moment_of_inertia: f64,
    pub transition_multipolarity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibrationalState {
    pub state_id: String,
    pub phonon_number: u32,
    pub energy_mev: f64,
}

impl NuclearPhysics {
    pub fn new() -> Self {
        Self {
            nuclear_id: String::from("nuclear_physics_v1"),
            nuclei: vec![
                Nucleus {
                    nucleus_id: String::from("fe56"),
                    atomic_number: 26,
                    mass_number: 56,
                    mass_amu: 55.9349,
                    binding_energy_mev: 492.3,
                    spin: 0.0,
                    parity: String::from("+"),
                    half_life_s: f64::INFINITY,
                },
                Nucleus {
                    nucleus_id: String::from("u235"),
                    atomic_number: 92,
                    mass_number: 235,
                    mass_amu: 235.044,
                    binding_energy_mev: 1786.0,
                    spin: 3.5,
                    parity: String::from("-"),
                    half_life_s: 2.2e16,
                },
            ],
            reactions: vec![
                NuclearReaction {
                    reaction_id: String::from("rxn_1"),
                    target: String::from("U-235"),
                    projectile: String::from("neutron"),
                    products: vec![String::from("Ba-141"), String::from("Kr-92"), String::from("3 neutrons")],
                    q_value_mev: 200.0,
                    threshold_energy_mev: 0.0,
                    cross_section_mb: 585.0,
                },
            ],
            decay_schemes: vec![
                DecayScheme {
                    scheme_id: String::from("decay_1"),
                    parent_nucleus: String::from("Co-60"),
                    decay_type: DecayType::BetaMinus,
                    daughter_nucleus: String::from("Ni-60"),
                    branching_ratio: 0.9999,
                    energy_release_mev: 2.82,
                },
            ],
            nuclear_structure: NuclearStructure {
                shell_model: ShellModel {
                    shells: vec![
                        Shell { shell_number: 1, degeneracy: 2, occupied: true },
                        Shell { shell_number: 2, degeneracy: 6, occupied: true },
                    ],
                    level_energies: HashMap::new(),
                },
                collective_model: CollectiveModel {
                    rotational_bands: vec![],
                    vibrational_states: vec![],
                },
                magic_numbers: vec![2, 8, 20, 28, 50, 82, 126],
            },
        }
    }

    pub fn compute_binding_energy(&self, mass_number: u32, atomic_number: u32, mass_amu: f64) -> f64 {
        let z = atomic_number as f64;
        let a = mass_number as f64;
        let n = a - z;
        let m_p = 1.00727;
        let m_n = 1.00866;
        (z * m_p + n * m_n - mass_amu) * 931.5
    }

    pub fn compute_half_life(&self, decay_constant: f64) -> f64 {
        0.693 / decay_constant
    }

    pub fn calculate_fission_energy(&self, nucleus: &Nucleus) -> FissionEnergy {
        FissionEnergy {
            nucleus_name: format!("A={}", nucleus.mass_number),
            energy_per_fission_mev: 200.0,
            neutrons_per_fission: 2.5,
            delayed_neutron_fraction: 0.006,
        }
    }

    pub fn compute_reaction_rate(&self, cross_section: f64, flux: f64, density: f64) -> f64 {
        cross_section * flux * density
    }

    pub fn analyze_shell_structure(&self, z: u32, n: u32) -> ShellAnalysis {
        ShellAnalysis {
            z: z,
            n: n,
            is_magic_z: self.nuclear_structure.magic_numbers.contains(&z),
            is_magic_n: self.nuclear_structure.magic_numbers.contains(&n),
            closed_shells: vec![String::from("Z shell closed"), String::from("N shell closed")],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FissionEnergy {
    pub nucleus_name: String,
    pub energy_per_fission_mev: f64,
    pub neutrons_per_fission: f64,
    pub delayed_neutron_fraction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellAnalysis {
    pub z: u32,
    pub n: u32,
    pub is_magic_z: bool,
    pub is_magic_n: bool,
    pub closed_shells: Vec<String>,
}

impl Default for NuclearPhysics {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binding_energy() {
        let np = NuclearPhysics::new();
        let be = np.compute_binding_energy(56, 26, 55.9349);
        assert!(be > 0.0);
    }
}
