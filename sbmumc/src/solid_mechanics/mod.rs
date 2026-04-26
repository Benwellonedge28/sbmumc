//! Solid Mechanics Module
//!
//! This module implements solid mechanics, stress-strain analysis,
//! and structural mechanics for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolidMechanics {
    pub solid_id: String,
    pub materials: Vec<Material>,
    pub stress_analysis: StressAnalysis,
    pub strain_analysis: StrainAnalysis,
    pub failure_criteria: Vec<FailureCriterion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material { pub material_id: String, pub material_name: String, pub youngs_modulus_gpa: f64, pub yield_strength_mpa: f64, pub density_kg_m3: f64, pub poisson_ratio: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressAnalysis { pub stress_tensor: [[f64; 3]; 3], pub principal_stresses: [f64; 3], pub von_mises_stress: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrainAnalysis { pub strain_tensor: [[f64; 3]; 3], pub principal_strains: [f64; 3], pub volumetric_strain: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureCriterion { pub criterion_id: String, pub criterion_name: String, pub material_limit: f64, pub safety_factor: f64 }

impl SolidMechanics {
    pub fn new() -> Self {
        Self {
            solid_id: String::from("solid_mechanics_v1"),
            materials: vec![
                Material { material_id: String::from("mat_steel"), material_name: String::from("Steel"), youngs_modulus_gpa: 200.0, yield_strength_mpa: 250.0, density_kg_m3: 7850.0, poisson_ratio: 0.3 },
                Material { material_id: String::from("mat_al"), material_name: String::from("Aluminum"), youngs_modulus_gpa: 70.0, yield_strength_mpa: 100.0, density_kg_m3: 2700.0, poisson_ratio: 0.33 },
            ],
            stress_analysis: StressAnalysis { stress_tensor: [[100.0, 0.0, 0.0], [0.0, 50.0, 0.0], [0.0, 0.0, 25.0]], principal_stresses: [100.0, 50.0, 25.0], von_mises_stress: 65.0 },
            strain_analysis: StrainAnalysis { strain_tensor: [[0.001, 0.0, 0.0], [0.0, 0.0005, 0.0], [0.0, 0.0, 0.00025]], principal_strains: [0.001, 0.0005, 0.00025], volumetric_strain: 0.00175 },
            failure_criteria: vec![
                FailureCriterion { criterion_id: String::from("crit_1"), criterion_name: String::from("von Mises"), material_limit: 200.0, safety_factor: 2.0 },
            ],
        }
    }

    pub fn compute_stress(&self, f: f64, a: f64) -> f64 { f / a }
    pub fn compute_strain(&self, dl: f64, l0: f64) -> f64 { dl / l0 }
    pub fn compute_youngs_modulus(&self, stress: f64, strain: f64) -> f64 { stress / strain }
    pub fn compute_thermal_stress(&self, alpha: f64, delta_t: f64, e: f64) -> f64 { alpha * delta_t * e }
}

impl Default for SolidMechanics { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_stress() { let sm = SolidMechanics::new(); assert!(sm.compute_stress(1000.0, 0.01) > 0.0); } }
