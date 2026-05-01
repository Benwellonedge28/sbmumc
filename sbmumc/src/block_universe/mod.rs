//! Block Universe Module
//!
//! This module implements the block universe theory,
//! and eternalism for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockUniverse {
    pub bu_id: String,
    pub description: String,
}

impl BlockUniverse {
    pub fn new() -> Self {
        Self {
            bu_id: String::from("block_universe_v1"),
            description: String::from("Past, present, and future all exist in spacetime"),
        }
    }
}

impl Default for BlockUniverse { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { let bu = BlockUniverse::new(); assert_eq!(bu.bu_id, "block_universe_v1"); } }
