//! Quantum Imaging Module (551)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumImaging {
    pub qi_id: String,
    pub technique: ImagingTechnique,
    pub resolution_nm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImagingTechnique {
    GhostImaging,
    QuantumIllumination,
    EntangledPhotonImaging,
    SubShotNoiseImaging,
}

impl QuantumImaging {
    pub fn new() -> Self {
        Self {
            qi_id: String::from("quantum_imaging_v1"),
            technique: ImagingTechnique::GhostImaging,
            resolution_nm: 1.0,
        }
    }
}

impl Default for QuantumImaging {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_imaging() {
        let img = QuantumImaging::new();
        assert!(img.resolution_nm < 10.0);
    }
}
