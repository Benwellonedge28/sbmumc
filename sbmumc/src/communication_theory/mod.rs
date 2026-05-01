//! Communication Theory Module
//!
//! This module implements communication theory, interpersonal communication,
//! and mass communication for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Communication theory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationTheory {
    pub ct_id: String,
    pub models: Vec<CommunicationModel>,
    pub contexts: Vec<CommunicationContext>,
    pub barriers: Vec<CommunicationBarrier>,
    pub technology: CommunicationTechnology,
}

/// Communication model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationModel {
    pub model_id: String,
    pub model_name: String,
    pub theorist: String,
    pub components: Vec<ModelComponent>,
    pub assumptions: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelComponent {
    pub component_name: String,
    pub description: String,
    pub role: String,
}

/// Communication context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationContext {
    pub context_name: String,
    pub context_type: ContextType,
    pub characteristics: Vec<String>,
    pub effective_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContextType {
    Interpersonal,
    Group,
    Organizational,
    Public,
    Mass,
    Intercultural,
}

/// Communication barrier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationBarrier {
    pub barrier_name: String,
    pub barrier_type: BarrierType,
    pub causes: Vec<String>,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BarrierType {
    Physical,
    Semantic,
    Psychological,
    Cultural,
    Organizational,
}

/// Communication technology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationTechnology {
    pub technologies: Vec<CommTechnology>,
    pub impact_on_communication: Vec<ImpactAssessment>,
    pub emerging_trends: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommTechnology {
    pub tech_name: String,
    pub tech_type: TechType,
    pub capabilities: Vec<String>,
    pub adoption_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TechType {
    Synchronous,
    Asynchronous,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub aspect: String,
    pub impact_type: ImpactType,
    pub magnitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImpactType {
    Positive,
    Negative,
    Mixed,
}

impl CommunicationTheory {
    pub fn new() -> Self {
        Self {
            ct_id: String::from("communication_theory_v1"),
            models: vec![
                CommunicationModel { model_id: String::from("model_shannon"), model_name: String::from("Shannon-Weaver"), theorist: String::from("Claude Shannon"), components: vec![ModelComponent { component_name: String::from("Encoder"), description: String::from("Transmits message"), role: String::from("Sender") }], assumptions: vec![String::from("Linear process")], limitations: vec![String::from("Ignores context")] },
            ],
            contexts: vec![
                CommunicationContext { context_name: String::from("Workplace"), context_type: ContextType::Organizational, characteristics: vec![String::from("Goal-oriented")], effective_strategies: vec![String::from("Clear messaging")] },
            ],
            barriers: vec![
                CommunicationBarrier { barrier_name: String::from("Language differences"), barrier_type: BarrierType::Cultural, causes: vec![String::from("Different native languages")], mitigation_strategies: vec![String::from("Use interpreters")] },
            ],
            technology: CommunicationTechnology { technologies: vec![CommTechnology { tech_name: String::from("Video conferencing"), tech_type: TechType::Synchronous, capabilities: vec![String::from("Real-time face-to-face")], adoption_rate: 0.8 }], impact_on_communication: vec![], emerging_trends: vec![String::from("AI assistants")] },
        }
    }

    pub fn analyze_communication_effectiveness(&self, context: &str) -> EffectivenessAnalysis {
        EffectivenessAnalysis { context: context.to_string(), clarity_score: 7.5, engagement_score: 7.0, barriers_present: vec![], recommendations: vec![String::from("Improve active listening")] }
    }

    pub fn diagnose_barriers(&self, interaction: &str) -> BarrierDiagnosis {
        BarrierDiagnosis { interaction_id: interaction.to_string(), identified_barriers: vec![String::from("Noise")], severity_levels: HashMap::new(), solutions: vec![String::from("Reduce physical distance")] }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectivenessAnalysis {
    pub context: String,
    pub clarity_score: f64,
    pub engagement_score: f64,
    pub barriers_present: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarrierDiagnosis {
    pub interaction_id: String,
    pub identified_barriers: Vec<String>,
    pub severity_levels: HashMap<String, f64>,
    pub solutions: Vec<String>,
}

impl Default for CommunicationTheory { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { letct = CommunicationTheory::new(); assert_eq!(ct.ct_id, "communication_theory_v1"); } }
