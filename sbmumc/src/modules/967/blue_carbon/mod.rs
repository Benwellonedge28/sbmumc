//! # SBMUMC Module 967: Blue Carbon
//! 
//! Blue carbon ecosystems and carbon sequestration frameworks.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlueCarbonEcosystem {
    Mangroves,
    Seagrass,
    SaltMarshes,
    Seaweeds,
    TidalMarshes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueCarbonProject {
    pub project_id: String,
    pub ecosystem: BlueCarbonEcosystem,
    pub location: String,
    pub area_hectares: f64,
    pub carbon_sequestration_rate: f64,
    pub blue_carbon_stock_tc: f64,
    pub coastal_protection_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueCarbonPortfolio {
    pub portfolio_id: String,
    pub projects: Vec<BlueCarbonProject>,
    pub total_area: f64,
    pub total_carbon_stock: f64,
    pub total_protection_value: f64,
}

impl BlueCarbonProject {
    pub fn new(ecosystem: BlueCarbonEcosystem, location: &str) -> Self {
        Self {
            project_id: format!("bcp_{}", uuid_simple()),
            ecosystem,
            location: location.to_string(),
            area_hectares: 0.0,
            carbon_sequestration_rate: 0.0,
            blue_carbon_stock_tc: 0.0,
            coastal_protection_value: 0.0,
        }
    }

    pub fn configure(&mut self, area: f64, rate: f64, stock: f64, protection: f64) {
        self.area_hectares = area;
        self.carbon_sequestration_rate = rate;
        self.blue_carbon_stock_tc = stock;
        self.coastal_protection_value = protection;
    }

    pub fn annual_sequestration(&self) -> f64 {
        self.area_hectares * self.carbon_sequestration_rate
    }
}

impl BlueCarbonPortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("bcport_{}", uuid_simple()),
            projects: Vec::new(),
            total_area: 0.0,
            total_carbon_stock: 0.0,
            total_protection_value: 0.0,
        }
    }

    pub fn add_project(&mut self, project: BlueCarbonProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_area = self.projects.iter()
            .map(|p| p.area_hectares)
            .sum();
        self.total_carbon_stock = self.projects.iter()
            .map(|p| p.blue_carbon_stock_tc)
            .sum();
        self.total_protection_value = self.projects.iter()
            .map(|p| p.coastal_protection_value)
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
    fn test_blue_carbon_project() {
        let mut project = BlueCarbonProject::new(
            BlueCarbonEcosystem::Mangroves,
            "Indonesia",
        );
        project.configure(5000.0, 8.0, 200000.0, 50000000.0);
        assert!(project.blue_carbon_stock_tc > 0.0);
    }

    #[test]
    fn test_blue_carbon_portfolio() {
        let mut portfolio = BlueCarbonPortfolio::new();
        portfolio.add_project(BlueCarbonProject::new(
            BlueCarbonEcosystem::Seagrass,
            "Mediterranean",
        ));
        assert!(portfolio.total_area >= 0.0);
    }
}
