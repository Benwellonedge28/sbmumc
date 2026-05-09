//! # SBMUMC Module 1180: Civic Education
//!
//! Preparation for active and informed citizenship.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CivicEducationDimension {
    Knowledge,
    Skills,
    Dispositions,
    Participation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivicEducationFramework {
    pub framework_id: String,
    pub education_dimension: CivicEducationDimension,
    pub civic_knowledge: f64,
    pub deliberative_skills: f64,
    pub democratic_values: f64,
    pub civic_engagement: f64,
}

impl CivicEducationFramework {
    pub fn new(education_dimension: CivicEducationDimension) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            education_dimension,
            civic_knowledge: 0.0,
            deliberative_skills: 0.0,
            democratic_values: 0.0,
            civic_engagement: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.education_dimension {
            CivicEducationDimension::Knowledge => {
                self.civic_knowledge = 0.85 + rand_simple() * 0.14;
                self.deliberative_skills = 0.70 + rand_simple() * 0.25;
            },
            CivicEducationDimension::Skills => {
                self.deliberative_skills = 0.85 + rand_simple() * 0.14;
                self.civic_engagement = 0.75 + rand_simple() * 0.22;
            },
            CivicEducationDimension::Dispositions => {
                self.democratic_values = 0.85 + rand_simple() * 0.14;
                self.civic_engagement = 0.80 + rand_simple() * 0.18;
            },
            CivicEducationDimension::Participation => {
                self.civic_engagement = 0.90 + rand_simple() * 0.10;
                self.deliberative_skills = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.democratic_values == 0.0 {
            self.democratic_values = (self.civic_knowledge + self.civic_engagement) / 2.0;
        }
        if self.civic_engagement == 0.0 {
            self.civic_engagement = 0.60 + rand_simple() * 0.35;
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
    fn test_participation_dimension() {
        let mut framework = CivicEducationFramework::new(CivicEducationDimension::Participation);
        framework.analyze_framework().unwrap();
        assert!(framework.civic_engagement > 0.7);
    }
}