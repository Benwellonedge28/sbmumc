//! Emotions Consciousness Module
//!
//! This module implements emotional consciousness, feeling states,
//! and the phenomenology of affect and mood.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct EmotionsConsciousness {
    pub emotions: Vec<Emotion>,
    pub moods: Vec<Mood>,
    pub feeling_states: Vec<FeelingState>,
    pub affects: Vec<Affect>,
}

impl EmotionsConsciousness {
    pub fn new() -> Self {
        EmotionsConsciousness {
            emotions: vec![
                Emotion { emotion_type: "Joy".to_string(), intensity: 0.8, valence: 1.0, arousal: 0.7 },
                Emotion { emotion_type: "Fear".to_string(), intensity: 0.6, valence: -0.8, arousal: 0.9 },
                Emotion { emotion_type: "Anger".to_string(), intensity: 0.7, valence: -0.5, arousal: 0.8 },
            ],
            moods: Vec::new(),
            feeling_states: Vec::new(),
            affects: Vec::new(),
        }
    }

    /// Experience emotion
    pub fn experience(&mut self, emotion_type: &str, intensity: f64) -> &Emotion {
        let valence = match emotion_type {
            "Joy" => 1.0,
            "Fear" => -0.8,
            "Anger" => -0.5,
            "Sadness" => -0.9,
            "Surprise" => 0.0,
            _ => 0.0,
        };
        let emotion = Emotion {
            emotion_type: emotion_type.to_string(),
            intensity,
            valence,
            arousal: intensity,
        };
        self.emotions.push(emotion);
        self.emotions.last().unwrap()
    }

    /// Create mood
    pub fn create_mood(&mut self, mood_type: &str, duration_hours: f64) -> &Mood {
        let mood = Mood {
            mood_id: format!("mood_{}", self.moods.len()),
            mood_type: mood_type.to_string(),
            duration_hours,
            intensity: 0.6,
        };
        self.moods.push(mood);
        self.moods.last().unwrap()
    }

    /// Record feeling state
    pub fn record_feeling(&mut self, description: &str, hedonic_tone: f64) -> &FeelingState {
        let state = FeelingState {
            state_id: format!("feel_{}", self.feeling_states.len()),
            description: description.to_string(),
            hedonic_tone,
            body_presence: 0.7,
        };
        self.feeling_states.push(state);
        self.feeling_states.last().unwrap()
    }

    /// Add affect
    pub fn add_affect(&mut self, affect_type: &str, magnitude: f64) -> &Affect {
        let affect = Affect {
            affect_id: format!("affect_{}", self.affects.len()),
            affect_type: affect_type.to_string(),
            magnitude,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        };
        self.affects.push(affect);
        self.affects.last().unwrap()
    }

    /// Calculate emotional balance
    pub fn emotional_balance(&self) -> EmotionalBalance {
        let total_valence: f64 = self.emotions.iter().map(|e| e.valence).sum();
        let count = self.emotions.len().max(1) as f64;
        EmotionalBalance {
            average_valence: total_valence / count,
            emotional_range: 2.0,
            stability: 0.7,
        }
    }
}

impl Default for EmotionsConsciousness { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emotion {
    pub emotion_type: String,
    pub intensity: f64,
    pub valence: f64,
    pub arousal: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mood {
    pub mood_id: String,
    pub mood_type: String,
    pub duration_hours: f64,
    pub intensity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeelingState {
    pub state_id: String,
    pub description: String,
    pub hedonic_tone: f64,
    pub body_presence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Affect {
    pub affect_id: String,
    pub affect_type: String,
    pub magnitude: f64,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalBalance {
    pub average_valence: f64,
    pub emotional_range: f64,
    pub stability: f64,
}
