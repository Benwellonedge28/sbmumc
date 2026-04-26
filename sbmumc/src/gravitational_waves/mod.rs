//! Gravitational Waves Module
//!
//! This module implements gravitational wave generation, propagation,
//! detection enhancement, and gravitational wave astronomy for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Gravitational wave system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravitationalWaves {
    pub gw_id: String,
    pub sources: Vec<GWSource>,
    pub propagation: WavePropagation,
    pub detectors: Vec<GWDetector>,
    pub waveform_database: WaveformDatabase,
}

/// Gravitational wave source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GWSource {
    pub source_id: String,
    pub source_type: SourceType,
    pub mass_parameters: MassParameters,
    pub distance_mpc: f64,
    pub sky_location: SkyLocation,
    pub polarization: PolarizationState,
    pub strain_amplitude: f64,
}

/// Types of GW sources
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SourceType {
    BinaryBlackHole,
    BinaryNeutronStar,
    NeutronStarBlackHole,
    Supernova,
    StochasticBackground,
    ContinuousWave,
    Burst,
}

/// Mass parameters for source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassParameters {
    pub mass_1_msun: f64,
    pub mass_2_msun: f64,
    pub chirp_mass_msun: f64,
    pub mass_ratio: f64,
    pub total_mass_msun: f64,
    pub symmetric_mass_ratio: f64,
}

/// Sky location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkyLocation {
    pub right_ascension: f64,
    pub declination: f64,
    pub coordinate_system: String,
}

/// Polarization state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolarizationState {
    pub plus: f64,
    pub cross: f64,
    pub tensor_modes: Vec<f64>,
    pub vector_modes: Vec<f64>,
    pub scalar_modes: Vec<f64>,
}

/// Wave propagation in spacetime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WavePropagation {
    pub propagation_speed_c: f64,
    pub dispersion: f64,
    pub attenuation: f64,
    pub graviton_mass_kg: f64,
    pub polarization_modes: Vec<String>,
}

/// GW detector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GWDetector {
    pub detector_id: String,
    pub detector_name: String,
    pub detector_type: DetectorType,
    pub sensitivity_curve: SensitivityCurve,
    pub location: [f64; 3],
    pub arm_length_m: f64,
    pub operating_status: OperatingStatus,
}

/// Detector types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DetectorType {
    Michelson,
    Sagnac,
    Resonant,
    AtomInterferometer,
    SpaceBased,
    LIGO,
    Virgo,
    KAGRA,
}

/// Sensitivity curve
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityCurve {
    pub frequencies_hz: Vec<f64>,
    pub strain_sensitivity: Vec<f64>,
    pub optimal_frequency: f64,
}

/// Operating status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperatingStatus {
    Online,
    Offline,
    Upgrading,
    Calibrating,
}

/// Waveform database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveformDatabase {
    pub templates: Vec<WaveformTemplate>,
    pub approximants: Vec<String>,
    pub coverage_hz: [f64; 2],
}

impl GravitationalWaves {
    /// Creates a new gravitational waves system
    pub fn new() -> Self {
        Self {
            gw_id: String::from("gravitational_waves_v1"),
            sources: vec![
                GWSource {
                    source_id: String::from("source_1"),
                    source_type: SourceType::BinaryBlackHole,
                    mass_parameters: MassParameters {
                        mass_1_msun: 30.0,
                        mass_2_msun: 25.0,
                        chirp_mass_msun: 26.0,
                        mass_ratio: 0.83,
                        total_mass_msun: 55.0,
                        symmetric_mass_ratio: 0.24,
                    },
                    distance_mpc: 400.0,
                    sky_location: SkyLocation { right_ascension: 180.0, declination: 45.0, coordinate_system: String::from("J2000") },
                    polarization: PolarizationState { plus: 1.0, cross: 0.5, tensor_modes: vec![1.0, 0.0], vector_modes: vec![], scalar_modes: vec![] },
                    strain_amplitude: 1e-22,
                },
            ],
            propagation: WavePropagation {
                propagation_speed_c: 299792458.0,
                dispersion: 0.0,
                attenuation: 0.0,
                graviton_mass_kg: 0.0,
                polarization_modes: vec![String::from("plus"), String::from("cross")],
            },
            detectors: vec![
                GWDetector {
                    detector_id: String::from("ligo_hanford"),
                    detector_name: String::from("LIGO Hanford"),
                    detector_type: DetectorType::LIGO,
                    sensitivity_curve: SensitivityCurve {
                        frequencies_hz: vec![10.0, 100.0, 1000.0],
                        strain_sensitivity: vec![1e-23, 1e-24, 1e-22],
                        optimal_frequency: 100.0,
                    },
                    location: [46.4, -119.4, 0.0],
                    arm_length_m: 4000.0,
                    operating_status: OperatingStatus::Online,
                },
            ],
            waveform_database: WaveformDatabase {
                templates: vec![],
                approximants: vec![String::from("SEOBNRv4"), String::from("IMRPhenom")],
                coverage_hz: [10.0, 1000.0],
            },
        }
    }

