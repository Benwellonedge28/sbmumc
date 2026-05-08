//! # SBMUMC Module 956: Desert Rewilding
//! 
//! Frameworks for restoring desert ecosystems for climate benefits.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewildingTechnique {
    WaterHarvesting,
    DuneStabilization,
    NativeSpeciesReintroduction,
    MycorrhizalNetworks,
    FogCollection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesertRewildingProject {
    pub project_id: String,
    pub desert_name: String,
    pub technique: RewildingTechnique,
    pub area_km2: f64,
    pub species_diversity_score: f64,
    pub carbon_sequestration_tc: f64,
    pub water_conservation_m3: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesertRewildingPortfolio {
    pub portfolio_id: String,
    pub projects: Vec<DesertRewildingProject>,
    pub total_area: f64,
    pub total_carbon: f64,
    pub biodiversity_index: f64,
}

impl DesertRewildingProject {
    pub fn new(desert: &str, technique: RewildingTechnique) -> Self {
        Self {
            project_id: format!("drp_{}", uuid_simple()),
            desert_name: desert.to_string(),
            technique,
            area_km2: 0.0,
            species_diversity_score: 0.0,
            carbon_sequestration_tc: 0.0,
            water_conservation_m3: 0.0,
        }
    }

    pub fn configure(&mut self, area: f64, biodiversity: f64, carbon: f64, water: f64) {
        self.area_km2 = area;
        self.species_diversity_score = biodiversity;
        self.carbon_sequestration_tc = carbon;
        self.water_conservation_m3 = water;
    }
}

impl DesertRewildingPortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("drport_{}", uuid_simple()),
            projects: Vec::new(),
            total_area: 0.0,
            total_carbon: 0.0,
            biodiversity_index: 0.0,
        }
    }

    pub fn add_project(&mut self, project: DesertRewildingProject) {
        self.projects.push(project);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_area = self.projects.iter()
            .map(|p| p.area_km2)
            .sum();
        self.total_carbon = self.projects.iter()
            .map(|p| p.carbon_sequestration_tc)
            .sum();
        self.biodiversity_index = self.projects.iter()
            .map(|p| p.species_diversity_score)
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
    fn test_desert_rewilding_project() {
        let mut project = DesertRewildingProject::new(
            "Sahara",
            RewildingTechnique::WaterHarvesting,
        );
        project.configure(1000.0, 0.7, 50000.0, 1000000.0);
        assert!(project.area_km2 > 0.0);
    }

    #[test]
    fn test_desert_rewilding_portfolio() {
        let mut portfolio = DesertRewildingPortfolio::new();
        portfolio.add_project(DesertRewildingProject::new(
            "Negev",
            RewildingTechnique::FogCollection,
        ));
        assert!(portfolio.total_area >= 0.0);
    }
}
