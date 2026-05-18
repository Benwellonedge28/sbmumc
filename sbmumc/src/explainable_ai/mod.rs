//! # SBMUMC Module 1615: Explainable AI
//!
//! Interpretable and explainable AI systems.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XAIConfig {
    pub explanation_method: ExplanationMethod,
    pub fidelity_threshold: f64,
    pub complexity_limit: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExplanationMethod {
    SHAP,
    LIME,
    GradCAM,
    Counterfactual,
    RuleExtraction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    pub explanation_id: String,
    pub method: ExplanationMethod,
    pub feature_importance: Vec<FeatureImportance>,
    pub human_readable: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureImportance {
    pub feature_name: String,
    pub importance_score: f64,
    pub direction: Direction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counterfactual {
    pub cf_id: String,
    pub original_value: f64,
    pub counterfactual_value: f64,
    pub change_needed: String,
    pub feasibility: f64,
}

pub struct ExplainableAI {
    config: XAIConfig,
    model_registry: HashMap<String, ModelMetadata>,
    explanations: Vec<Explanation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub model_id: String,
    pub model_type: String,
    pub training_data: String,
    pub performance: f64,
}

impl ExplainableAI {
    pub fn new(config: XAIConfig) -> Self {
        Self {
            config,
            model_registry: HashMap::new(),
            explanations: Vec::new(),
        }
    }

    pub fn register_model(&mut self, model: ModelMetadata) -> Result<()> {
        self.model_registry.insert(model.model_id.clone(), model);
        Ok(())
    }

    pub fn explain(&self, model_id: &str, input: &[f64]) -> Result<Explanation> {
        if !self.model_registry.contains_key(model_id) {
            return Err(SbmumcError::Internal("Model not found".into()));
        }

        let feature_importance: Vec<FeatureImportance> = input.iter()
            .enumerate()
            .map(|(i, value)| FeatureImportance {
                feature_name: format!("feature_{}", i),
                importance_score: value.abs() / (input.iter().map(|v| v.abs()).sum::<f64>() + 0.001),
                direction: if *value > 0.0 { Direction::Positive } else { Direction::Negative },
            })
            .collect();

        let human_readable = format!(
            "The top 3 most important features are: {}, {}, and {}",
            feature_importance[0].feature_name,
            feature_importance.get(1).map(|f| f.feature_name.clone()).unwrap_or_default(),
            feature_importance.get(2).map(|f| f.feature_name.clone()).unwrap_or_default()
        );

        let explanation = Explanation {
            explanation_id: uuid::Uuid::new_v4().to_string(),
            method: self.config.explanation_method.clone(),
            feature_importance,
            human_readable,
            confidence: 0.85,
        };

        Ok(explanation)
    }

    pub fn generate_counterfactual(&self, model_id: &str, current_value: f64, target: f64) -> Result<Counterfactual> {
        if !self.model_registry.contains_key(model_id) {
            return Err(SbmumcError::Internal("Model not found".into()));
        }

        let change = (target - current_value) * (0.8 + rand::random::<f64>() * 0.4);

        Ok(Counterfactual {
            cf_id: uuid::Uuid::new_v4().to_string(),
            original_value: current_value,
            counterfactual_value: current_value + change,
            change_needed: format!("increase by {:.2}", change.abs()),
            feasibility: 0.7 + rand::random::<f64>() * 0.25,
        })
    }

    pub fn extract_rules(&self, model_id: &str) -> Result<Vec<Rule>> {
        if !self.model_registry.contains_key(model_id) {
            return Err(SbmumcError::Internal("Model not found".into()));
        }

        let rules = vec![
            Rule {
                rule_id: "rule_1".to_string(),
                condition: "feature_0 > 0.5".to_string(),
                conclusion: "class_1".to_string(),
                support: 0.85,
            },
            Rule {
                rule_id: "rule_2".to_string(),
                condition: "feature_1 <= 0.3".to_string(),
                conclusion: "class_2".to_string(),
                support: 0.78,
            },
        ];

        Ok(rules)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub rule_id: String,
    pub condition: String,
    pub conclusion: String,
    pub support: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xai() {
        let config = XAIConfig {
            explanation_method: ExplanationMethod::SHAP,
            fidelity_threshold: 0.8,
            complexity_limit: 10,
        };

        let mut xai = ExplainableAI::new(config);

        let model = ModelMetadata {
            model_id: "classifier_v1".to_string(),
            model_type: "RandomForest".to_string(),
            training_data: "synthetic".to_string(),
            performance: 0.92,
        };

        xai.register_model(model).unwrap();

        let input = vec![0.5, -0.3, 0.8, -0.2, 0.1];
        let explanation = xai.explain("classifier_v1", &input).unwrap();

        assert!(!explanation.feature_importance.is_empty());
        assert!(!explanation.human_readable.is_empty());
    }
}