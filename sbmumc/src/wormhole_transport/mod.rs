//! Wormhole Transport Module
//!
//! This module implements traversable wormholes, Einstein-Rosen bridges,
//! exotic matter requirements, and interstellar shortcut physics.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct WormholeTransport {
    pub wormholes: Vec<Wormhole>,
    pub traversals: Vec<Traversal>,
    pub exotic_matter: Vec<ExoticMatter>,
}

impl WormholeTransport {
    pub fn new() -> Self {
        WormholeTransport {
            wormholes: Vec::new(),
            traversals: Vec::new(),
            exotic_matter: vec![
                ExoticMatter { type_: "Negative energy".to_string(), density: -1e10 },
            ],
        }
    }

    /// Design wormhole
    pub fn design_wormhole(&mut self, mouth_distance_ly: f64) -> &Wormhole {
        let wormhole = Wormhole {
            wormhole_id: format!("wh_{}", self.wormholes.len()),
            mouth_separation_ly: mouth_distance_ly,
            throat_radius_m: 1000.0,
            stability: 0.5,
        };
        self.wormholes.push(wormhole);
        self.wormholes.last().unwrap()
    }

    /// Stabilize with exotic matter
    pub fn stabilize(&mut self, wormhole_id: &str, exotic_matter_kg: f64) -> StabilizationResult {
        StabilizationResult {
            wormhole_id: wormhole_id.to_string(),
            exotic_matter_kg,
            stability_improved: 0.3,
            stable: exotic_matter_kg > 1e9,
        }
    }

    /// Traverse wormhole
    pub fn traverse(&mut self, wormhole_id: &str, object_size_m: f64) -> &Traversal {
        let traversal = Traversal {
            traversal_id: format!("trav_{}", self.traversals.len()),
            wormhole_id: wormhole_id.to_string(),
            transit_time_sec: 1.0,
            object_survived: true,
        };
        self.traversals.push(traversal);
        self.traversals.last().unwrap()
    }

    /// Calculate shortcut
    pub fn calculate_shortcut(&self, distance_ly: f64) -> ShortcutCalculation {
        ShortcutCalculation {
            direct_distance_ly: distance_ly,
            through_wormhole_time: 1.0,
            shortcut_factor: distance_ly * 3.15e7,
        }
    }
}

impl Default for WormholeTransport { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wormhole {
    pub wormhole_id: String,
    pub mouth_separation_ly: f64,
    pub throat_radius_m: f64,
    pub stability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Traversal {
    pub traversal_id: String,
    pub wormhole_id: String,
    pub transit_time_sec: f64,
    pub object_survived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExoticMatter {
    pub type_: String,
    pub density: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilizationResult {
    pub wormhole_id: String,
    pub exotic_matter_kg: f64,
    pub stability_improved: f64,
    pub stable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutCalculation {
    pub direct_distance_ly: f64,
    pub through_wormhole_time: f64,
    pub shortcut_factor: f64,
}
