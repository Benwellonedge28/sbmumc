//! # SBMUMC Module 1254: Lake Ecology
//!
//! Study and management of freshwater lake ecosystems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LakeEcologicalIssue {
    Eutrophication,
    AlienSpecies,
    Acidification,
    ThermalStratification,
    Sedimentation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LakeEcologySystem {
    pub system_id: String,
    pub ecological_issue: LakeEcologicalIssue,
    pub impact_severity: f64,
    pub monitoring_capacity: f64,
    pub restoration_feasibility: f64,
    pub ecosystem_resilience: f64,
}

impl LakeEcologySystem {
    pub fn new(ecological_issue: LakeEcologicalIssue) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            ecological_issue,
            impact_severity: 0.0,
            monitoring_capacity: 0.0,
            restoration_feasibility: 0.0,
            ecosystem_resilience: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.ecological_issue {
            LakeEcologicalIssue::Eutrophication => {
                self.impact_severity = 0.85 + rand_simple() * 0.14;
                self.restoration_feasibility = 0.60 + rand_simple() * 0.35;
            },
            LakeEcologicalIssue::AlienSpecies => {
                self.impact_severity = 0.75 + rand_simple() * 0.22;
                self.monitoring_capacity = 0.80 + rand_simple() * 0.18;
            },
            LakeEcologicalIssue::Acidification => {
                self.impact_severity = 0.70 + rand_simple() * 0.25;
                self.restoration_feasibility = 0.70 + rand_simple() * 0.25;
            },
            LakeEcologicalIssue::ThermalStratification => {
                self.impact_severity = 0.60 + rand_simple() * 0.30;
                self.monitoring_capacity = 0.85 + rand_simple() * 0.14;
            },
            LakeEcologicalIssue::Sedimentation => {
                self.impact_severity = 0.80 + rand_simple() * 0.18;
                self.restoration_feasibility = 0.50 + rand_simple() * 0.40;
            },
        }

        if self.ecosystem_resilience == 0.0 {
            self.ecosystem_resilience = (self.monitoring_capacity + self.restoration_feasibility) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_eutrophication() {
        let mut system = LakeEcologySystem::new(LakeEcologicalIssue::Eutrophication);
        system.analyze_system().unwrap();
        assert!(system.impact_severity > 0.6);
    }
}