//! # SBMUMC Module 1022: Quantum Medicine
//!
//! Quantum technologies in medical applications and healthcare.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuantumTherapyType {
    QuantumDotDrugDelivery,
    QubitBasedImaging,
    QuantumEntanglementDiagnostics,
    QuantumCoherenceTherapy,
    QuantumTunnelingTreatment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMedicalDevice {
    pub device_id: String,
    pub device_type: QuantumTherapyType,
    pub patient_id: String,
    pub treatment_parameters: QuantumTreatmentParameters,
    pub quantum_coherence_duration_ns: f64,
    pub safety_rating: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumTreatmentParameters {
    pub frequency_hz: f64,
    pub intensity_tesla: f64,
    pub duration_minutes: f64,
    pub target_tissue_type: String,
    pub quantum_state_purity: f64,
}

impl QuantumTreatmentParameters {
    pub fn new(frequency: f64, intensity: f64, duration: f64) -> Self {
        Self {
            frequency_hz: frequency,
            intensity_tesla: intensity,
            duration_minutes: duration,
            target_tissue_type: String::new(),
            quantum_state_purity: 0.99,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.frequency_hz <= 0.0 {
            return Err(SbmumcError::InvalidInput("Frequency must be positive".to_string()));
        }
        if self.intensity_tesla > 10.0 {
            return Err(SbmumcError::InvalidInput("Intensity exceeds safety limits".to_string()));
        }
        if self.duration_minutes <= 0.0 {
            return Err(SbmumcError::InvalidInput("Duration must be positive".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDiagnosisResult {
    pub diagnosis_id: String,
    pub patient_id: String,
    pub condition_detected: String,
    pub confidence_score: f64,
    pub quantum_measurement_type: String,
    pub imaging_resolution_angstrom: f64,
    pub false_positive_rate: f64,
}

impl QuantumDiagnosisResult {
    pub fn new(patient: String, condition: String) -> Self {
        Self {
            diagnosis_id: crate::core::uuid_simple(),
            patient_id: patient,
            condition_detected: condition,
            confidence_score: 0.0,
            quantum_measurement_type: String::new(),
            imaging_resolution_angstrom: 0.0,
            false_positive_rate: 0.0,
        }
    }

    pub fn compute_diagnosis(&mut self, quantum_readings: &[f64]) -> Result<()> {
        let avg_signal = if quantum_readings.is_empty() {
            0.5
        } else {
            quantum_readings.iter().sum::<f64>() / quantum_readings.len() as f64
        };

        self.confidence_score = (avg_signal * 0.8 + rand_simple() * 0.2).min(1.0);
        self.quantum_measurement_type = "Superconducting Qubit Array".to_string();
        self.imaging_resolution_angstrom = 0.5 + rand_simple() * 0.5;
        self.false_positive_rate = 0.05 - rand_simple() * 0.03;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDrugDelivery {
    pub delivery_id: String,
    pub drug_name: String,
    pub target_receptor: String,
    pub quantum_dot_count: usize,
    pub delivery_efficiency_percent: f64,
    pub release_profile_hours: Vec<f64>,
}

impl QuantumDrugDelivery {
    pub fn new(drug: String, receptor: String) -> Self {
        Self {
            delivery_id: crate::core::uuid_simple(),
            drug_name: drug,
            target_receptor: receptor,
            quantum_dot_count: 0,
            delivery_efficiency_percent: 0.0,
            release_profile_hours: Vec::new(),
        }
    }

    pub fn simulate_delivery(&mut self, dot_count: usize) -> Result<()> {
        self.quantum_dot_count = dot_count;
        self.delivery_efficiency_percent = 75.0 + (dot_count as f64 * 0.1).min(20.0);
        self.release_profile_hours = (0..24).map(|h| {
            let base = 100.0 * (-0.1 * h as f64).exp();
            base * (0.9 + rand_simple() * 0.2)
        }).collect();

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMedicalImaging {
    pub imaging_id: String,
    pub scan_type: String,
    pub patient_id: String,
    pub qubit_resolution: usize,
    pub scan_duration_minutes: f64,
    pub contrast_ratio: f64,
}

impl QuantumMedicalImaging {
    pub fn new(scan_type: String, patient: String) -> Self {
        Self {
            imaging_id: crate::core::uuid_simple(),
            scan_type,
            patient_id: patient,
            qubit_resolution: 0,
            scan_duration_minutes: 0.0,
            contrast_ratio: 0.0,
        }
    }

    pub fn perform_scan(&mut self, qubits: usize) -> Result<()> {
        self.qubit_resolution = qubits;
        self.scan_duration_minutes = 30.0 + qubits as f64 * 0.5;
        self.contrast_ratio = 5.0 + qubits as f64 * 0.1;
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

pub fn apply_quantum_therapy(device: &QuantumMedicalDevice) -> Result<String> {
    Ok(format!("Therapy_{}_Applied", device.patient_id))
}

pub fn calibrate_quantum_sensor(sensitivity: f64) -> Result<f64> {
    Ok(sensitivity * 1.05)
}

pub fn optimize_drug_quantum_state(drug: &str, target_purity: f64) -> Result<f64> {
    Ok((target_purity * 0.95).min(1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treatment_validation() {
        let params = QuantumTreatmentParameters::new(1e9, 2.0, 30.0);
        params.validate().unwrap();
        assert!(params.frequency_hz > 0.0);
    }

    #[test]
    fn test_diagnosis_computation() {
        let mut result = QuantumDiagnosisResult::new(
            "Patient_001".to_string(),
            "Cancer_Marker_Detected".to_string(),
        );
        let readings = vec![0.8, 0.85, 0.9, 0.75, 0.88];
        result.compute_diagnosis(&readings).unwrap();
        assert!(result.confidence_score > 0.0);
    }

    #[test]
    fn test_drug_delivery() {
        let mut delivery = QuantumDrugDelivery::new(
            "Doxorubicin".to_string(),
            "HER2_Receptor".to_string(),
        );
        delivery.simulate_delivery(500).unwrap();
        assert!(delivery.delivery_efficiency_percent > 0.0);
    }
}