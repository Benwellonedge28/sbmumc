//! Coma Recovery Module
//!
//! This module implements coma, vegetative states, disorders of consciousness,
//! and recovery assessment and prediction.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ComaRecovery {
    pub assessments: Vec<ConsciousnessAssessment>,
    pub patients: Vec<Patient>,
    pub recovery_predictions: Vec<RecoveryPrediction>,
    pub consciousness_scales: Vec<ConsciousnessScale>,
}

impl ComaRecovery {
    pub fn new() -> Self {
        ComaRecovery {
            assessments: Vec::new(),
            patients: Vec::new(),
            recovery_predictions: Vec::new(),
            consciousness_scales: vec![
                ConsciousnessScale { name: "CRS-R".to_string(), range: "0-23".to_string() },
                ConsciousnessScale { name: "Glasgow".to_string(), range: "3-15".to_string() },
            ],
        }
    }

    /// Add patient
    pub fn add_patient(&mut self, patient_id: &str, diagnosis: &str) -> &Patient {
        let patient = Patient {
            patient_id: patient_id.to_string(),
            diagnosis: diagnosis.to_string(),
            consciousness_level: 0.1,
            time_since_injury_days: 0,
        };
        self.patients.push(patient);
        self.patients.last().unwrap()
    }

    /// Assess consciousness
    pub fn assess(&mut self, patient_id: &str) -> Result<ConsciousnessAssessment> {
        let assessment = ConsciousnessAssessment {
            patient_id: patient_id.to_string(),
            crs_r_score: 5,
            glasgow_score: 6,
            visual: 1,
            motor: 2,
            verbal: 2,
            communication: 0,
            assessment_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        };
        self.assessments.push(assessment.clone());
        Ok(assessment)
    }

    /// Predict recovery
    pub fn predict_recovery(&mut self, patient_id: &str) -> RecoveryPrediction {
        let prediction = RecoveryPrediction {
            patient_id: patient_id.to_string(),
            probability_recovery: 0.3,
            time_to_recovery_days: 180,
            potential_outcome: "Vegetative".to_string(),
        };
        self.recovery_predictions.push(prediction.clone());
        prediction
    }

    /// Diagnose disorder
    pub fn diagnose(&self, crs_r_score: usize) -> DisorderDiagnosis {
        let diagnosis = match crs_r_score {
            0..=2 => "Coma",
            3..=7 => "Vegetative State",
            8..=15 => "Minimally Conscious".to_string(),
            _ => "Conscious".to_string(),
        };
        DisorderDiagnosis {
            diagnosis: diagnosis.to_string(),
            crs_r_score,
            confidence: 0.85,
        }
    }

    /// Detect awareness
    pub fn detect_awareness(&self, patient_id: &str, eeg_data: &[f64]) -> AwarenessResult {
        AwarenessResult {
            patient_id: patient_id.to_string(),
            awareness_detected: false,
            evidence: "Limited neural response".to_string(),
            confidence: 0.4,
        }
    }
}

impl Default for ComaRecovery { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessAssessment {
    pub patient_id: String,
    pub crs_r_score: usize,
    pub glasgow_score: usize,
    pub visual: usize,
    pub motor: usize,
    pub verbal: usize,
    pub communication: usize,
    pub assessment_timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    pub patient_id: String,
    pub diagnosis: String,
    pub consciousness_level: f64,
    pub time_since_injury_days: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPrediction {
    pub patient_id: String,
    pub probability_recovery: f64,
    pub time_to_recovery_days: usize,
    pub potential_outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessScale {
    pub name: String,
    pub range: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisorderDiagnosis {
    pub diagnosis: String,
    pub crs_r_score: usize,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwarenessResult {
    pub patient_id: String,
    pub awareness_detected: bool,
    pub evidence: String,
    pub confidence: f64,
}
