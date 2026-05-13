//! # SBMUMC Module 1491: Analytic Marxism
//!
//! Systems for analytic Marxism and historical materialism.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalyticMarxismTopic {
    HistoricalMaterialism,
    BaseSuperstructure,
    ClassAnalysis,
    ExploitationTheory,
    IdeologyCritique,
    SocialFormation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticMarxismSystem {
    pub system_id: String,
    pub analytic_marxism_topic: AnalyticMarxismTopic,
    pub materialist_analysis: f64,
    pub historical_dynamics: f64,
    pub class_structure: f64,
    pub ideological_critique: f64,
}

impl AnalyticMarxismSystem {
    pub fn new(analytic_marxism_topic: AnalyticMarxismTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            analytic_marxism_topic,
            materialist_analysis: 0.0,
            historical_dynamics: 0.0,
            class_structure: 0.0,
            ideological_critique: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.analytic_marxism_topic {
            AnalyticMarxismTopic::HistoricalMaterialism => {
                self.materialist_analysis = 0.95 + rand_simple() * 0.05;
                self.historical_dynamics = 0.90 + rand_simple() * 0.10;
                self.class_structure = 0.85 + rand_simple() * 0.14;
            },
            AnalyticMarxismTopic::BaseSuperstructure => {
                self.ideological_critique = 0.95 + rand_simple() * 0.05;
                self.materialist_analysis = 0.90 + rand_simple() * 0.10;
                self.historical_dynamics = 0.85 + rand_simple() * 0.14;
            },
            AnalyticMarxismTopic::ClassAnalysis => {
                self.class_structure = 0.95 + rand_simple() * 0.05;
                self.ideological_critique = 0.90 + rand_simple() * 0.10;
                self.materialist_analysis = 0.85 + rand_simple() * 0.14;
            },
            AnalyticMarxismTopic::ExploitationTheory => {
                self.historical_dynamics = 0.95 + rand_simple() * 0.05;
                self.class_structure = 0.90 + rand_simple() * 0.10;
                self.ideological_critique = 0.85 + rand_simple() * 0.14;
            },
            AnalyticMarxismTopic::IdeologyCritique => {
                self.materialist_analysis = 0.95 + rand_simple() * 0.05;
                self.ideological_critique = 0.90 + rand_simple() * 0.10;
                self.class_structure = 0.85 + rand_simple() * 0.14;
            },
            AnalyticMarxismTopic::SocialFormation => {
                self.historical_dynamics = 0.95 + rand_simple() * 0.05;
                self.materialist_analysis = 0.90 + rand_simple() * 0.10;
                self.ideological_critique = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.class_structure == 0.0 {
            self.class_structure = (self.materialist_analysis + self.historical_dynamics) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_historical() {
        let mut system = AnalyticMarxismSystem::new(AnalyticMarxismTopic::HistoricalMaterialism);
        system.analyze_system().unwrap();
        assert!(system.materialist_analysis > 0.8);
    }
}