//! # SBMUMC Module 970: Synthetic Biology Climate
//! 
//! Synthetic biology applications for climate solutions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SynBioApplication {
    CarbonCapture,
    Biofuels,
    BiodegradablePlastics,
    NitrogenFixation,
    MethaneReduction,
    AlgaeProduction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynBioProject {
    pub project_id: String,
    pub application: SynBioApplication,
    pub organism: String,
    pub scale: String,
    pub carbon_impact_tco2: f64,
    pub development_stage: String,
    pub timeline_years: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynBioClimatePortfolio {
    pub portfolio_id: String,
    pub projects: Vec<SynBioProject>,
    pub total_carbon_impact: f64,
    pub average_maturity: f64,
    pub pipeline_value: f64,
}

impl SynBioProject {
    pub fn new(application: SynBioApplication, organism: &str) -> Self {
        Self {
            project_id: format!("sbp_{}", uuid_simple()),
            application,
            organism: organism.to_string(),
            scale: "experimental".to_string(),
            carbon_impact_tco2: 0.0,
            development_stage: "research".to_string(),
            timeline_years: 0,
        }
    }

    pub fn configure(&mut self, scale: &str, impact: f64, stage: &str, timeline: u32) {
        self.scale = scale.to_string();
        self.carbon_impact_tco2 = impact;
        self.development_stage = stage.to_string();
        self.timeline_years = timeline;
    }

    pub fn maturity_score(&self) -> f64 {
        match self.development_stage.as_str() {
            "research" => 0.2,
            "development" => 0.4,
            "pilot" => 0.6,
            "commercial" => 0.8,
            "scaled" => 1.0,
            _ => 0.0,
        }
    }
}

impl SynBioClimatePortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("sbcp_{}", uuid_simple()),
            projects: Vec::new(),
            total_carbon_impact: 0.0,
            average_maturity: 0.0,
            pipeline_value: 0.0,
        }
    }

    pub fn add_project(&mut self, project: SynBioProject) {
        self.projects.push(project);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_carbon_impact = self.projects.iter()
            .map(|p| p.carbon_impact_tco2)
            .sum();
        self.average_maturity = self.projects.iter()
            .map(|p| p.maturity_score())
            .sum::<f64>() / self.projects.len().max(1) as f64;
        self.pipeline_value = self.total_carbon_impact * self.average_maturity;
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
    fn test_synbio_project() {
        let mut project = SynBioProject::new(
            SynBioApplication::CarbonCapture,
            "Engineered E. coli",
        );
        project.configure("pilot", 10000.0, "pilot", 5);
        assert!(project.maturity_score() > 0.5);
    }

    #[test]
    fn test_synbio_portfolio() {
        let mut portfolio = SynBioClimatePortfolio::new();
        portfolio.add_project(SynBioProject::new(
            SynBioApplication::Biofuels,
            "Algae",
        ));
        assert!(portfolio.total_carbon_impact >= 0.0);
    }
}
