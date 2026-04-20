//! Consciousness Valuation Module
//!
//! This module implements the value of consciousness, axiology,
//! and the worth and meaning of subjective experience.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ConsciousnessValuation {
    pub valuations: Vec<Valuation>,
    pub value_theories: Vec<ValueTheory>,
    pub experiences: Vec<ExperienceValue>,
}

impl ConsciousnessValuation {
    pub fn new() -> Self {
        ConsciousnessValuation {
            valuations: Vec::new(),
            value_theories: vec![
                ValueTheory { theory_name: "Hedonistic".to_string(), basis: "Pleasure maximization".to_string() },
                ValueTheory { theory_name: "Preferentist".to_string(), basis: "Preference satisfaction".to_string() },
                ValueTheory { theory_name: "Perfectionist".to_string(), basis: "Human flourishing".to_string() },
            ],
            experiences: Vec::new(),
        }
    }

    /// Value experience
    pub fn value_experience(&mut self, experience_type: &str, intensity: f64, duration: f64) -> &Valuation {
        let value = intensity * duration;
        let valuation = Valuation {
            valuation_id: format!("val_{}", self.valuations.len()),
            experience_type: experience_type.to_string(),
            value,
            intensity,
            duration,
        };
        self.valuations.push(valuation);
        self.valuations.last().unwrap()
    }

    /// Calculate total value
    pub fn calculate_total_value(&self) -> f64 {
        self.valuations.iter().map(|v| v.value).sum()
    }

    /// Record experience value
    pub fn record_experience(&mut self, experience: &str, hedonic_value: f64) -> &ExperienceValue {
        let exp_value = ExperienceValue {
            experience_id: format!("expval_{}", self.experiences.len()),
            experience: experience.to_string(),
            hedonic_value,
            meaning_value: hedonic_value * 1.2,
        };
        self.experiences.push(exp_value);
        self.experiences.last().unwrap()
    }

    /// Create theory
    pub fn create_theory(&mut self, name: &str, basis: &str) -> &ValueTheory {
        let theory = ValueTheory {
            theory_name: name.to_string(),
            basis: basis.to_string(),
        };
        self.value_theories.push(theory);
        self.value_theories.last().unwrap()
    }

    /// Assess meaning
    pub fn assess_meaning(&self, experience_type: &str) -> MeaningResult {
        MeaningResult {
            experience_type: experience_type.to_string(),
            meaningful: true,
            meaning_strength: 0.8,
        }
    }
}

impl Default for ConsciousnessValuation { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Valuation {
    pub valuation_id: String,
    pub experience_type: String,
    pub value: f64,
    pub intensity: f64,
    pub duration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueTheory {
    pub theory_name: String,
    pub basis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceValue {
    pub experience_id: String,
    pub experience: String,
    pub hedonic_value: f64,
    pub meaning_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeaningResult {
    pub experience_type: String,
    pub meaningful: bool,
    pub meaning_strength: f64,
}
