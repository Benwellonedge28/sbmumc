//! Chaos Theory Module
//!
//! This module implements chaos theory, nonlinear dynamics,
//! and deterministic chaos for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosTheory {
    pub chaos_id: String,
    pub chaotic_systems: Vec<ChaoticSystem>,
    pub bifurcation_diagrams: Vec<Bifurcation>,
    pub strange_attractors: Vec<StrangeAttractor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaoticSystem { pub sys_id: String, pub system_name: String, pub dimension: u32, pub equations: Vec<String>, pub lyapunov_exponent: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bifurcation { pub bif_id: String, pub parameter: String, pub bifurcation_type: BifurcationType, pub critical_values: Vec<f64> }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BifurcationType { SaddleNode, Hopf, PeriodDoubling, Crisis }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrangeAttractor { pub attractor_id: String, pub attractor_name: String, pub fractal_dimension: f64, pub lyapunov_dimension: f64 }

impl ChaosTheory {
    pub fn new() -> Self {
        Self {
            chaos_id: String::from("chaos_theory_v1"),
            chaotic_systems: vec![
                ChaoticSystem { sys_id: String::from("lorenz"), system_name: String::from("Lorenz system"), dimension: 3, equations: vec![String::from("dx/dt = sigma(y-x)"), String::from("dy/dt = x(rho - z) - y"), String::from("dz/dt = xy - beta z")], lyapunov_exponent: 0.9 },
            ],
            bifurcation_diagrams: vec![Bifurcation { bif_id: String::from("bif_1"), parameter: String::from("r"), bifurcation_type: BifurcationType::PeriodDoubling, critical_values: vec![3.0, 3.45, 3.54] }],
            strange_attractors: vec![StrangeAttractor { attractor_id: String::from("attr_lorenz"), attractor_name: String::from("Lorenz attractor"), fractal_dimension: 2.06, lyapunov_dimension: 2.06 }],
        }
    }

    pub fn compute_lyapunov_exponent(&self, trajectory: &[f64]) -> f64 {
        let n = trajectory.len();
        if n < 2 { return 0.0; }
        let growth = (trajectory[n-1] / trajectory[0]).ln() / (n as f64);
        growth.abs()
    }

    pub fn detect_bifurcation(&self, param: f64, system: &ChaoticSystem) -> Option<Bifurcation> {
        let _ = system;
        if param > 3.0 { Some(Bifurcation { bif_id: String::from("bif_detected"), parameter: String::from("param"), bifurcation_type: BifurcationType::Hopf, critical_values: vec![param] }) } else { None }
    }
}

impl Default for ChaosTheory { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_lyapunov() { let chaos = ChaosTheory::new(); assert!(chaos.compute_lyapunov_exponent(&[1.0, 2.0, 4.0]) > 0.0); } }
