//! # SBMUMC Module 985: Climate Communication
//! 
//! Strategies and frameworks for effective climate communication.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationStrategy {
    FearAppeal,
    HopeBased,
    ValuesBased,
    Scientific,
    Narrative,
    Actionable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateCampaign {
    pub campaign_id: String,
    pub name: String,
    pub strategy: CommunicationStrategy,
    pub target_audience: String,
    pub reach: u64,
    pub engagement_rate: f64,
    pub behavior_change_indicator: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateCommunicationPlan {
    pub plan_id: String,
    pub campaigns: Vec<ClimateCampaign>,
    pub total_reach: u64,
    pub average_engagement: f64,
    pub overall_impact_score: f64,
}

impl ClimateCampaign {
    pub fn new(name: &str, strategy: CommunicationStrategy, audience: &str) -> Self {
        Self {
            campaign_id: format!("cc_{}", uuid_simple()),
            name: name.to_string(),
            strategy,
            target_audience: audience.to_string(),
            reach: 0,
            engagement_rate: 0.0,
            behavior_change_indicator: 0.0,
        }
    }

    pub fn configure(&mut self, reach: u64, engagement: f64, behavior: f64) {
        self.reach = reach;
        self.engagement_rate = engagement.clamp(0.0, 1.0);
        self.behavior_change_indicator = behavior.clamp(0.0, 1.0);
    }

    pub fn effectiveness_score(&self) -> f64 {
        let reach_norm = (self.reach as f64 / 1000000.0).min(1.0);
        (reach_norm + self.engagement_rate + self.behavior_change_indicator) / 3.0
    }
}

impl ClimateCommunicationPlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("ccp_{}", uuid_simple()),
            campaigns: Vec::new(),
            total_reach: 0,
            average_engagement: 0.0,
            overall_impact_score: 0.0,
        }
    }

    pub fn add_campaign(&mut self, campaign: ClimateCampaign) {
        self.campaigns.push(campaign);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_reach = self.campaigns.iter()
            .map(|c| c.reach)
            .sum();
        self.average_engagement = self.campaigns.iter()
            .map(|c| c.engagement_rate)
            .sum::<f64>() / self.campaigns.len().max(1) as f64;
        self.overall_impact_score = self.campaigns.iter()
            .map(|c| c.effectiveness_score())
            .sum::<f64>() / self.campaigns.len().max(1) as f64;
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
    fn test_climate_campaign() {
        let mut campaign = ClimateCampaign::new(
            "Earth Day 2025",
            CommunicationStrategy::HopeBased,
            "Young adults",
        );
        campaign.configure(5000000, 0.15, 0.1);
        assert!(campaign.effectiveness_score() > 0.0);
    }

    #[test]
    fn test_climate_communication_plan() {
        let mut plan = ClimateCommunicationPlan::new();
        plan.add_campaign(ClimateCampaign::new(
            "Science Communication Series",
            CommunicationStrategy::Scientific,
            "General public",
        ));
        assert!(plan.total_reach >= 0);
    }
}
