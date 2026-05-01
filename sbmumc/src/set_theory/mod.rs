//! Set Theory Module
//!
//! This module implements set theory, set operations,
//! and cardinality for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Set theory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetTheory {
    pub st_id: String,
    pub foundations: SetFoundations,
    pub operations: SetOperations,
    pub cardinality: CardinalityTheory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetFoundations {
    pub axioms: Vec<SetAxiom>,
    pub primitive_notions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetAxiom {
    pub axiom_name: String,
    pub statement: String,
    pub significance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetOperations {
    pub basic_operations: Vec<BasicOperation>,
    pub derived_operations: Vec<DerivedOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicOperation {
    pub operation_name: String,
    pub symbol: String,
    pub definition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedOperation {
    pub operation_name: String,
    pub definition: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardinalityTheory {
    pub cardinal_numbers: Vec<CardinalNumber>,
    pub comparisons: Vec<CardinalityComparison>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardinalNumber {
    pub cardinal_name: String,
    pub symbol: String,
    pub definition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardinalityComparison {
    pub set1: String,
    pub set2: String,
    pub comparison: String,
}

impl SetTheory {
    pub fn new() -> Self {
        Self {
            st_id: String::from("set_theory_v1"),
            foundations: SetFoundations {
                axioms: vec![
                    SetAxiom { axiom_name: String::from("Extensionality"), statement: String::from("Sets equal if same elements"), significance: String::from("Fundamental equality") },
                ],
                primitive_notions: vec![String::from("Set"), String::from("Membership")],
            },
            operations: SetOperations {
                basic_operations: vec![
                    BasicOperation { operation_name: String::from("Union"), symbol: String::from("∪"), definition: String::from("All elements in A or B") },
                ],
                derived_operations: vec![],
            },
            cardinality: CardinalityTheory {
                cardinal_numbers: vec![
                    CardinalNumber { cardinal_name: String::from("Aleph null"), symbol: String::from("ℵ₀"), definition: String::from("Countably infinite") },
                ],
                comparisons: vec![],
            },
        }
    }

    pub fn union<T: Clone>(&self, a: &[T], b: &[T]) -> Vec<T> {
        let mut result = a.to_vec();
        for element in b {
            if !result.contains(element) {
                result.push(element.clone());
            }
        }
        result
    }

    pub fn intersection<T: Eq + Clone>(&self, a: &[T], b: &[T]) -> Vec<T> {
        a.iter().filter(|x| b.contains(x)).cloned().collect()
    }
}

impl Default for SetTheory { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { let st = SetTheory::new(); assert_eq!(st.st_id, "set_theory_v1"); } }
