//! # SBMUMC Module 936: Corrigibility
//! 
//! Frameworks for building corrigible and adaptable AGI systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CorrectionMechanism {
    HumanOverride,
    DynamicGoals,
    ImpactMinimization,
    AmenableUpgrade,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionCapability {
    pub capability_id: String,
    pub mechanism: CorrectionMechanism,
    pub activation_cost: f64,
    pub reliability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrigibilityMetrics {
    pub system_id: String,
    pub capabilities: Vec<CorrectionCapability>,
    pub overall_score: f64,
    pub risk_flags: Vec<String>,
}

impl CorrectionCapability {
    pub fn new(mechanism: CorrectionMechanism) -> Self {
        Self {
            capability_id: format!("cc_{}", uuid_simple()),
            mechanism,
            activation_cost: 0.1,
            reliability: 0.0,
        }
    }

    pub fn calibrate(&mut self, reliability: f64) {
        self.reliability = reliability.clamp(0.0, 1.0);
    }
}

impl CorrigibilityMetrics {
    pub fn new(system_id: &str) -> Self {
        Self {
            system_id: system_id.to_string(),
            capabilities: Vec::new(),
            overall_score: 0.0,
            risk_flags: Vec::new(),
        }
    }

    pub fn add_capability(&mut self, capability: CorrectionCapability) {
        self.capabilities.push(capability);
        self.compute_score();
    }

    pub fn compute_score(&mut self) {
        if self.capabilities.is_empty() {
            self.overall_score = 0.0;
            return;
        }
        let total: f64 = self.capabilities.iter()
            .map(|c| c.reliability * (1.0 - c.activation_cost))
            .sum();
        self.overall_score = total / self.capabilities.len() as f64;
    }

    pub fn check_risks(&mut self) {
        if self.overall_score < 0.5 {
            self.risk_flags.push("Low corrigibility score".to_string());
        }
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
    fn test_correction_capability() {
        let mut cap = CorrectionCapability::new(CorrectionMechanism::HumanOverride);
        cap.calibrate(0.95);
        assert!(cap.reliability > 0.9);
    }

    #[test]
    fn test_corrigibility_metrics() {
        let mut metrics = CorrigibilityMetrics::new("test_corrigible");
        metrics.add_capability(CorrectionCapability::new(CorrectionMechanism::HumanOverride));
        metrics.check_risks();
        assert!(!metrics.risk_flags.is_empty() || metrics.overall_score > 0.0);
    }
}
