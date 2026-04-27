//! Spectroscopy Module
//!
//! This module implements spectroscopy, spectral analysis,
//! and spectroscopic techniques for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spectroscopy {
    pub spec_id: String,
    pub spectral_regions: Vec<SpectralRegion>,
    pub spectroscopic_techniques: Vec<Technique>,
    pub spectral_lines: Vec<SpectralLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectralRegion { pub region_id: String, pub wavelength_range: [f64; 2], pub energy_range: [f64; 2], pub dominant_processes: Vec<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technique { pub tech_id: String, pub tech_name: String, pub resolution: f64, pub sensitivity: f64, pub applications: Vec<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectralLine { pub line_id: String, pub wavelength_nm: f64, pub intensity: f64, pub transition: String, pub identification: String }

impl Spectroscopy {
    pub fn new() -> Self {
        Self {
            spec_id: String::from("spectroscopy_v1"),
            spectral_regions: vec![
                SpectralRegion { region_id: String::from("vis"), wavelength_range: [400.0, 700.0], energy_range: [1.77e-19, 3.1e-19], dominant_processes: vec![String::from("Electronic transitions")] },
            ],
            spectroscopic_techniques: vec![
                Technique { tech_id: String::from("tech_1"), tech_name: String::from("UV-Vis spectroscopy"), resolution: 1.0, sensitivity: 0.01, applications: vec![String::from("Chemical analysis")] },
            ],
            spectral_lines: vec![
                SpectralLine { line_id: String::from("line_h_alpha"), wavelength_nm: 656.3, intensity: 1.0, transition: String::from("n=3 to n=2"), identification: String::from("Hydrogen Balmer line") },
            ],
        }
    }

    pub fn compute_spectral_line_position(&self, z: f64, rest_wavelength: f64) -> f64 { rest_wavelength * (1.0 + z) }
    pub fn compute_doppler_shift(&self, v: f64, rest_freq: f64) -> f64 {
        let c = 3e8;
        rest_freq * (v / c)
    }
}

impl Default for Spectroscopy { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_doppler() { let spec = Spectroscopy::new(); assert!(spec.compute_doppler_shift(1000.0, 5e14) > 0.0); } }
