//! # SBMUMC Module 1165: Academic Freedom
//!
//! Principles of intellectual autonomy in educational institutions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AcademicFreedomDimension {
    Research,
    Teaching,
    ExtramuralSpeech,
    Institutional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicFreedomFramework {
    pub framework_id: String,
    pub freedom_dimension: AcademicFreedomDimension,
    pub institutional_protection: f64,
    pub legal_support: f64,
    pub cultural_acceptance: f64,
    pub self_governance: f64,
}

impl AcademicFreedomFramework {
    pub fn new(freedom_dimension: AcademicFreedomDimension) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            freedom_dimension,
            institutional_protection: 0.0,
            legal_support: 0.0,
            cultural_acceptance: 0.0,
            self_governance: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.freedom_dimension {
            AcademicFreedomDimension::Research => {
                self.institutional_protection = 0.80 + rand_simple() * 0.18;
                self.legal_support = 0.75 + rand_simple() * 0.22;
                self.self_governance = 0.85 + rand_simple() * 0.14;
            },
            AcademicFreedomDimension::Teaching => {
                self.institutional_protection = 0.75 + rand_simple() * 0.22;
                self.legal_support = 0.70 + rand_simple() * 0.25;
                self.cultural_acceptance = 0.65 + rand_simple() * 0.30;
            },
            AcademicFreedomDimension::ExtramuralSpeech => {
                self.institutional_protection = 0.65 + rand_simple() * 0.30;
                self.legal_support = 0.80 + rand_simple() * 0.18;
                self.cultural_acceptance = 0.55 + rand_simple() * 0.40;
            },
            AcademicFreedomDimension::Institutional => {
                self.institutional_protection = 0.85 + rand_simple() * 0.14;
                self.self_governance = 0.80 + rand_simple() * 0.18;
                self.legal_support = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.cultural_acceptance == 0.0 {
            self.cultural_acceptance = 0.60 + rand_simple() * 0.35;
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
    fn test_research_freedom() {
        let mut framework = AcademicFreedomFramework::new(AcademicFreedomDimension::Research);
        framework.analyze_framework().unwrap();
        assert!(framework.institutional_protection > 0.6);
    }
}