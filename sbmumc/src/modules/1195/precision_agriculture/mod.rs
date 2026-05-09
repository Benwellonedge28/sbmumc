//! # SBMUMC Module 1195: Precision Agriculture
//!
//! Technology-driven farming using data and automation.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrecisionTechnology {
    GISSensing,
    VariableRate,
    DroneMonitoring,
    AutomatedMachinery,
    IoTSensors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecisionAgricultureSystem {
    pub system_id: String,
    pub precision_technology: PrecisionTechnology,
    pub input_optimization: f64,
    pub yield_increase: f64,
    pub cost_reduction: f64,
    pub environmental_monitoring: f64,
}

impl PrecisionAgricultureSystem {
    pub fn new(precision_technology: PrecisionTechnology) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            precision_technology,
            input_optimization: 0.0,
            yield_increase: 0.0,
            cost_reduction: 0.0,
            environmental_monitoring: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.precision_technology {
            PrecisionTechnology::GISSensing => {
                self.input_optimization = 0.85 + rand_simple() * 0.14;
                self.yield_increase = 0.20 + rand_simple() * 0.25;
            },
            PrecisionTechnology::VariableRate => {
                self.input_optimization = 0.90 + rand_simple() * 0.10;
                self.cost_reduction = 0.25 + rand_simple() * 0.20;
            },
            PrecisionTechnology::DroneMonitoring => {
                self.yield_increase = 0.15 + rand_simple() * 0.20;
                self.environmental_monitoring = 0.85 + rand_simple() * 0.14;
            },
            PrecisionTechnology::AutomatedMachinery => {
                self.cost_reduction = 0.30 + rand_simple() * 0.20;
                self.input_optimization = 0.80 + rand_simple() * 0.18;
            },
            PrecisionTechnology::IoTSensors => {
                self.environmental_monitoring = 0.90 + rand_simple() * 0.10;
                self.input_optimization = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.yield_increase == 0.0 {
            self.yield_increase = 0.10 + rand_simple() * 0.20;
        }
        if self.cost_reduction == 0.0 {
            self.cost_reduction = 0.15 + rand_simple() * 0.20;
        }
        if self.environmental_monitoring == 0.0 {
            self.environmental_monitoring = 0.60 + rand_simple() * 0.30;
        }
        Ok(())
    }
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_rate_tech() {
        let mut system = PrecisionAgricultureSystem::new(PrecisionTechnology::VariableRate);
        system.analyze_system().unwrap();
        assert!(system.input_optimization > 0.7);
    }
}