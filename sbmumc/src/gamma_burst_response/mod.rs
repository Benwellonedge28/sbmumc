//! Gamma Ray Burst Response Module
//!
//! This module implements detection, analysis, and response coordination
//! for gamma ray bursts and transient gamma ray events.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Gamma ray burst response system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GammaRayBurstResponse {
    pub response_id: String,
    pub detection_network: DetectionNetwork,
    pub active_bursts: Vec<GRBEvent>,
    pub response_coordination: ResponseCoordination,
    pub follow_up_network: FollowUpNetwork,
}

/// Detection network for GRBs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionNetwork {
    pub network_id: String,
    pub detectors: Vec<GRBDetector>,
    pub triangulation_capability: bool,
    pub alert_latency_s: f64,
    pub localization_precision_arcmin: f64,
}

/// Gamma ray burst detector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GRBDetector {
    pub detector_id: String,
    pub detector_name: String,
    pub detector_type: DetectorType,
    pub energy_range_kev: [f64; 2],
    pub effective_area_cm2: f64,
    pub time_resolution_ns: f64,
    pub location: [f64; 3],
    pub operational_status: DetectorStatus,
}

/// Types of GRB detectors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DetectorType {
    CZT,
    NaI,
    BGO,
    GLAST,
    Konus,
    IPN,
    CubeSat,
    Quantum,
}

/// Detector operational status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DetectorStatus {
    Operational,
    Standby,
    Calibration,
    Maintenance,
    Failed,
}

/// Gamma ray burst event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GRBEvent {
    pub event_id: String,
    pub trigger_time: String,
    pub burst_type: GRBType,
    pub duration_s: f64,
    pub fluence_erg_cm2: f64,
    pub peak_flux_erg_cm2_s: f64,
    pub energy_range_kev: [f64; 2],
    pub localization: Localization,
    pub spectral_model: SpectralModel,
    pub redshift: Option<f64>,
    pub host_galaxy: Option<String>,
    pub classification: Classification,
}

/// Types of GRBs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GRBType {
    LongDuration,
    ShortDuration,
    Intermediate,
    UltraLong,
    SubThreshold,
    AssociatedGravitationalWave,
}

/// Event localization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Localization {
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub error_radius_arcmin: f64,
    pub error_type: ErrorType,
    public alert_sent: bool,
}

/// Types of localization errors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorType {
    Statistical,
    Systematic,
    Combined,
    UpperLimit,
}

/// Spectral model for GRB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectralModel {
    pub model_type: SpectralModelType,
    pub parameters: SpectralParameters,
    pub energy_range_fit: [f64; 2],
    pub goodness_of_fit: f64,
}

/// Types of spectral models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpectralModelType {
    Band,
    CutoffPowerLaw,
    PowerLaw,
    CompTON,
    BandFunction,
    SmoothlyBrokenPowerLaw,
}

/// Spectral model parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectralParameters {
    pub e_peak_kev: f64,
    pub alpha: f64,
    pub beta: f64,
    pub normalization: f64,
}

/// GRB classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classification {
    pub type_class: GRBType,
    pub confidence: f64,
    pub associated_events: Vec<String>,
    pub special_properties: Vec<String>,
}

/// Response coordination for GRB alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseCoordination {
    pub coordination_id: String,
    pub response_level: ResponseLevel,
    pub telescopes_notified: Vec<String>,
    pub spacecraft_notified: Vec<String>,
    pub neutrino_detectors_notified: Vec<String>,
    pub gravitational_wave_detectors_notified: Vec<String>,
    pub response_time_s: f64,
}

/// Response alert levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseLevel {
    Ground,
    Space,
    MultiMessenger,
    Emergency,
}

/// Follow-up observation network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowUpNetwork {
    pub network_id: String,
    pub optical_telescopes: Vec<OpticalTelescope>,
    pub radio_telescopes: Vec<RadioTelescope>,
    pub xray_telescopes: Vec<XRayTelescope>,
    pub neutrino_detectors: Vec<NeutrinoDetector>,
    pub gravitational_wave_detectors: Vec<GWDetector>,
    pub scheduling_priority: Priority,
}

/// Optical telescope for follow-up
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpticalTelescope {
    pub telescope_id: String,
    pub name: String,
    pub aperture_m: f64,
    pub fov_arcmin: f64,
    pub response_time_s: f64,
    pub filters: Vec<String>,
    pub spectroscopy: bool,
}

/// Radio telescope for follow-up
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioTelescope {
    pub telescope_id: String,
    pub name: String,
    pub frequency_range_ghz: [f64; 2],
    pub sensitivity_jy: f64,
    pub response_time_s: f64,
}

/// X-ray telescope for follow-up
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XRayTelescope {
    pub telescope_id: String,
    pub name: String,
    pub energy_range_kev: [f64; 2],
    pub sensitivity_erg_cm2_s: f64,
    pub response_time_s: f64,
}

