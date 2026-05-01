//! Temporal Perception Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPerception {
    pub tper_id: String,
    pub perception_model: PerceptionModel,
    pub temporal_illusions: Vec<TemporalIllusion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionModel {
    pub present_duration_ms: f64,
    pub temporal_processing_speed: f64,
    pub time_retroaction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalIllusion {
    StoppedClock,
    TimeExpansion,
    TimeCompression,
    TemporalSmearing,
}

impl TemporalPerception {
    pub fn new() -> Self {
        Self {
            tper_id: String::from("temporal_perception_v1"),
            perception_model: PerceptionModel {
                present_duration_ms: 100.0,
                temporal_processing_speed: 1.0,
                time_retroaction: false,
            },
            temporal_illusions: vec![],
        }
    }
}

impl Default for TemporalPerception {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_perception_creation() {
        let perception = TemporalPerception::new();
        assert_eq!(perception.tper_id, "temporal_perception_v1");
    }
}
