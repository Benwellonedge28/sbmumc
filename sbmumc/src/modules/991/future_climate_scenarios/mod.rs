//! # SBMUMC Module 991: Future Climate Scenarios
//! 
//! Frameworks for projecting and analyzing future climate scenarios.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScenarioType {
    Optimistic,
    Moderate,
    Pessimistic,
    BusinessAsUsual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateScenario {
    pub scenario_id: String,
    pub scenario_type: ScenarioType,
    pub name: String,
    pub temperature_change_c: f64,
    pub sea_level_rise_m: f64,
    pub probability: f64,
    pub key_assumptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioAnalysis {
    pub analysis_id: String,
    pub scenarios: Vec<ClimateScenario>,
    pub expected_temperature: f64,
    pub risk_adjusted_projection: f64,
    pub decision_usefulness: f64,
}

impl ClimateScenario {
    pub fn new(scenario_type: ScenarioType, name: &str) -> Self {
        Self {
            scenario_id: format!("cs_{}", uuid_simple()),
            scenario_type,
            name: name.to_string(),
            temperature_change_c: 0.0,
            sea_level_rise_m: 0.0,
            probability: 0.0,
            key_assumptions: Vec::new(),
        }
    }

    pub fn configure(&mut self, temp: f64, sea: f64, probability: f64) {
        self.temperature_change_c = temp;
        self.sea_level_rise_m = sea;
        self.probability = probability.clamp(0.0, 1.0);
    }

    pub fn add_assumption(&mut self, assumption: &str) {
        self.key_assumptions.push(assumption.to_string());
    }
}

impl ScenarioAnalysis {
    pub fn new() -> Self {
        Self {
            analysis_id: format!("sa_{}", uuid_simple()),
            scenarios: Vec::new(),
            expected_temperature: 0.0,
            risk_adjusted_projection: 0.0,
            decision_usefulness: 0.0,
        }
    }

    pub fn add_scenario(&mut self, scenario: ClimateScenario) {
        self.scenarios.push(scenario);
        self.compute_projections();
    }

    pub fn compute_projections(&mut self) {
        self.expected_temperature = self.scenarios.iter()
            .map(|s| s.temperature_change_c * s.probability)
            .sum::<f64>();
        let total_prob: f64 = self.scenarios.iter()
            .map(|s| s.probability)
            .sum();
        if total_prob > 0.0 {
            self.risk_adjusted_projection = self.expected_temperature / total_prob;
        }
        self.decision_usefulness = (self.scenarios.len() as f64 / 4.0).min(1.0);
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{:x}{:x}", duration.as_secs(), duration.subsec_nanos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climate_scenario() {
        let mut scenario = ClimateScenario::new(
            ScenarioType::Optimistic,
            "Strong Climate Action 1.5C",
        );
        scenario.configure(1.5, 0.3, 0.3);
        scenario.add_assumption("Net zero by 2050");
        assert!(scenario.temperature_change_c > 0.0);
    }

    #[test]
    fn test_scenario_analysis() {
        let mut analysis = ScenarioAnalysis::new();
        analysis.add_scenario(ClimateScenario::new(
            ScenarioType::BusinessAsUsual,
            "No Additional Action",
        ));
        assert!(analysis.decision_usefulness >= 0.0);
    }
}
