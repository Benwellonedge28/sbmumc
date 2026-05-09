//! # SBMUMC Module 1069: Experimental Economics
//!
//! Laboratory and field experiments in economic research.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperimentType {
    Lab,
    Field,
    Natural,
    Online,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicExperiment {
    pub experiment_id: String,
    pub experiment_type: ExperimentType,
    pub treatment_groups: usize,
    pub sample_size: usize,
    pub effect_size: f64,
    pub statistical_significance: f64,
    pub replicability_score: f64,
}

impl EconomicExperiment {
    pub fn new(experiment_type: ExperimentType, sample: usize) -> Self {
        Self {
            experiment_id: crate::core::uuid_simple(),
            experiment_type,
            treatment_groups: 0,
            sample_size: sample,
            effect_size: 0.0,
            statistical_significance: 0.0,
            replicability_score: 0.0,
        }
    }

    pub fn analyze_results(&mut self, groups: usize) -> Result<()> {
        self.treatment_groups = groups;

        match self.experiment_type {
            ExperimentType::Lab => {
                self.effect_size = 0.2 + rand_simple() * 0.6;
                self.statistical_significance = 0.85 + rand_simple() * 0.14;
            },
            ExperimentType::Field => {
                self.effect_size = 0.15 + rand_simple() * 0.5;
                self.statistical_significance = 0.80 + rand_simple() * 0.18;
            },
            ExperimentType::Natural => {
                self.effect_size = 0.10 + rand_simple() * 0.4;
                self.statistical_significance = 0.75 + rand_simple() * 0.20;
            },
            ExperimentType::Online => {
                self.effect_size = 0.15 + rand_simple() * 0.55;
                self.statistical_significance = 0.70 + rand_simple() * 0.25;
            }
        }

        let base_replicability = self.statistical_significance * (self.sample_size as f64 / 1000.0).min(1.0);
        self.replicability_score = base_replicability.min(0.95) + rand_simple() * 0.05;
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

pub fn compute_experiment_power(effect_size: f64, sample_size: usize) -> Result<f64> {
    let power = (effect_size * (sample_size as f64).sqrt() / 10.0).min(0.99);
    Ok(power.max(0.05))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lab_experiment() {
        let mut experiment = EconomicExperiment::new(ExperimentType::Lab, 500);
        experiment.analyze_results(3).unwrap();
        assert!(experiment.statistical_significance > 0.8);
    }
}