//! # SBMUMC Module 978: Sustainable Manufacturing
//! 
//! Frameworks for sustainable manufacturing processes.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManufacturingImprovement {
    EnergyEfficiency,
    MaterialEfficiency,
    WaterConservation,
    WasteReduction,
    RenewableEnergy,
    ProcessOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SustainableManufacturingProject {
    pub project_id: String,
    pub improvement: ManufacturingImprovement,
    pub facility_name: String,
    pub implementation_cost: f64,
    pub annual_savings: f64,
    pub emissions_reduction_tco2: f64,
    pub payback_period_years: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturingSustainabilityPlan {
    pub plan_id: String,
    pub projects: Vec<SustainableManufacturingProject>,
    pub total_investment: f64,
    pub total_annual_savings: f64,
    pub total_emissions_reduction: f64,
}

impl SustainableManufacturingProject {
    pub fn new(improvement: ManufacturingImprovement, facility: &str) -> Self {
        Self {
            project_id: format!("smp_{}", uuid_simple()),
            improvement,
            facility_name: facility.to_string(),
            implementation_cost: 0.0,
            annual_savings: 0.0,
            emissions_reduction_tco2: 0.0,
            payback_period_years: 0.0,
        }
    }

    pub fn configure(&mut self, cost: f64, savings: f64, reduction: f64) {
        self.implementation_cost = cost;
        self.annual_savings = savings;
        self.emissions_reduction_tco2 = reduction;
        if self.annual_savings > 0.0 {
            self.payback_period_years = self.implementation_cost / self.annual_savings;
        }
    }
}

impl ManufacturingSustainabilityPlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("msp_{}", uuid_simple()),
            projects: Vec::new(),
            total_investment: 0.0,
            total_annual_savings: 0.0,
            total_emissions_reduction: 0.0,
        }
    }

    pub fn add_project(&mut self, project: SustainableManufacturingProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_investment = self.projects.iter()
            .map(|p| p.implementation_cost)
            .sum();
        self.total_annual_savings = self.projects.iter()
            .map(|p| p.annual_savings)
            .sum();
        self.total_emissions_reduction = self.projects.iter()
            .map(|p| p.emissions_reduction_tco2)
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
    fn test_sustainable_manufacturing_project() {
        let mut project = SustainableManufacturingProject::new(
            ManufacturingImprovement::EnergyEfficiency,
            "Auto Plant A",
        );
        project.configure(5000000.0, 800000.0, 2000.0);
        assert!(project.payback_period_years < 10.0);
    }

    #[test]
    fn test_manufacturing_sustainability_plan() {
        let mut plan = ManufacturingSustainabilityPlan::new();
        plan.add_project(SustainableManufacturingProject::new(
            ManufacturingImprovement::RenewableEnergy,
            "Electronics Factory",
        ));
        assert!(plan.total_investment >= 0.0);
    }
}
