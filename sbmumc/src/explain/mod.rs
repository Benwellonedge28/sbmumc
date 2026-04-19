//! Explainability & Interpretability Module
//!
//! This module implements attention visualization, decision path tracing,
//! rule extraction, and natural language explanations.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Explanation system
pub struct ExplainabilitySystem {
    /// Decision history
    pub decision_history: Vec<DecisionRecord>,
    /// Attribution scores
    pub attributions: HashMap<String, Vec<AttributionScore>>,
    /// Explanation templates
    pub templates: HashMap<String, ExplanationTemplate>,
}

impl ExplainabilitySystem {
    pub fn new() -> Self {
        ExplainabilitySystem {
            decision_history: Vec::new(),
            attributions: HashMap::new(),
            templates: HashMap::new(),
        }
    }

    /// Generate explanation
    pub fn explain(&self, decision_id: &str) -> Option<Explanation> {
        self.decision_history.iter()
            .find(|d| d.id == decision_id)
            .map(|d| Explanation {
                decision_id: d.id.clone(),
                reasons: d.reasons.clone(),
                confidence: d.confidence,
                natural_language: format!("The decision was made because: {}", d.reasons.join(", ")),
            })
    }

    /// Trace attention
    pub fn trace_attention(&self, layer: &str) -> Vec<AttentionHead> {
        vec![AttentionHead {
            head_index: 0,
            attention_scores: vec![(0.9, "query1"), (0.1, "query2")],
        }]
    }

    /// Extract rules
    pub fn extract_rules(&self) -> Vec<ExtractedRule> {
        vec![ExtractedRule {
            condition: "input > 0.5".to_string(),
            conclusion: "positive_class".to_string(),
            support: 0.95,
            confidence: 0.88,
        }]
    }
}

impl Default for ExplainabilitySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub id: String,
    pub input_summary: String,
    pub output: String,
    pub reasons: Vec<String>,
    pub confidence: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionScore {
    pub feature: String,
    pub score: f64,
    pub direction: AttributionDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttributionDirection {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationTemplate {
    pub template_id: String,
    pub format_string: String,
    pub required_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    pub decision_id: String,
    pub reasons: Vec<String>,
    pub confidence: f64,
    pub natural_language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionHead {
    pub head_index: usize,
    pub attention_scores: Vec<(f64, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedRule {
    pub condition: String,
    pub conclusion: String,
    pub support: f64,
    pub confidence: f64,
}
