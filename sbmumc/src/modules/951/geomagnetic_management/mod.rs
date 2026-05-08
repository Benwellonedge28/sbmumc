//! # SBMUMC Module 951: Geomagnetic Management
//! 
//! Frameworks for managing Earth's magnetic field for climate purposes.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeomagneticIntervention {
    FieldStrength,
    FieldStability,
    FieldDirection,
    Shielding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeomagneticProject {
    pub project_id: String,
    pub intervention: GeomagneticIntervention,
    pub objective: String,
    pub technical_feasibility: f64,
    pub climate_impact_potential: f64,
    pub risk_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeomagneticAssessment {
    pub assessment_id: String,
    pub projects: Vec<GeomagneticProject>,
    pub overall_feasibility: f64,
    pub total_impact_potential: f64,
}

impl GeomagneticProject {
    pub fn new(intervention: GeomagneticIntervention, objective: &str) -> Self {
        Self {
            project_id: format!("gp_{}", uuid_simple()),
            intervention,
            objective: objective.to_string(),
            technical_feasibility: 0.0,
            climate_impact_potential: 0.0,
            risk_level: 0.0,
        }
    }

    pub fn assess(&mut self, feasibility: f64, impact: f64, risk: f64) {
        self.technical_feasibility = feasibility.clamp(0.0, 1.0);
        self.climate_impact_potential = impact.clamp(0.0, 1.0);
        self.risk_level = risk.clamp(0.0, 1.0);
    }
}

impl GeomagneticAssessment {
    pub fn new() -> Self {
        Self {
            assessment_id: format!("ga_{}", uuid_simple()),
            projects: Vec::new(),
            overall_feasibility: 0.0,
            total_impact_potential: 0.0,
        }
    }

    pub fn add_project(&mut self, project: GeomagneticProject) {
        self.projects.push(project);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        if self.projects.is_empty() {
            return;
        }
        self.overall_feasibility = self.projects.iter()
            .map(|p| p.technical_feasibility)
            .sum::<f64>() / self.projects.len() as f64;
        self.total_impact_potential = self.projects.iter()
            .map(|p| p.climate_impact_potential)
            .sum::<f64>() / self.projects.len() as f64;
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
    fn test_geomagnetic_project() {
        let mut project = GeomagneticProject::new(
            GeomagneticIntervention::FieldStrength,
            "Increase cosmic ray deflection",
        );
        project.assess(0.3, 0.5, 0.8);
        assert!(project.risk_level > project.technical_feasibility);
    }

    #[test]
    fn test_geomagnetic_assessment() {
        let mut assessment = GeomagneticAssessment::new();
        assessment.add_project(GeomagneticProject::new(
            GeomagneticIntervention::Shielding,
            "Enhanced magnetosphere",
        ));
        assert!(assessment.projects.len() == 1);
    }
}
