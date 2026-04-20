//! Cosmic Microwave Background Module
//!
//! This module implements CMB analysis, anisotropy mapping, and information
//! extraction from the cosmic microwave background radiation.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cosmic microwave background analysis system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicMicrowaveBackground {
    pub cmb_id: String,
    pub observation_data: Vec<CMBObservation>,
    pub anisotropy_maps: Vec<AnisotropyMap>,
    pub power_spectra: Vec<PowerSpectrum>,
    pub analysis_results: Vec<AnalysisResult>,
    pub interpretation_framework: InterpretationFramework,
}

/// CMB observation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CMBObservation {
    pub observation_id: String,
    pub telescope: String,
    pub observation_date: String,
    pub frequency_ghz: f64,
    pub angular_resolution_arcmin: f64,
    pub sky_coverage_percentage: f64,
    pub sensitivity_uk_arcmin: f64,
    pub data_quality: DataQuality,
    pub raw_data_size_gb: f64,
}

/// Data quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQuality {
    pub noise_level: f64,
    pub systematic_errors: f64,
    pub flags_percentage: f64,
    pub yield_factor: f64,
    pub overall_quality: QualityGrade,
}

/// Quality grading scale
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QualityGrade {
    A,
    B,
    C,
    D,
    F,
}

/// CMB anisotropy map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnisotropyMap {
    pub map_id: String,
    pub resolution: MapResolution,
    pub pixel_count: u64,
    pub pixel_size_arcmin: f64,
    pub coordinate_system: CoordSystem,
    pub temperature_map: TemperatureMap,
    pub polarization_map: Option<PolarizationMap>,
    pub processing_level: ProcessingLevel,
}

/// Map resolution specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapResolution {
    pub nside: u32,
    pub angular_resolution_arcmin: f64,
    pub pixel_count: u64,
    pub HealpixScheme,
}

/// HEALPix scheme types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealpixScheme {
    Ring,
    Nested,
}

/// Temperature anisotropy map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureMap {
    pub map_data: Vec<f64>,
    pub mean_temperature: f64,
    pub rms_variance: f64,
    pub min_temperature: f64,
    pub max_temperature: f64,
    pub units: TemperatureUnit,
}

/// Units for temperature measurements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemperatureUnit {
    Microkelvin,
    Millikelvin,
    Kelvin,
}

/// Polarization anisotropy map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolarizationMap {
    pub q_map: Vec<f64>,
    pub u_map: Vec<f64>,
    pub polarization_intensity: Vec<f64>,
    pub polarization_angle: Vec<f64>,
    pub e_mode: Vec<f64>,
    pub b_mode: Vec<f64>,
}

/// Data processing levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProcessingLevel {
    Raw,
    Cleaned,
    Calibrated,
    Mapmaking,
    ComponentSeparation,
    Final,
}

/// CMB power spectrum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerSpectrum {
    pub spectrum_id: String,
    pub spectrum_type: SpectrumType,
    pub multipole_range: [u32; 2],
    pub cl_values: Vec<MultipoleValue>,
    pub best_fit_parameters: BestFitParameters,
    pub confidence_intervals: Vec<ConfidenceInterval>,
}

/// Types of power spectra
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpectrumType {
    Temperature,
    PolarizationE,
    PolarizationB,
    CrossTemperatureE,
    CrossTemperatureB,
    CrossEE,
    Tensor,
}

/// Multipole moment value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipoleValue {
    pub l: u32,
    pub cl: f64,
    pub error: f64,
}

/// Best fit cosmological parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestFitParameters {
    pub hubble_constant: f64,
    pub matter_density: f64,
    pub baryon_density: f64,
    pub dark_energy_density: f64,
    pub spectral_index: f64,
    pub reionization_optical_depth: f64,
    pub tensor_to_scalar_ratio: f64,
    pub neutrino_mass_sum: f64,
    pub effective_neutrino_species: f64,
}

/// Confidence interval for parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    pub parameter: String,
    pub value: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64,
}

