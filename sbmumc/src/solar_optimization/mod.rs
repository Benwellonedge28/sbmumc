//! Solar Optimization Module
//!
//! This module implements solar activity prediction, space weather forecasting,
//! and solar energy optimization for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Solar optimization and prediction system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarOptimization {
    pub optimization_id: String,
    pub solar_activity: SolarActivityMonitor,
    pub space_weather: SpaceWeatherForecast,
    pub energy_optimization: EnergyOptimization,
    pub prediction_models: Vec<PredictionModel>,
}

/// Solar activity monitoring system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarActivityMonitor {
    pub monitor_id: String,
    pub current_activity_level: ActivityLevel,
    pub sunspot_number: u32,
    pub solar_flux_10cm: f64,
    pub flare_activity: FlareActivity,
    pub cme_events: Vec<CMEEvent>,
    pub coronal_holes: Vec<CoronalHole>,
    pub monitoring_network: Vec<MonitoringStation>,
}

/// Solar activity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActivityLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
    Extreme,
}

/// Flare activity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlareActivity {
    pub total_flares_24h: u32,
    pub c_flares: u32,
    pub m_flares: u32,
    pub x_flares: u32,
    pub most_recent_flare: Option<FlareEvent>,
}

/// Solar flare event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlareEvent {
    pub flare_id: String,
    pub peak_time: String,
    pub classification: FlareClass,
    pub location: [f64; 2],
    pub duration_s: f64,
    pub associated_cme: Option<String>,
}

/// Solar flare classifications
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FlareClass {
    A,
    B,
    C,
    M,
    X,
    X10,
    X20,
    X30,
    X40,
    X50,
}

/// Coronal mass ejection event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CMEEvent {
    pub cme_id: String,
    pub detection_time: String,
    pub speed_km_s: f64,
    pub angular_width_deg: f64,
    pub position_angle_deg: f64,
    pub mass_estimate_kg: f64,
    pub energy_estimate_j: f64,
    pub冲向地球: bool,
    pub arrival_prediction: Option<String>,
}

/// Coronal hole on sun
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoronalHole {
    pub hole_id: String,
    pub location: [f64; 2],
    pub area_deg2: f64,
    pub polarity: String,
    pub speed_km_s: f64,
    pub earth_impact: bool,
}

/// Monitoring station for solar activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringStation {
    pub station_id: String,
    pub name: String,
    pub location: [f64; 3],
    pub instruments: Vec<Instrument>,
    pub data_latency_s: f64,
}

/// Instrument at monitoring station
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub instrument_id: String,
    pub instrument_type: InstrumentType,
    pub wavelength_angstrom: f64,
    pub cadence_s: f64,
    pub spatial_resolution_arcsec: f64,
}

/// Types of solar monitoring instruments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstrumentType {
    H-alphaTelescope,
    WhiteLightTelescope,
    EUVTelescope,
    XRayTelescope,
    Coronagraph,
    Magnetograph,
    Spectrograph,
    Radiometer,
}

/// Space weather forecasting system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceWeatherForecast {
    pub forecast_id: String,
    pub forecast_period: ForecastPeriod,
    pub geomagnetic_activity: GeomagneticForecast,
    pub radiation_storm: RadiationForecast,
    pub radio_blackout: RadioBlackoutForecast,
    pub solar_wind: SolarWindForecast,
}

/// Forecast time periods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastPeriod {
    pub nowcast: bool,
    pub hourly_forecast: usize,
    pub daily_forecast: usize,
    pub weekly_forecast: usize,
}

/// Geomagnetic activity forecast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeomagneticForecast {
    pub kp_forecast: Vec<KpValue>,
    pub ap_forecast: Vec<f64>,
    pub conditions: Vec<String>,
    pub aurora_visibility: Vec<bool>,
}

/// Kp index value with time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KpValue {
    pub time: String,
    pub kp: u32,
    pub kp_description: KpDescription,
}

/// Kp index descriptions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KpDescription {
    Quiet,
    Unsettled,
    Active,
    G1Minor,
    G2Moderate,
    G3Strong,
    G4Severe,
    G5Extreme,
}

