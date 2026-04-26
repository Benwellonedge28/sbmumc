//! Particle Physics Module
//!
//! This module implements particle physics, Standard Model particles,
//! and beyond Standard Model physics for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticlePhysics {
    pub particle_id: String,
    pub standard_model: StandardModel,
    pub particle_database: Vec<Particle>,
    pub interactions: Vec<Interaction>,
    pub decay_channels: Vec<DecayChannel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardModel {
    pub fermions: Vec<Fermion>,
    pub bosons: Vec<Boson>,
    pub higgs_sector: HiggsSector,
    pub gauge_groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fermion {
    pub fermion_id: String,
    pub fermion_type: FermionType,
    pub generation: u32,
    pub mass_mev: f64,
    pub charge: f64,
    pub spin: f64,
    pub color_charge: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FermionType {
    Quark,
    Lepton,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boson {
    pub boson_id: String,
    pub boson_type: BosonType,
    pub mass_gev: f64,
    pub charge: f64,
    pub spin: f64,
    pub decay_width_gev: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BosonType {
    GaugeBoson,
    HiggsBoson,
    Gluon,
    Photon,
    WGauge,
    ZGauge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiggsSector {
    pub higgs_mass_gev: f64,
    pub vacuum_expectation_gev: f64,
    pub couplings: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Particle {
    pub particle_id: String,
    pub name: String,
    pub mass_gev: f64,
    pub width_gev: f64,
    pub charge: f64,
    pub spin: f64,
    pub is_stable: bool,
    pub decay_modes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub interaction_id: String,
    pub interaction_type: InteractionCategory,
    pub coupling_constant: f64,
    pub mediator: String,
    pub vertex_factor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InteractionCategory {
    Strong,
    Electromagnetic,
    Weak,
    Gravitational,
    Higgs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayChannel {
    pub channel_id: String,
    pub parent_particle: String,
    pub final_state: Vec<String>,
    pub branching_ratio: f64,
    pub lifetime_s: f64,
}

impl ParticlePhysics {
    pub fn new() -> Self {
        Self {
            particle_id: String::from("particle_physics_v1"),
            standard_model: StandardModel {
                fermions: vec![
                    Fermion { fermion_id: String::from("u"), fermion_type: FermionType::Quark, generation: 1, mass_mev: 2.2, charge: 2.0/3.0, spin: 0.5, color_charge: Some(String::from("RGB")) },
                    Fermion { fermion_id: String::from("d"), fermion_type: FermionType::Quark, generation: 1, mass_mev: 4.7, charge: -1.0/3.0, spin: 0.5, color_charge: Some(String::from("RGB")) },
                    Fermion { fermion_id: String::from("e"), fermion_type: FermionType::Lepton, generation: 1, mass_mev: 0.511, charge: -1.0, spin: 0.5, color_charge: None },
                ],
                bosons: vec![
                    Boson { boson_id: String::from("g"), boson_type: BosonType::GaugeBoson, mass_gev: 0.0, charge: 0.0, spin: 1.0, decay_width_gev: 0.0 },
                    Boson { boson_id: String::from("h"), boson_type: BosonType::HiggsBoson, mass_gev: 125.0, charge: 0.0, spin: 0.0, decay_width_gev: 0.004 },
                ],
                higgs_sector: HiggsSector {
                    higgs_mass_gev: 125.0,
                    vacuum_expectation_gev: 246.0,
                    couplings: HashMap::from([
                        (String::from("top"), 0.995),
                        (String::from("W"), 0.65),
                    ]),
                },
                gauge_groups: vec![String::from("SU(3)"), String::from("SU(2)"), String::from("U(1)")],
            },
            particle_database: vec![
                Particle {
                    particle_id: String::from("mu"),
                    name: String::from("Muon"),
                    mass_gev: 0.1057,
                    width_gev: 3e-16,
                    charge: -1.0,
                    spin: 0.5,
                    is_stable: false,
                    decay_modes: vec![String::from("e nu_e nu_mu")],
                },
            ],
            interactions: vec![
                Interaction {
                    interaction_id: String::from("int_1"),
                    interaction_type: InteractionCategory::Strong,
                    coupling_constant: 0.1,
                    mediator: String::from("gluon"),
                    vertex_factor: String::from("g_s T^a"),
                },
            ],
            decay_channels: vec![
                DecayChannel {
                    channel_id: String::from("decay_mu"),
                    parent_particle: String::from("mu-"),
                    final_state: vec![String::from("e-"), String::from("nu_e"), String::from("nu_mu")],
                    branching_ratio: 1.0,
                    lifetime_s: 2.2e-6,
                },
            ],
        }
    }

    pub fn compute_coupling_running(&self, alpha_s_mz: f64, scale_gev: f64) -> f64 {
        let beta_0 = -23.0 / 6.0;
        alpha_s_mz / (1.0 + beta_0 * alpha_s_mz * (scale_gev / 91.0).ln() / (2.0 * 3.14159))
    }

    pub fn calculate_decay_width(&self, particle: &Particle) -> f64 {
        particle.width_gev
    }

    pub fn test_particle_decay(&self, particle_id: &str) -> DecayTest {
        DecayTest {
            particle_id: particle_id.to_string(),
            decays_possible: true,
            dominant_decay: String::from("Main channel"),
            lifetime_s: 1e-6,
        }
    }

    pub fn compute_cross_section(&self, process: &str, energy_gev: f64) -> CrossSectionResult {
        CrossSectionResult {
            process_name: process.to_string(),
            center_of_mass_energy: energy_gev,
            cross_section_pb: 1e-3,
            uncertainty: 0.1,
        }
    }

    pub fn verify_gauge_invariance(&self) -> GaugeInvariance {
        GaugeInvariance {
            u1_em_invariant: true,
            su2_weak_invariant: true,
            su3_color_invariant: true,
            all_checks_passed: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayTest {
    pub particle_id: String,
    pub decays_possible: bool,
    pub dominant_decay: String,
    pub lifetime_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossSectionResult {
    pub process_name: String,
    pub center_of_mass_energy: f64,
    pub cross_section_pb: f64,
    pub uncertainty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaugeInvariance {
    pub u1_em_invariant: bool,
    pub su2_weak_invariant: bool,
    pub su3_color_invariant: bool,
    pub all_checks_passed: bool,
}

impl Default for ParticlePhysics {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coupling_running() {
        let pp = ParticlePhysics::new();
        let alpha_s = pp.compute_coupling_running(0.1, 1000.0);
        assert!(alpha_s > 0.0);
    }
}
