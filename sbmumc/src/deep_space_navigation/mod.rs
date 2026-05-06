//! Deep Space Navigation Module (667)
//!
//! Interplanetary navigation, trajectory determination, and autonomous navigation systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NavigationMethod {
    GroundTracking,
    OpticalNavigation,
    RadioNavigation,
    Autonomous,
    PulsarNavigation,
    MultiMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSpaceNavigation {
    pub navigation_system: String,
    pub nav_method: NavigationMethod,
    pub position_accuracy: f64,       // km
    pub velocity_accuracy: f64,      // km/s
    pub update_rate: f64,            // per day
    pub autonomous_level: f64,       // percent
    pub star_catalog_size: u32,
    pub trajectory_corrections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryDetermination {
    pub trajectory_id: String,
    pub departure_body: String,
    pub arrival_body: String,
    pub departure_date: f64,          // MJD
    pub arrival_date: f64,            // MJD
    pub delta_v_total: f64,           // km/s
    pub trajectory_type: String,
    pub midcourse_corrections: u32,
}

impl DeepSpaceNavigation {
    pub fn new(navigation_system: String, nav_method: NavigationMethod) -> Self {
        Self {
            navigation_system,
            nav_method,
            position_accuracy: 0.0,
            velocity_accuracy: 0.0,
            update_rate: 0.0,
            autonomous_level: 0.0,
            star_catalog_size: 0,
            trajectory_corrections: 0,
        }
    }

    pub fn assess_accuracy(&self) -> String {
        if self.position_accuracy < 1.0 {
            "Excellent".into()
        } else if self.position_accuracy < 10.0 {
            "Good".into()
        } else {
            "Adequate".into()
        }
    }
}

impl TrajectoryDetermination {
    pub fn new(trajectory_id: String) -> Self {
        Self {
            trajectory_id,
            departure_body: "Earth".into(),
            arrival_body: "Mars".into(),
            departure_date: 0.0,
            arrival_date: 0.0,
            delta_v_total: 0.0,
            trajectory_type: "Hohmann".into(),
            midcourse_corrections: 0,
        }
    }

    pub fn flight_duration(&self) -> f64 {
        self.arrival_date - self.departure_date
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deep_space_navigation() {
        let nav = DeepSpaceNavigation::new("DSN Integration".into(), NavigationMethod::OpticalNavigation);
        assert!(matches!(nav.nav_method, NavigationMethod::OpticalNavigation));
    }
}