/// Radiation storm forecast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationForecast {
    pub proton_event_probability: f64,
    pub proton_flux_threshold: f64,
    pub radiation_alert_level: RadiationLevel,
    pub solar particle event: Option<ParticleEvent>,
}

/// Radiation alert levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RadiationLevel {
    Green,
    Yellow,
    Orange,
    Red,
}

/// Solar particle event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleEvent {
    pub event_id: String,
    pub start_time: String,
    pub peak_time: String,
    pub end_time: Option<String>,
    pub peak_flux_pfu: f64,
    pub energy_range_mev: [f64; 2],
    pub duration_h: f64,
}

/// Radio blackout forecast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioBlackoutForecast {
    pub blackout_probability: f64,
    pub affected_frequencies: Vec<String>,
    pub duration_min: f64,
    pub associated_flares: Vec<String>,
}

/// Solar wind forecast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarWindForecast {
    pub velocity_km_s: f64,
    pub density_pcm3: f64,
    pub magnetic_field_strength_nt: f64,
    pub magnetic_field_direction: String,
    pub speed_ram: bool,
    pub arrival_time: Option<String>,
}

/// Energy optimization for solar infrastructure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyOptimization {
    pub optimization_id: String,
    pub generation_optimization: GenerationOptimization,
    pub grid_integration: GridIntegration,
    pub storage_optimization: StorageOptimization,
}

/// Solar generation optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationOptimization {
    pub current_generation_mw: f64,
    pub predicted_generation_mw: f64,
    pub capacity_factor: f64,
    pub efficiency_loss: f64,
    pub panel_orientation_optimization: Vec<OrientationAdjustment>,
}

/// Panel orientation adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrientationAdjustment {
    pub panel_id: String,
    pub azimuth_adjustment_deg: f64,
    pub elevation_adjustment_deg: f64,
    pub expected_gain_percentage: f64,
}

/// Grid integration for solar power
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridIntegration {
    pub grid_connection_capacity_mw: f64,
    pub current_feed_mw: f64,
    pub grid_stability_factor: f64,
    pub curtailment_risk: f64,
    pub demand_matching: f64,
}

/// Storage optimization for excess generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageOptimization {
    pub storage_capacity_mwh: f64,
    pub current_charge_level_mwh: f64,
    pub charge_rate_mw: f64,
    pub discharge_rate_mw: f64,
    pub round_trip_efficiency: f64,
    pub optimization_strategy: OptimizationStrategy,
}

/// Storage optimization strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptimizationStrategy {
    PeakShaving,
    LoadBalancing,
    DemandResponse,
    EmergencyReserve,
    Economic,
    TimeShift,
}

/// Prediction model for solar activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionModel {
    pub model_id: String,
    pub model_type: ModelType,
    pub prediction_target: PredictionTarget,
    pub accuracy: f64,
    pub prediction_horizon_h: f64,
    pub last_trained: String,
}

/// Types of prediction models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelType {
    MachineLearning,
    NeuralNetwork,
    DeepLearning,
    PhysicalBased,
    Statistical,
    Ensemble,
    Quantum,
}

/// Prediction target variables
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PredictionTarget {
    SunspotNumber,
    FlareActivity,
    CMESpeed,
    GeomagneticIndex,
    SolarWindSpeed,
    TotalIrradiance,
    GenerationOutput,
}

