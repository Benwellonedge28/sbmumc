//! # SBMUMC Module 1310: Accessible Design
//!
//! Systems for universal accessibility in buildings.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessibilityFeature {
    WheelchairAccess,
    VisualAccessibility,
    AuditoryAccessibility,
    CognitiveAccessibility,
    MobilityAidSupport,
    EmergencyEgress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibleDesignSystem {
    pub system_id: String,
    pub accessibility_feature: AccessibilityFeature,
    pub compliance_score: f64,
    pub usability_rating: f64,
    pub aesthetic_integration: f64,
    pub cost_effectiveness: f64,
}

impl AccessibleDesignSystem {
    pub fn new(accessibility_feature: AccessibilityFeature) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            accessibility_feature,
            compliance_score: 0.0,
            usability_rating: 0.0,
            aesthetic_integration: 0.0,
            cost_effectiveness: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.accessibility_feature {
            AccessibilityFeature::WheelchairAccess => {
                self.compliance_score = 0.95 + rand_simple() * 0.05;
                self.usability_rating = 0.90 + rand_simple() * 0.10;
                self.cost_effectiveness = 0.75 + rand_simple() * 0.22;
            },
            AccessibilityFeature::VisualAccessibility => {
                self.usability_rating = 0.90 + rand_simple() * 0.10;
                self.aesthetic_integration = 0.85 + rand_simple() * 0.14;
                self.compliance_score = 0.85 + rand_simple() * 0.14;
            },
            AccessibilityFeature::AuditoryAccessibility => {
                self.usability_rating = 0.90 + rand_simple() * 0.10;
                self.compliance_score = 0.85 + rand_simple() * 0.14;
                self.cost_effectiveness = 0.80 + rand_simple() * 0.18;
            },
            AccessibilityFeature::CognitiveAccessibility => {
                self.usability_rating = 0.85 + rand_simple() * 0.14;
                self.aesthetic_integration = 0.80 + rand_simple() * 0.18;
                self.compliance_score = 0.80 + rand_simple() * 0.18;
            },
            AccessibilityFeature::MobilityAidSupport => {
                self.usability_rating = 0.90 + rand_simple() * 0.10;
                self.compliance_score = 0.85 + rand_simple() * 0.14;
                self.cost_effectiveness = 0.70 + rand_simple() * 0.25;
            },
            AccessibilityFeature::EmergencyEgress => {
                self.compliance_score = 0.95 + rand_simple() * 0.05;
                self.usability_rating = 0.85 + rand_simple() * 0.14;
                self.cost_effectiveness = 0.65 + rand_simple() * 0.30;
            },
        }

        if self.cost_effectiveness == 0.0 {
            self.cost_effectiveness = (self.compliance_score + self.usability_rating) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_wheelchair_access() {
        let mut system = AccessibleDesignSystem::new(AccessibilityFeature::WheelchairAccess);
        system.analyze_system().unwrap();
        assert!(system.compliance_score > 0.8);
    }
}
