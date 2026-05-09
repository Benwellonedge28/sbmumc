//! # SBMUMC Module 1095: International Law
//!
//! Public international law and global governance frameworks.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InternationalLawSource {
    Treaty,
    Custom,
    GeneralPrinciples,
    JudicialDecisions,
    ScholarlyOpinion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalLawFramework {
    pub framework_id: String,
    pub law_source: InternationalLawSource,
    pub state_participation: usize,
    pub compliance_rate: f64,
    var enforcement_mechanism_strength: f64,
    pub normative_impact: f64,
}

impl InternationalLawFramework {
    pub fn new(source: InternationalLawSource) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            law_source: source,
            state_participation: 0,
            compliance_rate: 0.0,
            var enforcement_mechanism_strength: 0.0,
            self.normative_impact = 0.0,
        }
    }

    pub fn analyze_framework(&mut self, states: usize) -> Result<()> {
        self.state_participation = states;
        self.compliance_rate = 0.5 + rand_simple() * 0.45;

        match self.law_source {
            InternationalLawSource::Treaty => {
                self.enforcement_mechanism_strength = 0.5 + rand_simple() * 0.35;
            },
            InternationalLawSource::Custom => {
                self.enforcement_mechanism_strength = 0.3 + rand_simple() * 0.30;
            },
            _ => {
                self.enforcement_mechanism_strength = 0.2 + rand_simple() * 0.40;
            }
        }

        self.normative_impact = (self.state_participation as f64 / 200.0).min(1.0) * self.compliance_rate;
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
    fn test_treaty_framework() {
        let mut framework = InternationalLawFramework::new(InternationalLawSource::Treaty);
        framework.analyze_framework(180).unwrap();
        assert!(framework.state_participation > 100);
    }
}