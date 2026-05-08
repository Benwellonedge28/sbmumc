//! # SBMUMC Module 881: Rail Signaling
//! 
//! Railway signaling and train control systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// Signal aspects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalAspect {
    Clear,
    Caution,
    Stop,
    Restricting,
    CallOn,
}

/// Block types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockType {
    Fixed,
    Moving,
    Virtual,
}

/// Train detection method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetectionMethod {
    TrackCircuit,
    AxleCounter,
    Camera,
    Treadle,
}

/// Signal control point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalControlPoint {
    pub point_id: String,
    pub signals: Vec<SignalConfig>,
    pub switches: Vec<SwitchConfig>,
    pub detection_zones: Vec<DetectionZone>,
}

/// Signal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalConfig {
    pub signal_id: String,
    pub aspect: SignalAspect,
    pub route_associated: Option<String>,
    pub approach_control: bool,
    pub speed_indication: Option<u32>,
}

/// Route setting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteSetting {
    pub route_id: String,
    pub origin_point: String,
    pub destination_point: String,
    pub route_elements: Vec<RouteElement>,
    pub signal_aspects: Vec<SignalAspect>,
    pub route_locked: bool,
}

impl RailSignaling {
    /// Create new rail signaling system
    pub fn new() -> Self {
        Self
    }

    /// Set route
    pub fn set_route(&self, setting: &mut RouteSetting) -> Result<bool> {
        setting.route_locked = true;
        setting.signal_aspects = vec![SignalAspect::Clear];
        Ok(true)
    }

    /// Calculate braking curve
    pub fn calculate_braking_curve(&self, target: f64, current_speed: f64, gradient: f64) -> Result<BrakingCurve> {
        let service_deceleration = 0.8 - gradient * 0.01;
        let braking_distance = current_speed.powi(2) / (2.0 * service_deceleration * 9.81);
        Ok(BrakingCurve {
            warning_point: target - braking_distance * 1.5,
            braking_point: target - braking_distance,
            stop_point: target,
        })
    }

    /// Check train separation
    pub fn check_train_separation(&self, following_pos: f64, preceding_pos: f64, safe_distance: f64) -> Result<bool> {
        Ok(preceding_pos - following_pos >= safe_distance)
    }
}

impl Default for RailSignaling {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RailSignaling;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrakingCurve {
    pub warning_point: f64,
    pub braking_point: f64,
    pub stop_point: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchConfig {
    pub switch_id: String,
    pub normal_position: String,
    pub reverse_position: String,
    pub current_position: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionZone {
    pub zone_id: String,
    pub start_m: f64,
    pub end_m: f64,
    pub detection_type: DetectionMethod,
    pub occupied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteElement {
    pub element_type: String,
    pub element_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braking_curve() {
        let system = RailSignaling::new();
        let curve = system.calculate_braking_curve(1000.0, 30.0, 0.0);
        assert!(curve.is_ok());
    }

    #[test]
    fn test_train_separation() {
        let system = RailSignaling::new();
        let separated = system.check_train_separation(500.0, 1200.0, 500.0);
        assert!(separated.is_ok());
        assert!(separated.unwrap());
    }
}
