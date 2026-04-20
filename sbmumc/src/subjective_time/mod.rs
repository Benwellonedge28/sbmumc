//! Subjective Time Module
//!
//! This module implements temporal phenomenology, time perception,
//! and the subjective flow of conscious experience.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub struct SubjectiveTime {
    pub temporal_maps: Vec<TemporalMap>,
    pub durations: Vec<SubjectiveDuration>,
    pub sequences: VecDeque<TemporalSequence>,
    pub now_points: Vec<NowPoint>,
}

impl SubjectiveTime {
    pub fn new() -> Self {
        SubjectiveTime {
            temporal_maps: Vec::new(),
            durations: Vec::new(),
            sequences: VecDeque::new(),
            now_points: Vec::new(),
        }
    }

    /// Create temporal map
    pub fn create_temporal_map(&mut self, scale: &str) -> &TemporalMap {
        let map = TemporalMap {
            map_id: format!("tm_{}", self.temporal_maps.len()),
            scale: scale.to_string(),
            subjective_to_physical_ratio: match scale {
                "Present" => 1.0,
                "Short" => 10.0,
                "Long" => 0.1,
                _ => 1.0,
            },
        };
        self.temporal_maps.push(map);
        self.temporal_maps.last().unwrap()
    }

    /// Measure subjective duration
    pub fn measure_duration(&mut self, event: &str, physical_secs: f64) -> SubjectiveDuration {
        let subjective_secs = physical_secs * 2.5; // Time feels longer during novel events
        let duration = SubjectiveDuration {
            event: event.to_string(),
            physical_duration: physical_secs,
            subjective_duration: subjective_secs,
            dilation_factor: subjective_secs / physical_secs,
        };
        self.durations.push(duration.clone());
        duration
    }

    /// Record now point
    pub fn record_now(&mut self, content: &str) -> &NowPoint {
        let point = NowPoint {
            point_id: format!("now_{}", self.now_points.len()),
            content: content.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            temporal_width_ms: 100.0,
        };
        self.now_points.push(point);
        self.now_points.last().unwrap()
    }

    /// Add to sequence
    pub fn add_to_sequence(&mut self, event: &str, duration_ms: f64) {
        self.sequences.push_back(TemporalSequence {
            event: event.to_string(),
            duration_ms,
            order: self.sequences.len(),
        });
    }

    /// Calculate temporal flow
    pub fn calculate_flow(&self) -> TemporalFlow {
        TemporalFlow {
            flow_rate: 1.0,
            direction: "Forward".to_string(),
            stability: 0.85,
        }
    }
}

impl Default for SubjectiveTime { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalMap {
    pub map_id: String,
    pub scale: String,
    pub subjective_to_physical_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectiveDuration {
    pub event: String,
    pub physical_duration: f64,
    pub subjective_duration: f64,
    pub dilation_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalSequence {
    pub event: String,
    pub duration_ms: f64,
    pub order: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowPoint {
    pub point_id: String,
    pub content: String,
    pub timestamp: f64,
    pub temporal_width_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalFlow {
    pub flow_rate: f64,
    pub direction: String,
    pub stability: f64,
}
