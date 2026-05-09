//! # SBMUMC Module 1039: Quantum Translation
//!
//! Quantum effects in protein synthesis and translation.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RibosomeState {
    Initiation,
    Elongation,
    Translocation,
    Termination,
    RibosomalPausing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumTranslationState {
    pub translation_id: String,
    pub mRNA_sequence: String,
    pub protein_sequence: String,
    pub ribosome_state: RibosomeState,
    pub elongation_rate_aa_per_sec: f64,
    pub quantum_coherence_speed: f64,
    pub frameshift_probability: f64,
}

impl QuantumTranslationState {
    pub fn new(mRNA: String) -> Self {
        Self {
            translation_id: crate::core::uuid_simple(),
            mRNA_sequence: mRNA,
            protein_sequence: String::new(),
            ribosome_state: RibosomeState::Initiation,
            elongation_rate_aa_per_sec: 0.0,
            quantum_coherence_speed: 0.0,
            frameshift_probability: 0.0,
        }
    }

    pub fn start_translation(&mut self) -> Result<()> {
        self.ribosome_state = RibosomeState::Elongation;
        self.elongation_rate_aa_per_sec = 5.0 + rand_simple() * 5.0;
        self.quantum_coherence_speed = 1.15 + rand_simple() * 0.25;
        self.frameshift_probability = 0.0001 + rand_simple() * 0.0009;
        Ok(())
    }

    pub fn simulate_elongation(&mut self, codon_count: usize) -> Result<String> {
        let mut protein = String::new();
        for i in 0..codon_count {
            if rand_simple() < self.frameshift_probability {
                continue;
            }
            let codon = &self.mRNA_sequence[i * 3..(i + 1) * 3.min(self.mRNA_sequence.len() / 3)];
            let aa = match codon {
                "AUG" => "M",
                "UUU" | "UUC" => "F",
                "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG" => "L",
                _ => "A",
            };
            protein.push_str(aa);
        }
        self.protein_sequence = protein.clone();
        Ok(protein)
    }

    pub fn complete_translation(&mut self) -> Result<()> {
        self.ribosome_state = RibosomeState::Termination;
        self.elongation_rate_aa_per_sec *= self.quantum_coherence_speed;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumTRNAInteraction {
    pub interaction_id: String,
    pub codon: String,
    pub anticodon: String,
    pub binding_affinity: f64,
    pub quantum_codon_recognition: f64,
    pub wobble_base_pairing_efficiency: f64,
}

impl QuantumTRNAInteraction {
    pub fn new(codon: String, anticodon: String) -> Self {
        Self {
            interaction_id: crate::core::uuid_simple(),
            codon,
            anticodon,
            binding_affinity: 0.0,
            quantum_codon_recognition: 0.0,
            wobble_base_pairing_efficiency: 0.0,
        }
    }

    pub fn compute_binding(&mut self) -> Result<()> {
        let perfect_match = self.codon.len() == self.anticodon.len();
        self.binding_affinity = if perfect_match {
            0.95 + rand_simple() * 0.05
        } else {
            0.7 + rand_simple() * 0.25
        };

        self.quantum_codon_recognition = self.binding_affinity * (1.0 + rand_simple() * 0.2);
        self.wobble_base_pairing_efficiency = if !perfect_match {
            0.8 + rand_simple() * 0.2
        } else {
            1.0
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumRibosomePausing {
    pub pausing_id: String,
    pub pause_position: usize,
    pub pause_duration_ms: f64,
    pub quantum_coherence_extension: f64,
    pub frameshift_probability_enhancement: f64,
}

impl QuantumRibosomePausing {
    pub fn new(position: usize) -> Self {
        Self {
            pausing_id: crate::core::uuid_simple(),
            pause_position: position,
            pause_duration_ms: 0.0,
            quantum_coherence_extension: 0.0,
            frameshift_probability_enhancement: 0.0,
        }
    }

    pub fn analyze_pausing(&mut self) -> Result<()> {
        self.pause_duration_ms = 10.0 + rand_simple() * 90.0;
        self.quantum_coherence_extension = 1.2 + rand_simple() * 0.8;
        self.frameshift_probability_enhancement = self.pause_duration_ms / 1000.0;
        Ok(())
    }
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

pub fn compute_translation_efficiency(mRNA: &str) -> Result<f64> {
    let length_factor = (mRNA.len() as f64 / 1000.0).min(3.0);
    Ok((2.0 + rand_simple() * 3.0) / length_factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_initiation() {
        let mut state = QuantumTranslationState::new(
            "AUGGCUUUAUAA".to_string(),
        );
        state.start_translation().unwrap();
        assert!(state.elongation_rate_aa_per_sec > 0.0);
    }

    #[test]
    fn test_elongation() {
        let mut state = QuantumTranslationState::new(
            "AUGGCUUUAUAAUAGUAG".to_string(),
        );
        state.start_translation().unwrap();
        let protein = state.simulate_elongation(4).unwrap();
        assert!(!protein.is_empty());
    }

    #[test]
    fn test_trna_binding() {
        let mut interaction = QuantumTRNAInteraction::new(
            "AUG".to_string(),
            "UAC".to_string(),
        );
        interaction.compute_binding().unwrap();
        assert!(interaction.quantum_codon_recognition > 0.9);
    }

    #[test]
    fn test_ribosome_pausing() {
        let mut pausing = QuantumRibosomePausing::new(100);
        pausing.analyze_pausing().unwrap();
        assert!(pausing.frameshift_probability_enhancement > 0.0);
    }
}