//! Social Psychology Module
//!
//! This module implements social psychology, group dynamics,
//! and human behavior in social contexts for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Social psychology system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialPsychology {
    pub sp_id: String,
    pub theories: Vec<SPTheory>,
    pub phenomena: Vec<SocialPhenomenon>,
    pub attitudes: AttitudeFramework,
    pub groups: GroupDynamics,
}

/// Social psychology theory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPTheory {
    pub theory_id: String,
    pub theory_name: String,
    pub theorist: String,
    pub core_principles: Vec<String>,
    pub empirical_support: f64,
}

/// Social phenomenon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialPhenomenon {
    pub phenomenon_name: String,
    pub description: String,
    pub conditions: Vec<String>,
    pub real_world_examples: Vec<String>,
}

/// Attitude framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttitudeFramework {
    pub attitudes: Vec<Attitude>,
    pub formation_process: Vec<String>,
    pub change_mechanisms: Vec<ChangeMechanism>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attitude {
    pub attitude_id: String,
    pub object: String,
    pub components: AttitudeComponents,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttitudeComponents {
    pub cognitive: String,
    pub affective: String,
    pub behavioral: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeMechanism {
    pub mechanism_name: String,
    pub effectiveness: f64,
    pub conditions: Vec<String>,
}

/// Group dynamics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDynamics {
    pub group_types: Vec<GroupType>,
    pub processes: Vec<GroupProcess>,
    pub leadership: LeadershipFramework,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupType {
    pub type_name: String,
    pub characteristics: Vec<String>,
    pub optimal_size: [u32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupProcess {
    pub process_name: String,
    pub description: String,
    pub outcomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadershipFramework {
    pub styles: Vec<LeadershipStyle>,
    pub effectiveness_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadershipStyle {
    pub style_name: String,
    pub description: String,
    pub applicable_scenarios: Vec<String>,
}

impl SocialPsychology {
    pub fn new() -> Self {
        Self {
            sp_id: String::from("social_psychology_v1"),
            theories: vec![
                SPTheory { theory_id: String::from("theory_cog_dissonance"), theory_name: String::from("Cognitive Dissonance"), theorist: String::from("Leon Festinger"), core_principles: vec![String::from("Inconsistency causes discomfort")], empirical_support: 0.85 },
            ],
            phenomena: vec![
                SocialPhenomenon { phenomenon_name: String::from("Conformity"), description: String::from("Adjusting behavior to group norms"), conditions: vec![String::from("Group size")], real_world_examples: vec![String::from("Asch experiments")] },
            ],
            attitudes: AttitudeFramework { attitudes: vec![], formation_process: vec![String::from("Direct experience")], change_mechanisms: vec![ChangeMechanism { mechanism_name: String::from("Persuasion"), effectiveness: 0.7, conditions: vec![] }] },
            groups: GroupDynamics { group_types: vec![GroupType { type_name: String::from("Work group"), characteristics: vec![String::from("Goal-oriented")], optimal_size: [3, 12] }], processes: vec![GroupProcess { process_name: String::from("Groupthink"), description: String::from("Defective decision-making"), outcomes: vec![] }], leadership: LeadershipFramework { styles: vec![], effectiveness_factors: vec![] } },
        }
    }

    pub fn analyze_attitude_change(&self, attitude_id: &str) -> AttitudeChangeAnalysis {
        AttitudeChangeAnalysis { attitude_id: attitude_id.to_string(), change_probability: 0.4, effective_strategies: vec![String::from("Source credibility")], barriers: vec![] }
    }

    pub fn predict_group_behavior(&self, group_id: &str) -> GroupBehaviorPrediction {
        GroupBehaviorPrediction { group_id: group_id.to_string(), likely_processes: vec![String::from("Social loafing")], performance_prediction: 7.0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttitudeChangeAnalysis {
    pub attitude_id: String,
    pub change_probability: f64,
    pub effective_strategies: Vec<String>,
    pub barriers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupBehaviorPrediction {
    pub group_id: String,
    pub likely_processes: Vec<String>,
    pub performance_prediction: f64,
}

impl Default for SocialPsychology { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { let sp = SocialPsychology::new(); assert_eq!(sp.sp_id, "social_psychology_v1"); } }
