//! # SBMUMC Module 1047: Leadership Development
//!
//! Frameworks for developing effective leaders and leadership capabilities.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LeadershipStyle {
    Transformational,
    Servant,
    Authentic,
    Situational,
    Visionary,
    Collaborative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderProfile {
    pub profile_id: String,
    pub leader_name: String,
    pub primary_style: LeadershipStyle,
    pub strategic_thinking: f64,
    pub emotional_intelligence: f64,
    pub communication_skill: f64,
    pub adaptability: f64,
    pub overall_effectiveness: f64,
}

impl LeaderProfile {
    pub fn new(name: String, style: LeadershipStyle) -> Self {
        Self {
            profile_id: crate::core::uuid_simple(),
            leader_name: name,
            primary_style: style,
            strategic_thinking: 0.0,
            emotional_intelligence: 0.0,
            communication_skill: 0.0,
            adaptability: 0.0,
            overall_effectiveness: 0.0,
        }
    }

    pub fn assess_leadership(&mut self) -> Result<()> {
        match self.primary_style {
            LeadershipStyle::Transformational => {
                self.strategic_thinking = 0.8 + rand_simple() * 0.2;
                self.emotional_intelligence = 0.75 + rand_simple() * 0.25;
            },
            LeadershipStyle::Servant => {
                self.strategic_thinking = 0.6 + rand_simple() * 0.2;
                self.emotional_intelligence = 0.9 + rand_simple() * 0.1;
            },
            LeadershipStyle::Visionary => {
                self.strategic_thinking = 0.85 + rand_simple() * 0.15;
                self.emotional_intelligence = 0.65 + rand_simple() * 0.25;
            },
            _ => {
                self.strategic_thinking = 0.6 + rand_simple() * 0.3;
                self.emotional_intelligence = 0.6 + rand_simple() * 0.3;
            }
        }

        self.communication_skill = 0.7 + rand_simple() * 0.3;
        self.adaptability = 0.65 + rand_simple() * 0.35;

        self.overall_effectiveness = (self.strategic_thinking + self.emotional_intelligence
            + self.communication_skill + self.adaptability) / 4.0;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadershipDevelopmentPlan {
    pub plan_id: String,
    pub participant_id: String,
    pub current_level: u8,
    pub target_level: u8,
    pub development_areas: Vec<String>,
    pub growth_rate: f64,
    pub estimated_completion_months: u32,
}

impl LeadershipDevelopmentPlan {
    pub fn new(participant: String, current: u8, target: u8) -> Self {
        Self {
            plan_id: crate::core::uuid_simple(),
            participant_id: participant,
            current_level: current,
            target_level: target,
            development_areas: Vec::new(),
            growth_rate: 0.0,
            estimated_completion_months: 0,
        }
    }

    pub fn create_plan(&mut self) -> Result<()> {
        let levels_to_gain = (self.target_level - self.current_level) as f64;
        self.development_areas = vec![
            "Strategic Planning".to_string(),
            "Team Management".to_string(),
            "Communication".to_string(),
        ];
        self.growth_rate = 0.15 + rand_simple() * 0.15;
        self.estimated_completion_months = ((levels_to_gain / self.growth_rate) * 12.0) as u32;
        Ok(())
    }
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

pub fn evaluate_leadership_potential(leader_id: &str) -> Result<f64> {
    Ok(0.6 + rand_simple() * 0.4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transformational_leader() {
        let mut profile = LeaderProfile::new(
            "Jane Smith".to_string(),
            LeadershipStyle::Transformational,
        );
        profile.assess_leadership().unwrap();
        assert!(profile.overall_effectiveness > 0.6);
    }

    #[test]
    fn test_development_plan() {
        let mut plan = LeadershipDevelopmentPlan::new("EMP_001".to_string(), 3, 7);
        plan.create_plan().unwrap();
        assert!(plan.estimated_completion_months > 0);
    }
}