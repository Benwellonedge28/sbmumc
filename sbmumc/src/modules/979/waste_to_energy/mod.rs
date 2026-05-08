//! # SBMUMC Module 979: Waste to Energy
//! 
//! Technologies for converting waste to energy resources.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WasteToEnergyTech {
    Incineration,
    AnaerobicDigestion,
    Gasification,
    Pyrolysis,
    LandfillGas,
    Biofuel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasteToEnergyProject {
    pub project_id: String,
    pub technology: WasteToEnergyTech,
    pub location: String,
    pub waste_processed_tons_per_year: f64,
    pub energy_output_mwh_per_year: f64,
    pub emissions_tco2_per_year: f64,
    pub net_emissions_savings: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasteToEnergyPortfolio {
    pub portfolio_id: String,
    pub projects: Vec<WasteToEnergyProject>,
    pub total_waste_processed: f64,
    pub total_energy_output: f64,
    pub total_emissions_savings: f64,
}

impl WasteToEnergyProject {
    pub fn new(technology: WasteToEnergyTech, location: &str) -> Self {
        Self {
            project_id: format!("wte_{}", uuid_simple()),
            technology,
            location: location.to_string(),
            waste_processed_tons_per_year: 0.0,
            energy_output_mwh_per_year: 0.0,
            emissions_tco2_per_year: 0.0,
            net_emissions_savings: 0.0,
        }
    }

    pub fn configure(&mut self, waste: f64, energy: f64, emissions: f64, savings: f64) {
        self.waste_processed_tons_per_year = waste;
        self.energy_output_mwh_per_year = energy;
        self.emissions_tco2_per_year = emissions;
        self.net_emissions_savings = savings;
    }

    pub fn energy_efficiency(&self) -> f64 {
        if self.waste_processed_tons_per_year == 0.0 { 0.0 }
        else { self.energy_output_mwh_per_year / self.waste_processed_tons_per_year }
    }
}

impl WasteToEnergyPortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("wteport_{}", uuid_simple()),
            projects: Vec::new(),
            total_waste_processed: 0.0,
            total_energy_output: 0.0,
            total_emissions_savings: 0.0,
        }
    }

    pub fn add_project(&mut self, project: WasteToEnergyProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_waste_processed = self.projects.iter()
            .map(|p| p.waste_processed_tons_per_year)
            .sum();
        self.total_energy_output = self.projects.iter()
            .map(|p| p.energy_output_mwh_per_year)
            .sum();
        self.total_emissions_savings = self.projects.iter()
            .map(|p| p.net_emissions_savings)
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
    fn test_waste_to_energy_project() {
        let mut project = WasteToEnergyProject::new(
            WasteToEnergyTech::AnaerobicDigestion,
            "Municipal Waste Plant",
        );
        project.configure(100000.0, 20000.0, 5000.0, 30000.0);
        assert!(project.energy_output_mwh_per_year > 0.0);
    }

    #[test]
    fn test_waste_to_energy_portfolio() {
        let mut portfolio = WasteToEnergyPortfolio::new();
        portfolio.add_project(WasteToEnergyProject::new(
            WasteToEnergyTech::Gasification,
            "Industrial Complex",
        ));
        assert!(portfolio.total_waste_processed >= 0.0);
    }
}
