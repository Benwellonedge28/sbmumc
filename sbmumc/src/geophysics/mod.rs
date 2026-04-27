//! Geophysics Module
//!
//! This module implements geophysics, earth systems,
//! and geological phenomena for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geophysics {
    pub geo_id: String,
    pub earth_layers: Vec<EarthLayer>,
    pub tectonic_plates: Vec<TectonicPlate>,
    pub seismic_waves: Vec<SeismicWave>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarthLayer { pub layer_id: String, pub layer_name: String, pub depth_km: f64, pub density_kg_m3: f64, pub composition: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TectonicPlate { pub plate_id: String, pub plate_name: String, pub velocity_mm_yr: f64, pub direction: String, pub boundary_type: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeismicWave { pub wave_id: String, pub wave_type: SeismicWaveType, pub velocity_km_s: f64, pub frequency_hz: f64 }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SeismicWaveType { P, S, Love, Rayleigh }

impl Geophysics {
    pub fn new() -> Self {
        Self {
            geo_id: String::from("geophysics_v1"),
            earth_layers: vec![
                EarthLayer { layer_id: String::from("layer_1"), layer_name: String::from("Crust"), depth_km: 35.0, density_kg_m3: 2700.0, composition: String::from("Silicates") },
                EarthLayer { layer_id: String::from("layer_2"), layer_name: String::from("Mantle"), depth_km: 2900.0, density_kg_m3: 4500.0, composition: String::from("Silicates and oxides") },
            ],
            tectonic_plates: vec![
                TectonicPlate { plate_id: String::from("plate_1"), plate_name: String::from("Pacific"), velocity_mm_yr: 8.0, direction: String::from("NW"), boundary_type: String::from("Convergent") },
            ],
            seismic_waves: vec![
                SeismicWave { wave_id: String::from("wave_1"), wave_type: SeismicWaveType::P, velocity_km_s: 6.0, frequency_hz: 1.0 },
            ],
        }
    }

    pub fn compute_earthquake_magnitude(&self, energy_j: f64) -> f64 {
        (energy_j.log10() - 4.8) / 1.5
    }

    pub fn compute_wave_travel_time(&self, depth_km: f64, wave_type: &SeismicWaveType) -> f64 {
        let base_velocity = match wave_type {
            SeismicWaveType::P => 6.0,
            SeismicWaveType::S => 4.0,
            _ => 5.0,
        };
        depth_km / base_velocity
    }
}

impl Default for Geophysics { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_magnitude() { let geo = Geophysics::new(); assert!(geo.compute_earthquake_magnitude(1e12) > 0.0); } }
