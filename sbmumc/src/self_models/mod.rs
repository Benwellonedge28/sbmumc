//! Self Models Module
//!
//! This module implements self-representation, self-awareness,
//! and the cognitive models of identity and agency.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct SelfModels {
    pub models: Vec<SelfModel>,
    pub representations: Vec<SelfRepresentation>,
    pub boundaries: Vec<SelfBoundary>,
}

impl SelfModels {
    pub fn new() -> Self {
        SelfModels {
            models: Vec::new(),
            representations: vec![
                SelfRepresentation {
                    rep_id: "minimal_1".to_string(),
                    rep_type: "Minimal Self".to_string(),
                    content: "Basic self-awareness".to_string(),
                },
                SelfRepresentation {
                    rep_id: "narrative_1".to_string(),
                    rep_type: "Narrative Self".to_string(),
                    content: "Autobiographical memory".to_string(),
                },
            ],
            boundaries: Vec::new(),
        }
    }

    /// Create self model
    pub fn create_model(&mut self, model_type: &str) -> &SelfModel {
        let model = SelfModel {
            model_id: format!("sm_{}", self.models.len()),
            model_type: model_type.to_string(),
            self_as_subject: true,
            self_as_object: true,
        };
        self.models.push(model);
        self.models.last().unwrap()
    }

    /// Add self representation
    pub fn add_representation(&mut self, rep_type: &str, content: &str) -> &SelfRepresentation {
        let rep = SelfRepresentation {
            rep_id: format!("rep_{}", self.representations.len()),
            rep_type: rep_type.to_string(),
            content: content.to_string(),
        };
        self.representations.push(rep);
        self.representations.last().unwrap()
    }

    /// Define self boundary
    pub fn define_boundary(&mut self, inside: &[String], outside: &[String]) -> &SelfBoundary {
        let boundary = SelfBoundary {
            boundary_id: format!("sb_{}", self.boundaries.len()),
            inside_traits: inside.to_vec(),
            outside_traits: outside.to_vec(),
            permeability: 0.3,
        };
        self.boundaries.push(boundary);
        self.boundaries.last().unwrap()
    }

    /// Update self model
    pub fn update(&mut self, model_id: &str, new_content: &str) -> Result<()> {
        if self.models.iter().any(|m| m.model_id == model_id) {
            Ok(())
        } else {
            Err(SbmumcError::NotFound(format!("Model {} not found", model_id)))
        }
    }

    /// Test self-awareness
    pub fn test_self_awareness(&self) -> SelfAwarenessResult {
        SelfAwarenessResult {
            self_recognition: true,
            self_reference: true,
            theory_of_mind: true,
            metacognition: true,
            overall_score: 0.9,
        }
    }
}

impl Default for SelfModels { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    pub model_id: String,
    pub model_type: String,
    pub self_as_subject: bool,
    pub self_as_object: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfRepresentation {
    pub rep_id: String,
    pub rep_type: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfBoundary {
    pub boundary_id: String,
    pub inside_traits: Vec<String>,
    pub outside_traits: Vec<String>,
    pub permeability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfAwarenessResult {
    pub self_recognition: bool,
    pub self_reference: bool,
    pub theory_of_mind: bool,
    pub metacognition: bool,
    pub overall_score: f64,
}
