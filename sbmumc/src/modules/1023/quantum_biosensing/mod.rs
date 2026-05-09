//! # SBMUMC Module 1023: Quantum Biosensing
//!
//! Quantum-enhanced biosensing technologies and applications.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensorTechnology {
    NV_CenterDiamond,
    SuperconductingQubit,
    TrappedIon,
    PhotonicQubit,
    MolecularQubit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumBiosensor {
    pub sensor_id: String,
    pub technology: SensorTechnology,
    pub target_biomolecule: String,
    pub sensitivity_limit: f64,
    pub noise_floor: f64,
    pub measurement_rate_hz: f64,
}

impl QuantumBiosensor {
    pub fn new(tech: SensorTechnology, target: String) -> Self {
        Self {
            sensor_id: crate::core::uuid_simple(),
            technology: tech,
            target_biomolecule: target,
            sensitivity_limit: 0.0,
            noise_floor: 0.0,
            measurement_rate_hz: 0.0,
        }
    }

    pub fn calibrate(&mut self) -> Result<()> {
        match self.technology {
            SensorTechnology::NV_CenterDiamond => {
                self.sensitivity_limit = 1e-12;
                self.noise_floor = 1e-15;
            },
            SensorTechnology::SuperconductingQubit => {
                self.sensitivity_limit = 1e-15;
                self.noise_floor = 1e-18;
            },
            SensorTechnology::TrappedIon => {
                self.sensitivity_limit = 1e-18;
                self.noise_floor = 1e-21;
            },
            _ => {
                self.sensitivity_limit = 1e-9;
                self.noise_floor = 1e-12;
            }
        }
        self.measurement_rate_hz = 1e6 + rand_simple() * 1e5;
        Ok(())
    }

    pub fn perform_measurement(&self, sample_concentration: f64) -> Result<f64> {
        let signal = (sample_concentration / self.sensitivity_limit).ln().max(0.0);
        let noise_contribution = self.noise_floor * rand_simple() * 10.0;
        Ok(signal + noise_contribution)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomoleculeDetection {
    pub detection_id: String,
    pub molecule_type: String,
    pub concentration_pM: f64,
    pub detection_limit: f64,
    pub signal_to_noise: f64,
    pub quantum_enhancement_factor: f64,
}

impl BiomoleculeDetection {
    pub fn new(molecule: String, concentration: f64) -> Self {
        Self {
            detection_id: crate::core::uuid_simple(),
            molecule_type: molecule,
            concentration_pM: concentration,
            detection_limit: 0.0,
            signal_to_noise: 0.0,
            quantum_enhancement_factor: 0.0,
        }
    }

    pub fn analyze(&mut self) -> Result<()> {
        self.detection_limit = 0.01 + rand_simple() * 0.1;
        self.signal_to_noise = 50.0 + rand_simple() * 100.0;
        self.quantum_enhancement_factor = 10.0 + rand_simple() * 90.0;

        if self.concentration_pM < self.detection_limit {
            return Err(SbmumcError::InvalidInput("Concentration below detection limit".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumImagingSensor {
    pub imaging_id: String,
    pub pixel_count: usize,
    pub quantum_efficiency: f64,
    pub wavelength_range_nm: (f64, f64),
    pub spatial_resolution_nm: f64,
}

impl QuantumImagingSensor {
    pub fn new(pixels: usize) -> Self {
        Self {
            imaging_id: crate::core::uuid_simple(),
            pixel_count: pixels,
            quantum_efficiency: 0.0,
            wavelength_range_nm: (400.0, 700.0),
            spatial_resolution_nm: 0.0,
        }
    }

    pub fn optimize_quantum_efficiency(&mut self) -> Result<()> {
        self.quantum_efficiency = 0.85 + rand_simple() * 0.15;
        self.spatial_resolution_nm = 5.0 / (self.pixel_count as f64).sqrt();
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleMoleculeSensor {
    pub sensor_id: String,
    pub molecule_id: String,
    pub detection_time_ms: f64,
    pub binding_affinity_pM: f64,
    pub quantum_measurement_precision: f64,
    pub temporal_resolution_ps: f64,
}

impl SingleMoleculeSensor {
    pub fn new(molecule_id: String) -> Self {
        Self {
            sensor_id: crate::core::uuid_simple(),
            molecule_id,
            detection_time_ms: 0.0,
            binding_affinity_pM: 0.0,
            quantum_measurement_precision: 0.0,
            temporal_resolution_ps: 0.0,
        }
    }

    pub fn detect_single_molecule(&mut self) -> Result<()> {
        self.detection_time_ms = 10.0 + rand_simple() * 90.0;
        self.binding_affinity_pM = 1.0 + rand_simple() * 99.0;
        self.quantum_measurement_precision = 0.01 + rand_simple() * 0.09;
        self.temporal_resolution_ps = 1000.0 + rand_simple() * 9000.0;
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

pub fn initialize_biosensor(tech: SensorTechnology) -> Result<QuantumBiosensor> {
    let sensor = QuantumBiosensor::new(tech, "Generic_Target".to_string());
    Ok(sensor)
}

pub fn process_quantum_readout(data: &[f64]) -> Result<f64> {
    let sum: f64 = data.iter().sum();
    Ok(sum / data.len() as f64)
}

pub fn optimize_sensor_entanglement(sensor: &QuantumBiosensor) -> Result<f64> {
    Ok(sensor.noise_floor * 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nv_sensor_calibration() {
        let mut sensor = QuantumBiosensor::new(
            SensorTechnology::NV_CenterDiamond,
            "DNA_Sequence".to_string(),
        );
        sensor.calibrate().unwrap();
        assert!(sensor.sensitivity_limit > 0.0);
    }

    #[test]
    fn test_biomolecule_detection() {
        let mut detection = BiomoleculeDetection::new(
            "SARS_CoV_2_Spike".to_string(),
            50.0,
        );
        detection.analyze().unwrap();
        assert!(detection.quantum_enhancement_factor > 1.0);
    }

    #[test]
    fn test_single_molecule_detection() {
        let mut sensor = SingleMoleculeSensor::new("BRCA1_Gene".to_string());
        sensor.detect_single_molecule().unwrap();
        assert!(sensor.temporal_resolution_ps > 0.0);
    }
}