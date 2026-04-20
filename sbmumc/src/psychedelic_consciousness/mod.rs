//! Psychedelic Consciousness Module
//!
//! This module implements psychedelic states, mystical experiences,
//! and altered states induced by psychoactive substances.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct PsychedelicConsciousness {
    pub experiences: Vec<PsychedelicExperience>,
    pub states: Vec<AlteredState>,
    pub mystical_measures: Vec<MysticalMeasure>,
}

impl PsychedelicConsciousness {
    pub fn new() -> Self {
        PsychedelicConsciousness {
            experiences: Vec::new(),
            states: Vec::new(),
            mystical_measures: vec![
                MysticalMeasure { dimension: "Munity".to_string(), score: 0.0 },
                MysticalMeasure { dimension: "Internal Unity".to_string(), score: 0.0 },
                MysticalMeasure { dimension: "Transcendence".to_string(), score: 0.0 },
            ],
        }
    }

    /// Record experience
    pub fn record_experience(&mut self, substance: &str, dose: f64) -> &PsychedelicExperience {
        let experience = PsychedelicExperience {
            experience_id: format!("psy_{}", self.experiences.len()),
            substance: substance.to_string(),
            dose_ug: dose,
            onset_minutes: 30.0,
            peak_minutes: 120.0,
            duration_minutes: 360.0,
        };
        self.experiences.push(experience);
        self.experiences.last().unwrap()
    }

    /// Measure mystical experience
    pub fn measure_mystical(&mut self, experience_id: &str, dimensions: &[String]) -> MysticalResult {
        let scores: Vec<f64> = dimensions.iter().map(|_| rand::random::<f64>() * 0.4 + 0.5).collect();
        MysticalResult {
            experience_id: experience_id.to_string(),
            dimensions: dimensions.iter().cloned().zip(scores.iter().cloned()).collect(),
            total_score: scores.iter().sum::<f64>() / scores.len() as f64,
            complete_mystical: scores.iter().all(|&s| s > 0.6),
        }
    }

    /// Enter altered state
    pub fn enter_state(&mut self, state_type: &str) -> &AlteredState {
        let state = AlteredState {
            state_id: format!("alt_{}", self.states.len()),
            state_type: state_type.to_string(),
            ego_dissolution: 0.5,
            sensory_enhancement: 0.7,
            unity_experience: 0.6,
        };
        self.states.push(state);
        self.states.last().unwrap()
    }

    /// Assess ego dissolution
    pub fn assess_ego_dissolution(&self) -> EgoDissolutionResult {
        EgoDissolutionResult {
            dissolution_level: 0.5,
            boundary_loss: 0.4,
            self_forgetting: 0.6,
        }
    }

    /// Integrate experience
    pub fn integrate(&mut self, experience_id: &str, insights: &[String]) -> IntegrationResult {
        IntegrationResult {
            experience_id: experience_id.to_string(),
            insights: insights.to_vec(),
            integration_score: 0.75,
            lasting_changes: 3,
        }
    }
}

impl Default for PsychedelicConsciousness { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychedelicExperience {
    pub experience_id: String,
    pub substance: String,
    pub dose_ug: f64,
    pub onset_minutes: f64,
    pub peak_minutes: f64,
    pub duration_minutes: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlteredState {
    pub state_id: String,
    pub state_type: String,
    pub ego_dissolution: f64,
    pub sensory_enhancement: f64,
    pub unity_experience: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysticalMeasure {
    pub dimension: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MysticalResult {
    pub experience_id: String,
    pub dimensions: HashMap<String, f64>,
    pub total_score: f64,
    pub complete_mystical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EgoDissolutionResult {
    pub dissolution_level: f64,
    pub boundary_loss: f64,
    pub self_forgetting: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationResult {
    pub experience_id: String,
    pub insights: Vec<String>,
    pub integration_score: f64,
    pub lasting_changes: usize,
}
