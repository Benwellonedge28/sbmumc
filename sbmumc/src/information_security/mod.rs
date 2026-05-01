//! Information Security Module
//!
//! This module implements information security, data protection,
//! and privacy for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Information security system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformationSecurity {
    pub is_id: String,
    pub classification: DataClassification,
    pub controls: Vec<SecurityControl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataClassification {
    pub levels: Vec<ClassificationLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationLevel {
    pub level_name: String,
    pub sensitivity: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityControl {
    pub control_name: String,
    pub control_type: ControlType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ControlType {
    Administrative,
    Technical,
    Physical,
}

impl InformationSecurity {
    pub fn new() -> Self {
        Self {
            is_id: String::from("information_security_v1"),
            classification: DataClassification { levels: vec![
                ClassificationLevel { level_name: String::from("Public"), sensitivity: 1 },
                ClassificationLevel { level_name: String::from("Confidential"), sensitivity: 5 },
            ]},
            controls: vec![
                SecurityControl { control_name: String::from("Access control"), control_type: ControlType::Technical },
            ],
        }
    }
}

impl Default for InformationSecurity { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { let is = InformationSecurity::new(); assert_eq!(is.is_id, "information_security_v1"); } }
