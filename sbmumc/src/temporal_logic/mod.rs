//! Temporal Logic Module
//!
//! This module implements temporal logic, time operators,
//! and logical systems for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalLogic {
    pub tl_id: String,
    pub operators: Vec<TemporalOperator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalOperator {
    pub operator_symbol: String,
    pub name: String,
    pub description: String,
}

impl TemporalLogic {
    pub fn new() -> Self {
        Self {
            tl_id: String::from("temporal_logic_v1"),
            operators: vec![
                TemporalOperator { operator_symbol: String::from("F"), name: String::from("Finally"), description: String::from("Will be true eventually") },
                TemporalOperator { operator_symbol: String::from("G"), name: String::from("Globally"), description: String::from("Always true") },
            ],
        }
    }
}

impl Default for TemporalLogic { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { let tl = TemporalLogic::new(); assert_eq!(tl.tl_id, "temporal_logic_v1"); } }
