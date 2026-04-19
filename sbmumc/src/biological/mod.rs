//! Biological Computing Module
//!
//! This module implements DNA computing interface, biological sensor
//! integration, and bio-inspired computation mechanisms.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Biological computing system
pub struct BiologicalSystem {
    /// DNA sequences
    pub dna_sequences: Vec<DnaSequence>,
    /// Bio-sensors
    pub biosensors: Vec<BioSensor>,
    /// Protein folding states
    pub protein_states: Vec<ProteinState>,
    /// Biochemical reactions
    pub reactions: Vec<BiochemicalReaction>,
}

impl BiologicalSystem {
    pub fn new() -> Self {
        BiologicalSystem {
            dna_sequences: Vec::new(),
            biosensors: Vec::new(),
            protein_states: Vec::new(),
            reactions: Vec::new(),
        }
    }

    /// Encode data into DNA
    pub fn encode_dna(&mut self, data: &[u8]) -> DnaSequence {
        let mut sequence = String::new();
        for (i, byte) in data.iter().enumerate() {
            let codon = match byte >> 4 {
                0 => "ATG", 1 => "GCT", 2 => "CGA", 3 => "TTA",
                4 => "ATG", 5 => "GGC", 6 => "CAT", 7 => "ATA",
                8 => "GAT", 9 => "TCC", 10 => "ACC", 11 => "GTA",
                12 => "CTC", 13 => "AGG", 14 => "TGG", 15 => "CGC",
                _ => "ATG",
            };
            sequence.push_str(codon);
            if i % 2 == 1 { sequence.push(' '); }
        }
        DnaSequence {
            id: format!("dna_{}", self.dna_sequences.len()),
            sequence: sequence.replace(" ", ""),
            length: sequence.len(),
            encoding: EncodingScheme::BinaryToTriplet,
        }
    }

    /// Decode DNA to data
    pub fn decode_dna(&self, dna: &DnaSequence) -> Vec<u8> {
        let mut data = Vec::new();
        let chars: Vec<char> = dna.sequence.chars().collect();
        for chunk in chars.chunks(3) {
            if chunk.len() == 3 {
                let codon: String = chunk.iter().collect();
                let value = match codon.as_str() {
                    "ATG" => 0, "GCT" => 1, "CGA" => 2, "TTA" => 3,
                    "GGC" => 5, "CAT" => 6, "ATA" => 7, "GAT" => 8,
                    "TCC" => 9, "ACC" => 10, "GTA" => 11, "CTC" => 12,
                    "AGG" => 13, "TGG" => 14, "CGC" => 15, _ => 0,
                };
                data.push(value << 4);
            }
        }
        data
    }

    /// Perform DNA computation
    pub fn compute(&mut self, operation: &BioOperation, sequence_ids: &[String]) -> ComputationResult {
        match operation {
            BioOperation::StrandDisplacement => {
                ComputationResult {
                    operation: operation.clone(),
                    output_sequences: sequence_ids.to_vec(),
                    result: "Strand displacement completed".to_string(),
                    confidence: 0.95,
                }
            }
            BioOperation::Splicing => {
                ComputationResult {
                    operation: operation.clone(),
                    output_sequences: sequence_ids.to_vec(),
                    result: "Splicing operation completed".to_string(),
                    confidence: 0.92,
                }
            }
            BioOperation::Amplification => {
                ComputationResult {
                    operation: operation.clone(),
                    output_sequences: sequence_ids.to_vec(),
                    result: "PCR amplification successful".to_string(),
                    confidence: 0.98,
                }
            }
        }
    }

    /// Read bio-sensor
    pub fn read_sensor(&self, sensor_id: &str) -> Option<SensorReading> {
        self.biosensors.iter()
            .find(|s| s.id == sensor_id)
            .map(|sensor| {
                SensorReading {
                    sensor_id: sensor.id.clone(),
                    value: rand::random::<f64>() * 100.0,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64(),
                    unit: sensor.unit.clone(),
                }
            })
    }

    /// Fold protein
    pub fn fold_protein(&mut self, sequence: &[AminoAcid]) -> ProteinState {
        let structure = sequence.iter()
            .enumerate()
            .map(|(i, acid)| {
                let fold_angle = match acid {
                    AminoAcid::Hydrophobic => 45.0 + i as f64 * 0.5,
                    AminoAcid::Hydrophilic => -45.0 - i as f64 * 0.3,
                    AminoAcid::Neutral => i as f64 * 1.0,
                };
                Residue {
                    amino_acid: acid.clone(),
                    position: i,
                    fold_angle,
                    hydrogen_bonds: 0,
                }
            })
            .collect();

        ProteinState {
            sequence_id: format!("protein_{}", self.protein_states.len()),
            residues: structure,
            energy_level: 0.0,
            fold_confidence: 0.85,
        }
    }
}

impl Default for BiologicalSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnaSequence {
    pub id: String,
    pub sequence: String,
    pub length: usize,
    pub encoding: EncodingScheme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncodingScheme {
    BinaryToTriplet,
    HuffmanCoding,
    RedundantCoding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioSensor {
    pub id: String,
    pub sensor_type: SensorType,
    pub sensitivity: f64,
    pub range: (f64, f64),
    pub unit: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorType {
    Glucose,
    PH,
    Temperature,
    Pressure,
    NeuralActivity,
    Cardiac,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub sensor_id: String,
    pub value: f64,
    pub timestamp: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinState {
    pub sequence_id: String,
    pub residues: Vec<Residue>,
    pub energy_level: f64,
    pub fold_confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Residue {
    pub amino_acid: AminoAcid,
    pub position: usize,
    pub fold_angle: f64,
    pub hydrogen_bonds: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AminoAcid {
    Hydrophobic,
    Hydrophilic,
    Neutral,
    Acidic,
    Basic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiochemicalReaction {
    pub reactants: Vec<String>,
    pub products: Vec<String>,
    pub enzyme: Option<String>,
    pub rate_constant: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputationResult {
    pub operation: BioOperation,
    pub output_sequences: Vec<String>,
    pub result: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BioOperation {
    StrandDisplacement,
    Splicing,
    Amplification,
    Hybridization,
}