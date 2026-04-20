//! Animal Consciousness Module
//!
//! This module implements animal minds, non-human consciousness assessment,
//! and the spectrum of awareness across species.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct AnimalConsciousness {
    pub species_assessments: Vec<SpeciesAssessment>,
    pub capabilities: Vec<Capability>,
    pub consciousness_levels: HashMap<String, f64>,
}

impl AnimalConsciousness {
    pub fn new() -> Self {
        AnimalConsciousness {
            species_assessments: Vec::new(),
            capabilities: vec![
                Capability { name: "Self-recognition".to_string(), species: vec!["Great Apes".to_string(), "Dolphins".to_string(), "Elephants".to_string()] },
                Capability { name: "Tool use".to_string(), species: vec!["Chimps".to_string(), "Crows".to_string(), "Octopi".to_string()] },
                Capability { name: "Theory of mind".to_string(), species: vec!["Great Apes".to_string(), "Corvids".to_string()] },
            ],
            consciousness_levels: HashMap::from([
                ("Humans".to_string(), 1.0),
                ("Great Apes".to_string(), 0.85),
                ("Dolphins".to_string(), 0.8),
                ("Dogs".to_string(), 0.6),
                ("Birds".to_string(), 0.5),
                ("Fish".to_string(), 0.3),
                ("Insects".to_string(), 0.1),
            ]),
        }
    }

    /// Assess species
    pub fn assess_species(&mut self, species: &str) -> &SpeciesAssessment {
        let level = self.consciousness_levels.get(species).copied().unwrap_or(0.5);
        let assessment = SpeciesAssessment {
            species: species.to_string(),
            consciousness_level: level,
            sentience_score: level,
            welfare_relevance: level > 0.4,
        };
        self.species_assessments.push(assessment);
        self.species_assessments.last().unwrap()
    }

    /// Check capability
    pub fn check_capability(&self, capability: &str, species: &str) -> CapabilityResult {
        let has_capability = self.capabilities.iter()
            .any(|c| c.name == capability && c.species.iter().any(|s| s == species));
        CapabilityResult {
            capability: capability.to_string(),
            species: species.to_string(),
            has_capability,
            evidence_level: if has_capability { "Established" } else { "Unknown" }.to_string(),
        }
    }

    /// Measure sentience
    pub fn measure_sentience(&self, species: &str) -> SentienceResult {
        SentienceResult {
            species: species.to_string(),
            sentience_score: self.consciousness_levels.get(species).copied().unwrap_or(0.5),
            pain_perception: true,
            emotional_capacity: true,
        }
    }

    /// Evaluate welfare relevance
    pub fn evaluate_welfare(&self, species: &str) -> WelfareResult {
        WelfareResult {
            species: species.to_string(),
            welfare_matters: true,
            ethical_consideration: "High".to_string(),
        }
    }
}

impl Default for AnimalConsciousness { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesAssessment {
    pub species: String,
    pub consciousness_level: f64,
    pub sentience_score: f64,
    pub welfare_relevance: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub species: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityResult {
    pub capability: String,
    pub species: String,
    pub has_capability: bool,
    pub evidence_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentienceResult {
    pub species: String,
    pub sentience_score: f64,
    pub pain_perception: bool,
    pub emotional_capacity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelfareResult {
    pub species: String,
    pub welfare_matters: bool,
    pub ethical_consideration: String,
}
