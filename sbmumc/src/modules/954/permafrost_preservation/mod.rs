//! # SBMUMC Module 954: Permafrost Preservation
//! 
//! Frameworks for preserving permafrost and preventing methane release.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreservationMethod {
    SurfaceInsulation,
    Thermosyphon,
    Shading,
    DrainageModification,
    VegetationManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermafrostProject {
    pub project_id: String,
    pub location: String,
    pub method: PreservationMethod,
    pub area_km2: f64,
    pub carbon_risk_gt: f64,
    pub preservation_effectiveness: f64,
    pub implementation_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermafrostStrategy {
    pub strategy_id: String,
    pub projects: Vec<PermafrostProject>,
    pub total_carbon_protected: f64,
    pub total_cost: f64,
    pub cost_per_ton_co2: f64,
}

impl PermafrostProject {
    pub fn new(location: &str, method: PreservationMethod) -> Self {
        Self {
            project_id: format!("pp_{}", uuid_simple()),
            location: location.to_string(),
            method,
            area_km2: 0.0,
            carbon_risk_gt: 0.0,
            preservation_effectiveness: 0.0,
            implementation_cost: 0.0,
        }
    }

    pub fn configure(&mut self, area: f64, risk: f64, effectiveness: f64, cost: f64) {
        self.area_km2 = area;
        self.carbon_risk_gt = risk;
        self.preservation_effectiveness = effectiveness.clamp(0.0, 1.0);
        self.implementation_cost = cost;
    }

    pub fn carbon_protected(&self) -> f64 {
        self.carbon_risk_gt * self.preservation_effectiveness * 1000000000.0
    }
}

impl PermafrostStrategy {
    pub fn new() -> Self {
        Self {
            strategy_id: format!("ps_{}", uuid_simple()),
            projects: Vec::new(),
            total_carbon_protected: 0.0,
            total_cost: 0.0,
            cost_per_ton_co2: 0.0,
        }
    }

    pub fn add_project(&mut self, project: PermafrostProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_carbon_protected = self.projects.iter()
            .map(|p| p.carbon_protected())
            .sum();
        self.total_cost = self.projects.iter()
            .map(|p| p.implementation_cost)
            .sum();
        if self.total_carbon_protected > 0.0 {
            self.cost_per_ton_co2 = self.total_cost / self.total_carbon_protected;
        }
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
    fn test_permafrost_project() {
        let mut project = PermafrostProject::new(
            "Siberian Arctic",
            PreservationMethod::SurfaceInsulation,
        );
        project.configure(100.0, 50.0, 0.8, 100000000.0);
        assert!(project.carbon_risk_gt > 0.0);
    }

    #[test]
    fn test_permafrost_strategy() {
        let mut strategy = PermafrostStrategy::new();
        strategy.add_project(PermafrostProject::new(
            "Alaska North Slope",
            PreservationMethod::Thermosyphon,
        ));
        assert!(strategy.total_carbon_protected >= 0.0);
    }
}
