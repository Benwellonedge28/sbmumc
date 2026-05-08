//! # SBMUMC Module 986: Citizen Science Climate
//! 
//! Frameworks for citizen science in climate monitoring.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataCollectionType {
    SpeciesMonitoring,
    WeatherObservation,
    AirQuality,
    WaterQuality,
    Phenology,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitizenScienceProject {
    pub project_id: String,
    pub name: String,
    pub collection_type: DataCollectionType,
    pub volunteer_count: u32,
    pub data_points_collected: u64,
    pub data_quality_score: f64,
    pub scientific_contribution: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitizenScienceNetwork {
    pub network_id: String,
    pub projects: Vec<CitizenScienceProject>,
    pub total_volunteers: u32,
    pub total_data_points: u64,
    pub aggregate_quality_score: f64,
    pub scientific_impact: f64,
}

impl CitizenScienceProject {
    pub fn new(name: &str, ctype: DataCollectionType) -> Self {
        Self {
            project_id: format!("csp_{}", uuid_simple()),
            name: name.to_string(),
            collection_type: ctype,
            volunteer_count: 0,
            data_points_collected: 0,
            data_quality_score: 0.0,
            scientific_contribution: 0.0,
        }
    }

    pub fn configure(&mut self, volunteers: u32, data_points: u64, quality: f64) {
        self.volunteer_count = volunteers;
        self.data_points_collected = data_points;
        self.data_quality_score = quality.clamp(0.0, 1.0);
        self.scientific_contribution = (data_points as f64 / 10000.0).min(1.0) * quality;
    }
}

impl CitizenScienceNetwork {
    pub fn new() -> Self {
        Self {
            network_id: format!("csn_{}", uuid_simple()),
            projects: Vec::new(),
            total_volunteers: 0,
            total_data_points: 0,
            aggregate_quality_score: 0.0,
            scientific_impact: 0.0,
        }
    }

    pub fn add_project(&mut self, project: CitizenScienceProject) {
        self.projects.push(project);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_volunteers = self.projects.iter()
            .map(|p| p.volunteer_count)
            .sum();
        self.total_data_points = self.projects.iter()
            .map(|p| p.data_points_collected)
            .sum();
        self.aggregate_quality_score = self.projects.iter()
            .map(|p| p.data_quality_score)
            .sum::<f64>() / self.projects.len().max(1) as f64;
        self.scientific_impact = self.projects.iter()
            .map(|p| p.scientific_contribution)
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
    fn test_citizen_science_project() {
        let mut project = CitizenScienceProject::new(
            "Globe Observer",
            DataCollectionType::Phenology,
        );
        project.configure(5000, 100000, 0.85);
        assert!(project.scientific_contribution > 0.0);
    }

    #[test]
    fn test_citizen_science_network() {
        let mut network = CitizenScienceNetwork::new();
        network.add_project(CitizenScienceProject::new(
            "AirQuality Monitoring",
            DataCollectionType::AirQuality,
        ));
        assert!(network.total_volunteers >= 0);
    }
}
