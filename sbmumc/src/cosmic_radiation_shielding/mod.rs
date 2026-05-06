//! Cosmic Radiation Shielding Module (661)
//!
//! Radiation protection systems for deep space, planetary surfaces, and orbital environments.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RadiationType {
    GCR,          // Galactic Cosmic Rays
    SPE,          // Solar Particle Events
    Trapped,
    Secondary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationShield {
    pub shield_type: String,
    pub material: String,
    pub thickness: f64,             // g/cm^2
    pub areal_density: f64,           // kg/m^2
    pub mass_reduction_factor: f64,
    pub effectiveness: f64,           // percent
    pub weight_penalty: f64,         // kg
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationEnvironment {
    pub environment_type: String,
    pub gcr_flux: f64,               // particles/cm^2/s
    pub spe_probability: f64,         // per year
    pub average_dose_rate: f64,      // mSv/day
    pub peak_spe_dose: f64,          // mSv
    pub albedo_contribution: f64,    // percent
}

impl RadiationShield {
    pub fn new(shield_type: String, material: String, thickness: f64) -> Self {
        Self {
            shield_type,
            material,
            thickness,
            areal_density: 0.0,
            mass_reduction_factor: 0.0,
            effectiveness: 0.0,
            weight_penalty: 0.0,
        }
    }

    pub fn calculate_dose_attenuation(&self) -> f64 {
        (-self.thickness / 100.0).exp()
    }

    pub fn annual_dose_with_shield(&self, base_dose: f64) -> f64 {
        base_dose * self.calculate_dose_attenuation()
    }
}

impl RadiationEnvironment {
    pub fn new(environment_type: String) -> Self {
        Self {
            environment_type,
            gcr_flux: 4.0,
            spe_probability: 0.3,
            average_dose_rate: 0.5,
            peak_spe_dose: 5000.0,
            albedo_contribution: 10.0,
        }
    }

    pub fn annual_dose(&self) -> f64 {
        let gcr_dose = self.average_dose_rate * 365.0;
        let spe_dose = self.spe_probability * self.peak_spe_dose;
        gcr_dose + spe_dose
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radiation_shield() {
        let shield = RadiationShield::new("Passive".into(), " polyethylene".into(), 20.0);
        assert!(shield.thickness > 0.0);
    }
}
