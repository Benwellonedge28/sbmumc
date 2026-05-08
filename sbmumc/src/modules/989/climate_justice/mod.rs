//! # SBMUMC Module 989: Climate Justice
//! 
//! Frameworks for climate justice and equitable climate action.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JusticeDimension {
    Distributive,
    Procedural,
    Recognition,
    Restorative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateJusticeIssue {
    pub issue_id: String,
    pub dimension: JusticeDimension,
    pub affected_community: String,
    pub description: String,
    pub severity: f64,
    pub compensation_demand: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateJusticeFramework {
    pub framework_id: String,
    pub issues: Vec<ClimateJusticeIssue>,
    pub total_compensation_demands: f64,
    pub equity_score: f64,
    pub policy_recommendations: Vec<String>,
}

impl ClimateJusticeIssue {
    pub fn new(dimension: JusticeDimension, community: &str, description: &str) -> Self {
        Self {
            issue_id: format!("cji_{}", uuid_simple()),
            dimension,
            affected_community: community.to_string(),
            description: description.to_string(),
            severity: 0.0,
            compensation_demand: 0.0,
        }
    }

    pub fn configure(&mut self, severity: f64, compensation: f64) {
        self.severity = severity.clamp(0.0, 1.0);
        self.compensation_demand = compensation;
    }
}

impl ClimateJusticeFramework {
    pub fn new() -> Self {
        Self {
            framework_id: format!("cjf_{}", uuid_simple()),
            issues: Vec::new(),
            total_compensation_demands: 0.0,
            equity_score: 0.0,
            policy_recommendations: Vec::new(),
        }
    }

    pub fn add_issue(&mut self, issue: ClimateJusticeIssue) {
        self.issues.push(issue);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_compensation_demands = self.issues.iter()
            .map(|i| i.compensation_demand)
            .sum();
        self.equity_score = self.issues.iter()
            .map(|i| i.severity)
            .sum::<f64>() / self.issues.len().max(1) as f64;
    }

    pub fn add_recommendation(&mut self, recommendation: &str) {
        self.policy_recommendations.push(recommendation.to_string());
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
    fn test_climate_justice_issue() {
        let mut issue = ClimateJusticeIssue::new(
            JusticeDimension::Distributive,
            "Pacific Island Nations",
            "Disproportionate impact of sea level rise",
        );
        issue.configure(0.9, 50000000000.0);
        assert!(issue.severity > 0.8);
    }

    #[test]
    fn test_climate_justice_framework() {
        let mut framework = ClimateJusticeFramework::new();
        framework.add_issue(ClimateJusticeIssue::new(
            JusticeDimension::Recognition,
            "Indigenous Communities",
            "Lack of recognition of traditional knowledge",
        ));
        assert!(framework.equity_score >= 0.0);
    }
}
