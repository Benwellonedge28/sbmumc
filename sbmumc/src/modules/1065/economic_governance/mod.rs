//! # SBMUMC Module 1065: Economic Governance
//!
//! Institutions, policies, and governance structures affecting economics.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceQuality {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicGovernance {
    pub governance_id: String,
    pub country: String,
    pub governance_quality: GovernanceQuality,
    pub regulatory_quality: f64,
    pub rule_of_law: f64,
    pub corruption_control: f64,
    pub political_stability: f64,
    pub governance_effectiveness: f64,
}

impl EconomicGovernance {
    pub fn new(country: String) -> Self {
        Self {
            governance_id: crate::core::uuid_simple(),
            country,
            governance_quality: GovernanceQuality::Medium,
            regulatory_quality: 0.0,
            rule_of_law: 0.0,
            corruption_control: 0.0,
            political_stability: 0.0,
            governance_effectiveness: 0.0,
        }
    }

    pub fn assess_governance(&mut self) -> Result<()> {
        self.regulatory_quality = -2.5 + rand_simple() * 5.0;
        self.rule_of_law = -2.5 + rand_simple() * 5.0;
        self.corruption_control = -2.5 + rand_simple() * 5.0;
        self.political_stability = -2.5 + rand_simple() * 5.0;

        let avg_score = (self.regulatory_quality + self.rule_of_law
            + self.corruption_control + self.political_stability) / 4.0;

        self.governance_effectiveness = (avg_score + 2.5) / 5.0;

        self.governance_quality = if avg_score < -1.5 {
            GovernanceQuality::VeryLow
        } else if avg_score < -0.5 {
            GovernanceQuality::Low
        } else if avg_score < 0.5 {
            GovernanceQuality::Medium
        } else if avg_score < 1.5 {
            GovernanceQuality::High
        } else {
            GovernanceQuality::VeryHigh
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCredibility {
    pub credibility_id: String,
    pub policy_type: String,
    pub independence_score: f64,
    pub track_record: f64,
    pub stakeholder_confidence: f64,
    pub commitment_credibility: f64,
}

impl PolicyCredibility {
    pub fn new(policy_type: String) -> Self {
        Self {
            credibility_id: crate::core::uuid_simple(),
            policy_type,
            independence_score: 0.0,
            track_record: 0.0,
            stakeholder_confidence: 0.0,
            commitment_credibility: 0.0,
        }
    }

    pub fn evaluate_credibility(&mut self) -> Result<()> {
        self.independence_score = 0.5 + rand_simple() * 0.5;
        self.track_record = 0.4 + rand_simple() * 0.5;
        self.stakeholder_confidence = 0.45 + rand_simple() * 0.45;

        self.commitment_credibility = (self.independence_score * 0.4
            + self.track_record * 0.35
            + self.stakeholder_confidence * 0.25);
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

pub fn compute_governance_premium(governance_score: f64) -> Result<f64> {
    Ok(governance_score * 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governance_assessment() {
        let mut governance = EconomicGovernance::new("Germany".to_string());
        governance.assess_governance().unwrap();
        assert!(governance.governance_effectiveness >= 0.0 && governance.governance_effectiveness <= 1.0);
    }

    #[test]
    fn test_central_bank_credibility() {
        let mut credibility = PolicyCredibility::new("Monetary_Policy".to_string());
        credibility.evaluate_credibility().unwrap();
        assert!(credibility.commitment_credibility > 0.0);
    }
}