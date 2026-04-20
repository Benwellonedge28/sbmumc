//! Biohazard Management Module
//!
//! This module implements biosafety, biohazard containment,
//! pathogen control, and biological risk management.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct BiohazardManagement {
    pub containment_levels: Vec<ContainmentLevel>,
    pub biohazards: Vec<Biohazard>,
    pub protocols: Vec<BiohazardProtocol>,
}

impl BiohazardManagement {
    pub fn new() -> Self {
        BiohazardManagement {
            containment_levels: vec![
                ContainmentLevel { level: 1, description: "Low risk".to_string() },
                ContainmentLevel { level: 2, description: "Moderate risk".to_string() },
                ContainmentLevel { level: 3, description: "High risk".to_string() },
                ContainmentLevel { level: 4, description: "Highest risk".to_string() },
            ],
            biohazards: Vec::new(),
            protocols: Vec::new(),
        }
    }

    /// Classify biohazard
    pub fn classify(&mut self, agent: &str, risk_level: usize) -> &Biohazard {
        let biohazard = Biohazard {
            biohazard_id: format!("bh_{}", self.biohazards.len()),
            agent: agent.to_string(),
            risk_level,
            required_containment: risk_level,
        };
        self.biohazards.push(biohazard);
        self.biohazards.last().unwrap()
    }

    /// Create protocol
    pub fn create_protocol(&mut self, biohazard_id: &str) -> &BiohazardProtocol {
        let protocol = BiohazardProtocol {
            protocol_id: format!("proto_{}", self.protocols.len()),
            biohazard_id: biohazard_id.to_string(),
            containment_level: 3,
            procedures: vec!["Wear PPE".to_string(), "Negative pressure".to_string()],
        };
        self.protocols.push(protocol);
        self.protocols.last().unwrap()
    }

    /// Assess containment adequacy
    pub fn assess_containment(&self, level: usize) -> ContainmentAssessment {
        ContainmentAssessment {
            level,
            adequate: level >= 3,
            recommendations: if level < 3 { vec!["Increase containment".to_string()] } else { vec![] },
        }
    }
}

impl Default for BiohazardManagement { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainmentLevel {
    pub level: usize,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biohazard {
    pub biohazard_id: String,
    pub agent: String,
    pub risk_level: usize,
    pub required_containment: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiohazardProtocol {
    pub protocol_id: String,
    pub biohazard_id: String,
    pub containment_level: usize,
    pub procedures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainmentAssessment {
    pub level: usize,
    pub adequate: bool,
    pub recommendations: Vec<String>,
}
