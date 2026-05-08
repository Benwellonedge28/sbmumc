//! # SBMUMC Module 963: Grid Storage Tech
//! 
//! Energy storage technologies for grid stability and renewable integration.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageTechnology {
    LithiumIon,
    Flow,
    CompressedAir,
    PumpedHydro,
    Hydrogen,
    SolidState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProject {
    pub project_id: String,
    pub technology: StorageTechnology,
    pub capacity_mwh: f64,
    pub power_rating_mw: f64,
    pub efficiency: f64,
    pub discharge_hours: f64,
    pub lifecycle_cycles: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePortfolio {
    pub portfolio_id: String,
    pub projects: Vec<StorageProject>,
    pub total_capacity_mwh: f64,
    pub total_power_mw: f64,
    pub weighted_efficiency: f64,
}

impl StorageProject {
    pub fn new(technology: StorageTechnology, capacity: f64, power: f64) -> Self {
        Self {
            project_id: format!("sp_{}", uuid_simple()),
            technology,
            capacity_mwh: capacity,
            power_rating_mw: power,
            efficiency: 0.0,
            discharge_hours: 0.0,
            lifecycle_cycles: 0,
        }
    }

    pub fn configure(&mut self, efficiency: f64, cycles: u32) {
        self.efficiency = efficiency.clamp(0.0, 1.0);
        self.lifecycle_cycles = cycles;
        self.discharge_hours = self.capacity_mwh / self.power_rating_mw;
    }

    pub fn lifetime_energy(&self) -> f64 {
        self.capacity_mwh * self.lifecycle_cycles as f64 * self.efficiency
    }
}

impl StoragePortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("stport_{}", uuid_simple()),
            projects: Vec::new(),
            total_capacity_mwh: 0.0,
            total_power_mw: 0.0,
            weighted_efficiency: 0.0,
        }
    }

    pub fn add_project(&mut self, project: StorageProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_capacity_mwh = self.projects.iter()
            .map(|p| p.capacity_mwh)
            .sum();
        self.total_power_mw = self.projects.iter()
            .map(|p| p.power_rating_mw)
            .sum();
        let total_energy: f64 = self.projects.iter()
            .map(|p| p.capacity_mwh * p.efficiency)
            .sum();
        self.weighted_efficiency = if self.total_capacity_mwh > 0.0 {
            total_energy / self.total_capacity_mwh
        } else { 0.0 };
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
    fn test_storage_project() {
        let mut project = StorageProject::new(StorageTechnology::LithiumIon, 100.0, 50.0);
        project.configure(0.9, 5000);
        assert!(project.discharge_hours > 0.0);
    }

    #[test]
    fn test_storage_portfolio() {
        let mut portfolio = StoragePortfolio::new();
        portfolio.add_project(StorageProject::new(StorageTechnology::Flow, 200.0, 100.0));
        assert!(portfolio.total_capacity_mwh > 0.0);
    }
}
