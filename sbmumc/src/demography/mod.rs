//! Demography Module
//!
//! This module implements demography, population studies,
//! and demographic analysis for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Demography system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Demography {
    pub demo_id: String,
    pub populations: Vec<Population>,
    pub demographic_transition: DemographicTransition,
    pub migration: MigrationPatterns,
    pub projections: PopulationProjection,
}

/// Population
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    pub pop_id: String,
    pub country: String,
    pub total_population: u64,
    pub density: f64,
    pub age_distribution: AgeDistribution,
    pub gender_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeDistribution {
    pub age_groups: HashMap<String, u32>,
    pub median_age: f64,
    pub dependency_ratio: f64,
}

/// Demographic transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemographicTransition {
    pub current_stage: TransitionStage,
    pub birth_rate: f64,
    pub death_rate: f64,
    pub natural_increase: f64,
    pub life_expectancy: LifeExpectancy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransitionStage {
    PreTransition,
    EarlyTransition,
    LateTransition,
    PostTransition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeExpectancy {
    pub at_birth: f64,
    pub at_65: f64,
    pub gender_gap: f64,
}

/// Migration patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPatterns {
    pub net_migration: i64,
    pub emigration: u32,
    pub immigration: u32,
    pub push_factors: Vec<String>,
    pub pull_factors: Vec<String>,
}

/// Population projection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationProjection {
    pub projection_id: String,
    pub base_year: u32,
    pub target_year: u32,
    pub projected_population: u64,
    pub confidence_interval: [u64; 2],
}

impl Demography {
    pub fn new() -> Self {
        Self {
            demo_id: String::from("demography_v1"),
            populations: vec![
                Population { pop_id: String::from("pop_us"), country: String::from("United States"), total_population: 335_000_000, density: 36.0, age_distribution: AgeDistribution { age_groups: HashMap::new(), median_age: 38.5, dependency_ratio: 0.54 }, gender_ratio: 0.97 },
            ],
            demographic_transition: DemographicTransition { current_stage: TransitionStage::PostTransition, birth_rate: 0.011, death_rate: 0.008, natural_increase: 0.003, life_expectancy: LifeExpectancy { at_birth: 77.5, at_65: 18.0, gender_gap: 5.0 } },
            migration: MigrationPatterns { net_migration: 1_000_000, emigration: 100_000, immigration: 1_100_000, push_factors: vec![String::from("Economic")], pull_factors: vec![String::from("Opportunity")] },
            projections: PopulationProjection { projection_id: String::from("proj_1"), base_year: 2024, target_year: 2050, projected_population: 400_000_000, confidence_interval: [380_000_000, 420_000_000] },
        }
    }

    pub fn analyze_population_structure(&self, pop_id: &str) -> PopulationStructureAnalysis {
        PopulationStructureAnalysis { pop_id: pop_id.to_string(), youth_bulge: 0.25, aging_index: 0.45, dependency_ratio: 0.54, recommendations: vec![] }
    }

    pub fn project_demographic_change(&self, years: u32) -> DemographicProjection {
        DemographicProjection { years_ahead: years, population_change: 0.02, key_factors: vec![String::from("Fertility rates")], uncertainty: 0.15 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationStructureAnalysis {
    pub pop_id: String,
    pub youth_bulge: f64,
    pub aging_index: f64,
    pub dependency_ratio: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemographicProjection {
    pub years_ahead: u32,
    pub population_change: f64,
    pub key_factors: Vec<String>,
    pub uncertainty: f64,
}

impl Default for Demography { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let demo = Demography::new(); assert_eq!(demo.demo_id, "demography_v1"); }
}
