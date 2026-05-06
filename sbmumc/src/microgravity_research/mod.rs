//! Microgravity Research Module (662)
//!
//! Microgravity science, research facilities, and experimental protocols.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MicrogravityLevel {
    Microgravity,    // ~10^-6 g
    LowGravity,      // ~0.1 g
    MarsGravity,     // 0.38 g
    MoonGravity,     // 0.16 g
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrogravityResearch {
    pub research_name: String,
    pub facility: String,
    pub microgravity_level: MicrogravityLevel,
    pub research_area: String,
    pub duration_days: f64,
    pub sample_size: u32,
    pub controls_count: u32,
    pub publication_status: String,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidPhysicsExperiment {
    pub experiment_id: String,
    pub fluid_type: String,
    pub container_geometry: String,
    pub temperature_range: (f64, f64),
    pub observation_methods: Vec<String>,
    pub data_points: u32,
    pub results_summary: String,
}

impl MicrogravityResearch {
    pub fn new(research_name: String, facility: String) -> Self {
        Self {
            research_name,
            facility,
            microgravity_level: MicrogravityLevel::Microgravity,
            research_area: "Undetermined".into(),
            duration_days: 0.0,
            sample_size: 0,
            controls_count: 0,
            publication_status: "In Progress".into(),
            findings: Vec::new(),
        }
    }

    pub fn add_finding(&mut self, finding: String) {
        self.findings.push(finding);
    }

    pub fn is_publishable(&self) -> bool {
        self.findings.len() >= 5 && self.sample_size >= 30
    }
}

impl FluidPhysicsExperiment {
    pub fn new(experiment_id: String) -> Self {
        Self {
            experiment_id,
            fluid_type: "Unknown".into(),
            container_geometry: "Cylindrical".into(),
            temperature_range: (0.0, 100.0),
            observation_methods: Vec::new(),
            data_points: 0,
            results_summary: "TBD".into(),
        }
    }

    pub fn calculate_surface_tension_effect(&self) -> f64 {
        1.0 - (self.container_geometry.len() as f64 / 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microgravity_research() {
        let research = MicrogravityResearch::new("Bone Loss Study".into(), "ISS".into());
        assert_eq!(research.research_name, "Bone Loss Study");
    }
}
