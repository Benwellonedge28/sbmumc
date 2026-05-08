//! # SBMUMC Module 937: Interpretability Tools
//! 
//! Tools and techniques for interpreting AGI decision-making.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterpretMethod {
    FeatureVisualization,
    AttentionMapping,
    ConceptProbing,
    FeatureAttribution,
    Counterfactual,
    PrototypeAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterpretationResult {
    pub result_id: String,
    pub method: InterpretMethod,
    pub explanation: String,
    pub confidence: f64,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterpretabilityReport {
    pub report_id: String,
    pub system_id: String,
    pub interpretations: Vec<InterpretationResult>,
    pub overall_explainability: f64,
}

impl InterpretationResult {
    pub fn new(method: InterpretMethod, explanation: &str) -> Self {
        Self {
            result_id: format!("ir_{}", uuid_simple()),
            method,
            explanation: explanation.to_string(),
            confidence: 0.0,
            evidence: Vec::new(),
        }
    }

    pub fn add_evidence(&mut self, evidence: &str) {
        self.evidence.push(evidence.to_string());
    }

    pub fn set_confidence(&mut self, confidence: f64) {
        self.confidence = confidence.clamp(0.0, 1.0);
    }
}

impl InterpretabilityReport {
    pub fn new(system_id: &str) -> Self {
        Self {
            report_id: format!("ireport_{}", uuid_simple()),
            system_id: system_id.to_string(),
            interpretations: Vec::new(),
            overall_explainability: 0.0,
        }
    }

    pub fn add_interpretation(&mut self, result: InterpretationResult) {
        self.interpretations.push(result);
        self.compute_explainability();
    }

    pub fn compute_explainability(&mut self) {
        if self.interpretations.is_empty() {
            self.overall_explainability = 0.0;
            return;
        }
        let total: f64 = self.interpretations.iter()
            .map(|r| r.confidence)
            .sum();
        self.overall_explainability = total / self.interpretations.len() as f64;
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
    fn test_interpretation_result() {
        let mut result = InterpretationResult::new(
            InterpretMethod::FeatureAttribution,
            "Model focuses on edge features for classification",
        );
        result.add_evidence("Gradient analysis shows high activation at edges");
        result.set_confidence(0.88);
        assert!(result.confidence > 0.8);
    }

    #[test]
    fn test_interpretability_report() {
        let mut report = InterpretabilityReport::new("vision_model_v2");
        report.add_interpretation(InterpretationResult::new(
            InterpretMethod::AttentionMapping,
            "Attention heads focus on relevant image regions",
        ));
        assert!(report.overall_explainability >= 0.0);
    }
}
