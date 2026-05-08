//! # SBMUMC Module 953: Forest Management
//! 
//! Frameworks for sustainable forest management for climate benefits.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForestManagementType {
    SustainableLogging,
    Afforestation,
    Reforestation,
    Agroforestry,
    ForestConservation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForestProject {
    pub project_id: String,
    pub management_type: ForestManagementType,
    pub forest_type: String,
    pub area_hectares: f64,
    pub carbon_stock_tc: f64,
    pub biodiversity_value: f64,
    pub implementation_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForestManagementPlan {
    pub plan_id: String,
    pub projects: Vec<ForestProject>,
    pub total_carbon_stock: f64,
    pub total_area: f64,
    pub biodiversity_score: f64,
}

impl ForestProject {
    pub fn new(mtype: ForestManagementType, forest_type: &str) -> Self {
        Self {
            project_id: format!("fp_{}", uuid_simple()),
            management_type: mtype,
            forest_type: forest_type.to_string(),
            area_hectares: 0.0,
            carbon_stock_tc: 0.0,
            biodiversity_value: 0.0,
            implementation_cost: 0.0,
        }
    }

    pub fn configure(&mut self, area: f64, carbon: f64, biodiversity: f64, cost: f64) {
        self.area_hectares = area;
        self.carbon_stock_tc = carbon;
        self.biodiversity_value = biodiversity;
        self.implementation_cost = cost;
    }

    pub fn carbon_value(&self) -> f64 {
        self.carbon_stock_tc * 50.0
    }
}

impl ForestManagementPlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("fmp_{}", uuid_simple()),
            projects: Vec::new(),
            total_carbon_stock: 0.0,
            total_area: 0.0,
            biodiversity_score: 0.0,
        }
    }

    pub fn add_project(&mut self, project: ForestProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_carbon_stock = self.projects.iter()
            .map(|p| p.carbon_stock_tc)
            .sum();
        self.total_area = self.projects.iter()
            .map(|p| p.area_hectares)
            .sum();
        self.biodiversity_score = self.projects.iter()
            .map(|p| p.biodiversity_value)
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
    fn test_forest_project() {
        let mut project = ForestProject::new(
            ForestManagementType::Reforestation,
            "Temperate Deciduous",
        );
        project.configure(5000.0, 100000.0, 0.8, 5000000.0);
        assert!(project.carbon_stock_tc > 0.0);
    }

    #[test]
    fn test_forest_management_plan() {
        let mut plan = ForestManagementPlan::new();
        plan.add_project(ForestProject::new(
            ForestManagementType::Agroforestry,
            "Tropical",
        ));
        assert!(plan.total_area >= 0.0);
    }
}
