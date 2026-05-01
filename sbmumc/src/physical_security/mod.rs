//! Physical Security Module
//!
//! This module implements physical security, facility protection,
//! and access control for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Physical security system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalSecurity {
    pub ps_id: String,
    pub access_control: AccessControl,
    pub surveillance: SurveillanceSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControl {
    pub mechanisms: Vec<AccessMechanism>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessMechanism {
    pub mechanism_name: String,
    pub mechanism_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurveillanceSystem {
    pub cameras: u32,
    pub coverage: f64,
}

impl PhysicalSecurity {
    pub fn new() -> Self {
        Self {
            ps_id: String::from("physical_security_v1"),
            access_control: AccessControl { mechanisms: vec![
                AccessMechanism { mechanism_name: String::from("Biometric"), mechanism_type: String::from("Fingerprint") },
            ]},
            surveillance: SurveillanceSystem { cameras: 50, coverage: 0.95 },
        }
    }
}

impl Default for PhysicalSecurity { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { let ps = PhysicalSecurity::new(); assert_eq!(ps.ps_id, "physical_security_v1"); } }
