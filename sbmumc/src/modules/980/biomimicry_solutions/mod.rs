//! # SBMUMC Module 980: Biomimicry Solutions
//! 
//! Nature-inspired climate solutions and innovations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiomimicryApproach {
    Structure,
    Process,
    System,
    Material,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomimicryInnovation {
    pub innovation_id: String,
    pub name: String,
    pub inspiration: String,
    pub approach: BiomimicryApproach,
    pub climate_application: String,
    pub carbon_impact_tco2: f64,
    pub development_stage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomimicryPortfolio {
    pub portfolio_id: String,
    pub innovations: Vec<BiomimicryInnovation>,
    pub total_potential_impact: f64,
    pub average_maturity: f64,
    pub sector_diversity: u32,
}

impl BiomimicryInnovation {
    pub fn new(name: &str, inspiration: &str, approach: BiomimicryApproach) -> Self {
        Self {
            innovation_id: format!("bi_{}", uuid_simple()),
            name: name.to_string(),
            inspiration: inspiration.to_string(),
            approach,
            climate_application: String::new(),
            carbon_impact_tco2: 0.0,
            development_stage: "Concept".to_string(),
        }
    }

    pub fn configure(&mut self, application: &str, impact: f64, stage: &str) {
        self.climate_application = application.to_string();
        self.carbon_impact_tco2 = impact;
        self.development_stage = stage.to_string();
    }

    pub fn maturity_score(&self) -> f64 {
        match self.development_stage.as_str() {
            "Concept" => 0.1,
            "Research" => 0.3,
            "Prototype" => 0.5,
            "Pilot" => 0.7,
            "Commercial" => 0.9,
            "Scaled" => 1.0,
            _ => 0.0,
        }
    }
}

impl BiomimicryPortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("bport_{}", uuid_simple()),
            innovations: Vec::new(),
            total_potential_impact: 0.0,
            average_maturity: 0.0,
            sector_diversity: 0,
        }
    }

    pub fn add_innovation(&mut self, innovation: BiomimicryInnovation) {
        self.innovations.push(innovation);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_potential_impact = self.innovations.iter()
            .map(|i| i.carbon_impact_tco2 * i.maturity_score())
            .sum();
        self.average_maturity = self.innovations.iter()
            .map(|i| i.maturity_score())
            .sum::<f64>() / self.innovations.len().max(1) as f64;
        self.sector_diversity = self.innovations.iter()
            .map(|i| i.climate_application.chars().count())
            .sum::<usize>() as u32;
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
    fn test_biomimicry_innovation() {
        let mut innovation = BiomimicryInnovation::new(
            "Termite Mound Cooling",
            "Termite mounds",
            BiomimicryApproach::Structure,
        );
        innovation.configure("Building cooling", 5000.0, "Pilot");
        assert!(innovation.maturity_score() > 0.5);
    }

    #[test]
    fn test_biomimicry_portfolio() {
        let mut portfolio = BiomimicryPortfolio::new();
        portfolio.add_innovation(BiomimicryInnovation::new(
            "Lotus Leaf Effect",
            "Lotus leaf",
            BiomimicryApproach::Material,
        ));
        assert!(portfolio.total_potential_impact >= 0.0);
    }
}