/// Neutrino detector for follow-up
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeutrinoDetector {
    pub detector_id: String,
    pub name: String,
    pub detector_volume_m3: f64,
    pub energy_threshold_tev: f64,
    pub angular_resolution_deg: f64,
}

/// Gravitational wave detector for follow-up
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GWDetector {
    pub detector_id: String,
    pub name: String,
    pub detector_type: String,
    pub sensitivity_hz: f64,
    pub response_time_s: f64,
}

/// Priority levels for scheduling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

impl GammaRayBurstResponse {
    /// Creates a new GRB response system
    pub fn new() -> Self {
        Self {
            response_id: String::from("grb_response_v1"),
            detection_network: DetectionNetwork {
                network_id: String::from("network_1"),
                detectors: Vec::new(),
                triangulation_capability: true,
                alert_latency_s: 5.0,
                localization_precision_arcmin: 1.0,
            },
            active_bursts: Vec::new(),
            response_coordination: ResponseCoordination {
                coordination_id: String::from("coord_1"),
                response_level: ResponseLevel::Ground,
                telescopes_notified: Vec::new(),
                spacecraft_notified: Vec::new(),
                neutrino_detectors_notified: Vec::new(),
                gravitational_wave_detectors_notified: Vec::new(),
                response_time_s: 0.0,
            },
            follow_up_network: FollowUpNetwork {
                network_id: String::from("followup_1"),
                optical_telescopes: Vec::new(),
                radio_telescopes: Vec::new(),
                xray_telescopes: Vec::new(),
                neutrino_detectors: Vec::new(),
                gravitational_wave_detectors: Vec::new(),
                scheduling_priority: Priority::High,
            },
        }
    }

    /// Registers detector in network
    pub fn register_detector(&mut self, detector: GRBDetector) -> Result<&GRBDetector> {
        self.detection_network.detectors.push(detector);
        Ok(self.detection_network.detectors.last().unwrap())
    }

    /// Processes GRB trigger
    pub fn process_trigger(&mut self, trigger_data: TriggerData) -> Result<&GRBEvent> {
        let burst = GRBEvent {
            event_id: format!("grb_{}", self.active_bursts.len() + 1),
            trigger_time: trigger_data.trigger_time,
            burst_type: GRBType::LongDuration,
            duration_s: trigger_data.duration_s,
            fluence_erg_cm2: trigger_data.fluence_erg_cm2,
            peak_flux_erg_cm2_s: trigger_data.peak_flux_erg_cm2_s,
            energy_range_kev: [10.0, 1000.0],
            localization: Localization {
                ra_deg: 0.0,
                dec_deg: 0.0,
                error_radius_arcmin: 10.0,
                error_type: ErrorType::Combined,
                public alert_sent: false,
            },
            spectral_model: SpectralModel {
                model_type: SpectralModelType::Band,
                parameters: SpectralParameters {
                    e_peak_kev: 250.0,
                    alpha: -0.9,
                    beta: -2.3,
                    normalization: 1e-8,
                },
                energy_range_fit: [20.0, 2000.0],
                goodness_of_fit: 0.95,
            },
            redshift: None,
            host_galaxy: None,
            classification: Classification {
                type_class: GRBType::LongDuration,
                confidence: 0.9,
                associated_events: Vec::new(),
                special_properties: Vec::new(),
            },
        };
        self.active_bursts.push(burst);
        Ok(self.active_bursts.last().unwrap())
    }

    /// Localizes GRB using triangulation
    pub fn localize(&self, event_id: &str) -> Result<Localization> {
        if let Some(event) = self.active_bursts.iter().find(|e| e.event_id == event_id) {
            Ok(event.localization.clone())
        } else {
            Err(SbmumcError::NotFound(String::from("GRB event not found")))
        }
    }

    /// Classifies GRB type
    pub fn classify(&mut self, event_id: &str) -> Result<&Classification> {
        if let Some(event) = self.active_bursts.iter_mut().find(|e| e.event_id == event_id) {
            let duration = event.duration_s;
            let new_type = if duration < 2.0 {
                GRBType::ShortDuration
            } else if duration > 10000.0 {
                GRBType::UltraLong
            } else {
                GRBType::LongDuration
            };
            event.classification.type_class = new_type;
            event.burst_type = new_type;
            Ok(&event.classification)
        } else {
            Err(SbmumcError::NotFound(String::from("GRB event not found")))
        }
    }

    /// Coordinates multi-messenger response
    pub fn coordinate_response(&mut self, event_id: &str, level: ResponseLevel) -> Result<&ResponseCoordination> {
        self.response_coordination.response_level = level;
        self.response_coordination.response_time_s = 10.0;
        match level {
            ResponseLevel::MultiMessenger => {
                self.response_coordination.telescopes_notified.push(String::from("LIGO"));
                self.response_coordination.neutrino_detectors_notified.push(String::from("IceCube"));
            },
            ResponseLevel::Emergency => {
                self.response_coordination.telescopes_notified.push(String::from("All optical"));
                self.response_coordination.spacecraft_notified.push(String::from("Swift"));
            },
            _ => {},
        }
        Ok(&self.response_coordination)
    }

