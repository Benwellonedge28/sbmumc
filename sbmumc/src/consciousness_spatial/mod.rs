//! Consciousness Spatial Module
//!
//! This module implements spatial consciousness, body awareness,
//! and the subjective experience of space and自己的身体.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ConsciousnessSpatial {
    pub spatial_experiences: Vec<SpatialExperience>,
    pub body_schema: Vec<BodySchema>,
    pub spatial_maps: Vec<SpatialMap>,
}

impl ConsciousnessSpatial {
    pub fn new() -> Self {
        ConsciousnessSpatial {
            spatial_experiences: Vec::new(),
            body_schema: vec![
                BodySchema { body_part: "Hand".to_string(), position: [0.0, 0.0, 0.0], awareness: 0.9 },
                BodySchema { body_part: "Head".to_string(), position: [0.0, 1.5, 0.0], awareness: 0.95 },
                BodySchema { body_part: "Torso".to_string(), position: [0.0, 0.75, 0.0], awareness: 0.7 },
            ],
            spatial_maps: Vec::new(),
        }
    }

    /// Experience space
    pub fn experience_space(&mut self, space_type: &str) -> &SpatialExperience {
        let experience = SpatialExperience {
            experience_id: format!("sexp_{}", self.spatial_experiences.len()),
            space_type: space_type.to_string(),
            dimensionality: if space_type == "3D" { 3 } else { 2 },
            precision: 0.85,
        };
        self.spatial_experiences.push(experience);
        self.spatial_experiences.last().unwrap()
    }

    /// Update body schema
    pub fn update_body_schema(&mut self, body_part: &str, position: [f64; 3]) -> Result<()> {
        if let Some(part) = self.body_schema.iter_mut().find(|b| b.body_part == body_part) {
            part.position = position;
            part.awareness = 0.9;
            Ok(())
        } else {
            self.body_schema.push(BodySchema {
                body_part: body_part.to_string(),
                position,
                awareness: 0.8,
            });
            Ok(())
        }
    }

    /// Create spatial map
    pub fn create_spatial_map(&mut self, environment: &str) -> &SpatialMap {
        let map = SpatialMap {
            map_id: format!("smap_{}", self.spatial_maps.len()),
            environment: environment.to_string(),
            resolution: 0.1,
            extent_meters: 10.0,
        };
        self.spatial_maps.push(map);
        self.spatial_maps.last().unwrap()
    }

    /// Localize self
    pub fn localize_self(&self) -> SelfLocalization {
        SelfLocalization {
            position: [0.0, 1.7, 0.0], // Standing human height
            orientation: "Forward".to_string(),
            confidence: 0.95,
        }
    }

    /// Perceive distance
    pub fn perceive_distance(&self, physical_meters: f64) -> DistancePerception {
        DistancePerception {
            physical_distance: physical_meters,
            perceived_distance: physical_meters * 1.1, // Slight overestimation
            accuracy: 0.8,
        }
    }
}

impl Default for ConsciousnessSpatial { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialExperience {
    pub experience_id: String,
    pub space_type: String,
    pub dimensionality: usize,
    pub precision: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodySchema {
    pub body_part: String,
    pub position: [f64; 3],
    pub awareness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialMap {
    pub map_id: String,
    pub environment: String,
    pub resolution: f64,
    pub extent_meters: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfLocalization {
    pub position: [f64; 3],
    pub orientation: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistancePerception {
    pub physical_distance: f64,
    pub perceived_distance: f64,
    pub accuracy: f64,
}
