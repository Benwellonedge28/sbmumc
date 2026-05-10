//! # SBMUMC Module 1247: Desalination
//!
//! Removal of salts from seawater to produce fresh water.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DesalinationMethod {
    ReverseOsmosis,
    Thermal,
    Electrochemical,
    MembraneDistillation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesalinationSystem {
    pub system_id: String,
    pub desal_method: DesalinationMethod,
    pub water_recovery_rate: f64,
    pub energy_efficiency: f64,
    pub brine_management: f64,
    pub cost_per_cubic: f64,
}

impl DesalinationSystem {
    pub fn new(desal_method: DesalinationMethod) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            desal_method,
            water_recovery_rate: 0.0,
            energy_efficiency: 0.0,
            brine_management: 0.0,
            cost_per_cubic: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.desal_method {
            DesalinationMethod::ReverseOsmosis => {
                self.water_recovery_rate = 0.45 + rand_simple() * 0.30;
                self.energy_efficiency = 0.70 + rand_simple() * 0.25;
            },
            DesalinationMethod::Thermal => {
                self.water_recovery_rate = 0.40 + rand_simple() * 0.30;
                self.energy_efficiency = 0.40 + rand_simple() * 0.35;
                self.brine_management = 0.50 + rand_simple() * 0.35;
            },
            DesalinationMethod::Electrochemical => {
                self.energy_efficiency = 0.80 + rand_simple() * 0.18;
                self.water_recovery_rate = 0.50 + rand_simple() * 0.35;
            },
            DesalinationMethod::MembraneDistillation => {
                self.water_recovery_rate = 0.70 + rand_simple() * 0.25;
                self.energy_efficiency = 0.60 + rand_simple() * 0.30;
            },
        }

        if self.brine_management == 0.0 {
            self.brine_management = (1.0 - self.water_recovery_rate) * (0.5 + rand_simple() * 0.5);
        }
        self.cost_per_cubic = (1.0 - self.energy_efficiency) * (0.6 + rand_simple() * 0.4);
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
    fn test_ro_desalination() {
        let mut system = DesalinationSystem::new(DesalinationMethod::ReverseOsmosis);
        system.analyze_system().unwrap();
        assert!(system.energy_efficiency > 0.5);
    }
}