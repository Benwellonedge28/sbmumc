//! # SBMUMC Module 1163: Instructional Design
//!
//! Systematic creation of educational experiences and materials.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstructionalModel {
    ADDIE,
    SAM,
    BackwardDesign,
    Agile,
    DickAndCarey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionalDesignFramework {
    pub framework_id: String,
    pub model: InstructionalModel,
    pub needs_analysis: f64,
    pub learning_objective_clarity: f64,
    pub content_alignment: f64,
    pub evaluation_design: f64,
}

impl InstructionalDesignFramework {
    pub fn new(model: InstructionalModel) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            model,
            needs_analysis: 0.0,
            learning_objective_clarity: 0.0,
            content_alignment: 0.0,
            evaluation_design: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.model {
            InstructionalModel::ADDIE => {
                self.needs_analysis = 0.80 + rand_simple() * 0.18;
                self.learning_objective_clarity = 0.75 + rand_simple() * 0.22;
                self.evaluation_design = 0.75 + rand_simple() * 0.22;
            },
            InstructionalModel::BackwardDesign => {
                self.needs_analysis = 0.75 + rand_simple() * 0.22;
                self.learning_objective_clarity = 0.90 + rand_simple() * 0.10;
                self.evaluation_design = 0.85 + rand_simple() * 0.14;
            },
            InstructionalModel::SAM => {
                self.needs_analysis = 0.65 + rand_simple() * 0.30;
                self.content_alignment = 0.80 + rand_simple() * 0.18;
                self.evaluation_design = 0.70 + rand_simple() * 0.25;
            },
            InstructionalModel::Agile => {
                self.needs_analysis = 0.60 + rand_simple() * 0.35;
                self.content_alignment = 0.85 + rand_simple() * 0.14;
                self.evaluation_design = 0.65 + rand_simple() * 0.30;
            },
            InstructionalModel::DickAndCarey => {
                self.needs_analysis = 0.75 + rand_simple() * 0.22;
                self.learning_objective_clarity = 0.80 + rand_simple() * 0.18;
                self.evaluation_design = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.content_alignment == 0.0 {
            self.content_alignment = (self.needs_analysis + self.learning_objective_clarity) / 2.0;
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
    fn test_backward_design() {
        let mut framework = InstructionalDesignFramework::new(InstructionalModel::BackwardDesign);
        framework.analyze_framework().unwrap();
        assert!(framework.learning_objective_clarity > 0.7);
    }
}