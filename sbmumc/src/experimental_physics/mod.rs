//! Experimental Physics Module
//!
//! This module implements experimental physics, particle detection,
//! and data analysis for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentalPhysics {
    pub exp_id: String,
    pub particle_detectors: Vec<ParticleDetector>,
    pub experiments: Vec<Experiment>,
    pub data_analysis: DataAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleDetector {
    pub detector_id: String,
    pub detector_name: String,
    pub detector_type: DetectorType,
    pub technology: String,
    pub spatial_resolution_mm: f64,
    pub time_resolution_ns: f64,
    pub efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DetectorType { Tracking, Calorimeter, Cherenkov, TransitionRadiation, Scintillator, Semiconductor }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub exp_id: String,
    pub exp_name: String,
    pub location: String,
    pub collaboration: String,
    pub energy_range_gev: [f64; 2],
    pub luminosity_cm2s: f64,
    pub key_results: Vec<KeyResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyResult { pub result_id: String, pub discovery: String, pub significance: f64, pub year: u32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAnalysis {
    pub analysis_methods: Vec<AnalysisMethod>,
    pub monte_carlo: MonteCarloSimulation,
    pub statistical_methods: StatisticalMethods,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMethod { pub method_id: String, pub method_name: String, pub purpose: String, pub software_tools: Vec<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonteCarloSimulation { pub mc_id: String, pub generator: String, pub event_count: u64, pub physics_validation: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalMethods {
    pub hypothesis_testing: HypothesisTesting,
    pub confidence_intervals: ConfidenceIntervals,
    pub systematic_errors: Vec<SystematicError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypothesisTesting { pub null_hypothesis: String, pub p_value: f64, pub significance_level: f64, pub test_statistic: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceIntervals { pub interval_type: String, pub coverage_probability: f64, pub interval_range: [f64; 2] }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystematicError { pub error_source: String, pub error_magnitude: f64, pub mitigation_strategy: String }

impl ExperimentalPhysics {
    pub fn new() -> Self {
        Self {
            exp_id: String::from("experimental_physics_v1"),
            particle_detectors: vec![
                ParticleDetector { detector_id: String::from("det_atlas"), detector_name: String::from("ATLAS Detector"), detector_type: DetectorType::Tracking, technology: String::from("Silicon pixels"), spatial_resolution_mm: 0.01, time_resolution_ns: 0.5, efficiency: 0.95 },
                ParticleDetector { detector_id: String::from("det_cms"), detector_name: String::from("CMS Detector"), detector_type: DetectorType::Calorimeter, technology: String::from("Lead tungstate crystals"), spatial_resolution_mm: 0.05, time_resolution_ns: 0.1, efficiency: 0.90 },
            ],
            experiments: vec![
                Experiment { exp_id: String::from("exp_lhc"), exp_name: String::from("Large Hadron Collider"), location: String::from("CERN, Geneva"), collaboration: String::from("ATLAS, CMS, LHCb"), energy_range_gev: [100.0, 14000.0], luminosity_cm2s: 2e34, key_results: vec![KeyResult { result_id: String::from("res_higgs"), discovery: String::from("Higgs boson"), significance: 5.0, year: 2012 }] },
            ],
            data_analysis: DataAnalysis {
                analysis_methods: vec![AnalysisMethod { method_id: String::from("method_1"), method_name: String::from("Maximum likelihood"), purpose: String::from("Parameter estimation"), software_tools: vec![String::from("ROOT")] }],
                monte_carlo: MonteCarloSimulation { mc_id: String::from("mc_1"), generator: String::from("MadGraph + Pythia"), event_count: 1e9, physics_validation: String::from("Validated") },
                statistical_methods: StatisticalMethods {
                    hypothesis_testing: HypothesisTesting { null_hypothesis: String::from("No signal"), p_value: 0.05, significance_level: 0.05, test_statistic: 1.96 },
                    confidence_intervals: ConfidenceIntervals { interval_type: String::from("Frequentist"), coverage_probability: 0.95, interval_range: [0.5, 1.5] },
                    systematic_errors: vec![SystematicError { error_source: String::from("Luminosity"), error_magnitude: 0.02, mitigation_strategy: String::from("Van der Meer scan") }],
                },
            },
        }
    }

    pub fn compute_significance(&self, signal: f64, background: f64, background_uncert: f64) -> f64 {
        if background > 0.0 && background_uncert > 0.0 { ((signal + background) * (1.0 + (signal / background).ln())).sqrt() } else { 0.0 }
    }

    pub fn compute_efficiency(&self, passed: u64, total: u64) -> f64 { if total > 0 { passed as f64 / total as f64 } else { 0.0 } }

    pub fn compute_upper_limit(&self, observed: u64, expected_bkg: f64, confidence_level: f64) -> UpperLimit {
        UpperLimit { cl: confidence_level, observed_limit: 10.0, expected_limit: 12.0, sigma_1_band: [8.0, 15.0], sigma_2_band: [6.0, 18.0] }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpperLimit { pub cl: f64, pub observed_limit: f64, pub expected_limit: f64, pub sigma_1_band: [f64; 2], pub sigma_2_band: [f64; 2] }

impl Default for ExperimentalPhysics { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_efficiency() { let ep = ExperimentalPhysics::new(); assert_eq!(ep.compute_efficiency(950, 1000), 0.95); } }
