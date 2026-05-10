//! # SBMUMC Module 1259: Drought Management
//!
//! Planning and response for drought conditions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DroughtManagementMeasure {
    WaterRestrictions,
    ReservoirManagement,
    GroundwaterRecharge,
    CropAdaptation,
    PublicAwareness,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DroughtManagementSystem {
    pub system_id: String,
    pub management_measure: DroughtManagementMeasure,
    pub mitigation_effectiveness: f64,
    pub water_savings: f64,
    pub economic_impact: f64,
    pub social_adaptation: f64,
}

impl DroughtManagementSystem {
    pub fn new(management_measure: DroughtManagementMeasure) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            management_measure,
            mitigation_effectiveness: 0.0,
            water_savings: 0.0,
            economic_impact: 0.0,
            social_adaptation: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.management_measure {
            DroughtManagementMeasure::WaterRestrictions => {
                self.water_savings = 0.80 + rand_simple() * 0.18;
                self.mitigation_effectiveness = 0.70 + rand_simple() * 0.25;
                self.social_adaptation = 0.55 + rand_simple() * 0.40;
            },
            DroughtManagementMeasure::ReservoirManagement => {
                self.mitigation_effectiveness = 0.80 + rand_simple() * 0.18;
                self.water_savings = 0.65 + rand_simple() * 0.30;
            },
            DroughtManagementMeasure::GroundwaterRecharge => {
                self.mitigation_effectiveness = 0.75 + rand_simple() * 0.22;
                self.water_savings = 0.60 + rand_simple() * 0.35;
                self.economic_impact = 0.60 + rand_simple() * 0.35;
            },
            DroughtManagementMeasure::CropAdaptation => {
                self.economic_impact = 0.75 + rand_simple() * 0.22;
                self.social_adaptation = 0.70 + rand_simple() * 0.25;
            },
            DroughtManagementMeasure::PublicAwareness => {
                self.social_adaptation = 0.80 + rand_simple() * 0.18;
                self.mitigation_effectiveness = 0.60 + rand_simple() * 0.35;
            },
        }

        if self.economic_impact == 0.0 {
            self.economic_impact = (self.mitigation_effectiveness + self.water_savings) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_water_restrictions() {
        let mut system = DroughtManagementSystem::new(DroughtManagementMeasure::WaterRestrictions);
        system.analyze_system().unwrap();
        assert!(system.water_savings > 0.6);
    }
}