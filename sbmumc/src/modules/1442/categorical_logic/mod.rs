//! # SBMUMC Module 1442: Categorical Logic
//!
//! Systems for categorical logic and topos theory.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CategoricalLogicTopic {
    ToposTheory,
    CategoryLocallyPresentable,
    EnrichedCategory,
    HigherCategories,
    FiberedCategories,
    SyntheticDomain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoricalLogicSystem {
    pub system_id: String,
    pub categorical_logic_topic: CategoricalLogicTopic,
    pub internal_logic: f64,
    pub sheaf_semantics: f64,
    pub geometric_morphisms: f64,
    pub logical_truth_values: f64,
}

impl CategoricalLogicSystem {
    pub fn new(categorical_logic_topic: CategoricalLogicTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            categorical_logic_topic,
            internal_logic: 0.0,
            sheaf_semantics: 0.0,
            geometric_morphisms: 0.0,
            logical_truth_values: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.categorical_logic_topic {
            CategoricalLogicTopic::ToposTheory => {
                self.internal_logic = 0.95 + rand_simple() * 0.05;
                self.sheaf_semantics = 0.90 + rand_simple() * 0.10;
                self.geometric_morphisms = 0.85 + rand_simple() * 0.14;
            },
            CategoricalLogicTopic::CategoryLocallyPresentable => {
                self.logical_truth_values = 0.95 + rand_simple() * 0.05;
                self.internal_logic = 0.90 + rand_simple() * 0.10;
                self.sheaf_semantics = 0.85 + rand_simple() * 0.14;
            },
            CategoricalLogicTopic::EnrichedCategory => {
                self.geometric_morphisms = 0.95 + rand_simple() * 0.05;
                self.logical_truth_values = 0.90 + rand_simple() * 0.10;
                self.internal_logic = 0.85 + rand_simple() * 0.14;
            },
            CategoricalLogicTopic::HigherCategories => {
                self.sheaf_semantics = 0.95 + rand_simple() * 0.05;
                self.geometric_morphisms = 0.90 + rand_simple() * 0.10;
                self.logical_truth_values = 0.85 + rand_simple() * 0.14;
            },
            CategoricalLogicTopic::FiberedCategories => {
                self.internal_logic = 0.95 + rand_simple() * 0.05;
                self.logical_truth_values = 0.90 + rand_simple() * 0.10;
                self.sheaf_semantics = 0.85 + rand_simple() * 0.14;
            },
            CategoricalLogicTopic::SyntheticDomain => {
                self.logical_truth_values = 0.95 + rand_simple() * 0.05;
                self.sheaf_semantics = 0.90 + rand_simple() * 0.10;
                self.geometric_morphisms = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.geometric_morphisms == 0.0 {
            self.geometric_morphisms = (self.internal_logic + self.sheaf_semantics) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_topos() {
        let mut system = CategoricalLogicSystem::new(CategoricalLogicTopic::ToposTheory);
        system.analyze_system().unwrap();
        assert!(system.internal_logic > 0.8);
    }
}