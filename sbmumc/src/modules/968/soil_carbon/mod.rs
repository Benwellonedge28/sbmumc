//! # SBMUMC Module 968: Soil Carbon
//! 
//! Soil carbon sequestration and management frameworks.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SoilManagement {
    NoTill,
    CoverCrops,
    CropRotation,
    Compost,
    Biochar,
    GrazingManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilCarbonProject {
    pub project_id: String,
    pub management_practice: SoilManagement,
    pub land_type: String,
    pub area_hectares: f64,
    pub soil_depth_cm: f64,
    pub carbon_stock_tc: f64,
    pub sequestration_rate_tc_per_ha: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilCarbonPortfolio {
    pub portfolio_id: String,
    pub projects: Vec<SoilCarbonProject>,
    pub total_area: f64,
    pub total_carbon_stock: f64,
    pub annual_sequestration: f64,
}

impl SoilCarbonProject {
    pub fn new(practice: SoilManagement, land_type: &str) -> Self {
        Self {
            project_id: format!("scp_{}", uuid_simple()),
            management_practice: practice,
            land_type: land_type.to_string(),
            area_hectares: 0.0,
            soil_depth_cm: 30.0,
            carbon_stock_tc: 0.0,
            sequestration_rate_tc_per_ha: 0.0,
        }
    }

    pub fn configure(&mut self, area: f64, stock: f64, rate: f64) {
        self.area_hectares = area;
        self.carbon_stock_tc = stock;
        self.sequestration_rate_tc_per_ha = rate;
    }

    pub fn total_sequestration(&self) -> f64 {
        self.area_hectares * self.sequestration_rate_tc_per_ha
    }
}

impl SoilCarbonPortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("scport_{}", uuid_simple()),
            projects: Vec::new(),
            total_area: 0.0,
            total_carbon_stock: 0.0,
            annual_sequestration: 0.0,
        }
    }

    pub fn add_project(&mut self, project: SoilCarbonProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_area = self.projects.iter()
            .map(|p| p.area_hectares)
            .sum();
        self.total_carbon_stock = self.projects.iter()
            .map(|p| p.carbon_stock_tc)
            .sum();
        self.annual_sequestration = self.projects.iter()
            .map(|p| p.total_sequestration())
            .sum();
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
    fn test_soil_carbon_project() {
        let mut project = SoilCarbonProject::new(
            SoilManagement::Biochar,
            "Agricultural",
        );
        project.configure(10000.0, 500000.0, 5.0);
        assert!(project.total_sequestration() > 0.0);
    }

    #[test]
    fn test_soil_carbon_portfolio() {
        let mut portfolio = SoilCarbonPortfolio::new();
        portfolio.add_project(SoilCarbonProject::new(
            SoilManagement::CoverCrops,
            "Grazing land",
        ));
        assert!(portfolio.total_area >= 0.0);
    }
}
