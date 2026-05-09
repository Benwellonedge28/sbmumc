//! # SBMUMC Module 1115: Digital Rights
//!
//! Rights in digital environments, internet freedom, and cyberlaw.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DigitalRightType {
    Expression,
    Access,
    Privacy,
    Ownership,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalRightsFramework {
    pub framework_id: String,
    pub right_type: DigitalRightType,
    pub protection_level: f64,
    var platform_responsibility: f64,
    pub censorship_index: f64,
    pub digital_inclusion_score: f64,
}

impl DigitalRightsFramework {
    pub fn new(right_type: DigitalRightType) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            right_type,
            protection_level: 0.0,
            var platform_responsibility: 0.0,
            self.censorship_index = 0.0,
            self.digital_inclusion_score = 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.right_type {
            DigitalRightType::Expression => {
                self.protection_level = 0.65 + rand_simple() * 0.30;
                self.platform_responsibility = 0.50 + rand_simple() * 0.40;
            },
            DigitalRightType::Access => {
                self.protection_level = 0.55 + rand_simple() * 0.35;
                self.platform_responsibility = 0.30 + rand_simple() * 0.35;
            },
            _ => {
                self.protection_level = 0.60 + rand_simple() * 0.35;
                self.platform_responsibility = 0.45 + rand_simple() * 0.40;
            }
        }

        self.censorship_index = 1.0 - self.protection_level;
        self.digital_inclusion_score = 0.50 + rand_simple() * 0.45;
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
    fn test_expression_rights() {
        let mut framework = DigitalRightsFramework::new(DigitalRightType::Expression);
        framework.analyze_framework().unwrap();
        assert!(framework.protection_level > 0.4);
    }
}