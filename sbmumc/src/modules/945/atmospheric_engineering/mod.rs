//! # SBMUMC Module 945: Atmospheric Engineering
//! 
//! Frameworks for modifying atmospheric composition and properties.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AtmosphericTarget {
    GreenhouseGases,
    Aerosols,
    Ozone,
    WaterVapor,
    Particulates,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphericModification {
    pub modification_id: String,
    pub target: AtmosphericTarget,
    pub target_compound: String,
    pub reduction_goal: f64,
    pub current_progress: f64,
    pub technology: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphericPlan {
    pub plan_id: String,
    pub modifications: Vec<AtmosphericModification>,
    pub projected_temp_reduction: f64,
    pub implementation_timeline: String,
}

impl AtmosphericModification {
    pub fn new(target: AtmosphericTarget, compound: &str) -> Self {
        Self {
            modification_id: format!("am_{}", uuid_simple()),
            target,
            target_compound: compound.to_string(),
            reduction_goal: 0.0,
            current_progress: 0.0,
            technology: "unknown".to_string(),
        }
    }

    pub fn set_goal(&mut self, goal: f64) {
        self.reduction_goal = goal.clamp(0.0, 1.0);
    }

    pub fn update_progress(&mut self, progress: f64) {
        self.current_progress = progress.clamp(0.0, 1.0);
    }

    pub fn completion_percentage(&self) -> f64 {
        if self.reduction_goal == 0.0 { 0.0 }
        else { (self.current_progress / self.reduction_goal).min(1.0) }
    }
}

impl AtmosphericPlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("aplan_{}", uuid_simple()),
            modifications: Vec::new(),
            projected_temp_reduction: 0.0,
            implementation_timeline: "unknown".to_string(),
        }
    }

    pub fn add_modification(&mut self, modification: AtmosphericModification) {
        self.modifications.push(modification);
    }

    pub fn compute_reduction(&mut self) {
        self.projected_temp_reduction = self.modifications.iter()
            .map(|m| m.completion_percentage() * 0.1)
            .sum::<f64>()
            .min(2.0);
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
    fn test_atmospheric_modification() {
        let mut modif = AtmosphericModification::new(
            AtmosphericTarget::GreenhouseGases,
            "CO2",
        );
        modif.set_goal(0.5);
        modif.update_progress(0.25);
        assert!(modif.completion_percentage() > 0.0);
    }

    #[test]
    fn test_atmospheric_plan() {
        let mut plan = AtmosphericPlan::new();
        plan.add_modification(AtmosphericModification::new(
            AtmosphericTarget::Aerosols,
            "SO2",
        ));
        assert!(plan.modifications.len() == 1);
    }
}
