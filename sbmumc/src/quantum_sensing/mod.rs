//! Quantum Sensing Module
//!
//! This module implements quantum metrology, gravitational wave
//! detection, and atomic sensors for precision measurement.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct QuantumSensing {
    pub sensors: Vec<QuantumSensor>,
    pub metrology_schemes: Vec<MetrologyScheme>,
    pub detectors: Vec<SensorDetector>,
}

impl QuantumSensing {
    pub fn new() -> Self {
        QuantumSensing {
            sensors: Vec::new(),
            metrology_schemes: Vec::new(),
            detectors: Vec::new(),
        }
    }

    /// Create sensor
    pub fn create_sensor(&mut self, sensor_type: SensorType) -> &QuantumSensor {
        let sensor = QuantumSensor {
            sensor_id: format!("qs_{}", self.sensors.len()),
            sensor_type,
            sensitivity: 1e-12,
            bandwidth: 1000.0,
            operating_temperature: 0.015,
        };
        self.sensors.push(sensor);
        self.sensors.last().unwrap()
    }

    /// Measure with Heisenberg limit
    pub fn heisenberg_measure(&self, sensor_id: &str, time: f64) -> Result<Measurement> {
        if let Some(sensor) = self.sensors.iter().find(|s| s.sensor_id == sensor_id) {
            let precision = 1.0 / time.powi(2);
            Ok(Measurement {
                sensor_id: sensor_id.to_string(),
                value: rand::random::<f64>() * sensor.sensitivity * precision,
                precision,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            })
        } else {
            Err(SbmumcError::NotFound(format!("Sensor {} not found", sensor_id)))
        }
    }

    /// Create metrology scheme
    pub fn create_scheme(&mut self, name: &str, entanglement: bool) -> &MetrologyScheme {
        let scheme = MetrologyScheme {
            scheme_id: format!("ms_{}", self.metrology_schemes.len()),
            name: name.to_string(),
            entanglement_enhanced: entanglement,
            sql_improvement: if entanglement { 3.0 } else { 1.0 },
            heisenberg_advantage: if entanglement { 2.0 } else { 1.0 },
        };
        self.metrology_schemes.push(scheme);
        self.metrology_schemes.last().unwrap()
    }

    /// Detect gravitational wave
    pub fn detect_gravitational_wave(&self, strain: f64) -> GWDetection {
        let snr = strain * 1e21;
        GWDetection {
            detected: snr > 5.0,
            strain_amplitude: strain,
            signal_to_noise: snr,
            frequency: 100.0 + rand::random::<f64>() * 100.0,
        }
    }

    /// Create atomic clock
    pub fn create_atomic_clock(&mut self, accuracy: f64) -> &AtomicClock {
        let clock = AtomicClock {
            clock_id: format!("ac_{}", self.detectors.len()),
            accuracy,
            stability: accuracy * 0.1,
            coherence_time: 1.0 / accuracy,
        };
        self.detectors.push(SensorDetector::Atomic(clock.clone()));
        &self.detectors.last().unwrap().atomic().unwrap()
    }

    /// Nitrogen-vacancy sensing
    pub fn nv_sensing(&mut self, target: &str) -> NVResult {
        NVResult {
            target_id: target.to_string(),
            magnetic_field: 1e-6 + rand::random::<f64>() * 1e-5,
            electric_field: 1e-3 + rand::random::<f64>() * 0.01,
            temperature: 300.0 + rand::random::<f64>() * 50.0,
            spatial_resolution: 1e-9,
        }
    }
}

impl Default for QuantumSensing { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSensor {
    pub sensor_id: String,
    pub sensor_type: SensorType,
    pub sensitivity: f64,
    pub bandwidth: f64,
    pub operating_temperature: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorType {
    Magnetometer,
    Gravimeter,
    Gyroscope,
    Clock,
    Imager,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement {
    pub sensor_id: String,
    pub value: f64,
    pub precision: f64,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetrologyScheme {
    pub scheme_id: String,
    pub name: String,
    pub entanglement_enhanced: bool,
    pub sql_improvement: f64,
    pub heisenberg_advantage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GWDetection {
    pub detected: bool,
    pub strain_amplitude: f64,
    pub signal_to_noise: f64,
    pub frequency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomicClock {
    pub clock_id: String,
    pub accuracy: f64,
    pub stability: f64,
    pub coherence_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NVResult {
    pub target_id: String,
    pub magnetic_field: f64,
    pub electric_field: f64,
    pub temperature: f64,
    pub spatial_resolution: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensorDetector {
    Atomic(AtomicClock),
    NV { center: String },
    Superconducting { tc: f64 },
}

impl SensorDetector {
    pub fn atomic(&self) -> Option<&AtomicClock> {
        match self { SensorDetector::Atomic(c) => Some(c), _ => None }
    }
}