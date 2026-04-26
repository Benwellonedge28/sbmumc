//! Plasma Physics Module
//!
//! This module implements plasma physics, plasma confinement,
//! and fusion energy for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlasmaPhysics {
    pub plasma_id: String,
    pub plasma_sources: Vec<PlasmaSource>,
    pub confinement_schemes: Vec<ConfinementScheme>,
    pub fusion_reactions: Vec<FusionReaction>,
    pub plasma_diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlasmaSource {
    pub source_id: String,
    pub source_type: PlasmaSourceType,
    pub density_m3: f64,
    pub temperature_ev: f64,
    pub plasma_beta: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlasmaSourceType {
    Tokamak,
    Stellarator,
    InertialConfinement,
    MagneticConfinement,
    InertialElectrostatic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfinementScheme {
    pub scheme_id: String,
    pub confinement_type: ConfinementType,
    pub confinement_time_s: f64,
    pub energy_gain: f64,
    pub stability_parameters: StabilityParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfinementType {
    Magnetic,
    Inertial,
    Electrostatic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityParameters {
    pub beta_limit: f64,
    pub q_safety_factor: f64,
    pub rotational_transform: f64,
    pub kink_stability: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionReaction {
    pub reaction_id: String,
    pub reaction_equation: String,
    pub fuel: Vec<String>,
    pub products: Vec<String>,
    pub energy_release_mev: f64,
    pub cross_section_mb: f64,
    pub reactivity_m3_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub diagnostic_id: String,
    pub diagnostic_type: DiagnosticType,
    pub measured_quantity: String,
    pub spatial_resolution: f64,
    pub temporal_resolution: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiagnosticType {
    ThomsonScattering,
    Interferometry,
    Bolometry,
    Spectroscopy,
    SoftXray,
}

impl PlasmaPhysics {
    pub fn new() -> Self {
        Self {
            plasma_id: String::from("plasma_physics_v1"),
            plasma_sources: vec![
                PlasmaSource {
                    source_id: String::from("source_1"),
                    source_type: PlasmaSourceType::Tokamak,
                    density_m3: 1e20,
                    temperature_ev: 15000000.0,
                    plasma_beta: 0.05,
                },
            ],
            confinement_schemes: vec![
                ConfinementScheme {
                    scheme_id: String::from("conf_1"),
                    confinement_type: ConfinementType::Magnetic,
                    confinement_time_s: 3.0,
                    energy_gain: 10.0,
                    stability_parameters: StabilityParameters {
                        beta_limit: 0.05,
                        q_safety_factor: 3.0,
                        rotational_transform: 1.5,
                        kink_stability: true,
                    },
                },
            ],
            fusion_reactions: vec![
                FusionReaction {
                    reaction_id: String::from("rxn_dt"),
                    reaction_equation: String::from("D + T -> He-4 + n"),
                    fuel: vec![String::from("Deuterium"), String::from("Tritium")],
                    products: vec![String::from("Helium-4"), String::from("Neutron")],
                    energy_release_mev: 17.6,
                    cross_section_mb: 5.0,
                    reactivity_m3_s: 1e-22,
                },
                FusionReaction {
                    reaction_id: String::from("rxn_dd"),
                    reaction_equation: String::from("D + D -> He-3 + n"),
                    fuel: vec![String::from("Deuterium"), String::from("Deuterium")],
                    products: vec![String::from("Helium-3"), String::from("Neutron")],
                    energy_release_mev: 3.27,
                    cross_section_mb: 0.1,
                    reactivity_m3_s: 1e-24,
                },
            ],
            plasma_diagnostics: vec![
                Diagnostic {
                    diagnostic_id: String::from("diag_1"),
                    diagnostic_type: DiagnosticType::ThomsonScattering,
                    measured_quantity: String::from("Electron temperature and density"),
                    spatial_resolution: 1e-3,
                    temporal_resolution: 1e-6,
                },
            ],
        }
    }

    pub fn compute_fusion_power(&self, reactivity: f64, density: f64, energy_per_reaction: f64) -> f64 {
        0.5 * reactivity * density * density * energy_per_reaction * 1e-6
    }

    pub fn calculate_ignition_temperature(&self, reaction: &FusionReaction) -> f64 {
        let k_t = 1e4;
        if reaction.reaction_id == "rxn_dt" { k_t * 5.0 } else { k_t * 20.0 }
    }

    pub fn analyze_plasma_stability(&self, scheme: &ConfinementScheme) -> StabilityAnalysis {
        StabilityAnalysis {
            scheme_name: scheme.confinement_type.clone(),
            mhd_stable: scheme.stability_parameters.kink_stability,
            beta_limit_respected: true,
            confinement_adequate: scheme.confinement_time_s > 1.0,
        }
    }

    pub fn compute_safety_factor(&self, q: f64, epsilon: f64) -> f64 {
        q * epsilon.powi(3) / (1.0 + epsilon.powi(3))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityAnalysis {
    pub scheme_name: ConfinementType,
    pub mhd_stable: bool,
    pub beta_limit_respected: bool,
    pub confinement_adequate: bool,
}

impl Default for PlasmaPhysics {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fusion_power() {
        let pp = PlasmaPhysics::new();
        let power = pp.compute_fusion_power(1e-22, 1e20, 17.6);
        assert!(power > 0.0);
    }
}
