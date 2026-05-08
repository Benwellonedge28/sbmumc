//! # SBMUMC Module 971: Climate Data Analytics
//! 
//! Data analytics frameworks for climate science and policy.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    Satellite,
    GroundStation,
    OceanBuoy,
    WeatherBalloon,
    CitizenScience,
    ModelOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateDataset {
    pub dataset_id: String,
    pub source: DataSource,
    pub variable: String,
    pub coverage: String,
    pub temporal_resolution: String,
    pub data_points: u64,
    pub quality_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateAnalysis {
    pub analysis_id: String,
    pub datasets: Vec<ClimateDataset>,
    pub methodology: String,
    pub findings: Vec<String>,
    pub confidence_level: f64,
}

impl ClimateDataset {
    pub fn new(source: DataSource, variable: &str, coverage: &str) -> Self {
        Self {
            dataset_id: format!("cd_{}", uuid_simple()),
            source,
            variable: variable.to_string(),
            coverage: coverage.to_string(),
            temporal_resolution: "daily".to_string(),
            data_points: 0,
            quality_score: 0.0,
        }
    }

    pub fn configure(&mut self, resolution: &str, points: u64, quality: f64) {
        self.temporal_resolution = resolution.to_string();
        self.data_points = points;
        self.quality_score = quality.clamp(0.0, 1.0);
    }

    pub fn value_density(&self) -> f64 {
        if self.data_points == 0 { 0.0 }
        else { self.quality_score * 1000000.0 / self.data_points as f64 }
    }
}

impl ClimateAnalysis {
    pub fn new(methodology: &str) -> Self {
        Self {
            analysis_id: format!("ca_{}", uuid_simple()),
            datasets: Vec::new(),
            methodology: methodology.to_string(),
            findings: Vec::new(),
            confidence_level: 0.0,
        }
    }

    pub fn add_dataset(&mut self, dataset: ClimateDataset) {
        self.datasets.push(dataset);
    }

    pub fn add_finding(&mut self, finding: &str) {
        self.findings.push(finding.to_string());
    }

    pub fn compute_confidence(&mut self) {
        if self.datasets.is_empty() {
            self.confidence_level = 0.0;
            return;
        }
        self.confidence_level = self.datasets.iter()
            .map(|d| d.quality_score)
            .sum::<f64>() / self.datasets.len() as f64;
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
    fn test_climate_dataset() {
        let mut dataset = ClimateDataset::new(
            DataSource::Satellite,
            "Sea Surface Temperature",
            "Global",
        );
        dataset.configure("daily", 1000000, 0.9);
        assert!(dataset.quality_score > 0.8);
    }

    #[test]
    fn test_climate_analysis() {
        let mut analysis = ClimateAnalysis::new("Multi-source synthesis");
        analysis.add_dataset(ClimateDataset::new(DataSource::GroundStation, "Temperature", "Regional"));
        analysis.add_finding("Temperature increasing at 0.2C per decade");
        assert!(analysis.findings.len() == 1);
    }
}
