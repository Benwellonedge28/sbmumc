//! # SBMUMC Module 1313: Facade Design
//!
//! Systems for building envelope and facade design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FacadeSystem {
    CurtainWall,
    Rainscreen,
    DoubleSkin,
    BriseSoleil,
    LivingFacade,
    DynamicFacade,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacadeDesignSystem {
    pub system_id: String,
    pub facade_system: FacadeSystem,
    pub thermal_performance: f64,
    pub daylight_access: f64,
    pub aesthetic_expression: f64,
    pub maintenance_requirement: f64,
}

impl FacadeDesignSystem {
    pub fn new(facade_system: FacadeSystem) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            facade_system,
            thermal_performance: 0.0,
            daylight_access: 0.0,
            aesthetic_expression: 0.0,
            maintenance_requirement: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.facade_system {
            FacadeSystem::CurtainWall => {
                self.daylight_access = 0.90 + rand_simple() * 0.10;
                self.thermal_performance = 0.80 + rand_simple() * 0.18;
                self.aesthetic_expression = 0.85 + rand_simple() * 0.14;
            },
            FacadeSystem::Rainscreen => {
                self.thermal_performance = 0.85 + rand_simple() * 0.14;
                self.aesthetic_expression = 0.80 + rand_simple() * 0.18;
                self.maintenance_requirement = 0.60 + rand_simple() * 0.35;
            },
            FacadeSystem::DoubleSkin => {
                self.thermal_performance = 0.90 + rand_simple() * 0.10;
                self.daylight_access = 0.85 + rand_simple() * 0.14;
                self.maintenance_requirement = 0.70 + rand_simple() * 0.25;
            },
            FacadeSystem::BriseSoleil => {
                self.thermal_performance = 0.85 + rand_simple() * 0.14;
                self.daylight_access = 0.75 + rand_simple() * 0.22;
                self.aesthetic_expression = 0.80 + rand_simple() * 0.18;
            },
            FacadeSystem::LivingFacade => {
                self.thermal_performance = 0.80 + rand_simple() * 0.18;
                self.aesthetic_expression = 0.90 + rand_simple() * 0.10;
                self.maintenance_requirement = 0.75 + rand_simple() * 0.22;
            },
            FacadeSystem::DynamicFacade => {
                self.thermal_performance = 0.90 + rand_simple() * 0.10;
                self.aesthetic_expression = 0.85 + rand_simple() * 0.14;
                self.maintenance_requirement = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.maintenance_requirement == 0.0 {
            self.maintenance_requirement = (1.0 - self.thermal_performance) * (0.5 + rand_simple() * 0.5);
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
    fn test_double_skin() {
        let mut system = FacadeDesignSystem::new(FacadeSystem::DoubleSkin);
        system.analyze_system().unwrap();
        assert!(system.thermal_performance > 0.7);
    }
}
