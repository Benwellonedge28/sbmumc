//! Space Rescue Module (686)
//!
//! Emergency response, rescue operations, and safety systems for space missions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RescueType {
    InFlight,
    Orbital,
    Lunar,
    Mars,
    DeepSpace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceRescueSystem {
    pub rescue_system_name: String,
    pub rescue_type: RescueType,
    pub response_time_hours: f64,
    pub crew_capacity: u32,
    pub range_km: f64,
    pub rendezvous_capability: bool,
    pub medical_facilities: u8,
    pub recovery_method: String,
    pub success_rate: f64,              // percent
    pub training_level: String,
}

impl SpaceRescueSystem {
    pub fn new(rescue_system_name: String, rescue_type: RescueType) -> Self {
        Self {
            rescue_system_name,
            rescue_type,
            response_time_hours: 0.0,
            crew_capacity: 0,
            range_km: 0.0,
            rendezvous_capability: false,
            medical_facilities: 0,
            recovery_method: "Helicopter".into(),
            success_rate: 0.0,
            training_level: "Advanced".into(),
        }
    }

    pub fn rescue_window(&self) -> f64 {
        self.range_km / 28000.0 // approx km/s orbital velocity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_rescue() {
        let rescue = SpaceRescueSystem::new("ISS Rescue".into(), RescueType::Orbital);
        assert_eq!(rescue.rescue_system_name, "ISS Rescue");
    }
}
