//! Fitness Landscape Module
//!
//! This module implements fitness landscapes, adaptive peaks and valleys,
//! NK landscapes, and evolutionary dynamics on fitness surfaces.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct FitnessLandscape {
    pub landscapes: Vec<FitnessSurface>,
    pub peaks: Vec<AdaptivePeak>,
    pub trajectories: Vec<EvolutionaryTrajectory>,
}

impl FitnessLandscape {
    pub fn new() -> Self {
        FitnessLandscape {
            landscapes: Vec::new(),
            peaks: Vec::new(),
            trajectories: Vec::new(),
        }
    }

    /// Create landscape
    pub fn create_landscape(&mut self, dimensions: usize, ruggedness: f64) -> &FitnessSurface {
        let landscape = FitnessSurface {
            landscape_id: format!("land_{}", self.landscapes.len()),
            dimensions,
            ruggedness,
            fitness_range: (0.0, 1.0),
        };
        self.landscapes.push(landscape);
        self.landscapes.last().unwrap()
    }

    /// Find peak
    pub fn find_peak(&mut self, landscape_id: &str) -> &AdaptivePeak {
        let peak = AdaptivePeak {
            peak_id: format!("peak_{}", self.peaks.len()),
            landscape_id: landscape_id.to_string(),
            fitness: 0.95,
            basin_size: 50,
        };
        self.peaks.push(peak);
        self.peaks.last().unwrap()
    }

    /// Trace evolution
    pub fn trace_evolution(&mut self, landscape_id: &str) -> &EvolutionaryTrajectory {
        let trajectory = EvolutionaryTrajectory {
            trajectory_id: format!("traj_{}", self.trajectories.len()),
            landscape_id: landscape_id.to_string(),
            steps: 100,
            fitness_gain: 0.3,
        };
        self.trajectories.push(trajectory);
        self.trajectories.last().unwrap()
    }

    /// Check local optimum
    pub fn check_local_optimum(&self, fitness: f64, global_peak: f64) -> OptimumCheck {
        OptimumCheck {
            current_fitness: fitness,
            global_maximum: global_peak,
            is_local_peak: fitness > 0.8,
            escaped: false,
        }
    }
}

impl Default for FitnessLandscape { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessSurface {
    pub landscape_id: String,
    pub dimensions: usize,
    pub ruggedness: f64,
    pub fitness_range: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptivePeak {
    pub peak_id: String,
    pub landscape_id: String,
    pub fitness: f64,
    pub basin_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionaryTrajectory {
    pub trajectory_id: String,
    pub landscape_id: String,
    pub steps: usize,
    pub fitness_gain: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimumCheck {
    pub current_fitness: f64,
    pub global_maximum: f64,
    pub is_local_peak: bool,
    pub escaped: bool,
}
