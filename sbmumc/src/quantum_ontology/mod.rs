//! Quantum Ontology Module
//!
//! This module implements quantum ontology, wave function realism,
//! many-worlds interface, and quantum interpretations.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct QuantumOntology {
    pub interpretations: Vec<Interpretation>,
    pub wavefunctions: Vec<Wavefunction>,
}

impl QuantumOntology {
    pub fn new() -> Self {
        QuantumOntology {
            interpretations: vec![
                Interpretation { name: "Copenhagen".to_string(), collapse: true },
                Interpretation { name: "ManyWorlds".to_string(), collapse: false },
                Interpretation { name: "PilotWave".to_string(), collapse: false },
            ],
            wavefunctions: Vec::new(),
        }
    }

    /// Create wavefunction
    pub fn create_wavefunction(&mut self, qubits: usize) -> &Wavefunction {
        let size = 2usize.pow(qubits as u32);
        let amplitudes: Vec<(f64, f64)> = (0..size)
            .map(|_| (rand::random::<f64>(), rand::random::<f64>()))
            .collect();

        let wf = Wavefunction {
            wavefunction_id: format!("wf_{}", self.wavefunctions.len()),
            qubits,
            amplitudes,
            normalized: false,
        };
        self.wavefunctions.push(wf);
        self.wavefunctions.last().unwrap()
    }

    /// Normalize wavefunction
    pub fn normalize(&mut self, wf_id: &str) -> Result<()> {
        if let Some(wf) = self.wavefunctions.iter_mut().find(|w| w.wavefunction_id == wf_id) {
            let norm = wf.amplitudes.iter()
                .map(|(r, i)| r * r + i * i)
                .sum::<f64>()
                .sqrt();

            for (r, i) in wf.amplitudes.iter_mut() {
                *r /= norm;
                *i /= norm;
            }
            wf.normalized = true;
            Ok(())
        } else {
            Err(SbmumcError::NotFound(format!("Wavefunction {} not found", wf_id)))
        }
    }

    /// Measure in basis
    pub fn measure(&self, wf_id: &str, basis: &[usize]) -> MeasurementOutcome {
        if let Some(wf) = self.wavefunctions.iter().find(|w| w.wavefunction_id == wf_id) {
            let outcome = basis.iter().sum::<usize>() % wf.amplitudes.len();
            MeasurementOutcome {
                wavefunction_id: wf_id.to_string(),
                outcome,
                probability: 0.1 + rand::random::<f64>() * 0.2,
                post_measurement_state: format!("wf_collapsed_{}", outcome),
            }
        } else {
            MeasurementOutcome {
                wavefunction_id: wf_id.to_string(),
                outcome: 0,
                probability: 0.0,
                post_measurement_state: "error".to_string(),
            }
        }
    }

    /// Collapse wavefunction (Copenhagen)
    pub fn collapse(&mut self, wf_id: &str) -> Result<()> {
        if let Some(wf) = self.wavefunctions.iter_mut().find(|w| w.wavefunction_id == wf_id) {
            let idx = (rand::random::<f64>() * wf.amplitudes.len() as f64) as usize;
            for i in 0..wf.amplitudes.len() {
                wf.amplitudes[i] = if i == idx { (1.0, 0.0) } else { (0.0, 0.0) };
            }
            Ok(())
        } else {
            Err(SbmumcError::NotFound(format!("Wavefunction {} not found", wf_id)))
        }
    }

    /// Branch (Many-Worlds)
    pub fn branch(&mut self, wf_id: &str, num_branches: usize) -> Vec<String> {
        let branches: Vec<String> = (0..num_branches)
            .map(|i| format!("branch_{}_{}", wf_id, i))
            .collect();
        branches
    }

    /// Pilot wave position
    pub fn pilot_wave_position(&self, wf_id: &str) -> Vec<f64> {
        vec![0.5; 2usize.pow(self.wavefunctions.iter().find(|w| w.wavefunction_id == wf_id).map(|w| w.qubits).unwrap_or(1) as u32)]
    }

    /// Get interpretation
    pub fn get_interpretation(&self, name: &str) -> Option<&Interpretation> {
        self.interpretations.iter().find(|i| i.name == name)
    }
}

impl Default for QuantumOntology { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interpretation {
    pub name: String,
    pub collapse: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wavefunction {
    pub wavefunction_id: String,
    pub qubits: usize,
    pub amplitudes: Vec<(f64, f64)>,
    pub normalized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementOutcome {
    pub wavefunction_id: String,
    pub outcome: usize,
    pub probability: f64,
    pub post_measurement_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub branch_id: String,
    pub amplitude: f64,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiddenVariable {
    pub variable_id: String,
    pub value: f64,
    pub trajectory: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyResult {
    pub interpretation: String,
    pub compatible: bool,
    pub predictions: Vec<String>,
}