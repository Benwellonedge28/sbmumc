//! # SBMUMC Module 1248: Oceanography
//!
//! Scientific study of oceans and their phenomena.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OceanographicDomain {
    Physical,
    Chemical,
    Biological,
    Geological,
    Atmospheric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OceanographySystem {
    pub system_id: String,
    pub oceanographic_domain: OceanographicDomain,
    pub measurement_coverage: f64,
    pub prediction_capability: f64,
    pub climate_modeling: f64,
    pub resource_assessment: f64,
}

impl OceanographySystem {
    pub fn new(oceanographic_domain: OceanographicDomain) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            oceanographic_domain,
            measurement_coverage: 0.0,
            prediction_capability: 0.0,
            climate_modeling: 0.0,
            resource_assessment: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.oceanographic_domain {
            OceanographicDomain::Physical => {
                self.measurement_coverage = 0.80 + rand_simple() * 0.18;
                self.prediction_capability = 0.75 + rand_simple() * 0.22;
                self.climate_modeling = 0.85 + rand_simple() * 0.14;
            },
            OceanographicDomain::Chemical => {
                self.measurement_coverage = 0.75 + rand_simple() * 0.22;
                self.climate_modeling = 0.80 + rand_simple() * 0.18;
            },
            OceanographicDomain::Biological => {
                self.resource_assessment = 0.80 + rand_simple() * 0.18;
                self.measurement_coverage = 0.70 + rand_simple() * 0.25;
            },
            OceanographicDomain::Geological => {
                self.resource_assessment = 0.85 + rand_simple() * 0.14;
                self.measurement_coverage = 0.65 + rand_simple() * 0.30;
            },
            OceanographicDomain::Atmospheric => {
                self.prediction_capability = 0.85 + rand_simple() * 0.14;
                self.climate_modeling = 0.90 + rand_simple() * 0.10;
            },
        }

        if self.resource_assessment == 0.0 {
            self.resource_assessment = (self.measurement_coverage + self.prediction_capability) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_atmospheric_oceanography() {
        let mut system = OceanographySystem::new(OceanographicDomain::Atmospheric);
        system.analyze_system().unwrap();
        assert!(system.climate_modeling > 0.7);
    }
}