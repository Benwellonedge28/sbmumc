//! # SBMUMC Module 962: Renewable Integration
//! 
//! Frameworks for integrating renewable energy into existing systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RenewableSource {
    Solar,
    Wind,
    Hydro,
    Geothermal,
    Biomass,
    Tidal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenewableProject {
    pub project_id: String,
    pub source: RenewableSource,
    pub capacity_mw: f64,
    pub capacity_factor: f64,
    pub annual_generation_mwh: f64,
    pub avoided_emissions_tco2: f64,
    pub integration_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenewablePortfolio {
    pub portfolio_id: String,
    pub projects: Vec<RenewableProject>,
    pub total_capacity_mw: f64,
    pub total_generation_mwh: f64,
    pub total_avoided_emissions: f64,
}

impl RenewableProject {
    pub fn new(source: RenewableSource, capacity: f64) -> Self {
        Self {
            project_id: format!("rp_{}", uuid_simple()),
            source,
            capacity_mw: capacity,
            capacity_factor: 0.0,
            annual_generation_mwh: 0.0,
            avoided_emissions_tco2: 0.0,
            integration_cost: 0.0,
        }
    }

    pub fn configure(&mut self, factor: f64, cost: f64) {
        self.capacity_factor = factor.clamp(0.0, 1.0);
        self.integration_cost = cost;
        self.annual_generation_mwh = self.capacity_mw * self.capacity_factor * 8760.0;
        self.avoided_emissions_tco2 = self.annual_generation_mwh * 0.4;
    }
}

impl RenewablePortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("ren_port_{}", uuid_simple()),
            projects: Vec::new(),
            total_capacity_mw: 0.0,
            total_generation_mwh: 0.0,
            total_avoided_emissions: 0.0,
        }
    }

    pub fn add_project(&mut self, project: RenewableProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_capacity_mw = self.projects.iter()
            .map(|p| p.capacity_mw)
            .sum();
        self.total_generation_mwh = self.projects.iter()
            .map(|p| p.annual_generation_mwh)
            .sum();
        self.total_avoided_emissions = self.projects.iter()
            .map(|p| p.avoided_emissions_tco2)
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
    fn test_renewable_project() {
        let mut project = RenewableProject::new(RenewableSource::Solar, 100.0);
        project.configure(0.25, 50000000.0);
        assert!(project.annual_generation_mwh > 0.0);
    }

    #[test]
    fn test_renewable_portfolio() {
        let mut portfolio = RenewablePortfolio::new();
        portfolio.add_project(RenewableProject::new(RenewableSource::Wind, 200.0));
        assert!(portfolio.total_capacity_mw > 0.0);
    }
}
