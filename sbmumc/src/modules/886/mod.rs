//! # SBMUMC Module 886: Connected Vehicles
//! 
//! Vehicle-to-everything (V2X) communication systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// V2X message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum V2XMessageType {
    BSM,
    MAP,
    SPaT,
    TIM,
    PSM,
}

/// Communication protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationProtocol {
    DSRC,
    C_V2X,
    LTE_V2X,
    NR_V2X,
}

/// Basic safety message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicSafetyMessage {
    pub vehicle_id: String,
    pub timestamp: u64,
    pub position: (f64, f64, f64),
    pub heading: f64,
    pub speed: f64,
    pub acceleration: f64,
    pub size: (f64, f64),
}

/// SPaT message for traffic signals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPaTMessage {
    pub intersection_id: String,
    pub status: String,
    pub phases: Vec<PhaseState>,
}

/// Traffic signal phase state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseState {
    pub phase_number: u32,
    pub current_state: String,
    pub time_to_change_s: f64,
    pub min_time_remaining: f64,
}

impl ConnectedVehicles {
    /// Create new connected vehicles system
    pub fn new() -> Self {
        Self
    }

    /// Process BSM
    pub fn process_bsm(&self, bsm: &BasicSafetyMessage) -> Result<CollisionWarning> {
        let warning_level = if bsm.speed > 20.0 && bsm.acceleration.abs() > 3.0 {
            "high"
        } else if bsm.speed > 10.0 {
            "medium"
        } else {
            "low"
        };
        Ok(CollisionWarning {
            vehicle_id: bsm.vehicle_id.clone(),
            warning_level: warning_level.to_string(),
            time_to_collision_s: 10.0,
        })
    }

    /// Calculate vehicle awareness
    pub fn calculate_awareness(&self, nearby_bsms: &[BasicSafetyMessage]) -> Result<AwarenessMetrics> {
        Ok(AwarenessMetrics {
            vehicles_detected: nearby_bsms.len() as u32,
            awareness_range_m: 300.0,
            update_frequency_hz: 10.0,
            message_reception_rate: 0.95,
        })
    }

    /// Optimize communication scheduling
    pub fn optimize_scheduling(&self, vehicles: u32, channel_capacity: f64) -> Result<f64> {
        let interval_ms = (1000.0 / (channel_capacity / 200.0)).max(100.0);
        Ok(interval_ms)
    }
}

impl Default for ConnectedVehicles {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConnectedVehicles;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollisionWarning {
    pub vehicle_id: String,
    pub warning_level: String,
    pub time_to_collision_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwarenessMetrics {
    pub vehicles_detected: u32,
    pub awareness_range_m: f64,
    pub update_frequency_hz: f64,
    pub message_reception_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsm_processing() {
        let system = ConnectedVehicles::new();
        let bsm = BasicSafetyMessage {
            vehicle_id: "V001".to_string(),
            timestamp: 0,
            position: (40.0, -74.0, 0.0),
            heading: 90.0,
            speed: 15.0,
            acceleration: 2.0,
            size: (4.5, 1.8),
        };
        let warning = system.process_bsm(&bsm);
        assert!(warning.is_ok());
    }
}
