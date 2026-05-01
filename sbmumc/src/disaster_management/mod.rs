//! Disaster Management Module
//!
//! This module implements disaster management, risk reduction,
//! and recovery for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterManagement {
    pub dm_id: String,
    pub phases: Vec<DisasterPhase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterPhase {
    pub phase_name: String,
    pub activities: Vec<String>,
}

impl DisasterManagement {
    pub fn new() -> Self {
        Self {
            dm_id: String::from("disaster_management_v1"),
            phases: vec![
                DisasterPhase { phase_name: String::from("Mitigation"), activities: vec![String::from("Risk assessment")] },
                DisasterPhase { phase_name: String::from("Preparedness"), activities: vec![String::from("Training")] },
                DisasterPhase { phase_name: String::from("Response"), activities: vec![String::from("Emergency services")] },
                DisasterPhase { phase_name: String::from("Recovery"), activities: vec![String::from("Reconstruction")] },
            ],
        }
    }
}

impl Default for DisasterManagement { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { let dm = DisasterManagement::new(); assert_eq!(dm.dm_id, "disaster_management_v1"); } }
