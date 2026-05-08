//! # SBMUMC Module 959: Climate Adaptation AI
//! 
//! AI systems for climate adaptation planning and implementation.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdaptationStrategy {
    Infrastructure,
    Agricultural,
    Coastal,
    Urban,
    Social,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationRecommendation {
    pub recommendation_id: String,
    pub strategy: AdaptationStrategy,
    pub location: String,
    pub priority_score: f64,
    pub implementation_cost: f64,
    pub climate_benefit: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationPlan {
    pub plan_id: String,
    pub recommendations: Vec<AdaptationRecommendation>,
    pub total_investment: f64,
    pub risk_reduction: f64,
    pub climate_resilience_score: f64,
}

impl AdaptationRecommendation {
    pub fn new(strategy: AdaptationStrategy, location: &str) -> Self {
        Self {
            recommendation_id: format!("ar_{}", uuid_simple()),
            strategy,
            location: location.to_string(),
            priority_score: 0.0,
            implementation_cost: 0.0,
            climate_benefit: 0.0,
        }
    }

    pub fn configure(&mut self, priority: f64, cost: f64, benefit: f64) {
        self.priority_score = priority.clamp(0.0, 1.0);
        self.implementation_cost = cost;
        self.climate_benefit = benefit.clamp(0.0, 1.0);
    }

    pub fn cost_effectiveness(&self) -> f64 {
        if self.implementation_cost == 0.0 { 0.0 }
        else { self.climate_benefit / (self.implementation_cost / 1000000.0) }
    }
}

impl AdaptationPlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("adapt_plan_{}", uuid_simple()),
            recommendations: Vec::new(),
            total_investment: 0.0,
            risk_reduction: 0.0,
            climate_resilience_score: 0.0,
        }
    }

    pub fn add_recommendation(&mut self, rec: AdaptationRecommendation) {
        self.recommendations.push(rec);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_investment = self.recommendations.iter()
            .map(|r| r.implementation_cost)
            .sum();
        self.risk_reduction = self.recommendations.iter()
            .map(|r| r.priority_score * r.climate_benefit)
            .sum::<f64>() / self.recommendations.len().max(1) as f64;
        self.climate_resilience_score = self.risk_reduction;
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
    fn test_adaptation_recommendation() {
        let mut rec = AdaptationRecommendation::new(
            AdaptationStrategy::Coastal,
            "Bangladesh Delta",
        );
        rec.configure(0.9, 500000000.0, 0.85);
        assert!(rec.priority_score > 0.8);
    }

    #[test]
    fn test_adaptation_plan() {
        let mut plan = AdaptationPlan::new();
        plan.add_recommendation(AdaptationRecommendation::new(
            AdaptationStrategy::Urban,
            "Mumbai",
        ));
        assert!(plan.total_investment >= 0.0);
    }
}
