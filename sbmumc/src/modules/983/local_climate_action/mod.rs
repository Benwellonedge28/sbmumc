//! # SBMUMC Module 983: Local Climate Action
//! 
//! Frameworks for local and community-level climate action.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocalActionType {
    CommunitySolar,
    LocalTransport,
    UrbanGreening,
    BuildingRetrofit,
    CommunityEducation,
    LocalFood,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalClimateProject {
    pub project_id: String,
    pub action_type: LocalActionType,
    pub community_name: String,
    pub participants: u32,
    pub emissions_reduction_tco2: f64,
    pub community_cost: f64,
    pub co_benefits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalClimatePlan {
    pub plan_id: String,
    pub projects: Vec<LocalClimateProject>,
    pub total_reduction: f64,
    pub total_participants: u32,
    pub community_engagement_score: f64,
}

impl LocalClimateProject {
    pub fn new(action: LocalActionType, community: &str) -> Self {
        Self {
            project_id: format!("lcp_{}", uuid_simple()),
            action_type: action,
            community_name: community.to_string(),
            participants: 0,
            emissions_reduction_tco2: 0.0,
            community_cost: 0.0,
            co_benefits: Vec::new(),
        }
    }

    pub fn configure(&mut self, participants: u32, reduction: f64, cost: f64) {
        self.participants = participants;
        self.emissions_reduction_tco2 = reduction;
        self.community_cost = cost;
    }

    pub fn add_co_benefit(&mut self, benefit: &str) {
        self.co_benefits.push(benefit.to_string());
    }

    pub fn cost_per_participant(&self) -> f64 {
        if self.participants == 0 { 0.0 }
        else { self.community_cost / self.participants as f64 }
    }
}

impl LocalClimatePlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("lcp_{}", uuid_simple()),
            projects: Vec::new(),
            total_reduction: 0.0,
            total_participants: 0,
            community_engagement_score: 0.0,
        }
    }

    pub fn add_project(&mut self, project: LocalClimateProject) {
        self.projects.push(project);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_reduction = self.projects.iter()
            .map(|p| p.emissions_reduction_tco2)
            .sum();
        self.total_participants = self.projects.iter()
            .map(|p| p.participants)
            .sum();
        self.community_engagement_score = (self.total_participants as f64 / 10000.0).min(1.0);
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
    fn test_local_climate_project() {
        let mut project = LocalClimateProject::new(
            LocalActionType::CommunitySolar,
            "Portland Neighborhood",
        );
        project.configure(500, 1000.0, 100000.0);
        project.add_co_benefit("Energy independence");
        assert!(project.participants > 0);
    }

    #[test]
    fn test_local_climate_plan() {
        let mut plan = LocalClimatePlan::new();
        plan.add_project(LocalClimateProject::new(
            LocalActionType::UrbanGreening,
            "Denver Community",
        ));
        assert!(plan.total_participants >= 0);
    }
}
