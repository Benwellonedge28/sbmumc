//! # SBMUMC Module 943: Climate Intervention
//! 
//! Frameworks for large-scale climate intervention technologies.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterventionType {
    CarbonRemoval,
    SolarRadiationManagement,
    WeatherModification,
    Atmospheric,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionProject {
    pub project_id: String,
    pub intervention_type: InterventionType,
    pub name: String,
    pub scale: String,
    pub effectiveness: f64,
    pub risk_profile: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionPlan {
    pub plan_id: String,
    pub interventions: Vec<InterventionProject>,
    pub overall_impact: f64,
    pub timeline_years: u32,
}

impl InterventionProject {
    pub fn new(name: &str, itype: InterventionType) -> Self {
        Self {
            project_id: format!("ip_{}", uuid_simple()),
            intervention_type: itype,
            name: name.to_string(),
            scale: "regional".to_string(),
            effectiveness: 0.0,
            risk_profile: 0.0,
        }
    }

    pub fn set_effectiveness(&mut self, effectiveness: f64) {
        self.effectiveness = effectiveness.clamp(0.0, 1.0);
    }

    pub fn set_risk(&mut self, risk: f64) {
        self.risk_profile = risk.clamp(0.0, 1.0);
    }
}

impl InterventionPlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("plan_{}", uuid_simple()),
            interventions: Vec::new(),
            overall_impact: 0.0,
            timeline_years: 0,
        }
    }

    pub fn add_intervention(&mut self, project: InterventionProject) {
        self.interventions.push(project);
        self.compute_impact();
    }

    pub fn compute_impact(&mut self) {
        if self.interventions.is_empty() {
            self.overall_impact = 0.0;
            return;
        }
        let total: f64 = self.interventions.iter()
            .map(|i| i.effectiveness * (1.0 - i.risk_profile))
            .sum();
        self.overall_impact = total / self.interventions.len() as f64;
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
    fn test_intervention_project() {
        let mut project = InterventionProject::new(
            "Marine Cloud Brightening",
            InterventionType::SolarRadiationManagement,
        );
        project.set_effectiveness(0.75);
        project.set_risk(0.2);
        assert!(project.effectiveness > 0.7);
    }

    #[test]
    fn test_intervention_plan() {
        let mut plan = InterventionPlan::new();
        plan.add_intervention(InterventionProject::new(
            "Direct Air Capture",
            InterventionType::CarbonRemoval,
        ));
        assert!(plan.interventions.len() == 1);
    }
}