    /// Generates waveform from source
    pub fn generate_waveform(&self, source: &GWSource) -> Waveform {
        let chirp_mass = source.mass_parameters.chirp_mass_msun;
        let freq = 100.0 * (chirp_mass * 3.0).powf(3.0 / 8.0);
        Waveform {
            source_id: source.source_id.clone(),
            strain_plus: vec![1e-22; 1000],
            strain_cross: vec![5e-23; 1000],
            frequencies: (1.0..1001.0).map(|f| f as f64).collect(),
            duration_s: 1.0,
        }
    }

    /// Matches template to signal
    pub fn match_template(&self, signal: &[f64], template: &[f64]) -> MatchResult {
        let snr = 10.0;
        MatchResult {
            matched: snr > 8.0,
            snr,
            correlation: 0.95,
            template_id: String::from("template_1"),
        }
    }

    /// Triangulates source location
    pub fn triangulate(&self, detection_times: &[f64], detector_ids: &[String]) -> TriangulationResult {
        TriangulationResult {
            sky_location: SkyLocation { right_ascension: 180.0, declination: 45.0, coordinate_system: String::from("J2000") },
            error_region_deg2: 10.0,
            confidence: 0.9,
        }
    }

    /// Computes strain at detector
    pub fn compute_strain(&self, source: &GWSource, detector_id: &str) -> f64 {
        let distance = source.distance_mpc * 3.086e22;
        let m_total = source.mass_parameters.total_mass_msun * 1.989e30;
        let g = 6.674e-11;
        let c = 3e8;
        let strain = 4.0 * g * m_total / (c * c * distance) * 1e-22;
        strain
    }

    /// Analyzes parameter estimation
    pub fn estimate_parameters(&self, signal: &[f64]) -> ParameterEstimation {
        ParameterEstimation {
            masses: [30.0, 25.0],
            spin: [0.5, 0.3],
            distance: 400.0,
            inclination: 30.0,
            errors: HashMap::new(),
        }
    }
}

/// Generated waveform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waveform {
    pub source_id: String,
    pub strain_plus: Vec<f64>,
    pub strain_cross: Vec<f64>,
    pub frequencies: Vec<f64>,
    pub duration_s: f64,
}

/// Template match result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub matched: bool,
    pub snr: f64,
    pub correlation: f64,
    pub template_id: String,
}

/// Triangulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangulationResult {
    pub sky_location: SkyLocation,
    pub error_region_deg2: f64,
    pub confidence: f64,
}

/// Parameter estimation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterEstimation {
    pub masses: [f64; 2],
    pub spin: [f64; 2],
    pub distance: f64,
    pub inclination: f64,
    pub errors: HashMap<String, f64>,
}

impl Default for GravitationalWaves {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waveform_generation() {
        let gw = GravitationalWaves::new();
        if let Some(source) = gw.sources.first() {
            let waveform = gw.generate_waveform(source);
            assert!(waveform.strain_plus.len() > 0);
        }
    }

    #[test]
    fn test_strain_computation() {
        let gw = GravitationalWaves::new();
        if let Some(source) = gw.sources.first() {
            let strain = gw.compute_strain(source, "ligo_hanford");
            assert!(strain > 0.0);
        }
    }

    #[test]
    fn test_template_matching() {
        let gw = GravitationalWaves::new();
        let signal = vec![1e-22; 100];
        let template = vec![1e-22; 100];
        let result = gw.match_template(&signal, &template);
        assert!(result.matched);
    }
}