impl SolarOptimization {
    /// Creates a new solar optimization system
    pub fn new() -> Self {
        Self {
            optimization_id: String::from("solar_optimization_v1"),
            solar_activity: SolarActivityMonitor {
                monitor_id: String::from("monitor_1"),
                current_activity_level: ActivityLevel::Medium,
                sunspot_number: 50,
                solar_flux_10cm: 150.0,
                flare_activity: FlareActivity {
                    total_flares_24h: 5,
                    c_flares: 4,
                    m_flares: 1,
                    x_flares: 0,
                    most_recent_flare: None,
                },
                cme_events: Vec::new(),
                coronal_holes: Vec::new(),
                monitoring_network: Vec::new(),
            },
            space_weather: SpaceWeatherForecast {
                forecast_id: String::from("forecast_1"),
                forecast_period: ForecastPeriod {
                    nowcast: true,
                    hourly_forecast: 24,
                    daily_forecast: 7,
                    weekly_forecast: 4,
                },
                geomagnetic_activity: GeomagneticForecast {
                    kp_forecast: Vec::new(),
                    ap_forecast: Vec::new(),
                    conditions: vec![String::from("Quiet")],
                    aurora_visibility: vec![false; 24],
                },
                radiation_storm: RadiationForecast {
                    proton_event_probability: 0.1,
                    proton_flux_threshold: 10.0,
                    radiation_alert_level: RadiationLevel::Green,
                    solar particle event: None,
                },
                radio_blackout: RadioBlackoutForecast {
                    blackout_probability: 0.05,
                    affected_frequencies: vec![String::from("HF"), String::from("VHF")],
                    duration_min: 30.0,
                    associated_flares: Vec::new(),
                },
                solar_wind: SolarWindForecast {
                    velocity_km_s: 400.0,
                    density_pcm3: 5.0,
                    magnetic_field_strength_nt: 5.0,
                    magnetic_field_direction: String::from("Southward"),
                    speed_ram: false,
                    arrival_time: None,
                },
            },
            energy_optimization: EnergyOptimization {
                optimization_id: String::from("energy_opt_1"),
                generation_optimization: GenerationOptimization {
                    current_generation_mw: 1000.0,
                    predicted_generation_mw: 950.0,
                    capacity_factor: 0.25,
                    efficiency_loss: 0.05,
                    panel_orientation_optimization: Vec::new(),
                },
                grid_integration: GridIntegration {
                    grid_connection_capacity_mw: 2000.0,
                    current_feed_mw: 1000.0,
                    grid_stability_factor: 0.95,
                    curtailment_risk: 0.1,
                    demand_matching: 0.9,
                },
                storage_optimization: StorageOptimization {
                    storage_capacity_mwh: 500.0,
                    current_charge_level_mwh: 250.0,
                    charge_rate_mw: 200.0,
                    discharge_rate_mw: 200.0,
                    round_trip_efficiency: 0.9,
                    optimization_strategy: OptimizationStrategy::Economic,
                },
            },
            prediction_models: Vec::new(),
        }
    }

    /// Predicts solar flare activity
    pub fn predict_flares(&self, horizon_h: u32) -> FlarePrediction {
        let base_probability = self.solar_activity.sunspot_number as f64 / 200.0;
        FlarePrediction {
            prediction_time: String::from("2024-01-01"),
            horizon_hours: horizon_h,
            c_flare_probability: (base_probability * 1.5).min(1.0),
            m_flare_probability: (base_probability * 0.5).min(1.0),
            x_flare_probability: (base_probability * 0.1).min(1.0),
            expected_total: (base_probability * 10.0) as u32,
            most_likely_location: [45.0, 10.0],
            confidence: 0.8,
        }
    }

    /// Predicts geomagnetic activity
    pub fn predict_geomagnetic(&self, horizon_h: u32) -> GeomagneticPrediction {
        let has_cme = !self.solar_activity.cme_events.is_empty();
        let kp_baseline = if has_cme { 5 } else { 2 };
        GeomagneticPrediction {
            prediction_time: String::from("2024-01-01"),
            horizon_hours: horizon_h,
            kp_prediction: kp_baseline as u32,
            kp_range: [kp_baseline - 1, kp_baseline + 2],
            aurora_visibility_latitude_deg: 65.0 - (kp_baseline as f64 * 3.0),
            confidence: if has_cme { 0.7 } else { 0.85 },
        }
    }

    /// Optimizes solar panel orientation
    pub fn optimize_orientation(&self, latitude: f64, time_of_day: f64) -> OrientationAdjustment {
        let optimal_azimuth = 180.0;
        let optimal_elevation = (90.0 - latitude).abs() + (time_of_day / 12.0 * 23.0 - 90.0).abs() * 0.3;
        OrientationAdjustment {
            panel_id: String::from("optimized_panel"),
            azimuth_adjustment_deg: optimal_azimuth,
            elevation_adjustment_deg: optimal_elevation.max(0.0),
            expected_gain_percentage: 0.25,
        }
    }

