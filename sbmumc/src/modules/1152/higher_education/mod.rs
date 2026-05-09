//! # SBMUMC Module 1152: Higher Education
//!
//! Universities, colleges, and advanced academic institutions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HigherEducationType {
    ResearchUniversity,
    LiberalArtsCollege,
    TechnicalInstitute,
    CommunityCollege,
    OnlineUniversity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HigherEducationSystem {
    pub system_id: String,
    pub education_type: HigherEducationType,
    pub research_output: f64,
    pub teaching_quality: f64,
    pub graduate_employability: f64,
    pub academic_reputation: f64,
}

impl HigherEducationSystem {
    pub fn new(education_type: HigherEducationType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            education_type,
            research_output: 0.0,
            teaching_quality: 0.0,
            graduate_employability: 0.0,
            academic_reputation: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.education_type {
            HigherEducationType::ResearchUniversity => {
                self.research_output = 0.85 + rand_simple() * 0.14;
                self.teaching_quality = 0.70 + rand_simple() * 0.25;
                self.academic_reputation = 0.85 + rand_simple() * 0.14;
            },
            HigherEducationType::LiberalArtsCollege => {
                self.research_output = 0.40 + rand_simple() * 0.35;
                self.teaching_quality = 0.85 + rand_simple() * 0.14;
                self.academic_reputation = 0.75 + rand_simple() * 0.22;
            },
            HigherEducationType::TechnicalInstitute => {
                self.research_output = 0.70 + rand_simple() * 0.25;
                self.teaching_quality = 0.75 + rand_simple() * 0.22;
                self.graduate_employability = 0.90 + rand_simple() * 0.10;
            },
            HigherEducationType::CommunityCollege => {
                self.research_output = 0.20 + rand_simple() * 0.30;
                self.teaching_quality = 0.70 + rand_simple() * 0.25;
                self.graduate_employability = 0.75 + rand_simple() * 0.22;
            },
            HigherEducationType::OnlineUniversity => {
                self.research_output = 0.30 + rand_simple() * 0.35;
                self.teaching_quality = 0.65 + rand_simple() * 0.30;
                self.graduate_employability = 0.60 + rand_simple() * 0.35;
            },
        }

        if self.graduate_employability == 0.0 {
            self.graduate_employability = 0.55 + rand_simple() * 0.40;
        }
        if self.academic_reputation == 0.0 {
            self.academic_reputation = 0.50 + rand_simple() * 0.45;
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
    fn test_research_university() {
        let mut system = HigherEducationSystem::new(HigherEducationType::ResearchUniversity);
        system.analyze_system().unwrap();
        assert!(system.research_output > 0.7);
    }
}