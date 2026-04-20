//! Consciousness Temporal Module
//!
//! This module implements temporal consciousness, time experience,
//! and the subjective awareness of the flow and structure of time.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ConsciousnessTemporal {
    pub temporal_experiences: Vec<TemporalExperience>,
    pub time_perception: Vec<TimePerception>,
    pub temporal_structures: Vec<TemporalStructure>,
}

impl ConsciousnessTemporal {
    pub fn new() -> Self {
        ConsciousnessTemporal {
            temporal_experiences: Vec::new(),
            time_perception: vec![
                TimePerception { time_type: "Present".to_string(), duration_ms: 100.0 },
                TimePerception { time_type: "Short-term".to_string(), duration_secs: 30.0 },
                TimePerception { time_type: "Long-term".to_string(), duration_years: 70.0 },
            ],
            temporal_structures: Vec::new(),
        }
    }

    /// Experience time flow
    pub fn experience_time_flow(&mut self, flow_rate: f64) -> &TemporalExperience {
        let experience = TemporalExperience {
            experience_id: format!("texp_{}", self.temporal_experiences.len()),
            flow_rate,
            direction: "Forward".to_string(),
            perceived_duration: 1.0,
        };
        self.temporal_experiences.push(experience);
        self.temporal_experiences.last().unwrap()
    }

    /// Perceive duration
    pub fn perceive_duration(&self, physical_duration: f64) -> DurationPerception {
        let subjective_duration = if physical_duration < 1.0 {
            physical_duration * 2.0 // Time dilation for short durations
        } else {
            physical_duration
        };
        DurationPerception {
            physical_duration,
            subjective_duration,
            accuracy: 0.7,
        }
    }

    /// Structure temporal experience
    pub fn structure_temporal(&mut self, experience: &[String]) -> &TemporalStructure {
        let structure = TemporalStructure {
            structure_id: format!("tstruct_{}", self.temporal_structures.len()),
            events: experience.to_vec(),
            temporal_order: "Chronological".to_string(),
            relationships: vec!["Before".to_string(), "After".to_string(), "During".to_string()],
        };
        self.temporal_structures.push(structure);
        self.temporal_structures.last().unwrap()
    }

    /// Anticipate future
    pub fn anticipate_future(&self, horizon: f64) -> FutureAnticipation {
        FutureAnticipation {
            horizon_years: horizon / 365.0,
            confidence: 0.5,
            details: "Vague".to_string(),
        }
    }

    /// Remember past
    pub fn remember_past(&self, event: &str) -> PastMemory {
        PastMemory {
            event: event.to_string(),
            vividness: 0.7,
            accuracy: 0.6,
            emotional_charge: 0.5,
        }
    }
}

impl Default for ConsciousnessTemporal { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalExperience {
    pub experience_id: String,
    pub flow_rate: f64,
    pub direction: String,
    pub perceived_duration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePerception {
    pub time_type: String,
    pub duration_ms: f64,
    pub duration_secs: f64,
    pub duration_years: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalStructure {
    pub structure_id: String,
    pub events: Vec<String>,
    pub temporal_order: String,
    pub relationships: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationPerception {
    pub physical_duration: f64,
    pub subjective_duration: f64,
    pub accuracy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureAnticipation {
    pub horizon_years: f64,
    pub confidence: f64,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PastMemory {
    pub event: String,
    pub vividness: f64,
    pub accuracy: f64,
    pub emotional_charge: f64,
}