    /// Calculates solar generation potential
    pub fn calculate_generation(&self, irradiance_wm2: f64, panel_area_m2: f64, efficiency: f64) -> f64 {
        irradiance_wm2 * panel_area_m2 * efficiency
    }

    /// Predicts CME arrival
    pub fn predict_cme_arrival(&self, cme_id: &str) -> Result<CMEArrivalPrediction> {
        if let Some(cme) = self.solar_activity.cme_events.iter().find(|c| c.cme_id == cme_id) {
            let transit_time_h = 4e6 / cme.speed_km_s;
            let arrival_time_h = transit_time_h / 24.0;
            let prediction = CMEArrivalPrediction {
                cme_id: cme_id.to_string(),
                predicted_arrival: String::from("2024-01-02"),
                transit_time_hours: transit_time_h,
                expected_speed_km_s: cme.speed_km_s * 0.9,
                expected_kp: if cme.冲向地球 { 5 } else { 2 },
                confidence: 0.75,
            };
            Ok(prediction)
        } else {
            Err(SbmumcError::NotFound(String::from("CME not found")))
        }
    }

    /// Optimizes energy storage
    pub fn optimize_storage(&self, load_profile: &[f64], generation_profile: &[f64]) -> StorageOptimizationResult {
        let total_load: f64 = load_profile.iter().sum();
        let total_generation: f64 = generation_profile.iter().sum();
        let excess = (total_generation - total_load).max(0.0);
        let deficit = (total_load - total_generation).max(0.0);
        StorageOptimizationResult {
            optimal_charge_level_mwh: excess.min(self.energy_optimization.storage_optimization.storage_capacity_mwh),
            optimal_discharge_level_mwh: deficit,
            peak_shaving_potential_mw: (excess - deficit).max(0.0) / 24.0,
            economic_value: (excess - deficit) * 0.10,
            recommendations: vec![
                String::from("Charge during peak generation"),
                String::from("Discharge during peak demand"),
            ],
        }
    }
}

/// Flare prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlarePrediction {
    pub prediction_time: String,
    pub horizon_hours: u32,
    pub c_flare_probability: f64,
    pub m_flare_probability: f64,
    pub x_flare_probability: f64,
    pub expected_total: u32,
    pub most_likely_location: [f64; 2],
    pub confidence: f64,
}

/// Geomagnetic activity prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeomagneticPrediction {
    pub prediction_time: String,
    pub horizon_hours: u32,
    pub kp_prediction: u32,
    pub kp_range: [u32; 2],
    pub aurora_visibility_latitude_deg: f64,
    pub confidence: f64,
}

/// CME arrival time prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CMEArrivalPrediction {
    pub cme_id: String,
    pub predicted_arrival: String,
    pub transit_time_hours: f64,
    pub expected_speed_km_s: f64,
    pub expected_kp: u32,
    pub confidence: f64,
}

/// Storage optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageOptimizationResult {
    pub optimal_charge_level_mwh: f64,
    pub optimal_discharge_level_mwh: f64,
    pub peak_shaving_potential_mw: f64,
    pub economic_value: f64,
    pub recommendations: Vec<String>,
}

impl Default for SolarOptimization {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flare_prediction() {
        let solar = SolarOptimization::new();
        let prediction = solar.predict_flares(24);
        assert!(prediction.c_flare_probability > 0.0);
    }

    #[test]
    fn test_orientation_optimization() {
        let solar = SolarOptimization::new();
        let orientation = solar.optimize_orientation(45.0, 12.0);
        assert!(orientation.expected_gain_percentage > 0.0);
    }

    #[test]
    fn test_generation_calculation() {
        let solar = SolarOptimization::new();
        let generation = solar.calculate_generation(1000.0, 100.0, 0.2);
        assert_eq!(generation, 20000.0);
    }
}
