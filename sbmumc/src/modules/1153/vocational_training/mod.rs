//! # SBMUMC Module 1153: Vocational Training
//!
//! Skill-based education for specific trades and professions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VocationalSector {
    Healthcare,
    Construction,
    Manufacturing,
    Hospitality,
    Technology,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocationalTrainingSystem {
    pub system_id: String,
    pub sector: VocationalSector,
    pub skill_alignment: f64,
    pub job_placement_rate: f64,
    pub industry_relevance: f64,
    pub practical_competency: f64,
}

impl VocationalTrainingSystem {
    pub fn new(sector: VocationalSector) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            sector,
            skill_alignment: 0.0,
            job_placement_rate: 0.0,
            industry_relevance: 0.0,
            practical_competency: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.sector {
            VocationalSector::Healthcare => {
                self.skill_alignment = 0.85 + rand_simple() * 0.14;
                self.job_placement_rate = 0.80 + rand_simple() * 0.18;
                self.industry_relevance = 0.90 + rand_simple() * 0.10;
            },
            VocationalSector::Technology => {
                self.skill_alignment = 0.80 + rand_simple() * 0.18;
                self.job_placement_rate = 0.85 + rand_simple() * 0.14;
                self.industry_relevance = 0.85 + rand_simple() * 0.14;
            },
            VocationalSector::Construction => {
                self.skill_alignment = 0.75 + rand_simple() * 0.22;
                self.job_placement_rate = 0.70 + rand_simple() * 0.25;
                self.industry_relevance = 0.80 + rand_simple() * 0.18;
            },
            _ => {
                self.skill_alignment = 0.70 + rand_simple() * 0.25;
                self.job_placement_rate = 0.65 + rand_simple() * 0.30;
                self.industry_relevance = 0.70 + rand_simple() * 0.25;
            },
        }

        self.practical_competency = (self.skill_alignment + self.job_placement_rate) / 2.0;
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
    fn test_healthcare_vocational() {
        let mut system = VocationalTrainingSystem::new(VocationalSector::Healthcare);
        system.analyze_system().unwrap();
        assert!(system.skill_alignment > 0.7);
    }
}