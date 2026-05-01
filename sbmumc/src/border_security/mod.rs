//! Border Security Module
//!
//! This module implements border security, immigration control,
//! and customs for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderSecurity {
    pub bs_id: String,
    pub checkpoints: Vec<Checkpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub checkpoint_name: String,
    pub location: String,
}

impl BorderSecurity {
    pub fn new() -> Self {
        Self {
            bs_id: String::from("border_security_v1"),
            checkpoints: vec![],
        }
    }
}

impl Default for BorderSecurity { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { let bs = BorderSecurity::new(); assert_eq!(bs.bs_id, "border_security_v1"); } }
