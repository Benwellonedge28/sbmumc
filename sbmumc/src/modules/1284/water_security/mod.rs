//! # SBMUMC Module 1284: Water Security
//!
//! Systems for ensuring access to safe water resources.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WaterSecurityDimension {
    AccessAvailability,
    WaterQuality,
    WaterQuantity,
    FacilityAccess,
    SustainableManagement,
    ResiliencePlanning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterSecuritySystem {
    pub system_id: String,
    pub security_dimension: WaterSecurityDimension,
    pub security_level: f64,
    pub service_reliability: f64,
    pub equity_access: f64,
    pub disaster_resilience: f64,
}

impl WaterSecuritySystem {
    pub fn new(security_dimension: WaterSecurityDimension) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            security_dimension,
            security_level: 0.0,
            service_reliability: 0.0,
            equity_access: 0.0,
            disaster_resilience: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.security_dimension {
            WaterSecurityDimension::AccessAvailability => {
                self.security_level = 0.85 + rand_simple() * 0.14;
                self.equity_access = 0.75 + rand_simple() * 0.22;
                self.service_reliability = 0.80 + rand_simple() * 0.18;
            },
            WaterSecurityDimension::WaterQuality => {
                self.security_level = 0.80 + rand_simple() * 0.18;
                self.service_reliability = 0.85 + rand_simple() * 0.14;
                self.disaster_resilience = 0.70 + rand_simple() * 0.25;
            },
            WaterSecurityDimension::WaterQuantity => {
                self.security_level = 0.90 + rand_simple() * 0.10;
                self.equity_access = 0.80 + rand_simple() * 0.18;
                self.disaster_resilience = 0.75 + rand_simple() * 0.22;
            },
            WaterSecurityDimension::FacilityAccess => {
                self.service_reliability = 0.90 + rand_simple() * 0.10;
                self.security_level = 0.85 + rand_simple() * 0.14;
                self.equity_access = 0.70 + rand_simple() * 0.25;
            },
            WaterSecurityDimension::SustainableManagement => {
                self.security_level = 0.75 + rand_simple() * 0.22;
                self.disaster_resilience = 0.85 + rand_simple() * 0.14;
                self.service_reliability = 0.80 + rand_simple() * 0.18;
            },
            WaterSecurityDimension::ResiliencePlanning => {
                self.disaster_resilience = 0.90 + rand_simple() * 0.10;
                self.security_level = 0.85 + rand_simple() * 0.14;
                self.equity_access = 0.65 + rand_simple() * 0.30;
            },
        }

        if self.equity_access == 0.0 {
            self.equity_access = (self.security_level + self.service_reliability) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_water_quantity() {
        let mut system = WaterSecuritySystem::new(WaterSecurityDimension::WaterQuantity);
        system.analyze_system().unwrap();
        assert!(system.security_level > 0.7);
    }
}