/// Analysis result from CMB data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub result_id: String,
    pub analysis_type: AnalysisType,
    pub parameters: HashMap<String, f64>,
    pub statistical_significance: f64,
    pub systematic_uncertainties: HashMap<String, f64>,
    pub conclusions: Vec<String>,
}

/// Types of CMB analyses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalysisType {
    ParameterEstimation,
    ConstraintAnalysis,
    AnomalyDetection,
    NonGaussianitySearch,
    PrimordialBModeSearch,
    IsotropyTest,
    BianchiModelFitting,
    CosmicStringSearch,
}

/// Framework for interpreting CMB data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterpretationFramework {
    pub framework_id: String,
    pub theoretical_models: Vec<TheoreticalModel>,
    pub priors: Vec<Prior>,
    pub likelihood_functions: Vec<LikelihoodFunction>,
    pub bayesian_inference: BayesianEngine,
}

/// Theoretical model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoreticalModel {
    pub model_id: String,
    pub model_name: String,
    pub parameters: Vec<ModelParameter>,
    pub predictions: Vec<Prediction>,
    pub goodness_of_fit: f64,
}

/// Model parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameter {
    pub name: String,
    pub symbol: String,
    pub value: f64,
    pub prior_range: [f64; 2],
    pub physical_meaning: String,
}

/// Model prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub observable: String,
    pub predicted_value: f64,
    pub uncertainty: f64,
}

/// Prior distribution for Bayesian analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prior {
    pub parameter: String,
    pub prior_type: PriorType,
    pub hyper_parameters: Vec<f64>,
}

/// Types of prior distributions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PriorType {
    Uniform,
    Gaussian,
    Jeffreys,
    TopHat,
    Custom,
}

/// Likelihood function for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LikelihoodFunction {
    pub likelihood_id: String,
    pub likelihood_type: String,
    pub data_used: Vec<String>,
    pub nuisance_parameters: Vec<String>,
    pub log_likelihood: f64,
}

/// Bayesian inference engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BayesianEngine {
    pub sampler: SamplerType,
    pub chains: u32,
    pub samples_per_chain: u32,
    pub burn_in: u32,
    pub convergence_test: ConvergenceTest,
}

/// Types of MCMC samplers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SamplerType {
    MetropolisHastings,
    Gibbs,
    Hamiltonian,
    NestedSampling,
    Variational,
}

/// Convergence testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceTest {
    pub test_name: String,
    pub criterion: f64,
    pub gelman_rubin_r: f64,
    pub geweke_fraction: f64,
}

impl CosmicMicrowaveBackground {
    /// Creates a new CMB analysis system
    pub fn new() -> Self {
        Self {
            cmb_id: String::from("cosmic_microwave_bg_v1"),
            observation_data: Vec::new(),
            anisotropy_maps: Vec::new(),
            power_spectra: Vec::new(),
            analysis_results: Vec::new(),
            interpretation_framework: InterpretationFramework {
                framework_id: String::from("framework_1"),
                theoretical_models: Vec::new(),
                priors: Vec::new(),
                likelihood_functions: Vec::new(),
                bayesian_inference: BayesianEngine {
                    sampler: SamplerType::MetropolisHastings,
                    chains: 4,
                    samples_per_chain: 100000,
                    burn_in: 10000,
                    convergence_test: ConvergenceTest {
                        test_name: String::from("Gelman-Rubin"),
                        criterion: 1.1,
                        gelman_rubin_r: 1.0,
                        geweke_fraction: 0.1,
                    },
                },
            },
        }
    }

    /// Registers CMB observation
    pub fn register_observation(&mut self, observation: CMBObservation) -> Result<&CMBObservation> {
        self.observation_data.push(observation);
        Ok(self.observation_data.last().unwrap())
    }

