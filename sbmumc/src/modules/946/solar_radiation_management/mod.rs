//! # SBMUMC Module 946: Solar Radiation Management
//! 
//! Technologies for managing solar radiation to control global temperature.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SRMTechnique {
    StratosphericAerosolInjection,
    MarineCloudBrightening,
    CirrusCloudThinning,
    SurfaceAlbedoModification,
    SpaceBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRMProject {
    pub project_id: String,
    pub technique: SRMTechnique,
    pub deployment_scale: String,
    pub temperature_reduction_c: f64,
    pub regional_effects: Vec<String>,
    pub termination_shock_risk: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRMEvaluation {
    pub evaluation_id: String,
    pub technique: SRMTechnique,
    pub feasibility: f64,
    pub cost_estimate: f64,
    pub effectiveness: f64,
    pub risks: Vec<String>,
}

impl SRMProject {
    pub fn new(technique: SRMTechnique) -> Self {
        Self {
            project_id: format!("srm_{}", uuid_simple()),
            technique,
            deployment_scale: "experimental".to_string(),
            temperature_reduction_c: 0.0,
            regional_effects: Vec::new(),
            termination_shock_risk: 0.0,
        }
    }

    pub fn set_reduction(&mut self, reduction: f64) {
        self.temperature_reduction_c = reduction;
    }

    pub fn add_regional_effect(&mut self, effect: &str) {
        self.regional_effects.push(effect.to_string());
    }
}

impl SRMEvaluation {
    pub fn new(technique: SRMTechnique) -> Self {
        Self {
            evaluation_id: format!("srm_eval_{}", uuid_simple()),
            technique,
            feasibility: 0.0,
            cost_estimate: 0.0,
            effectiveness: 0.0,
            risks: Vec::new(),
        }
    }

    pub fn assess(&mut self, feasibility: f64, cost: f64, effectiveness: f64) {
        self.feasibility = feasibility.clamp(0.0, 1.0);
        self.cost_estimate = cost;
        self.effectiveness = effectiveness.clamp(0.0, 1.0);
    }

    pub fn add_risk(&mut self, risk: &str) {
        self.risks.push(risk.to_string());
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
    fn test_srm_project() {
        let mut project = SRMProject::new(SRMTechnique::StratosphericAerosolInjection);
        project.set_reduction(1.5);
        project.add_regional_effect("Asian monsoon disruption");
        assert!(project.temperature_reduction_c > 0.0);
    }

    #[test]
    fn test_srm_evaluation() {
        let mut eval = SRMEvaluation::new(SRMTechnique::MarineCloudBrightening);
        eval.assess(0.7, 5000000000.0, 0.8);
        assert!(eval.feasibility > 0.0);
    }
}
