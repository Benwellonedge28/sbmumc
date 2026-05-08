//! # SBMUMC Module 950: Weather Control
//! 
//! Technologies and frameworks for modifying weather patterns.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherModification {
    CloudSeeding,
    HurricaneModification,
    FogDissipation,
    RainEnhancement,
    HailSuppression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherProject {
    pub project_id: String,
    pub modification: WeatherModification,
    pub target_area: String,
    pub effectiveness: f64,
    pub side_effects: Vec<String>,
    pub ethical_concerns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherControlPlan {
    pub plan_id: String,
    pub projects: Vec<WeatherProject>,
    pub overall_effectiveness: f64,
    pub risk_assessment: f64,
}

impl WeatherProject {
    pub fn new(modification: WeatherModification, area: &str) -> Self {
        Self {
            project_id: format!("wp_{}", uuid_simple()),
            modification,
            target_area: area.to_string(),
            effectiveness: 0.0,
            side_effects: Vec::new(),
            ethical_concerns: Vec::new(),
        }
    }

    pub fn set_effectiveness(&mut self, effectiveness: f64) {
        self.effectiveness = effectiveness.clamp(0.0, 1.0);
    }

    pub fn add_side_effect(&mut self, effect: &str) {
        self.side_effects.push(effect.to_string());
    }

    pub fn add_ethical_concern(&mut self, concern: &str) {
        self.ethical_concerns.push(concern.to_string());
    }
}

impl WeatherControlPlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("wcp_{}", uuid_simple()),
            projects: Vec::new(),
            overall_effectiveness: 0.0,
            risk_assessment: 0.0,
        }
    }

    pub fn add_project(&mut self, project: WeatherProject) {
        self.projects.push(project);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        if self.projects.is_empty() {
            return;
        }
        self.overall_effectiveness = self.projects.iter()
            .map(|p| p.effectiveness)
            .sum::<f64>() / self.projects.len() as f64;
        self.risk_assessment = (self.projects.len() as f64 * 0.1).min(1.0);
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
    fn test_weather_project() {
        let mut project = WeatherProject::new(
            WeatherModification::CloudSeeding,
            "Colorado Mountains",
        );
        project.set_effectiveness(0.15);
        project.add_side_effect("Potential drought in neighboring regions");
        assert!(project.effectiveness > 0.0);
    }

    #[test]
    fn test_weather_control_plan() {
        let mut plan = WeatherControlPlan::new();
        plan.add_project(WeatherProject::new(
            WeatherModification::RainEnhancement,
            "California",
        ));
        assert!(plan.overall_effectiveness >= 0.0);
    }
}
