//! Photonics Compiler Module (513)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotonicsCompiler {
    pub pc_id: String,
    pub wavelength_nm: f64,
    pub mode_count: u32,
    pub waveguide_loss_db_cm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotonicCircuit {
    pub circuit_id: String,
    pub components: Vec<PhotonicComponent>,
    pub insertion_loss_db: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhotonicComponent {
    Waveguide { length_um: f64 },
    Coupler { coupling_ratio: f64 },
    PhaseShifter { phase_shift_rad: f64 },
    Detector,
    LaserSource { power_dbm: f64 },
    Filter { bandwidth_ghz: f64 },
}

impl PhotonicsCompiler {
    pub fn new() -> Self {
        Self {
            pc_id: String::from("photonics_compiler_v1"),
            wavelength_nm: 1550.0,
            mode_count: 64,
            waveguide_loss_db_cm: 0.1,
        }
    }

    pub fn compile(&self, components: Vec<PhotonicComponent>) -> PhotonicCircuit {
        let total_loss: f64 = components.iter().map(|c| match c {
            PhotonicComponent::Waveguide { length_um } => self.waveguide_loss_db_cm * (length_um / 10000.0),
            _ => 0.5,
        }).sum();
        
        PhotonicCircuit {
            circuit_id: format!("photonic_{}", components.len()),
            components,
            insertion_loss_db: total_loss,
        }
    }
}

impl Default for PhotonicsCompiler {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_photonics_compiler() {
        let compiler = PhotonicsCompiler::new();
        assert_eq!(compiler.wavelength_nm, 1550.0);
    }
}
