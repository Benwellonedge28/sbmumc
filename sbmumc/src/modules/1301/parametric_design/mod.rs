//! # SBMUMC Module 1301: Parametric Design
//!
//! Systems for algorithm-driven architectural design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParametricDesignApplication {
    FormFinding,
    StructuralOptimization,
    ClimateResponse,
    MaterialEfficiency,
    PatternGeneration,
    PerformativeFacades,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametricDesignSystem {
    pub system_id: String,
    pub design_application: ParametricDesignApplication,
    pub design_flexibility: f64,
    pub optimization_capability: f64,
    pub computational_complexity: f64,
    pub aesthetic_innovation: f64,
}

impl ParametricDesignSystem {
    pub fn new(design_application: ParametricDesignApplication) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            design_application,
            design_flexibility: 0.0,
            optimization_capability: 0.0,
            computational_complexity: 0.0,
            aesthetic_innovation: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.design_application {
            ParametricDesignApplication::FormFinding => {
                self.aesthetic_innovation = 0.95 + rand_simple() * 0.05;
                self.design_flexibility = 0.90 + rand_simple() * 0.10;
                self.computational_complexity = 0.85 + rand_simple() * 0.14;
            },
            ParametricDesignApplication::StructuralOptimization => {
                self.optimization_capability = 0.90 + rand_simple() * 0.10;
                self.computational_complexity = 0.85 + rand_simple() * 0.14;
                self.design_flexibility = 0.80 + rand_simple() * 0.18;
            },
            ParametricDesignApplication::ClimateResponse => {
                self.optimization_capability = 0.85 + rand_simple() * 0.14;
                self.design_flexibility = 0.80 + rand_simple() * 0.18;
                self.aesthetic_innovation = 0.75 + rand_simple() * 0.22;
            },
            ParametricDesignApplication::MaterialEfficiency => {
                self.optimization_capability = 0.90 + rand_simple() * 0.10;
                self.computational_complexity = 0.80 + rand_simple() * 0.18;
                self.design_flexibility = 0.75 + rand_simple() * 0.22;
            },
            ParametricDesignApplication::PatternGeneration => {
                self.aesthetic_innovation = 0.90 + rand_simple() * 0.10;
                self.design_flexibility = 0.85 + rand_simple() * 0.14;
                self.computational_complexity = 0.70 + rand_simple() * 0.25;
            },
            ParametricDesignApplication::PerformativeFacades => {
                self.optimization_capability = 0.85 + rand_simple() * 0.14;
                self.aesthetic_innovation = 0.80 + rand_simple() * 0.18;
                self.design_flexibility = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.computational_complexity == 0.0 {
            self.computational_complexity = (self.design_flexibility + self.optimization_capability) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_form_finding() {
        let mut system = ParametricDesignSystem::new(ParametricDesignApplication::FormFinding);
        system.analyze_system().unwrap();
        assert!(system.aesthetic_innovation > 0.8);
    }
}
