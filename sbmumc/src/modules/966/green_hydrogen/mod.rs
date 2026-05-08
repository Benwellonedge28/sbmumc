//! # SBMUMC Module 966: Green Hydrogen
//! 
//! Green hydrogen production and infrastructure frameworks.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HydrogenProduction {
    Electrolysis,
    Renewable,
    Nuclear,
    Biomass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreenHydrogenProject {
    pub project_id: String,
    pub production_method: HydrogenProduction,
    pub capacity_tons_per_day: f64,
    pub energy_source: String,
    pub production_cost_per_kg: f64,
    pub lifecycle_emissions_tco2_per_kg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreenHydrogenPortfolio {
    pub portfolio_id: String,
    pub projects: Vec<GreenHydrogenProject>,
    pub total_capacity: f64,
    pub average_cost: f64,
    pub emissions_intensity: f64,
}

impl GreenHydrogenProject {
    pub fn new(method: HydrogenProduction, source: &str) -> Self {
        Self {
            project_id: format!("ghp_{}", uuid_simple()),
            production_method: method,
            capacity_tons_per_day: 0.0,
            energy_source: source.to_string(),
            production_cost_per_kg: 0.0,
            lifecycle_emissions_tco2_per_kg: 0.0,
        }
    }

    pub fn configure(&mut self, capacity: f64, cost: f64, emissions: f64) {
        self.capacity_tons_per_day = capacity;
        self.production_cost_per_kg = cost;
        self.lifecycle_emissions_tco2_per_kg = emissions;
    }

    pub fn annual_production(&self) -> f64 {
        self.capacity_tons_per_day * 365.0
    }
}

impl GreenHydrogenPortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("ghport_{}", uuid_simple()),
            projects: Vec::new(),
            total_capacity: 0.0,
            average_cost: 0.0,
            emissions_intensity: 0.0,
        }
    }

    pub fn add_project(&mut self, project: GreenHydrogenProject) {
        self.projects.push(project);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_capacity = self.projects.iter()
            .map(|p| p.capacity_tons_per_day)
            .sum();
        if !self.projects.is_empty() {
            self.average_cost = self.projects.iter()
                .map(|p| p.production_cost_per_kg)
                .sum::<f64>() / self.projects.len() as f64;
            let total_emissions: f64 = self.projects.iter()
                .map(|p| p.lifecycle_emissions_tco2_per_kg)
                .sum();
            self.emissions_intensity = total_emissions / self.projects.len() as f64;
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
    fn test_green_hydrogen_project() {
        let mut project = GreenHydrogenProject::new(
            HydrogenProduction::Electrolysis,
            "Solar",
        );
        project.configure(100.0, 4.5, 0.5);
        assert!(project.annual_production() > 0.0);
    }

    #[test]
    fn test_green_hydrogen_portfolio() {
        let mut portfolio = GreenHydrogenPortfolio::new();
        portfolio.add_project(GreenHydrogenProject::new(
            HydrogenProduction::Renewable,
            "Wind",
        ));
        assert!(portfolio.total_capacity >= 0.0);
    }
}
