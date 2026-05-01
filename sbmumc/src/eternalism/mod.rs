//! Eternalism Module
//!
//! This module implements eternalism, the view that all times exist,
//! and philosophical implications for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eternalism {
    pub et_id: String,
    pub view: String,
}

impl Eternalism {
    pub fn new() -> Self {
        Self {
            et_id: String::from("eternalism_v1"),
            view: String::from("All moments in time exist equally"),
        }
    }
}

impl Default for Eternalism { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { let e = Eternalism::new(); assert_eq!(e.et_id, "eternalism_v1"); } }
