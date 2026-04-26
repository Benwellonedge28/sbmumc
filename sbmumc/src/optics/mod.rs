//! Optics Module
//!
//! This module implements optics, light propagation,
//! and optical systems for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Optics {
    pub optics_id: String,
    pub optical_elements: Vec<OpticalElement>,
    pub optical_systems: Vec<OpticalSystem>,
    pub light_properties: LightProperties,
    pub polarization: Polarization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpticalElement {
    pub element_id: String,
    pub element_type: ElementType,
    pub focal_length_m: f64,
    pub aperture_m: f64,
    pub transmission: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElementType { Lens, Mirror, Prism, Grating, Filter, Detector }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpticalSystem {
    pub system_id: String,
    pub elements: Vec<String>,
    pub magnification: f64,
    pub numerical_aperture: f64,
    pub resolution_nm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightProperties {
    pub wavelength_nm: f64,
    pub frequency_hz: f64,
    pub speed_m_s: f64,
    pub intensity_w_m2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polarization {
    pub polarization_type: PolarizationType,
    pub stokes_parameters: [f64; 4],
    pub coherence_length_m: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolarizationType { Linear, Circular, Elliptical, Unpolarized }

impl Optics {
    pub fn new() -> Self {
        Self {
            optics_id: String::from("optics_v1"),
            optical_elements: vec![
                OpticalElement { element_id: String::from("elem_1"), element_type: ElementType::Lens, focal_length_m: 0.1, aperture_m: 0.05, transmission: 0.95 },
            ],
            optical_systems: vec![
                OpticalSystem { system_id: String::from("sys_1"), elements: vec![String::from("Lens1")], magnification: 10.0, numerical_aperture: 0.8, resolution_nm: 200.0 },
            ],
            light_properties: LightProperties { wavelength_nm: 550.0, frequency_hz: 5.45e14, speed_m_s: 3e8, intensity_w_m2: 100.0 },
            polarization: Polarization { polarization_type: PolarizationType::Linear, stokes_parameters: [1.0, 0.5, 0.0, 0.0], coherence_length_m: 1e-6 },
        }
    }

    pub fn compute_refractive_index(&self, wavelength_nm: f64, material: &str) -> f64 {
        match material {
            "glass" => 1.5 - 0.01 * (wavelength_nm - 550.0) / 100.0,
            "water" => 1.33,
            _ => 1.0,
        }
    }

    pub fn compute_rayleigh_criterion(&self, wavelength_nm: f64, numerical_aperture: f64) -> f64 {
        0.61 * wavelength_nm / numerical_aperture
    }

    pub fn compute_diffraction_angle(&self, wavelength_nm: f64, slit_width_um: f64) -> f64 {
        wavelength_nm * 1e-6 / (slit_width_um * 1e-6)
    }
}

impl Default for Optics { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_refractive_index() { let opt = Optics::new(); assert!(opt.compute_refractive_index(550.0, "glass") > 1.0); } }
