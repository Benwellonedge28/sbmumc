//! # SBMUMC Module 930: Cognitive Development
//! 
//! Models of cognitive development and maturation in AGI systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DevelopmentalStage {
    Infant,
    Child,
    Adolescent,
    Adult,
    Expert,
    Master,
    Transcendent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveMilestone {
    pub milestone_id: String,
    pub stage: DevelopmentalStage,
    pub capability: String,
    pub age_equivalent: String,
    pub achieved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentalTrajectory {
    pub trajectory_id: String,
    pub start_stage: DevelopmentalStage,
    pub current_stage: DevelopmentalStage,
    pub milestones: Vec<CognitiveMilestone>,
    pub progression_rate: f64,
}

impl CognitiveMilestone {
    pub fn new(stage: DevelopmentalStage, capability: &str) -> Self {
        Self {
            milestone_id: format!("milestone_{}", uuid_simple()),
            stage,
            capability: capability.to_string(),
            age_equivalent: format!("{:?}", stage),
            achieved: false,
        }
    }

    pub fn achieve(&mut self) {
        self.achieved = true;
    }
}

impl DevelopmentalTrajectory {
    pub fn new(start: DevelopmentalStage) -> Self {
        Self {
            trajectory_id: format!("traj_{}", uuid_simple()),
            start_stage: start.clone(),
            current_stage: start,
            milestones: Vec::new(),
            progression_rate: 1.0,
        }
    }

    pub fn advance(&mut self, stage: DevelopmentalStage) {
        if self.current_stage != stage {
            self.progression_rate *= 1.1;
        }
        self.current_stage = stage;
    }

    pub fn add_milestone(&mut self, milestone: CognitiveMilestone) {
        self.milestones.push(milestone);
    }

    pub fn completion_percentage(&self) -> f64 {
        let achieved = self.milestones.iter().filter(|m| m.achieved).count() as f64;
        let total = self.milestones.len() as f64;
        if total == 0.0 { 0.0 } else { achieved / total }
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
    fn test_cognitive_milestone() {
        let mut milestone = CognitiveMilestone::new(
            DevelopmentalStage::Child,
            "Object permanence",
        );
        milestone.achieve();
        assert!(milestone.achieved);
    }

    #[test]
    fn test_developmental_trajectory() {
        let mut trajectory = DevelopmentalTrajectory::new(DevelopmentalStage::Infant);
        trajectory.add_milestone(CognitiveMilestone::new(DevelopmentalStage::Infant, "Reflexes"));
        trajectory.advance(DevelopmentalStage::Child);
        assert!(trajectory.completion_percentage() >= 0.0);
    }
}
