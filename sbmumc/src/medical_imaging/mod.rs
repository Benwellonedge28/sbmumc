//! Medical Imaging Module (712)
//!
//! Medical imaging modalities, image processing, and diagnostic visualization.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImagingModality {
    XRay,
    CT,
    MRI,
    Ultrasound,
    PET,
    SPECT,
    OCT,
    Thermal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalImage {
    pub image_id: String,
    pub modality: ImagingModality,
    pub resolution_um: f64,
    pub slices: u32,
    pub file_size_mb: f64,
    pub contrast_agent: bool,
    pub reconstruction_algorithm: String,
    pub diagnostic_quality: String,
}

impl MedicalImage {
    pub fn new(image_id: String, modality: ImagingModality) -> Self {
        Self {
            image_id,
            modality,
            resolution_um: 0.0,
            slices: 0,
            file_size_mb: 0.0,
            contrast_agent: false,
            reconstruction_algorithm: "Filtered Back Projection".into(),
            diagnostic_quality: "Diagnostic".into(),
        }
    }

    pub fn calculate_dose(&self, modality: &ImagingModality) -> f64 {
        match modality {
            ImagingModality::XRay => 0.1,
            ImagingModality::CT => 10.0,
            ImagingModality::PET => 15.0,
            _ => 0.0,
        }
    }
}

pub struct MedicalImageAnalysis;

impl MedicalImageAnalysis {
    pub fn contrast_to_noise(signal: f64, noise: f64) -> f64 {
        if noise == 0.0 { return 0.0; }
        signal / noise
    }

    pub fn lesion_detection(true_positives: u32, false_negatives: u32) -> f64 {
        if true_positives + false_negatives == 0 { return 0.0; }
        (true_positives as f64 / (true_positives + false_negatives) as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_medical_image() {
        let image = MedicalImage::new("IMG-001".into(), ImagingModality::MRI);
        assert!(matches!(image.modality, ImagingModality::MRI));
    }
}
