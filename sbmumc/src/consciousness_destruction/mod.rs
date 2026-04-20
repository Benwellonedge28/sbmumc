//! Consciousness Destruction Module
//!
//! This module implements the cessation of consciousness, annihilation,
//! and the conditions under which awareness ends.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ConsciousnessDestruction {
    pub destruction_events: Vec<DestructionEvent>,
    pub annihilation_paths: Vec<AnnihilationPath>,
    pub reversibility: Vec<ReversibilityAnalysis>,
}

impl ConsciousnessDestruction {
    pub fn new() -> Self {
        ConsciousnessDestruction {
            destruction_events: Vec::new(),
            annihilation_paths: vec![
                AnnihilationPath { path_name: "Physical Death".to_string(), reversibility: "Irreversible".to_string() },
                AnnihilationPath { path_name: "Brain Damage".to_string(), reversibility: "Sometimes".to_string() },
                AnnihilationPath { path_name: "Information Loss".to_string(), reversibility: "Irreversible".to_string() },
            ],
            reversibility: Vec::new(),
        }
    }

    /// Record destruction
    pub fn record_destruction(&mut self, consciousness_id: &str, cause: &str) -> &DestructionEvent {
        let event = DestructionEvent {
            event_id: format!("dest_{}", self.destruction_events.len()),
            consciousness_id: consciousness_id.to_string(),
            cause: cause.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            finality: 1.0,
        };
        self.destruction_events.push(event);
        self.destruction_events.last().unwrap()
    }

    /// Analyze reversibility
    pub fn analyze_reversibility(&mut self, destruction_type: &str) -> &ReversibilityAnalysis {
        let reversible = destruction_type.contains("temporary");
        let analysis = ReversibilityAnalysis {
            analysis_id: format!("rev_{}", self.reversibility.len()),
            destruction_type: destruction_type.to_string(),
            reversible,
            recovery_probability: if reversible { 0.7 } else { 0.0 },
        };
        self.reversibility.push(analysis);
        self.reversibility.last().unwrap()
    }

    /// Check finality
    pub fn check_finality(&self, event_id: &str) -> FinalityResult {
        FinalityResult {
            event_id: event_id.to_string(),
            final: true,
            certainty: 0.99,
        }
    }

    /// Calculate destruction probability
    pub fn destruction_probability(&self, cause: &str, duration_secs: f64) -> f64 {
        match cause {
            "Physical Death" => 1.0,
            "Brain Damage" => (duration_secs / 3600.0).min(1.0) * 0.5,
            "Information Loss" => 0.9,
            _ => 0.0,
        }
    }
}

impl Default for ConsciousnessDestruction { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestructionEvent {
    pub event_id: String,
    pub consciousness_id: String,
    pub cause: String,
    pub timestamp: f64,
    pub finality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnihilationPath {
    pub path_name: String,
    pub reversibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReversibilityAnalysis {
    pub analysis_id: String,
    pub destruction_type: String,
    pub reversible: bool,
    pub recovery_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalityResult {
    pub event_id: String,
    pub final: bool,
    pub certainty: f64,
}
