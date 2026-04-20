//! Extinction Prevention Module
//!
//! This module implements conservation biology, extinction prevention,
//! species recovery, and biodiversity preservation.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ExtinctionPrevention {
    pub species_at_risk: Vec<AtRiskSpecies>,
    pub conservation_actions: Vec<ConservationAction>,
    pub recovery_plans: Vec<RecoveryPlan>,
}

impl ExtinctionPrevention {
    pub fn new() -> Self {
        ExtinctionPrevention {
            species_at_risk: Vec::new(),
            conservation_actions: vec![
                ConservationAction { action_type: "Habitat protection".to_string(), effectiveness: 0.7 },
                ConservationAction { action_type: "Captive breeding".to_string(), effectiveness: 0.6 },
            ],
            recovery_plans: Vec::new(),
        }
    }

    /// Assess extinction risk
    pub fn assess_risk(&mut self, species_name: &str) -> &AtRiskSpecies {
        let species = AtRiskSpecies {
            species_id: format!("risk_{}", self.species_at_risk.len()),
            name: species_name.to_string(),
            status: "Endangered".to_string(),
            population: 500,
            extinction_probability: 0.1,
        };
        self.species_at_risk.push(species);
        self.species_at_risk.last().unwrap()
    }

    /// Design conservation action
    pub fn design_action(&mut self, action_type: &str) -> &ConservationAction {
        let action = ConservationAction {
            action_type: action_type.to_string(),
            effectiveness: 0.7,
        };
        self.conservation_actions.push(action);
        self.conservation_actions.last().unwrap()
    }

    /// Create recovery plan
    pub fn create_recovery_plan(&mut self, species_id: &str) -> &RecoveryPlan {
        let plan = RecoveryPlan {
            plan_id: format!("recover_{}", self.recovery_plans.len()),
            species_id: species_id.to_string(),
            target_population: 1000,
            timeline_years: 10,
            cost_usd: 1000000.0,
        };
        self.recovery_plans.push(plan);
        self.recovery_plans.last().unwrap()
    }

    /// Monitor recovery
    pub fn monitor_recovery(&self, plan_id: &str) -> RecoveryMonitoring {
        RecoveryMonitoring {
            plan_id: plan_id.to_string(),
            population_change: 0.1,
            progress: 0.3,
        }
    }
}

impl Default for ExtinctionPrevention { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtRiskSpecies {
    pub species_id: String,
    pub name: String,
    pub status: String,
    pub population: usize,
    pub extinction_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConservationAction {
    pub action_type: String,
    pub effectiveness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPlan {
    pub plan_id: String,
    pub species_id: String,
    pub target_population: usize,
    pub timeline_years: usize,
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryMonitoring {
    pub plan_id: String,
    pub population_change: f64,
    pub progress: f64,
}
