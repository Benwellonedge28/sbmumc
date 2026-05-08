//! # SBMUMC Module 957: Wetland Restoration
//! 
//! Frameworks for restoring wetland ecosystems for climate benefits.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WetlandType {
    Marsh,
    Swamp,
    Bog,
    Fen,
    Mangrove,
    Estuary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WetlandRestorationProject {
    pub project_id: String,
    pub wetland_type: WetlandType,
    pub location: String,
    pub area_hectares: f64,
    pub carbon_sequestration_rate: f64,
    pub water_quality_improvement: f64,
    pub biodiversity_index: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WetlandRestorationPlan {
    pub plan_id: String,
    pub projects: Vec<WetlandRestorationProject>,
    pub total_area: f64,
    pub total_carbon_sequestration: f64,
    pub ecosystem_services_value: f64,
}

impl WetlandRestorationProject {
    pub fn new(wtype: WetlandType, location: &str) -> Self {
        Self {
            project_id: format!("wrp_{}", uuid_simple()),
            wetland_type: wtype,
            location: location.to_string(),
            area_hectares: 0.0,
            carbon_sequestration_rate: 0.0,
            water_quality_improvement: 0.0,
            biodiversity_index: 0.0,
        }
    }

    pub fn configure(&mut self, area: f64, carbon: f64, water: f64, biodiversity: f64) {
        self.area_hectares = area;
        self.carbon_sequestration_rate = carbon;
        self.water_quality_improvement = water;
        self.biodiversity_index = biodiversity;
    }

    pub fn annual_carbon_sequestration(&self) -> f64 {
        self.area_hectares * self.carbon_sequestration_rate
    }
}

impl WetlandRestorationPlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("wrplan_{}", uuid_simple()),
            projects: Vec::new(),
            total_area: 0.0,
            total_carbon_sequestration: 0.0,
            ecosystem_services_value: 0.0,
        }
    }

    pub fn add_project(&mut self, project: WetlandRestorationProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_area = self.projects.iter()
            .map(|p| p.area_hectares)
            .sum();
        self.total_carbon_sequestration = self.projects.iter()
            .map(|p| p.annual_carbon_sequestration())
            .sum();
        self.ecosystem_services_value = self.projects.iter()
            .map(|p| p.water_quality_improvement + p.biodiversity_index)
            .sum::<f64>() * 10000.0;
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
    fn test_wetland_restoration_project() {
        let mut project = WetlandRestorationProject::new(
            WetlandType::Mangrove,
            "Southeast Asia",
        );
        project.configure(5000.0, 15.0, 0.8, 0.9);
        assert!(project.annual_carbon_sequestration() > 0.0);
    }

    #[test]
    fn test_wetland_restoration_plan() {
        let mut plan = WetlandRestorationPlan::new();
        plan.add_project(WetlandRestorationProject::new(
            WetlandType::Bog,
            "Northern Europe",
        ));
        assert!(plan.total_area >= 0.0);
    }
}
