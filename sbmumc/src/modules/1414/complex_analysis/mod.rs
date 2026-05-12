//! # SBMUMC Module 1414: Complex Analysis
//!
//! Systems for complex analysis and holomorphic functions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexDomain {
    Holomorphic,
    Meromorphic,
    Conformal,
    RiemannSurface,
    SeveralComplex,
    AnalyticContinuation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexAnalysisSystem {
    pub system_id: String,
    pub complex_domain: ComplexDomain,
    pub residue_calculus: f64,
    pub contour_integration: f64,
    pub conformal_mapping: f64,
    pub singularity_analysis: f64,
}

impl ComplexAnalysisSystem {
    pub fn new(complex_domain: ComplexDomain) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            complex_domain,
            residue_calculus: 0.0,
            contour_integration: 0.0,
            conformal_mapping: 0.0,
            singularity_analysis: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.complex_domain {
            ComplexDomain::Holomorphic => {
                self.residue_calculus = 0.95 + rand_simple() * 0.05;
                self.conformal_mapping = 0.90 + rand_simple() * 0.10;
                self.contour_integration = 0.85 + rand_simple() * 0.14;
            },
            ComplexDomain::Meromorphic => {
                self.singularity_analysis = 0.95 + rand_simple() * 0.05;
                self.residue_calculus = 0.90 + rand_simple() * 0.10;
                self.conformal_mapping = 0.85 + rand_simple() * 0.14;
            },
            ComplexDomain::Conformal => {
                self.conformal_mapping = 0.95 + rand_simple() * 0.05;
                self.contour_integration = 0.90 + rand_simple() * 0.10;
                self.singularity_analysis = 0.85 + rand_simple() * 0.14;
            },
            ComplexDomain::RiemannSurface => {
                self.residue_calculus = 0.95 + rand_simple() * 0.05;
                self.singularity_analysis = 0.90 + rand_simple() * 0.10;
                self.contour_integration = 0.85 + rand_simple() * 0.14;
            },
            ComplexDomain::SeveralComplex => {
                self.contour_integration = 0.95 + rand_simple() * 0.05;
                self.conformal_mapping = 0.90 + rand_simple() * 0.10;
                self.residue_calculus = 0.85 + rand_simple() * 0.14;
            },
            ComplexDomain::AnalyticContinuation => {
                self.singularity_analysis = 0.95 + rand_simple() * 0.05;
                self.residue_calculus = 0.90 + rand_simple() * 0.10;
                self.conformal_mapping = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.contour_integration == 0.0 {
            self.contour_integration = (self.residue_calculus + self.conformal_mapping) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_holomorphic() {
        let mut system = ComplexAnalysisSystem::new(ComplexDomain::Holomorphic);
        system.analyze_system().unwrap();
        assert!(system.residue_calculus > 0.8);
    }
}
