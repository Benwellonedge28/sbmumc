//! # SBMUMC Module 990: Climate Ethics
//! 
//! Ethical frameworks for climate change decision-making.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EthicalFramework {
    Utilitarian,
    RightsBased,
    VirtueEthics,
    Intergenerational,
    Ecocentric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateEthicalDilemma {
    pub dilemma_id: String,
    pub description: String,
    pub frameworks_applied: Vec<EthicalFramework>,
    pub conflicting_values: Vec<String>,
    pub recommended_resolution: String,
    pub ethical_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateEthicsSystem {
    pub system_id: String,
    pub dilemmas: Vec<ClimateEthicalDilemma>,
    pub dominant_framework: EthicalFramework,
    pub ethical_coherence_score: f64,
}

impl ClimateEthicalDilemma {
    pub fn new(description: &str) -> Self {
        Self {
            dilemma_id: format!("ced_{}", uuid_simple()),
            description: description.to_string(),
            frameworks_applied: Vec::new(),
            conflicting_values: Vec::new(),
            recommended_resolution: String::new(),
            ethical_score: 0.0,
        }
    }

    pub fn add_framework(&mut self, framework: EthicalFramework) {
        self.frameworks_applied.push(framework);
    }

    pub fn add_conflict(&mut self, value: &str) {
        self.conflicting_values.push(value.to_string());
    }

    pub fn resolve(&mut self, resolution: &str) {
        self.recommended_resolution = resolution.to_string();
        self.ethical_score = (self.frameworks_applied.len() as f64 / 3.0).min(1.0);
    }
}

impl ClimateEthicsSystem {
    pub fn new() -> Self {
        Self {
            system_id: format!("ces_{}", uuid_simple()),
            dilemmas: Vec::new(),
            dominant_framework: EthicalFramework::Utilitarian,
            ethical_coherence_score: 0.0,
        }
    }

    pub fn add_dilemma(&mut self, dilemma: ClimateEthicalDilemma) {
        self.dilemmas.push(dilemma);
        self.compute_coherence();
    }

    pub fn compute_coherence(&mut self) {
        self.ethical_coherence_score = self.dilemmas.iter()
            .map(|d| d.ethical_score)
            .sum::<f64>() / self.dilemmas.len().max(1) as f64;
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
    fn test_climate_ethical_dilemma() {
        let mut dilemma = ClimateEthicalDilemma::new(
            "Geoengineering deployment without global consent",
        );
        dilemma.add_framework(EthicalFramework::Utilitarian);
        dilemma.add_framework(EthicalFramework::RightsBased);
        dilemma.add_conflict("Global benefit vs national sovereignty");
        dilemma.resolve("Require international governance framework");
        assert!(dilemma.ethical_score > 0.0);
    }

    #[test]
    fn test_climate_ethics_system() {
        let mut system = ClimateEthicsSystem::new();
        system.add_dilemma(ClimateEthicalDilemma::new(
            "Carbon tax regressive effects",
        ));
        assert!(system.ethical_coherence_score >= 0.0);
    }
}