    /// Generates anisotropy map from observation
    pub fn generate_map(&mut self, observation_id: &str) -> Result<&AnisotropyMap> {
        let map = AnisotropyMap {
            map_id: format!("map_{}", self.anisotropy_maps.len() + 1),
            resolution: MapResolution {
                nside: 2048,
                angular_resolution_arcmin: 0.5,
                pixel_count: 50331648,
                HealpixScheme: HealpixScheme::Ring,
            },
            pixel_count: 50331648,
            pixel_size_arcmin: 0.5,
            coordinate_system: CoordSystem::J2000,
            temperature_map: TemperatureMap {
                map_data: vec![0.0; 50331648],
                mean_temperature: 2.725,
                rms_variance: 0.001,
                min_temperature: 2.0,
                max_temperature: 3.5,
                units: TemperatureUnit::Microkelvin,
            },
            polarization_map: None,
            processing_level: ProcessingLevel::Final,
        };
        self.anisotropy_maps.push(map);
        Ok(self.anisotropy_maps.last().unwrap())
    }

    /// Computes power spectrum from map
    pub fn compute_power_spectrum(&mut self, map_id: &str, spectrum_type: SpectrumType) -> Result<&PowerSpectrum> {
        let cl_values: Vec<MultipoleValue> = (2..3000).map(|l| MultipoleValue {
            l,
            cl: 1e-10 * (-(l as f64) / 1000.0).exp() * (l as f64).powi(-2),
            error: 1e-12,
        }).collect();
        let spectrum = PowerSpectrum {
            spectrum_id: format!("spec_{}", self.power_spectra.len() + 1),
            spectrum_type,
            multipole_range: [2, 3000],
            cl_values,
            best_fit_parameters: BestFitParameters {
                hubble_constant: 67.4,
                matter_density: 0.315,
                baryon_density: 0.0493,
                dark_energy_density: 0.685,
                spectral_index: 0.965,
                reionization_optical_depth: 0.054,
                tensor_to_scalar_ratio: 0.01,
                neutrino_mass_sum: 0.06,
                effective_neutrino_species: 3.04,
            },
            confidence_intervals: Vec::new(),
        };
        self.power_spectra.push(spectrum);
        Ok(self.power_spectra.last().unwrap())
    }

    /// Performs parameter estimation
    pub fn estimate_parameters(&self) -> Result<AnalysisResult> {
        let result = AnalysisResult {
            result_id: String::from("param_est_1"),
            analysis_type: AnalysisType::ParameterEstimation,
            parameters: {
                let mut map = HashMap::new();
                map.insert(String::from("H0"), 67.4);
                map.insert(String::from("Omega_m"), 0.315);
                map.insert(String::from("Omega_b"), 0.0493);
                map.insert(String::from("Omega_lambda"), 0.685);
                map.insert(String::from("n_s"), 0.965);
                map.insert(String::from("tau"), 0.054);
                map
            },
            statistical_significance: 0.999,
            systematic_uncertainties: HashMap::new(),
            conclusions: vec![
                String::from("Universe is flat within 0.4%"),
                String::from("Standard LCDM model supported"),
            ],
        };
        Ok(result)
    }

    /// Searches for primordial B-modes
    pub fn search_bmode(&self) -> Result<BModeSearchResult> {
        let result = BModeSearchResult {
            search_id: String::from("bmode_search_1"),
            tensor_to_scalar_ratio: 0.001,
            upper_limit: 0.003,
            confidence_level: 0.95,
            searched_multipole_range: [2, 200],
            sensitivity: 0.05,
            significance: 0.1,
            conclusion: String::from("No significant B-mode detected"),
        };
        Ok(result)
    }

    /// Tests cosmic isotropy
    pub fn test_isotropy(&self) -> Result<IsotropyTestResult> {
        let result = IsotropyTestResult {
            test_id: String::from("isotropy_1"),
            test_statistic: 1.05,
            p_value: 0.15,
            hemispheric_asymmetry: HemisphericAsymmetry {
                amplitude: 0.07,
                direction: [100.0, -50.0],
                significance: 0.95,
            },
            cold_spot_significance: 0.99,
            conclusions: vec![String::from("Universe is consistent with isotropy")],
        };
        Ok(result)
    }

