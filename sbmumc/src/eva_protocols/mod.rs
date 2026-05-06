//! EVA Protocols Module (659)
//!
//! Extravehicular activity procedures, safety systems, and operational guidelines.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EVAType {
    Spacewalk,
    LunarEVA,
    MartianEVA,
    AsteroidEVA,
    Underwater,
    Training,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EVAProtocol {
    pub eva_type: EVAType,
    pub duration_minutes: f64,
    pub prebreath_time: f64,         // minutes
    pub buddy_system: bool,
    pub communication_interval: f64, // minutes
    pub safety tether: bool,
    pub backup_oxygen: bool,
    pub emergency_return_procedure: String,
    pub task_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EVAOperations {
    pub operations_center: String,
    pub flight_controller: String,
    pub medical_support: String,
    pub recovery_team: bool,
    pub training_level: u32,
}

impl EVAProtocol {
    pub fn new(eva_type: EVAType) -> Self {
        Self {
            eva_type,
            duration_minutes: 0.0,
            prebreath_time: 30.0,
            buddy_system: true,
            communication_interval: 10.0,
            safety_tether: true,
            backup_oxygen: true,
            emergency_return_procedure: "Immediate suit purge and return".into(),
            task_list: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: String) {
        self.task_list.push(task);
    }

    pub fn total_duration(&self) -> f64 {
        self.prebreath_time + self.duration_minutes + 15.0 // post-EVA
    }
}

impl EVAOperations {
    pub fn new(operations_center: String) -> Self {
        Self {
            operations_center,
            flight_controller: "CC".into(),
            medical_support: "Flight Surgeon".into(),
            recovery_team: true,
            training_level: 3,
        }
    }

    pub fn validate_readiness(&self) -> bool {
        self.training_level >= 3 && self.recovery_team
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eva_protocol() {
        let protocol = EVAProtocol::new(EVAType::Spacewalk);
        assert!(matches!(protocol.eva_type, EVAType::Spacewalk));
    }
}
