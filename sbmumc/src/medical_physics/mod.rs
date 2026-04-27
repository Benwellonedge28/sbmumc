//! Medical Physics Module
//!
//! This module implements medical physics, imaging techniques,
//! and radiation therapy for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalPhysics {
    pub med_id: String,
    pub imaging_modalities: Vec<ImagingModality>,
    pub radiation_therapy: RadiationTherapy,
    pub nuclear_medicine: NuclearMedicine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagingModality { pub modality_id: String, pub modality_name: String, pub resolution_mm: f64, pub contrast: f64, pub radiation_dose_mgy: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationTherapy { pub treatment_techniques: Vec<TreatmentTechnique>, pub dose_calculation: DoseCalculation }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatmentTechnique { pub technique_id: String, pub technique_name: String, pub fraction_dose_gy: f64, pub total_dose_gy: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoseCalculation { pub calculation_method: String, pub accuracy_percent: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearMedicine { pub radioisotopes: Vec<Radioisotope>, pub imaging_protocols: Vec<ImagingProtocol> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Radioisotope { pub isotope_id: String, pub isotope_name: String, pub half_life_h: f64, pub emission_type: String, pub energy_mev: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagingProtocol { pub protocol_id: String, pub protocol_name: String, pub administered_activity_mbq: f64, pub image_timing_min: f64 }

impl MedicalPhysics {
    pub fn new() -> Self {
        Self {
            med_id: String::from("medical_physics_v1"),
            imaging_modalities: vec![
                ImagingModality { modality_id: String::from("mod_ct"), modality_name: String::from("CT"), resolution_mm: 0.5, contrast: 0.8, radiation_dose_mgy: 10.0 },
                ImagingModality { modality_id: String::from("mod_mri"), modality_name: String::from("MRI"), resolution_mm: 1.0, contrast: 0.9, radiation_dose_mgy: 0.0 },
            ],
            radiation_therapy: RadiationTherapy {
                treatment_techniques: vec![TreatmentTechnique { technique_id: String::from("tech_imrt"), technique_name: String::from("IMRT"), fraction_dose_gy: 2.0, total_dose_gy: 70.0 }],
                dose_calculation: DoseCalculation { calculation_method: String::from("Monte Carlo"), accuracy_percent: 2.0 },
            },
            nuclear_medicine: NuclearMedicine {
                radioisotopes: vec![Radioisotope { isotope_id: String::from("iso_tc99m"), isotope_name: String::from("Tc-99m"), half_life_h: 6.0, emission_type: String::from("Gamma"), energy_mev: 0.14 }],
                imaging_protocols: vec![ImagingProtocol { protocol_id: String::from("proto_bone"), protocol_name: String::from("Bone scan"), administered_activity_mbq: 740.0, image_timing_min: 180.0 }],
            },
        }
    }

    pub fn compute_dose(&self, activity_mbq: f64, time_h: f64, energy_mev: f64) -> f64 {
        activity_mbq * time_h * energy_mev * 1e-6
    }

    pub fn compute_image_quality(&self, snr: f64, resolution_mm: f64) -> f64 { snr / resolution_mm }
}

impl Default for MedicalPhysics { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_dose() { let mp = MedicalPhysics::new(); assert!(mp.compute_dose(740.0, 6.0, 0.14) > 0.0); } }
