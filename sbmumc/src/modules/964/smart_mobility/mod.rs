//! # SBMUMC Module 964: Smart Mobility
//! 
//! Intelligent transportation systems for emissions reduction.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MobilitySolution {
    ElectricVehicles,
    SharedMobility,
    AutonomousVehicles,
    PublicTransit,
    ActiveTransport,
    MobilityAsService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobilityProject {
    pub project_id: String,
    pub solution: MobilitySolution,
    pub scale: String,
    pub users_served: u32,
    pub emissions_reduction_tco2: f64,
    pub implementation_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartMobilityPlan {
    pub plan_id: String,
    pub projects: Vec<MobilityProject>,
    pub total_emissions_reduction: f64,
    pub total_users: u32,
    pub cost_effectiveness: f64,
}

impl MobilityProject {
    pub fn new(solution: MobilitySolution, scale: &str) -> Self {
        Self {
            project_id: format!("mp_{}", uuid_simple()),
            solution,
            scale: scale.to_string(),
            users_served: 0,
            emissions_reduction_tco2: 0.0,
            implementation_cost: 0.0,
        }
    }

    pub fn configure(&mut self, users: u32, reduction: f64, cost: f64) {
        self.users_served = users;
        self.emissions_reduction_tco2 = reduction;
        self.implementation_cost = cost;
    }

    pub fn cost_per_ton(&self) -> f64 {
        if self.emissions_reduction_tco2 == 0.0 { 0.0 }
        else { self.implementation_cost / self.emissions_reduction_tco2 }
    }
}

impl SmartMobilityPlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("smp_{}", uuid_simple()),
            projects: Vec::new(),
            total_emissions_reduction: 0.0,
            total_users: 0,
            cost_effectiveness: 0.0,
        }
    }

    pub fn add_project(&mut self, project: MobilityProject) {
        self.projects.push(project);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_emissions_reduction = self.projects.iter()
            .map(|p| p.emissions_reduction_tco2)
            .sum();
        self.total_users = self.projects.iter()
            .map(|p| p.users_served)
            .sum();
        let total_cost: f64 = self.projects.iter()
            .map(|p| p.implementation_cost)
            .sum();
        if self.total_emissions_reduction > 0.0 {
            self.cost_effectiveness = total_cost / self.total_emissions_reduction;
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
    fn test_mobility_project() {
        let mut project = MobilityProject::new(
            MobilitySolution::ElectricVehicles,
            "City-wide",
        );
        project.configure(50000, 25000.0, 100000000.0);
        assert!(project.emissions_reduction_tco2 > 0.0);
    }

    #[test]
    fn test_smart_mobility_plan() {
        let mut plan = SmartMobilityPlan::new();
        plan.add_project(MobilityProject::new(
            MobilitySolution::PublicTransit,
            "Metro expansion",
        ));
        assert!(plan.total_emissions_reduction >= 0.0);
    }
}
