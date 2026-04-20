//! Consciousness Perceptual Module
//!
//! This module implements perceptual consciousness, sensory awareness,
//! and the subjective experience of perception and sensation.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ConsciousnessPerceptual {
    pub perceptions: Vec<Perception>,
    pub sensory_experiences: Vec<SensoryExperience>,
    pub perceptual_states: Vec<PerceptualState>,
}

impl ConsciousnessPerceptual {
    pub fn new() -> Self {
        ConsciousnessPerceptual {
            perceptions: Vec::new(),
            sensory_experiences: vec![
                SensoryExperience { modality: "Visual".to_string(), bandwidth_hz: "4e14".to_string(), sensitivity: 0.9 },
                SensoryExperience { modality: "Auditory".to_string(), bandwidth_hz: "2e4".to_string(), sensitivity: 0.85 },
                SensoryExperience { modality: "Somatosensory".to_string(), bandwidth_hz: "1e3".to_string(), sensitivity: 0.8 },
            ],
            perceptual_states: Vec::new(),
        }
    }

    /// Create perception
    pub fn create_perception(&mut self, modality: &str, content: &str) -> &Perception {
        let perception = Perception {
            perception_id: format!("perc_{}", self.perceptions.len()),
            modality: modality.to_string(),
            content: content.to_string(),
            vividness: 0.85,
        };
        self.perceptions.push(perception);
        self.perceptions.last().unwrap()
    }

    /// Add sensory experience
    pub fn add_sensory_experience(&mut self, modality: &str, intensity: f64) -> &SensoryExperience {
        let experience = SensoryExperience {
            modality: modality.to_string(),
            bandwidth_hz: "Variable".to_string(),
            sensitivity: intensity,
        };
        self.sensory_experiences.push(experience);
        self.sensory_experiences.last().unwrap()
    }

    /// Enter perceptual state
    pub fn enter_state(&mut self, state_type: &str) -> &PerceptualState {
        let state = PerceptualState {
            state_id: format!("pstate_{}", self.perceptual_states.len()),
            state_type: state_type.to_string(),
            clarity: 0.8,
            stability: 0.7,
        };
        self.perceptual_states.push(state);
        self.perceptual_states.last().unwrap()
    }

    /// Qualify perception
    pub fn qualify_perception(&self, modality: &str) -> PerceptionQualia {
        PerceptionQualia {
            modality: modality.to_string(),
            color_qualia: if modality == "Visual" { Some("Red".to_string()) } else { None },
            sound_qualia: if modality == "Auditory" { Some("Loud".to_string()) } else { None },
        }
    }

    /// Perceive qualia
    pub fn perceive_qualia(&self, modality: &str) -> QualiaResult {
        QualiaResult {
            modality: modality.to_string(),
            qualia_present: true,
            richness: 0.8,
        }
    }
}

impl Default for ConsciousnessPerceptual { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perception {
    pub perception_id: String,
    pub modality: String,
    pub content: String,
    pub vividness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryExperience {
    pub modality: String,
    pub bandwidth_hz: String,
    pub sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptualState {
    pub state_id: String,
    pub state_type: String,
    pub clarity: f64,
    pub stability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionQualia {
    pub modality: String,
    pub color_qualia: Option<String>,
    pub sound_qualia: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualiaResult {
    pub modality: String,
    pub qualia_present: bool,
    pub richness: f64,
}
