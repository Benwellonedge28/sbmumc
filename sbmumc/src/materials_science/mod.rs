//! Materials Science Module
//!
//! This module implements materials science, material properties,
//! and material characterization for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialsScience {
    pub mat_id: String,
    pub materials: Vec<Material>,
    pub characterization_techniques: Vec<Technique>,
    pub phase_diagrams: Vec<PhaseDiagram>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material { pub mat_id: String, pub material_name: String, pub material_type: MaterialType, pub properties: MaterialProperties }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MaterialType { Metal, Ceramic, Polymer, Composite, Semiconductor }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialProperties { pub youngs_modulus_gpa: f64, pub hardness_hv: f64, pub density_kg_m3: f64, pub thermal_conductivity: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technique { pub tech_id: String, pub tech_name: String, pub resolution_nm: f64, pub information: Vec<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseDiagram { pub diagram_id: String, pub system: String, pub phases: Vec<Phase> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase { pub phase_id: String, pub phase_name: String, pub composition: Vec<f64>, pub temperature_range: [f64; 2] }

impl MaterialsScience {
    pub fn new() -> Self {
        Self {
            mat_id: String::from("materials_science_v1"),
            materials: vec![
                Material { mat_id: String::from("mat_steel"), material_name: String::from("Stainless steel"), material_type: MaterialType::Metal, properties: MaterialProperties { youngs_modulus_gpa: 200.0, hardness_hv: 200.0, density_kg_m3: 7850.0, thermal_conductivity: 16.0 } },
            ],
            characterization_techniques: vec![Technique { tech_id: String::from("ct_1"), tech_name: String::from("SEM"), resolution_nm: 1.0, information: vec![String::from("Surface morphology")] }],
            phase_diagrams: vec![PhaseDiagram { diagram_id: String::from("pd_fe_c"), system: String::from("Fe-C"), phases: vec![Phase { phase_id: String::from("phase_1"), phase_name: String::from("Austenite"), composition: vec![0.0], temperature_range: [727.0, 1493.0] }] }],
        }
    }

    pub fn compute_strength(&self, material: &Material, temp_c: f64) -> f64 {
        material.properties.youngs_modulus_gpa * (1.0 - 0.001 * temp_c)
    }

    pub fn predict_phase(&self, composition: &[f64], temp: f64) -> Option<String> {
        let _ = (composition, temp);
        Some(String::from("Predicted phase"))
    }
}

impl Default for MaterialsScience { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_strength() { let ms = MaterialsScience::new(); assert!(ms.compute_strength(&ms.materials[0], 100.0) > 0.0); } }
