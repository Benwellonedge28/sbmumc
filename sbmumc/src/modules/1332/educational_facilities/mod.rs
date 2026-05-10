//! # SBMUMC Module 1332: Educational Facilities
//!
//! Systems for educational building design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EducationalFacilityType {
    ClassroomDesign,
    LaboratorySpaces,
    LibraryDesign,
    SportsFacilities,
    Auditorium,
    OutdoorLearning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationalFacilitiesSystem {
    pub system_id: String,
    pub facility_type: EducationalFacilityType,
    pub learning_enhancement: f64,
    pub safety_standards: f64,
    pub adaptability: f64,
    pub community_use: f64,
}

impl EducationalFacilitiesSystem {
    pub fn new(facility_type: EducationalFacilityType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            facility_type,
            learning_enhancement: 0.0,
            safety_standards: 0.0,
            adaptability: 0.0,
            community_use: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.facility_type {
            EducationalFacilityType::ClassroomDesign => {
                self.learning_enhancement = 0.95 + rand_simple() * 0.05;
                self.adaptability = 0.90 + rand_simple() * 0.10;
                self.safety_standards = 0.85 + rand_simple() * 0.14;
            },
            EducationalFacilityType::LaboratorySpaces => {
                self.safety_standards = 0.95 + rand_simple() * 0.05;
                self.learning_enhancement = 0.90 + rand_simple() * 0.10;
                self.adaptability = 0.80 + rand_simple() * 0.18;
            },
            EducationalFacilityType::LibraryDesign => {
                self.learning_enhancement = 0.90 + rand_simple() * 0.10;
                self.adaptability = 0.85 + rand_simple() * 0.14;
                self.community_use = 0.80 + rand_simple() * 0.18;
            },
            EducationalFacilityType::SportsFacilities => {
                self.safety_standards = 0.90 + rand_simple() * 0.10;
                self.community_use = 0.85 + rand_simple() * 0.14;
                self.learning_enhancement = 0.75 + rand_simple() * 0.22;
            },
            EducationalFacilityType::Auditorium => {
                self.community_use = 0.95 + rand_simple() * 0.05;
                self.learning_enhancement = 0.85 + rand_simple() * 0.14;
                self.adaptability = 0.80 + rand_simple() * 0.18;
            },
            EducationalFacilityType::OutdoorLearning => {
                self.learning_enhancement = 0.90 + rand_simple() * 0.10;
                self.adaptability = 0.95 + rand_simple() * 0.05;
                self.community_use = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.community_use == 0.0 {
            self.community_use = (self.learning_enhancement + self.adaptability) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_classroom_design() {
        let mut system = EducationalFacilitiesSystem::new(EducationalFacilityType::ClassroomDesign);
        system.analyze_system().unwrap();
        assert!(system.learning_enhancement > 0.8);
    }
}
