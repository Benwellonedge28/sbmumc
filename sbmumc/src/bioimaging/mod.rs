//! Bioimaging Module (714)
//!
//! Biological microscopy, imaging techniques, and quantitative image analysis.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImagingTechnique {
    Confocal,
    TwoPhoton,
    STED,
    SIM,
    PALM,
    STORM,
    LightSheet,
    Electron,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioImage {
    pub image_id: String,
    pub technique: ImagingTechnique,
    pub magnification: f64,
    pub numerical_aperture: f64,
    pub resolution_xy_nm: f64,
    pub resolution_z_nm: f64,
    pub fluorophores: Vec<String>,
    pub frame_count: u32,
}

impl BioImage {
    pub fn new(image_id: String, technique: ImagingTechnique) -> Self {
        Self {
            image_id,
            technique,
            magnification: 0.0,
            numerical_aperture: 0.0,
            resolution_xy_nm: 0.0,
            resolution_z_nm: 0.0,
            fluorophores: Vec::new(),
            frame_count: 1,
        }
    }

    pub fn calculate_resolution(&self) -> f64 {
        0.61 * 500.0 / self.numerical_aperture
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bioimage() {
        let image = BioImage::new("BI-001".into(), ImagingTechnique::Confocal);
        assert!(matches!(image.technique, ImagingTechnique::Confocal));
    }
}