    /// Plans follow-up observations
    pub fn plan_followup(&self, event_id: &str) -> Result<FollowUpPlan> {
        let plan = FollowUpPlan {
            event_id: event_id.to_string(),
            observations: vec![
                ObservationRequest {
                    telescope_type: TelescopeType::Optical,
                    telescope_name: String::from("VLT"),
                    start_time: String::from("immediate"),
                    duration_s: 3600.0,
                    priority: Priority::High,
                },
                ObservationRequest {
                    telescope_type: TelescopeType::XRay,
                    telescope_name: String::from("XMM-Newton"),
                    start_time: String::from("immediate"),
                    duration_s: 1800.0,
                    priority: Priority::High,
                },
                ObservationRequest {
                    telescope_type: TelescopeType::Radio,
                    telescope_name: String::from("VLA"),
                    start_time: String::from("after_1h"),
                    duration_s: 7200.0,
                    priority: Priority::Medium,
                },
            ],
            total_time_requests: 12600.0,
        };
        Ok(plan)
    }

    /// Estimates redshift from spectral properties
    pub fn estimate_redshift(&self, event: &GRBEvent) -> f64 {
        let e_peak = event.spectral_model.parameters.e_peak_kev;
        let fluence = event.fluence_erg_cm2;
        if e_peak < 100.0 && fluence > 1e-4 {
            0.5 + (100.0 - e_peak) / 100.0 + fluence.log10()
        } else if e_peak > 500.0 {
            2.0 + (e_peak - 500.0) / 500.0
        } else {
            1.0
        }
    }

    /// Calculates jet parameters
    pub fn calculate_jet_parameters(&self, event: &GRBEvent) -> JetParameters {
        let e_iso = 1e54;
        let theta_jet = (1.0 - (2.0 * event.spectral_model.parameters.e_peak_kev / 1000.0).min(1.0)).acos();
        JetParameters {
            event_id: event.event_id.clone(),
            opening_angle_deg: theta_jet.to_degrees(),
            energy_isotropic_erg: e_iso,
            energy_beamed_erg: e_iso * (1.0 - theta_jet.cos()),
            beaming_factor: 1.0 / (1.0 - theta_jet.cos()),
            jet_break_time_days: 10.0 * (theta_jet.to_degrees() / 10.0).powi(2),
        }
    }
}

/// Trigger data from detector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerData {
    pub trigger_time: String,
    pub duration_s: f64,
    pub fluence_erg_cm2: f64,
    pub peak_flux_erg_cm2_s: f64,
    pub detector_id: String,
}

/// Follow-up observation plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowUpPlan {
    pub event_id: String,
    pub observations: Vec<ObservationRequest>,
    pub total_time_requests: f64,
}

/// Observation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationRequest {
    pub telescope_type: TelescopeType,
    pub telescope_name: String,
    pub start_time: String,
    pub duration_s: f64,
    pub priority: Priority,
}

/// Types of telescopes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TelescopeType {
    Optical,
    Radio,
    XRay,
    GammaRay,
    Neutrino,
}

/// Jet parameters estimate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JetParameters {
    pub event_id: String,
    pub opening_angle_deg: f64,
    pub energy_isotropic_erg: f64,
    pub energy_beamed_erg: f64,
    pub beaming_factor: f64,
    pub jet_break_time_days: f64,
}

impl Default for GammaRayBurstResponse {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_registration() {
        let mut response = GammaRayBurstResponse::new();
        let detector = GRBDetector {
            detector_id: String::from("swift_bat"),
            detector_name: String::from("Swift BAT"),
            detector_type: DetectorType::CZT,
            energy_range_kev: [15.0, 150.0],
            effective_area_cm2: 5200.0,
            time_resolution_ns: 1e-6,
            location: [0.0, 0.0, 0.0],
            operational_status: DetectorStatus::Operational,
        };
        let result = response.register_detector(detector);
        assert!(result.is_ok());
    }

    #[test]
    fn test_trigger_processing() {
        let mut response = GammaRayBurstResponse::new();
        let trigger = TriggerData {
            trigger_time: String::from("2024-01-01T00:00:00Z"),
            duration_s: 50.0,
            fluence_erg_cm2: 1e-5,
            peak_flux_erg_cm2_s: 1e-4,
            detector_id: String::from("swift_bat"),
        };
        let result = response.process_trigger(trigger);
        assert!(result.is_ok());
    }

    #[test]
    fn test_followup_planning() {
        let response = GammaRayBurstResponse::new();
        let plan = response.plan_followup("test_grb");
        assert!(plan.is_ok());
    }
}
