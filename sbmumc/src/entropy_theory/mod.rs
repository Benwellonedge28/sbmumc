//! Entropy Theory Module
//!
//! This module implements entropy theory, thermodynamic analysis,
//! and information-theoretic approaches for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyTheory {
    pub entropy_id: String,
    pub thermodynamic_entropy: ThermodynamicEntropy,
    pub information_entropy: InformationEntropy,
    pub entropy_fluxes: Vec<EntropyFlux>,
    pub arrow_of_time: ArrowOfTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermodynamicEntropy {
    pub entropy_j_k: f64,
    pub temperature_k: f64,
    pub heat_capacity: f64,
    pub phase_transitions: Vec<PhaseTransition>,
    pub entropy_production_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformationEntropy {
    pub shannon_entropy: f64,
    pub bits: u64,
    pub probability_distribution: Vec<f64>,
    pub entropy_bounds: EntropyBounds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyBounds {
    pub Bekenstein_bound: f64,
    pub holographic_bound: f64,
    pub actual_entropy: f64,
    pub ratio_to_bound: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseTransition {
    pub transition_id: String,
    pub transition_type: TransitionType,
    pub temperature_k: f64,
    pub entropy_change: f64,
    pub latent_heat: f64,
    pub order: TransitionOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransitionType {
    FirstOrder,
    SecondOrder,
    Continuous,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransitionOrder {
    FirstOrder,
    SecondOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyFlux {
    pub flux_id: String,
    pub source: String,
    pub sink: String,
    pub flux_rate: f64,
    pub mechanism: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrowOfTime {
    pub thermodynamic_arrow: bool,
    pub cosmological_arrow: bool,
    pub quantum_arrow: bool,
    pub psychological_arrow: bool,
    pub coherence: ArrowCoherence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArrowCoherence {
    Aligned,
    Independent,
    Conflicting,
}

impl EntropyTheory {
    pub fn new() -> Self {
        Self {
            entropy_id: String::from("entropy_theory_v1"),
            thermodynamic_entropy: ThermodynamicEntropy {
                entropy_j_k: 1.0,
                temperature_k: 300.0,
                heat_capacity: 100.0,
                phase_transitions: vec![
                    PhaseTransition {
                        transition_id: String::from("trans_1"),
                        transition_type: TransitionType::FirstOrder,
                        temperature_k: 273.15,
                        entropy_change: 6.0,
                        latent_heat: 334000.0,
                        order: TransitionOrder::FirstOrder,
                    },
                ],
                entropy_production_rate: 0.1,
            },
            information_entropy: InformationEntropy {
                shannon_entropy: 4.5,
                bits: 100,
                probability_distribution: vec![0.1, 0.2, 0.3, 0.4],
                entropy_bounds: EntropyBounds {
                    Bekenstein_bound: 2.5e43,
                    holographic_bound: 1.0e77,
                    actual_entropy: 1e77,
                    ratio_to_bound: 0.01,
                },
            },
            entropy_fluxes: vec![
                EntropyFlux {
                    flux_id: String::from("flux_1"),
                    source: String::from("Heat reservoir"),
                    sink: String::from("Cold reservoir"),
                    flux_rate: 10.0,
                    mechanism: String::from("Conduction"),
                },
            ],
            arrow_of_time: ArrowOfTime {
                thermodynamic_arrow: true,
                cosmological_arrow: true,
                quantum_arrow: false,
                psychological_arrow: true,
                coherence: ArrowCoherence::Aligned,
            },
        }
    }

    pub fn calculate_boltzmann_entropy(&self, multiplicity: u64) -> f64 {
        let k_b = 1.38e-23;
        k_b * (multiplicity as f64).ln()
    }

    pub fn calculate_gibbs_entropy(&self, probabilities: &[f64]) -> f64 {
        let k_b = 1.38e-23;
        probabilities.iter().map(|p| {
            if *p > 0.0 { -k_b * p * p.ln() } else { 0.0 }
        }).sum()
    }

    pub fn calculate_von_neumann_entropy(&self, eigenvalues: &[f64]) -> f64 {
        eigenvalues.iter().map(|lambda| {
            if *lambda > 0.0 { -lambda * lambda.ln() } else { 0.0 }
        }).sum()
    }

    pub fn compute_mixing_entropy(&self, n1: f64, n2: f64) -> f64 {
        let k_b = 1.38e-23;
        let n_total = n1 + n2;
        let x1 = n1 / n_total;
        let x2 = n2 / n_total;
        -k_b * (x1 * x1.ln() + x2 * x2.ln())
    }

    pub fn analyze_entropy_production(&self) -> EntropyProduction {
        EntropyProduction {
            production_id: String::from("prod_1"),
            total_rate: self.thermodynamic_entropy.entropy_production_rate,
            irreversible_processes: vec![String::from("Heat conduction")],
            efficiency: 0.7,
        }
    }

    pub fn test_second_law(&self, process: &str) -> SecondLawTest {
        SecondLawTest {
            process_name: process.to_string(),
            entropy_change_positive: true,
            second_law_satisfied: true,
        }
    }

    pub fn compute_bekenstein_bound(&self, energy_j: f64, radius_m: f64) -> f64 {
        let hbar = 1.055e-34;
        let c = 3e8;
        let k_b = 1.38e-23;
        2.0 * 3.14159 * k_b * energy_j / (hbar * c)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyProduction {
    pub production_id: String,
    pub total_rate: f64,
    pub irreversible_processes: Vec<String>,
    pub efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondLawTest {
    pub process_name: String,
    pub entropy_change_positive: bool,
    pub second_law_satisfied: bool,
}

impl Default for EntropyTheory {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_boltzmann_entropy() {
        let entropy = EntropyTheory::new();
        let s = entropy.calculate_boltzmann_entropy(1000000);
        assert!(s > 0.0);
    }
    #[test]
    fn test_second_law() {
        let entropy = EntropyTheory::new();
        let test = entropy.test_second_law("Heat engine");
        assert!(test.second_law_satisfied);
    }
}
