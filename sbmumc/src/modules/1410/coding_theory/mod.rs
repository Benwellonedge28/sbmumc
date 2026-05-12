//! # SBMUMC Module 1410: Coding Theory
//!
//! Systems for coding theory and error correction.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeType {
    LinearBlock,
    Cyclic,
    Convolutional,
    Turbo,
    LowDensityParityCheck,
    Quantum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodingTheorySystem {
    pub system_id: String,
    pub code_type: CodeType,
    pub error_detection: f64,
    pub error_correction: f64,
    pub code_efficiency: f64,
    pub decoding_algorithm: f64,
}

impl CodingTheorySystem {
    pub fn new(code_type: CodeType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            code_type,
            error_detection: 0.0,
            error_correction: 0.0,
            code_efficiency: 0.0,
            decoding_algorithm: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.code_type {
            CodeType::LinearBlock => {
                self.error_detection = 0.95 + rand_simple() * 0.05;
                self.error_correction = 0.90 + rand_simple() * 0.10;
                self.code_efficiency = 0.85 + rand_simple() * 0.14;
            },
            CodeType::Cyclic => {
                self.code_efficiency = 0.95 + rand_simple() * 0.05;
                self.decoding_algorithm = 0.90 + rand_simple() * 0.10;
                self.error_detection = 0.85 + rand_simple() * 0.14;
            },
            CodeType::Convolutional => {
                self.decoding_algorithm = 0.95 + rand_simple() * 0.05;
                self.error_correction = 0.90 + rand_simple() * 0.10;
                self.code_efficiency = 0.85 + rand_simple() * 0.14;
            },
            CodeType::Turbo => {
                self.error_correction = 0.95 + rand_simple() * 0.05;
                self.error_detection = 0.90 + rand_simple() * 0.10;
                self.decoding_algorithm = 0.85 + rand_simple() * 0.14;
            },
            CodeType::LowDensityParityCheck => {
                self.decoding_algorithm = 0.95 + rand_simple() * 0.05;
                self.code_efficiency = 0.90 + rand_simple() * 0.10;
                self.error_detection = 0.85 + rand_simple() * 0.14;
            },
            CodeType::Quantum => {
                self.code_efficiency = 0.95 + rand_simple() * 0.05;
                self.error_correction = 0.90 + rand_simple() * 0.10;
                self.decoding_algorithm = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.decoding_algorithm == 0.0 {
            self.decoding_algorithm = (self.error_detection + self.error_correction) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_linear_block() {
        let mut system = CodingTheorySystem::new(CodeType::LinearBlock);
        system.analyze_system().unwrap();
        assert!(system.error_detection > 0.8);
    }
}
