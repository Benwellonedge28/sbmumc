//! # SBMUMC Module 1072: Evolutionary Economics
//!
//! Economic change through evolutionary processes.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectionEnvironment {
    Market,
    Technological,
    Institutional,
    Social,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionaryEconomicModel {
    pub model_id: String,
    pub selection_environment: SelectionEnvironment,
    pub variation_rate: f64,
    pub selection_intensity: f64,
    pub replication_rate: f64,
    pub innovation_fitness_gain: f64,
}

impl EvolutionaryEconomicModel {
    pub fn new(environment: SelectionEnvironment) -> Self {
        Self {
            model_id: crate::core::uuid_simple(),
            selection_environment: environment,
            variation_rate: 0.0,
            selection_intensity: 0.0,
            replication_rate: 0.0,
            innovation_fitness_gain: 0.0,
        }
    }

    pub fn simulate_evolution(&mut self) -> Result<()> {
        match self.selection_environment {
            SelectionEnvironment::Market => {
                self.variation_rate = 0.1 + rand_simple() * 0.3;
                self.selection_intensity = 0.7 + rand_simple() * 0.25;
            },
            SelectionEnvironment::Technological => {
                self.variation_rate = 0.15 + rand_simple() * 0.35;
                self.selection_intensity = 0.6 + rand_simple() * 0.30;
            },
            SelectionEnvironment::Institutional => {
                self.variation_rate = 0.05 + rand_simple() * 0.20;
                self.selection_intensity = 0.4 + rand_simple() * 0.35;
            },
            SelectionEnvironment::Social => {
                self.variation_rate = 0.08 + rand_simple() * 0.25;
                self.selection_intensity = 0.5 + rand_simple() * 0.30;
            }
        }

        self.replication_rate = self.selection_intensity * (0.8 + rand_simple() * 0.4);
        self.innovation_fitness_gain = self.variation_rate * self.selection_intensity * 2.0;
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

pub fn compute_evolutionary_trajectory(model_id: &str) -> Result<f64> {
    Ok(rand_simple() * 10.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_evolution() {
        let mut model = EvolutionaryEconomicModel::new(SelectionEnvironment::Market);
        model.simulate_evolution().unwrap();
        assert!(model.innovation_fitness_gain > 0.0);
    }
}