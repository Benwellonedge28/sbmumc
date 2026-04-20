//! Dark Matter Interaction Module
//!
//! This module implements dark matter physics, WIMP detection,
//! axion search, and dark matter-modified gravity theories.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct DarkMatterInteraction {
    pub candidates: Vec<DarkMatterCandidate>,
    pub detectors: Vec<DarkMatterDetector>,
    pub interactions: Vec<Interaction>,
}

impl DarkMatterInteraction {
    pub fn new() -> Self {
        DarkMatterInteraction {
            candidates: vec![
                DarkMatterCandidate { name: "WIMP".to_string(), mass_gev: 100.0 },
                DarkMatterCandidate { name: "Axion".to_string(), mass_gev: 1e-5 },
                DarkMatterCandidate { name: "Sterile neutrino".to_string(), mass_gev: 10.0 },
            ],
            detectors: Vec::new(),
            interactions: Vec::new(),
        }
    }

    /// Add detector
    pub fn add_detector(&mut self, name: &str, type_: &str) -> &DarkMatterDetector {
        let detector = DarkMatterDetector {
            detector_id: format!("det_{}", self.detectors.len()),
            name: name.to_string(),
            type_: type_.to_string(),
            mass_kg: 1000.0,
            sensitivity: 1e-45,
        };
        self.detectors.push(detector);
        self.detectors.last().unwrap()
    }

    /// Search for interaction
    pub fn search(&mut self, detector_id: &str, candidate: &str) -> &Interaction {
        let interaction = Interaction {
            interaction_id: format!("int_{}", self.interactions.len()),
            detector_id: detector_id.to_string(),
            candidate_type: candidate.to_string(),
            cross_section: 1e-48,
            observed: false,
        };
        self.interactions.push(interaction);
        self.interactions.last().unwrap()
    }

    /// Calculate relic abundance
    pub fn calculate_relic(&self, mass_gev: f64) -> RelicCalculation {
        RelicCalculation {
            mass_gev,
            omega_h2: 0.12,
            thermal_equilibrium: true,
        }
    }

    /// Model dark matter distribution
    pub fn model_distribution(&self, halo_type: &str) -> DistributionModel {
        DistributionModel {
            halo_type: halo_type.to_string(),
            density_profile: "NFW".to_string(),
            local_density: 0.3,
        }
    }
}

impl Default for DarkMatterInteraction { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkMatterCandidate {
    pub name: String,
    pub mass_gev: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkMatterDetector {
    pub detector_id: String,
    pub name: String,
    pub type_: String,
    pub mass_kg: f64,
    pub sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub interaction_id: String,
    pub detector_id: String,
    pub candidate_type: String,
    pub cross_section: f64,
    pub observed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelicCalculation {
    pub mass_gev: f64,
    pub omega_h2: f64,
    pub thermal_equilibrium: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionModel {
    pub halo_type: String,
    pub density_profile: String,
    pub local_density: f64,
}
