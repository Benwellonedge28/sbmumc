//! Intelligence Analysis Module
//!
//! This module implements intelligence analysis, threat assessment,
//! and intelligence gathering for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Intelligence analysis system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceAnalysis {
    pub ia_id: String,
    pub sources: Vec<IntelligenceSource>,
    pub assessments: Vec<ThreatAssessment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceSource {
    pub source_name: String,
    pub source_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAssessment {
    pub threat_name: String,
    pub confidence: f64,
}

impl IntelligenceAnalysis {
    pub fn new() -> Self {
        Self {
            ia_id: String::from("intelligence_analysis_v1"),
            sources: vec![
                IntelligenceSource { source_name: String::from("HUMINT"), source_type: String::from("Human") },
            ],
            assessments: vec![],
        }
    }
}

impl Default for IntelligenceAnalysis { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { let ia = IntelligenceAnalysis::new(); assert_eq!(ia.ia_id, "intelligence_analysis_v1"); } }
