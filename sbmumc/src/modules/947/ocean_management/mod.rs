//! # SBMUMC Module 947: Ocean Management
//! 
//! Frameworks for ocean-based climate interventions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OceanIntervention {
    OceanFertilization,
    AlkalinityEnhancement,
    Upwelling,
    BlueCarbon,
    ArtificialUpwelling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OceanProject {
    pub project_id: String,
    pub intervention: OceanIntervention,
    pub location: String,
    pub scale: String,
    pub carbon_removal_potential: f64,
    pub ecosystem_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OceanManagementPlan {
    pub plan_id: String,
    pub projects: Vec<OceanProject>,
    pub total_carbon_removal: f64,
    pub ecosystem_benefits: f64,
}

impl OceanProject {
    pub fn new(intervention: OceanIntervention, location: &str) -> Self {
        Self {
            project_id: format!("op_{}", uuid_simple()),
            intervention,
            location: location.to_string(),
            scale: "local".to_string(),
            carbon_removal_potential: 0.0,
            ecosystem_impact: 0.0,
        }
    }

    pub fn set_potential(&mut self, potential: f64) {
        self.carbon_removal_potential = potential;
    }

    pub fn assess_ecosystem(&mut self, impact: f64) {
        self.ecosystem_impact = impact.clamp(-1.0, 1.0);
    }
}

impl OceanManagementPlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("omp_{}", uuid_simple()),
            projects: Vec::new(),
            total_carbon_removal: 0.0,
            ecosystem_benefits: 0.0,
        }
    }

    pub fn add_project(&mut self, project: OceanProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_carbon_removal = self.projects.iter()
            .map(|p| p.carbon_removal_potential)
            .sum();
        self.ecosystem_benefits = self.projects.iter()
            .map(|p| p.ecosystem_impact)
            .sum::<f64>() / self.projects.len().max(1) as f64;
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{:x}{:x}", duration.as_secs(), duration.subsec_nanos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocean_project() {
        let mut project = OceanProject::new(
            OceanIntervention::BlueCarbon,
            "Pacific Mangroves",
        );
        project.set_potential(500000.0);
        project.assess_ecosystem(0.7);
        assert!(project.carbon_removal_potential > 0.0);
    }

    #[test]
    fn test_ocean_management_plan() {
        let mut plan = OceanManagementPlan::new();
        plan.add_project(OceanProject::new(
            OceanIntervention::OceanFertilization,
            "Southern Ocean",
        ));
        assert!(plan.total_carbon_removal >= 0.0);
    }
}
