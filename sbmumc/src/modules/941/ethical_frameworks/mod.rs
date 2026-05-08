//! # SBMUMC Module 941: Ethical Frameworks
//! 
//! Ethical frameworks and decision-making principles for AGI systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EthicalTheory {
    Deontological,
    Consequentialist,
    VirtueEthics,
    CareEthics,
    RightsBased,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalPrinciple {
    pub principle_id: String,
    pub name: String,
    pub theory: EthicalTheory,
    pub description: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalDecision {
    pub decision_id: String,
    pub context: String,
    pub principles_applied: Vec<String>,
    pub chosen_action: String,
    pub justification: String,
    pub ethical_score: f64,
}

impl EthicalPrinciple {
    pub fn new(name: &str, theory: EthicalTheory) -> Self {
        Self {
            principle_id: format!("ep_{}", uuid_simple()),
            name: name.to_string(),
            theory,
            description: String::new(),
            weight: 1.0,
        }
    }

    pub fn set_weight(&mut self, weight: f64) {
        self.weight = weight.clamp(0.0, 1.0);
    }
}

impl EthicalDecision {
    pub fn new(context: &str, action: &str) -> Self {
        Self {
            decision_id: format!("ed_{}", uuid_simple()),
            context: context.to_string(),
            principles_applied: Vec::new(),
            chosen_action: action.to_string(),
            justification: String::new(),
            ethical_score: 0.0,
        }
    }

    pub fn apply_principle(&mut self, principle: &str) {
        self.principles_applied.push(principle.to_string());
    }

    pub fn justify(&mut self, justification: &str) {
        self.justification = justification.to_string();
    }

    pub fn compute_score(&mut self) {
        self.ethical_score = (self.principles_applied.len() as f64 / 5.0).min(1.0);
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
    fn test_ethical_principle() {
        let mut principle = EthicalPrinciple::new(
            "Do No Harm",
            EthicalTheory::Deontological,
        );
        principle.set_weight(0.95);
        assert!(principle.weight > 0.9);
    }

    #[test]
    fn test_ethical_decision() {
        let mut decision = EthicalDecision::new(
            "Medical diagnosis scenario",
            "Provide detailed explanation to patient",
        );
        decision.apply_principle("Respect autonomy");
        decision.apply_principle("Beneficence");
        decision.compute_score();
        assert!(decision.ethical_score > 0.0);
    }
}
