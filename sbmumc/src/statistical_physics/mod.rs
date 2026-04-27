//! Statistical Physics Module
//!
//! This module implements statistical mechanics, partition functions,
//! and ensemble theory for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalPhysics {
    pub stat_id: String,
    pub ensembles: Vec<Ensemble>,
    pub partition_functions: Vec<PartitionFunction>,
    pub distributions: Vec<Distribution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ensemble {
    pub ensemble_id: String,
    pub ensemble_type: EnsembleType,
    pub thermodynamic_potential: String,
    pub variables: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnsembleType { Microcanonical, Canonical, GrandCanonical, IsothermalIsobaric }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionFunction {
    pub pf_id: String,
    pub ensemble_type: EnsembleType,
    pub expression: String,
    pub computed_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distribution {
    pub dist_id: String,
    pub distribution_type: DistributionType,
    pub parameters: Vec<f64>,
    pub entropy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DistributionType { MaxwellBoltzmann, BoseEinstein, FermiDirac, Planck }

impl StatisticalPhysics {
    pub fn new() -> Self {
        Self {
            stat_id: String::from("statistical_physics_v1"),
            ensembles: vec![
                Ensemble { ensemble_id: String::from("ens_canonical"), ensemble_type: EnsembleType::Canonical, thermodynamic_potential: String::from("Free energy F = -kT ln Z"), variables: vec![String::from("N"), String::from("V"), String::from("T")] },
            ],
            partition_functions: vec![
                PartitionFunction { pf_id: String::from("pf_1"), ensemble_type: EnsembleType::Canonical, expression: String::from("Z = sum exp(-beta E_n)"), computed_value: 1.0e30 },
            ],
            distributions: vec![
                Distribution { dist_id: String::from("dist_mb"), distribution_type: DistributionType::MaxwellBoltzmann, parameters: vec![1.0, 300.0], entropy: 10.0 },
            ],
        }
    }

    pub fn compute_partition_function(&self, energies: &[f64], beta: f64) -> f64 {
        energies.iter().map(|e| (-beta * e).exp()).sum()
    }

    pub fn compute_boltzmann_distribution(&self, energy: f64, beta: f64, z: f64) -> f64 {
        (-beta * energy).exp() / z
    }

    pub fn compute_fermi_dirac(&self, energy: f64, mu: f64, t: f64) -> f64 {
        1.0 / ((energy - mu) / (8.617e-5 * t)).exp() + 1.0
    }

    pub fn compute_bose_einstein(&self, energy: f64, mu: f64, t: f64) -> f64 {
        1.0 / ((energy - mu) / (8.617e-5 * t)).exp() - 1.0
    }
}

impl Default for StatisticalPhysics { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_partition() { let sp = StatisticalPhysics::new(); assert!(sp.compute_partition_function(&[1.0, 2.0, 3.0], 1.0) > 0.0); } }
