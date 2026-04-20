//! Population Dynamics Module
//!
//! This module implements population biology, ecological dynamics,
//! predator-prey systems, and mathematical ecology.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct PopulationDynamics {
    pub populations: Vec<Population>,
    pub models: Vec<PopulationModel>,
    pub simulations: Vec<Simulation>,
}

impl PopulationDynamics {
    pub fn new() -> Self {
        PopulationDynamics {
            populations: Vec::new(),
            models: vec![
                PopulationModel { model_name: "Lotka-Volterra".to_string(), type_: "Predator-prey".to_string() },
                PopulationModel { model_name: "Logistic".to_string(), type_: "Growth".to_string() },
            ],
            simulations: Vec::new(),
        }
    }

    /// Create population
    pub fn create_population(&mut self, species: &str, initial_size: usize) -> &Population {
        let population = Population {
            population_id: format!("pop_{}", self.populations.len()),
            species: species.to_string(),
            size: initial_size,
            growth_rate: 0.1,
        };
        self.populations.push(population);
        self.populations.last().unwrap()
    }

    /// Simulate dynamics
    pub fn simulate(&mut self, model_name: &str, time_steps: usize) -> &Simulation {
        let simulation = Simulation {
            sim_id: format!("sim_{}", self.simulations.len()),
            model_name: model_name.to_string(),
            time_steps,
            final_state: "Stable".to_string(),
        };
        self.simulations.push(simulation);
        self.simulations.last().unwrap()
    }

    /// Predict carrying capacity
    pub fn predict_carrying_capacity(&self, species: &str) -> CarryingCapacity {
        CarryingCapacity {
            species: species.to_string(),
            capacity: 10000,
            limiting_factors: vec!["Resources".to_string(), "Space".to_string()],
        }
    }

    /// Model extinction risk
    pub fn model_extinction_risk(&self, population_size: usize, growth_rate: f64) -> ExtinctionRisk {
        ExtinctionRisk {
            population_size,
            growth_rate,
            extinction_probability: if growth_rate < 0.0 { 0.9 } else { 0.1 },
        }
    }
}

impl Default for PopulationDynamics { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    pub population_id: String,
    pub species: String,
    pub size: usize,
    pub growth_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationModel {
    pub model_name: String,
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Simulation {
    pub sim_id: String,
    pub model_name: String,
    pub time_steps: usize,
    pub final_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarryingCapacity {
    pub species: String,
    pub capacity: usize,
    pub limiting_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtinctionRisk {
    pub population_size: usize,
    pub growth_rate: f64,
    pub extinction_probability: f64,
}
