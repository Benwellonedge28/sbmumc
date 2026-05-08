//! # SBMUMC Module 940: AI Governance
//! 
//! Governance frameworks and policies for AGI development and deployment.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceLevel {
    International,
    National,
    Corporate,
    Institutional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernancePolicy {
    pub policy_id: String,
    pub level: GovernanceLevel,
    pub title: String,
    pub scope: Vec<String>,
    pub enforcement_strength: f64,
    pub compliance_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceFramework {
    pub framework_id: String,
    pub name: String,
    pub policies: Vec<GovernancePolicy>,
    pub coverage_score: f64,
    pub effectiveness_score: f64,
}

impl GovernancePolicy {
    pub fn new(level: GovernanceLevel, title: &str) -> Self {
        Self {
            policy_id: format!("gp_{}", uuid_simple()),
            level,
            title: title.to_string(),
            scope: Vec::new(),
            enforcement_strength: 0.0,
            compliance_status: "unknown".to_string(),
        }
    }

    pub fn add_scope(&mut self, scope_item: &str) {
        self.scope.push(scope_item.to_string());
    }

    pub fn set_enforcement(&mut self, strength: f64) {
        self.enforcement_strength = strength.clamp(0.0, 1.0);
    }

    pub fn evaluate_compliance(&mut self, compliant: bool) {
        self.compliance_status = if compliant {
            "compliant".to_string()
        } else {
            "non_compliant".to_string()
        };
    }
}

impl GovernanceFramework {
    pub fn new(name: &str) -> Self {
        Self {
            framework_id: format!("gf_{}", uuid_simple()),
            name: name.to_string(),
            policies: Vec::new(),
            coverage_score: 0.0,
            effectiveness_score: 0.0,
        }
    }

    pub fn add_policy(&mut self, policy: GovernancePolicy) {
        self.policies.push(policy);
        self.compute_scores();
    }

    pub fn compute_scores(&mut self) {
        if self.policies.is_empty() {
            return;
        }
        let total_enforcement: f64 = self.policies.iter()
            .map(|p| p.enforcement_strength)
            .sum();
        self.effectiveness_score = total_enforcement / self.policies.len() as f64;
        self.coverage_score = (self.policies.len() as f64 / 20.0).min(1.0);
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
    fn test_governance_policy() {
        let mut policy = GovernancePolicy::new(
            GovernanceLevel::International,
            "AI Safety Convention",
        );
        policy.add_scope("Autonomous weapons prohibition");
        policy.set_enforcement(0.85);
        assert!(policy.enforcement_strength > 0.8);
    }

    #[test]
    fn test_governance_framework() {
        let mut framework = GovernanceFramework::new("EU AI Act Framework");
        framework.add_policy(GovernancePolicy::new(
            GovernanceLevel::National,
            "High-Risk AI Regulations",
        ));
        assert!(framework.policies.len() == 1);
    }
}
