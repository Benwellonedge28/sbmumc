//! Agency Consciousness Module
//!
//! This module implements sense of agency, volition, free will models,
//! and conscious control of actions and decisions.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct AgencyConsciousness {
    pub volitions: Vec<Volition>,
    pub intentions: Vec<Intention>,
    pub agency_assessments: Vec<AgencyAssessment>,
    pub control_signals: Vec<ControlSignal>,
}

impl AgencyConsciousness {
    pub fn new() -> Self {
        AgencyConsciousness {
            volitions: Vec::new(),
            intentions: Vec::new(),
            agency_assessments: Vec::new(),
            control_signals: Vec::new(),
        }
    }

    /// Generate volition
    pub fn generate_volition(&mut self, desire: &str) -> &Volition {
        let volition = Volition {
            volition_id: format!("vol_{}", self.voliions.len()),
            desire: desire.to_string(),
            strength: 0.8,
            conscious_origin: true,
        };
        self.voliions.push(volition);
        self.voliions.last().unwrap()
    }

    /// Form intention
    pub fn form_intention(&mut self, goal: &str, plan: &[String]) -> &Intention {
        let intention = Intention {
            intention_id: format!("int_{}", self.intentions.len()),
            goal: goal.to_string(),
            plan: plan.to_vec(),
            commitment: 0.9,
            formed_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        };
        self.intentions.push(intention);
        self.intentions.last().unwrap()
    }

    /// Assess agency
    pub fn assess_agency(&mut self, action: &str) -> &AgencyAssessment {
        let assessment = AgencyAssessment {
            action: action.to_string(),
            sense_of_agency: 0.85,
            authorship: 0.8,
            control_level: "High".to_string(),
        };
        self.agency_assessments.push(assessment);
        self.agency_assessments.last().unwrap()
    }

    /// Generate control signal
    pub fn generate_control_signal(&mut self, intention_id: &str, motor_command: &str) -> &ControlSignal {
        let signal = ControlSignal {
            signal_id: format!("ctrl_{}", self.control_signals.len()),
            intention_id: intention_id.to_string(),
            motor_command: motor_command.to_string(),
            timing_ms: 150.0,
        };
        self.control_signals.push(signal);
        self.control_signals.last().unwrap()
    }

    /// Check sense of agency
    pub fn check_sense_of_agency(&self, action: &str) -> SenseOfAgencyResult {
        SenseOfAgencyResult {
            action: action.to_string(),
            agency_present: true,
            misattribution_risk: 0.1,
        }
    }
}

impl Default for AgencyConsciousness { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volition {
    pub volition_id: String,
    pub desire: String,
    pub strength: f64,
    pub conscious_origin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intention {
    pub intention_id: String,
    pub goal: String,
    pub plan: Vec<String>,
    pub commitment: f64,
    pub formed_at: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgencyAssessment {
    pub action: String,
    pub sense_of_agency: f64,
    pub authorship: f64,
    pub control_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlSignal {
    pub signal_id: String,
    pub intention_id: String,
    pub motor_command: String,
    pub timing_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenseOfAgencyResult {
    pub action: String,
    pub agency_present: bool,
    pub misattribution_risk: f64,
}
