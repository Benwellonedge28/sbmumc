//! Threat Analysis Module
//!
//! This module implements threat analysis, vulnerability assessment,
//! and threat modeling for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysis {
    pub ta_id: String,
    pub methodologies: Vec<ThreatMethodology>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatMethodology {
    pub methodology_name: String,
    pub description: String,
}

impl ThreatAnalysis {
    pub fn new() -> Self {
        Self {
            ta_id: String::from("threat_analysis_v1"),
            methodologies: vec![
                ThreatMethodology { methodology_name: String::from("STRIDE"), description: String::from("Spoofing, Tampering, Repudiation, Information Disclosure, DoS, Elevation of Privilege") },
            ],
        }
    }
}

impl Default for ThreatAnalysis { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { let ta = ThreatAnalysis::new(); assert_eq!(ta.ta_id, "threat_analysis_v1"); } }
