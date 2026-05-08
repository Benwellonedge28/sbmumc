//! # SBMUMC Module 958: Biodiversity Carbon
//! 
//! Integration of biodiversity and carbon sequestration frameworks.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiodiversityMetric {
    SpeciesRichness,
    ShannonIndex,
    SimpsonIndex,
    FunctionalDiversity,
    PhylogeneticDiversity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioCarbonProject {
    pub project_id: String,
    pub name: String,
    pub biodiversity_score: f64,
    pub carbon_stock_tc: f64,
    pub co_benefits: Vec<String>,
    pub trade_offs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioCarbonPortfolio {
    pub portfolio_id: String,
    pub projects: Vec<BioCarbonProject>,
    pub biodiversity_index: f64,
    pub total_carbon: f64,
    pub co_benefit_score: f64,
}

impl BioCarbonProject {
    pub fn new(name: &str) -> Self {
        Self {
            project_id: format!("bcp_{}", uuid_simple()),
            name: name.to_string(),
            biodiversity_score: 0.0,
            carbon_stock_tc: 0.0,
            co_benefits: Vec::new(),
            trade_offs: Vec::new(),
        }
    }

    pub fn configure(&mut self, biodiversity: f64, carbon: f64) {
        self.biodiversity_score = biodiversity.clamp(0.0, 1.0);
        self.carbon_stock_tc = carbon;
    }

    pub fn add_co_benefit(&mut self, benefit: &str) {
        self.co_benefits.push(benefit.to_string());
    }

    pub fn add_trade_off(&mut self, tradeoff: &str) {
        self.trade_offs.push(tradeoff.to_string());
    }
}

impl BioCarbonPortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("bcport_{}", uuid_simple()),
            projects: Vec::new(),
            biodiversity_index: 0.0,
            total_carbon: 0.0,
            co_benefit_score: 0.0,
        }
    }

    pub fn add_project(&mut self, project: BioCarbonProject) {
        self.projects.push(project);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.biodiversity_index = self.projects.iter()
            .map(|p| p.biodiversity_score)
            .sum::<f64>() / self.projects.len().max(1) as f64;
        self.total_carbon = self.projects.iter()
            .map(|p| p.carbon_stock_tc)
            .sum();
        self.co_benefit_score = self.projects.iter()
            .map(|p| p.co_benefits.len() as f64)
            .sum::<f64>() / self.projects.len().max(1) as f64;
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
    fn test_bio_carbon_project() {
        let mut project = BioCarbonProject::new("Amazon REDD+");
        project.configure(0.85, 250000.0);
        project.add_co_benefit("Water conservation");
        assert!(project.biodiversity_score > 0.0);
    }

    #[test]
    fn test_bio_carbon_portfolio() {
        let mut portfolio = BioCarbonPortfolio::new();
        portfolio.add_project(BioCarbonProject::new("Congo Basin"));
        assert!(portfolio.biodiversity_index >= 0.0);
    }
}
