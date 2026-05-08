//! # SBMUMC Module 955: Methane Reduction
//! 
//! Frameworks for reducing methane emissions across sectors.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MethaneSource {
    Agriculture,
    FossilFuels,
    Landfills,
    Wetlands,
    Livestock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethaneReductionProject {
    pub project_id: String,
    pub source: MethaneSource,
    pub technology: String,
    pub reduction_tg: f64,
    pub cost_per_tg: f64,
    pub co2_equivalent_reduction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethaneReductionPlan {
    pub plan_id: String,
    pub projects: Vec<MethaneReductionProject>,
    pub total_reduction: f64,
    pub total_co2_equivalent: f64,
    pub cost_effectiveness: f64,
}

impl MethaneReductionProject {
    pub fn new(source: MethaneSource, technology: &str) -> Self {
        Self {
            project_id: format!("mrp_{}", uuid_simple()),
            source,
            technology: technology.to_string(),
            reduction_tg: 0.0,
            cost_per_tg: 0.0,
            co2_equivalent_reduction: 0.0,
        }
    }

    pub fn configure(&mut self, reduction: f64, cost: f64) {
        self.reduction_tg = reduction;
        self.cost_per_tg = cost;
        self.co2_equivalent_reduction = reduction * 80.0;
    }

    pub fn cost_effectiveness(&self) -> f64 {
        if self.cost_per_tg == 0.0 { 0.0 }
        else { 1000.0 / self.cost_per_tg }
    }
}

impl MethaneReductionPlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("mrplan_{}", uuid_simple()),
            projects: Vec::new(),
            total_reduction: 0.0,
            total_co2_equivalent: 0.0,
            cost_effectiveness: 0.0,
        }
    }

    pub fn add_project(&mut self, project: MethaneReductionProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_reduction = self.projects.iter()
            .map(|p| p.reduction_tg)
            .sum();
        self.total_co2_equivalent = self.projects.iter()
            .map(|p| p.co2_equivalent_reduction)
            .sum();
        let total_cost: f64 = self.projects.iter()
            .map(|p| p.cost_per_tg * p.reduction_tg)
            .sum();
        if self.total_co2_equivalent > 0.0 {
            self.cost_effectiveness = total_cost / self.total_co2_equivalent;
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
    fn test_methane_reduction_project() {
        let mut project = MethaneReductionProject::new(
            MethaneSource::Agriculture,
            "Anaerobic digesters",
        );
        project.configure(1.5, 50000000.0);
        assert!(project.co2_equivalent_reduction > 0.0);
    }

    #[test]
    fn test_methane_reduction_plan() {
        let mut plan = MethaneReductionPlan::new();
        plan.add_project(MethaneReductionProject::new(
            MethaneSource::Landfills,
            "Landfill gas capture",
        ));
        assert!(plan.total_reduction >= 0.0);
    }
}
