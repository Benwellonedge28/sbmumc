//! # SBMUMC Module 948: Ecosystem Restoration
//! 
//! Frameworks for restoring ecosystems for climate benefits.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EcosystemType {
    Forest,
    Wetland,
    Grassland,
    Mangrove,
    CoralReef,
    Seagrass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorationProject {
    pub project_id: String,
    pub ecosystem: EcosystemType,
    pub location: String,
    pub area_hectares: f64,
    pub carbon_sequestration_rate: f64,
    pub biodiversity_score: f64,
    pub timeline_years: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorationPortfolio {
    pub portfolio_id: String,
    pub projects: Vec<RestorationProject>,
    pub total_area: f64,
    pub total_carbon_potential: f64,
    pub biodiversity_benefits: f64,
}

impl RestorationProject {
    pub fn new(ecosystem: EcosystemType, location: &str) -> Self {
        Self {
            project_id: format!("rp_{}", uuid_simple()),
            ecosystem,
            location: location.to_string(),
            area_hectares: 0.0,
            carbon_sequestration_rate: 0.0,
            biodiversity_score: 0.0,
            timeline_years: 0,
        }
    }

    pub fn configure(&mut self, area: f64, rate: f64, years: u32) {
        self.area_hectares = area;
        self.carbon_sequestration_rate = rate;
        self.timeline_years = years;
    }

    pub fn set_biodiversity(&mut self, score: f64) {
        self.biodiversity_score = score.clamp(0.0, 1.0);
    }

    pub fn carbon_potential(&self) -> f64 {
        self.area_hectares * self.carbon_sequestration_rate * self.timeline_years as f64
    }
}

impl RestorationPortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("rport_{}", uuid_simple()),
            projects: Vec::new(),
            total_area: 0.0,
            total_carbon_potential: 0.0,
            biodiversity_benefits: 0.0,
        }
    }

    pub fn add_project(&mut self, project: RestorationProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_area = self.projects.iter()
            .map(|p| p.area_hectares)
            .sum();
        self.total_carbon_potential = self.projects.iter()
            .map(|p| p.carbon_potential())
            .sum();
        self.biodiversity_benefits = self.projects.iter()
            .map(|p| p.biodiversity_score)
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
    fn test_restoration_project() {
        let mut project = RestorationProject::new(
            EcosystemType::Forest,
            "Amazon Basin",
        );
        project.configure(10000.0, 10.0, 30);
        assert!(project.carbon_potential() > 0.0);
    }

    #[test]
    fn test_restoration_portfolio() {
        let mut portfolio = RestorationPortfolio::new();
        portfolio.add_project(RestorationProject::new(
            EcosystemType::Mangrove,
            "Southeast Asia",
        ));
        assert!(portfolio.total_area >= 0.0);
    }
}
