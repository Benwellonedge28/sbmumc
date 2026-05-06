//! Medical Robotics Module (726)
//!
//! Surgical robotics, rehabilitation robots, and medical automation systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RobotType {
    Surgical,
    Rehabilitation,
    Diagnostic,
    Disinfection,
    Pharmacy,
    Companion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalRobot {
    pub robot_id: String,
    pub robot_type: RobotType,
    pub manufacturer: String,
    pub degrees_of_freedom: u8,
    pub precision_mm: f64,
    pub workspace_radius_mm: f64,
    pub autonomy_level: u8,
    pub training_required_hours: f64,
    pub procedure_count: u32,
}

impl MedicalRobot {
    pub fn new(robot_id: String, robot_type: RobotType) -> Self {
        Self {
            robot_id,
            robot_type,
            manufacturer: "Unknown".into(),
            degrees_of_freedom: 0,
            precision_mm: 0.0,
            workspace_radius_mm: 0.0,
            autonomy_level: 1,
            training_required_hours: 40.0,
            procedure_count: 0,
        }
    }

    pub fn clinical_utility(&self) -> f64 {
        (self.precision_mm / 1.0 * 100.0).min(100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_robot() {
        let robot = MedicalRobot::new("MR-001".into(), RobotType::Surgical);
        assert!(matches!(robot.robot_type, RobotType::Surgical));
    }
}
