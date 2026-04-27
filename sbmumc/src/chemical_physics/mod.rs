//! Chemical Physics Module
//!
//! This module implements chemical physics, reaction dynamics,
//! and molecular interactions for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChemicalPhysics {
    pub chem_id: String,
    pub reaction_dynamics: ReactionDynamics,
    pub molecular_spectroscopy: Vec<MolecularSpectrum>,
    pub chemical_kinetics: ChemicalKinetics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionDynamics { pub reactions: Vec<Reaction>, pub transition_states: Vec<TransitionState> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction { pub reaction_id: String, pub reactants: Vec<String>, pub products: Vec<String>, pub activation_energy_kj_mol: f64, pub rate_constant: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionState { pub ts_id: String, pub geometry: String, pub energy_kj_mol: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MolecularSpectrum { pub spectrum_id: String, pub molecule: String, pub absorption_lines: Vec<f64>, pub emission_lines: Vec<f64> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChemicalKinetics { pub rate_laws: Vec<RateLaw>, pub equilibrium_constants: Vec<f64> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLaw { pub law_id: String, pub order: u32, pub rate_expression: String }

impl ChemicalPhysics {
    pub fn new() -> Self {
        Self {
            chem_id: String::from("chemical_physics_v1"),
            reaction_dynamics: ReactionDynamics {
                reactions: vec![Reaction { reaction_id: String::from("rxn_1"), reactants: vec![String::from("A")], products: vec![String::from("B")], activation_energy_kj_mol: 50.0, rate_constant: 1e-3 }],
                transition_states: vec![TransitionState { ts_id: String::from("ts_1"), geometry: String::from("Saddle point"), energy_kj_mol: 55.0 }],
            },
            molecular_spectroscopy: vec![
                MolecularSpectrum { spectrum_id: String::from("spec_1"), molecule: String::from("CO2"), absorption_lines: vec![2300.0], emission_lines: vec![] },
            ],
            chemical_kinetics: ChemicalKinetics {
                rate_laws: vec![RateLaw { law_id: String::from("rl_1"), order: 2, rate_expression: String::from("k[A][B]") }],
                equilibrium_constants: vec![1e5],
            },
        }
    }

    pub fn compute_arrhenius_rate(&self, a: f64, ea: f64, t: f64) -> f64 {
        let r = 8.314;
        a * (-ea / (r * t)).exp()
    }

    pub fn compute_equilibrium_constant(&self, delta_g: f64, t: f64) -> f64 {
        let r = 8.314;
        (-delta_g / (r * t)).exp()
    }
}

impl Default for ChemicalPhysics { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_arrhenius() { let cp = ChemicalPhysics::new(); assert!(cp.compute_arrhenius_rate(1e10, 50000.0, 300.0) > 0.0); } }