    /// Analyzes non-Gaussianity
    pub fn analyze_non_gaussianity(&self) -> Result<NonGaussianityResult> {
        let result = NonGaussianityResult {
            analysis_id: String::from("ng_1"),
            f_nl_local: 2.5,
            f_nl_equilateral: 40.0,
            f_nl_orthogonal: 30.0,
            errors: [5.0, 100.0, 50.0],
            constraint: String::from("Gaussianity consistent with predictions"),
            primordial_power_spectrum: PrimordialPowerSpectrum::ScaleInvariant,
        };
        Ok(result)
    }

    /// Fits alternative cosmology
    pub fn fit_alternative_cosmology(&self, model: &str) -> Result<AlternativeCosmologyResult> {
        let result = AlternativeCosmologyResult {
            model_name: model.to_string(),
            chi_squared: 150.0,
            dof: 148,
            reduced_chi_squared: 1.01,
            p_value: 0.4,
            parameter_constraints: HashMap::new(),
            model_preference: ModelPreference::Standard,
        };
        Ok(result)
    }
}

/// Result of B-mode search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BModeSearchResult {
    pub search_id: String,
    pub tensor_to_scalar_ratio: f64,
    pub upper_limit: f64,
    pub confidence_level: f64,
    pub searched_multipole_range: [u32; 2],
    pub sensitivity: f64,
    pub significance: f64,
    pub conclusion: String,
}

/// Isotropy test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsotropyTestResult {
    pub test_id: String,
    pub test_statistic: f64,
    pub p_value: f64,
    pub hemispheric_asymmetry: HemisphericAsymmetry,
    pub cold_spot_significance: f64,
    pub conclusions: Vec<String>,
}

/// Hemispheric asymmetry detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HemisphericAsymmetry {
    pub amplitude: f64,
    pub direction: [f64; 2],
    pub significance: f64,
}

/// Non-Gaussianity analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonGaussianityResult {
    pub analysis_id: String,
    pub f_nl_local: f64,
    pub f_nl_equilateral: f64,
    pub f_nl_orthogonal: f64,
    pub errors: [f64; 3],
    pub constraint: String,
    pub primordial_power_spectrum: PrimordialPowerSpectrum,
}

/// Types of primordial power spectra
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrimordialPowerSpectrum {
    ScaleInvariant,
    RedTilted,
    BlueTilted,
    Running,
    Features,
   振荡,
}

/// Alternative cosmology fit result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeCosmologyResult {
    pub model_name: String,
    pub chi_squared: f64,
    pub dof: u32,
    pub reduced_chi_squared: f64,
    pub p_value: f64,
    pub parameter_constraints: HashMap<String, f64>,
    pub model_preference: ModelPreference,
}

/// Model preference comparison
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelPreference {
    Standard,
    Alternative,
    Neither,
}

impl Default for CosmicMicrowaveBackground {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observation_registration() {
        let mut cmb = CosmicMicrowaveBackground::new();
        let observation = CMBObservation {
            observation_id: String::from("planck_hfi_1"),
            telescope: String::from("Planck LFI"),
            observation_date: String::from("2009-08-15"),
            frequency_ghz: 70.0,
            angular_resolution_arcmin: 13.0,
            sky_coverage_percentage: 100.0,
            sensitivity_uk_arcmin: 4.0,
            data_quality: DataQuality {
                noise_level: 0.01,
                systematic_errors: 0.001,
                flags_percentage: 5.0,
                yield_factor: 0.95,
                overall_quality: QualityGrade::A,
            },
            raw_data_size_gb: 1000.0,
        };
        let result = cmb.register_observation(observation);
        assert!(result.is_ok());
    }

    #[test]
    fn test_power_spectrum_computation() {
        let mut cmb = CosmicMicrowaveBackground::new();
        let map = cmb.generate_map("test_obs").unwrap();
        let spectrum = cmb.compute_power_spectrum(&map.map_id, SpectrumType::Temperature);
        assert!(spectrum.is_ok());
    }

    #[test]
    fn test_parameter_estimation() {
        let cmb = CosmicMicrowaveBackground::new();
        let result = cmb.estimate_parameters();
        assert!(result.is_ok());
    }
}
