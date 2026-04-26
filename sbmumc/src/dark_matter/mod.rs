//! Dark Matter Module
//!
//! This module implements dark matter theory, WIMPs, axions,
//! and detection methods for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkMatter {
    pub dm_id: String,
    pub candidates: Vec<DMCandidate>,
    pub distribution: DMDistribution,
    pub detection_experiments: Vec<DetectionExperiment>,
    pub relic_abundance: RelicAbundance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMCandidate {
    pub candidate_id: String,
    pub candidate_name: String,
    pub particle_type: DMType,
    pub mass_gev: f64,
    pub cross_section_pb: f64,
    pub interaction_type: InteractionType,
    pub viability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DMType {
    WIMP,
    Axion,
    SterileNeutrino,
    FuzzyDarkMatter,
    PrimordialBlackHole,
    SelfInteracting,
    Millicharged,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InteractionType {
    WeaklyInteracting,
    Gravitational,
    Millicharged,
    SuperWIMP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMDistribution {
    pub halo_model: HaloModel,
    pub density_profile: DensityProfile,
    pub local_density_gev_cm3: f64,
    pub velocity_distribution: VelocityDistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HaloModel {
    NFW,
    Einasto,
    Isothermal,
    Burkert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DensityProfile {
    pub profile_type: String,
    pub central_density: f64,
    pub scale_radius: f64,
    pub parameters: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VelocityDistribution {
    pub distribution_type: String,
    pub most_probable_velocity: f64,
    pub dispersion: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionExperiment {
    pub exp_id: String,
    pub experiment_name: String,
    pub detection_type: DetectionType,
    pub target_material: String,
    pub exposure_kg_days: f64,
    pub sensitivity: Sensitivity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DetectionType {
    DirectDetection,
    IndirectDetection,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sensitivity {
    pub cross_section_limit: f64,
    pub mass_range_gev: [f64; 2],
    pub status: SensitivityStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SensitivityStatus {
    SetLimits,
    SignalDetected,
    Ongoing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelicAbundance {
    pub omega_h2: f64,
    pub target_omega: f64,
    pub thermal_relic: bool,
    pub freeze_out_temperature: f64,
}

impl DarkMatter {
    pub fn new() -> Self {
        Self {
            dm_id: String::from("dark_matter_v1"),
            candidates: vec![
                DMCandidate {
                    candidate_id: String::from("wimp_1"),
                    candidate_name: String::from("Neutralino"),
                    particle_type: DMType::WIMP,
                    mass_gev: 100.0,
                    cross_section_pb: 1e-6,
                    interaction_type: InteractionType::WeaklyInteracting,
                    viability: 0.8,
                },
                DMCandidate {
                    candidate_id: String::from("axion_1"),
                    candidate_name: String::from("QCD Axion"),
                    particle_type: DMType::Axion,
                    mass_gev: 1e-5,
                    cross_section_pb: 1e-20,
                    interaction_type: InteractionType::Gravitational,
                    viability: 0.7,
                },
            ],
            distribution: DMDistribution {
                halo_model: HaloModel::NFW,
                density_profile: DensityProfile {
                    profile_type: String::from("NFW"),
                    central_density: 0.3,
                    scale_radius: 20.0,
                    parameters: vec![1.0, 20.0],
                },
                local_density_gev_cm3: 0.3,
                velocity_distribution: VelocityDistribution {
                    distribution_type: String::from("Maxwell-Boltzmann"),
                    most_probable_velocity: 220.0,
                    dispersion: 156.0,
                },
            },
            detection_experiments: vec![
                DetectionExperiment {
                    exp_id: String::from("exp_1"),
                    experiment_name: String::from("XENON1T"),
                    detection_type: DetectionType::DirectDetection,
                    target_material: String::from("Xenon"),
                    exposure_kg_days: 1e4,
                    sensitivity: Sensitivity {
                        cross_section_limit: 1e-48,
                        mass_range_gev: [10.0, 1000.0],
                        status: SensitivityStatus::SetLimits,
                    },
                },
            ],
            relic_abundance: RelicAbundance {
                omega_h2: 0.12,
                target_omega: 0.12,
                thermal_relic: true,
                freeze_out_temperature: 1e-4,
            },
        }
    }

    pub fn calculate_relic_abundance(&self, sigma_v: f64, mass_gev: f64) -> f64 {
        let m_pl = 1.22e19;
        let g_star = 100.0;
        3e-27 * mass_gev.powi(2) / (sigma_v * (m_pl / mass_gev).sqrt())
    }

    pub fn compute_detection_rate(&self, exposure: f64, cross_section: f64, local_density: f64) -> f64 {
        exposure * cross_section * local_density * 1e-3
    }

    pub fn test_wimp_candidates(&self) -> WIMPTest {
        WIMPTest {
            test_id: String::from("wimp_test_1"),
            candidates_tested: self.candidates.iter().filter(|c| c.particle_type == DMType::WIMP).count(),
            viable_candidates: 1,
            excluded_masses: vec![50.0, 500.0],
            conclusions: String::from("WIMP parameter space significantly constrained"),
        }
    }

    pub fn analyze_dwarf_galaxy_limits(&self) -> DwarfLimits {
        DwarfLimits {
            galaxy_count: 20,
            strongest_limit: 1e-46,
            average_limit: 5e-46,
            implications: String::from("Strong constraints on WIMP models"),
        }
    }

    pub fn compute_scattering_rate(&self, candidate: &DMCandidate, experiment: &DetectionExperiment) -> ScatteringRate {
        ScatteringRate {
            candidate_name: candidate.candidate_name.clone(),
            experiment_name: experiment.experiment_name.clone(),
            events_per_kg_year: 0.01,
            expected_events: 0.1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WIMPTest {
    pub test_id: String,
    pub candidates_tested: usize,
    pub viable_candidates: usize,
    pub excluded_masses: Vec<f64>,
    pub conclusions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DwarfLimits {
    pub galaxy_count: u32,
    pub strongest_limit: f64,
    pub average_limit: f64,
    pub implications: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScatteringRate {
    pub candidate_name: String,
    pub experiment_name: String,
    pub events_per_kg_year: f64,
    pub expected_events: f64,
}

impl Default for DarkMatter {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_relic_abundance() {
        let dm = DarkMatter::new();
        let omega = dm.calculate_relic_abundance(1e-26, 100.0);
        assert!(omega > 0.0);
    }
    #[test]
    fn test_detection_rate() {
        let dm = DarkMatter::new();
        let rate = dm.compute_detection_rate(1e4, 1e-45, 0.3);
        assert!(rate >= 0.0);
    }
}
