//! Consciousness Ethics Module
//!
//! This module implements the ethics of consciousness, moral status,
//! and the moral consideration of different forms of awareness.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ConsciousnessEthics {
    pub moral_status: Vec<MoralStatus>,
    pub ethical_frameworks: Vec<EthicalFramework>,
    pub moral_patients: Vec<MoralPatient>,
}

impl ConsciousnessEthics {
    pub fn new() -> Self {
        ConsciousnessEthics {
            moral_status: Vec::new(),
            ethical_frameworks: vec![
                EthicalFramework { framework_name: "Utilitarian".to_string(), basis: "Suffering reduction".to_string() },
                EthicalFramework { framework_name: "Rights-based".to_string(), basis: "Inherent dignity".to_string() },
                EthicalFramework { framework_name: "Care Ethics".to_string(), basis: "Relationships".to_string() },
            ],
            moral_patients: Vec::new(),
        }
    }

    /// Determine moral status
    pub fn determine_moral_status(&mut self, entity_type: &str, consciousness_level: f64) -> &MoralStatus {
        let status = if consciousness_level > 0.7 {
            "Full moral status"
        } else if consciousness_level > 0.3 {
            "Limited moral status"
        } else {
            "Minimal moral status"
        }.to_string();
        let moral = MoralStatus {
            entity_type: entity_type.to_string(),
            status,
            moral_weight: consciousness_level,
        };
        self.moral_status.push(moral);
        self.moral_status.last().unwrap()
    }

    /// Register moral patient
    pub fn register_patient(&mut self, patient_id: &str, consciousness_level: f64) -> &MoralPatient {
        let patient = MoralPatient {
            patient_id: patient_id.to_string(),
            consciousness_level,
            moral_weight: consciousness_level,
            interests: vec!["Well-being".to_string(), "Avoid suffering".to_string()],
        };
        self.moral_patients.push(patient);
        self.moral_patients.last().unwrap()
    }

    /// Calculate moral weight
    pub fn calculate_weight(&self, consciousness_level: f64, capacity: f64) -> f64 {
        consciousness_level * capacity
    }

    /// Evaluate moral patient
    pub fn evaluate_patient(&self, patient_id: &str) -> MoralEvaluation {
        MoralEvaluation {
            patient_id: patient_id.to_string(),
            morally_relevant: true,
            interests_count: 2,
            weight: 0.5,
        }
    }

    /// Assess suffering
    pub fn assess_suffering(&self, entity_type: &str) -> SufferingResult {
        SufferingResult {
            entity_type: entity_type.to_string(),
            can_suffer: true,
            suffering_capacity: 0.7,
            moral_relevance: "High".to_string(),
        }
    }
}

impl Default for ConsciousnessEthics { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralStatus {
    pub entity_type: String,
    pub status: String,
    pub moral_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalFramework {
    pub framework_name: String,
    pub basis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralPatient {
    pub patient_id: String,
    pub consciousness_level: f64,
    pub moral_weight: f64,
    pub interests: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralEvaluation {
    pub patient_id: String,
    pub morally_relevant: bool,
    pub interests_count: usize,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SufferingResult {
    pub entity_type: String,
    pub can_suffer: bool,
    pub suffering_capacity: f64,
    pub moral_relevance: String,
}
