//! Information Theory Module
//!
//! This module implements information theory, entropy,
//! and data compression for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Information theory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformationTheory {
    pub it_id: String,
    pub entropy: EntropyMeasures,
    pub coding: CodingTheory,
    pub channels: ChannelCapacity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyMeasures {
    pub entropy_definitions: Vec<EntropyDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyDefinition {
    pub entropy_name: String,
    pub formula: String,
    pub interpretation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodingTheory {
    pub source_coding: SourceCoding,
    pub channel_coding: ChannelCoding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceCoding {
    pub coding_schemes: Vec<CodingScheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodingScheme {
    pub scheme_name: String,
    pub efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelCoding {
    pub error_correction: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelCapacity {
    pub capacity_formula: String,
    pub channel_types: Vec<String>,
}

impl InformationTheory {
    pub fn new() -> Self {
        Self {
            it_id: String::from("information_theory_v1"),
            entropy: EntropyMeasures {
                entropy_definitions: vec![
                    EntropyDefinition { entropy_name: String::from("Shannon entropy"), formula: String::from("H(X) = -sum p(x) log p(x)"), interpretation: String::from("Bits of information") },
                ],
            },
            coding: CodingTheory {
                source_coding: SourceCoding { coding_schemes: vec![CodingScheme { scheme_name: String::from("Huffman coding"), efficiency: 0.95 }] },
                channel_coding: ChannelCoding { error_correction: vec![String::from("Hamming codes")] },
            },
            channels: ChannelCapacity { capacity_formula: String::from("C = B log2(1 + S/N)"), channel_types: vec![String::from("AWGN")] },
        }
    }

    pub fn calculate_entropy(&self, probabilities: &[f64]) -> f64 {
        probabilities.iter().map(|p| if *p > 0.0 { -p * p.log2() } else { 0.0 }).sum()
    }
}

impl Default for InformationTheory { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { let it = InformationTheory::new(); assert_eq!(it.it_id, "information_theory_v1"); } }
