//! Speciation Module
//!
//! This module implements speciation, species formation, reproductive isolation,
//! and evolutionary divergence mechanisms.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Speciation {
    pub species: Vec<Species>,
    pub isolation_mechanisms: Vec<IsolationMechanism>,
    pub speciation_events: Vec<SpeciationEvent>,
}

impl Speciation {
    pub fn new() -> Self {
        Speciation {
            species: Vec::new(),
            isolation_mechanisms: vec![
                IsolationMechanism { mechanism_type: "Prezygotic".to_string(), description: "Prevents mating".to_string() },
                IsolationMechanism { mechanism_type: "Postzygotic".to_string(), description: "Reduces hybrid fitness".to_string() },
            ],
            speciation_events: Vec::new(),
        }
    }

    /// Define species
    pub fn define_species(&mut self, species_name: &str, genome: &str) -> &Species {
        let species = Species {
            species_id: format!("sp_{}", self.species.len()),
            name: species_name.to_string(),
            genome_size: genome.len(),
            population_size: 10000,
        };
        self.species.push(species);
        self.species.last().unwrap()
    }

    /// Record speciation event
    pub fn record_speciation(&mut self, ancestral: &str, derived: &str, mechanism: &str) -> &SpeciationEvent {
        let event = SpeciationEvent {
            event_id: format!("spec_{}", self.speciation_events.len()),
            ancestral_species: ancestral.to_string(),
            derived_species: derived.to_string(),
            mechanism: mechanism.to_string(),
            time_ago_mya: 1.0,
        };
        self.speciation_events.push(event);
        self.speciation_events.last().unwrap()
    }

    /// Test reproductive isolation
    pub fn test_isolation(&self, species_a: &str, species_b: &str) -> IsolationTest {
        IsolationTest {
            species_a: species_a.to_string(),
            species_b: species_b.to_string(),
            isolated: true,
            mechanism: "Geographic".to_string(),
        }
    }

    /// Model divergence
    pub fn model_divergence(&self, time_mya: f64) -> DivergenceModel {
        DivergenceModel {
            time_years: time_mya * 1e6,
            expected_divergence: 0.1 * time_mya,
        }
    }
}

impl Default for Speciation { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Species {
    pub species_id: String,
    pub name: String,
    pub genome_size: usize,
    pub population_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsolationMechanism {
    pub mechanism_type: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciationEvent {
    pub event_id: String,
    pub ancestral_species: String,
    pub derived_species: String,
    pub mechanism: String,
    pub time_ago_mya: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsolationTest {
    pub species_a: String,
    pub species_b: String,
    pub isolated: bool,
    pub mechanism: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivergenceModel {
    pub time_years: f64,
    pub expected_divergence: f64,
}
