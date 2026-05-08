//! # SBMUMC Module 952: Ice Core Stabilization
//! 
//! Frameworks for stabilizing ice sheets and glaciers.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StabilizationMethod {
    SnowFencing,
    ArtificialSnowmaking,
    IceSurfaceCoating,
    SubglacialDrainage,
    SurfaceAlbedo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceStabilizationProject {
    pub project_id: String,
    pub location: String,
    pub method: StabilizationMethod,
    pub area_km2: f64,
    pub ice_mass_preserved_gt: f64,
    pub cost_per_gt: f64,
    pub sea_level_impact_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceStabilizationPortfolio {
    pub portfolio_id: String,
    pub projects: Vec<IceStabilizationProject>,
    pub total_ice_preserved: f64,
    pub total_sea_level_protection: f64,
}

impl IceStabilizationProject {
    pub fn new(location: &str, method: StabilizationMethod) -> Self {
        Self {
            project_id: format!("isp_{}", uuid_simple()),
            location: location.to_string(),
            method,
            area_km2: 0.0,
            ice_mass_preserved_gt: 0.0,
            cost_per_gt: 0.0,
            sea_level_impact_mm: 0.0,
        }
    }

    pub fn configure(&mut self, area: f64, mass: f64, cost: f64, sea_impact: f64) {
        self.area_km2 = area;
        self.ice_mass_preserved_gt = mass;
        self.cost_per_gt = cost;
        self.sea_level_impact_mm = sea_impact;
    }

    pub fn cost_effectiveness(&self) -> f64 {
        if self.cost_per_gt == 0.0 { 0.0 }
        else { 1.0 / (self.cost_per_gt / 1000000000.0) }
    }
}

impl IceStabilizationPortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("isport_{}", uuid_simple()),
            projects: Vec::new(),
            total_ice_preserved: 0.0,
            total_sea_level_protection: 0.0,
        }
    }

    pub fn add_project(&mut self, project: IceStabilizationProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_ice_preserved = self.projects.iter()
            .map(|p| p.ice_mass_preserved_gt)
            .sum();
        self.total_sea_level_protection = self.projects.iter()
            .map(|p| p.sea_level_impact_mm)
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
    fn test_ice_stabilization_project() {
        let mut project = IceStabilizationProject::new(
            "Greenland Ice Sheet",
            StabilizationMethod::SnowFencing,
        );
        project.configure(1000.0, 50.0, 2000000000.0, 0.05);
        assert!(project.ice_mass_preserved_gt > 0.0);
    }

    #[test]
    fn test_ice_stabilization_portfolio() {
        let mut portfolio = IceStabilizationPortfolio::new();
        portfolio.add_project(IceStabilizationProject::new(
            "Antarctic Ice Sheet",
            StabilizationMethod::ArtificialSnowmaking,
        ));
        assert!(portfolio.total_ice_preserved >= 0.0);
    }
}
