//! # SBMUMC Module 1068: Behavioral Economics
//!
//! Psychological factors in economic decision-making.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CognitiveBias {
    Anchoring,
    LossAversion,
    Overconfidence,
    SunkCost,
    Herding,
    PresentBias,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralEconomicModel {
    pub model_id: String,
    pub bias_type: CognitiveBias,
    pub deviation_from_rationality: f64,
    pub impact_magnitude: f64,
    pub frequency_population: f64,
    pub economic_cost_billion: f64,
}

impl BehavioralEconomicModel {
    pub fn new(bias_type: CognitiveBias) -> Self {
        Self {
            model_id: crate::core::uuid_simple(),
            bias_type,
            deviation_from_rationality: 0.0,
            impact_magnitude: 0.0,
            frequency_population: 0.0,
            economic_cost_billion: 0.0,
        }
    }

    pub fn analyze_bias(&mut self) -> Result<()> {
        match self.bias_type {
            CognitiveBias::LossAversion => {
                self.deviation_from_rationality = 0.2 + rand_simple() * 0.15;
                self.impact_magnitude = 0.25 + rand_simple() * 0.20;
                self.frequency_population = 0.7 + rand_simple() * 0.25;
            },
            CognitiveBias::Anchoring => {
                self.deviation_from_rationality = 0.15 + rand_simple() * 0.15;
                self.impact_magnitude = 0.15 + rand_simple() * 0.20;
                self.frequency_population = 0.65 + rand_simple() * 0.30;
            },
            CognitiveBias::PresentBias => {
                self.deviation_from_rationality = 0.25 + rand_simple() * 0.25;
                self.impact_magnitude = 0.30 + rand_simple() * 0.25;
                self.frequency_population = 0.5 + rand_simple() * 0.35;
            },
            _ => {
                self.deviation_from_rationality = 0.10 + rand_simple() * 0.20;
                self.impact_magnitude = 0.10 + rand_simple() * 0.25;
                self.frequency_population = 0.4 + rand_simple() * 0.40;
            }
        }

        self.economic_cost_billion = self.impact_magnitude * self.frequency_population * 100.0;
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

pub fn compute_behavioral_intervention_effectiveness(bias: &str) -> Result<f64> {
    let base = match bias {
        "LossAversion" => 0.3,
        "PresentBias" => 0.25,
        _ => 0.2,
    };
    Ok(base + rand_simple() * 0.3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loss_aversion_bias() {
        let mut model = BehavioralEconomicModel::new(CognitiveBias::LossAversion);
        model.analyze_bias().unwrap();
        assert!(model.deviation_from_rationality > 0.1);
    }
}