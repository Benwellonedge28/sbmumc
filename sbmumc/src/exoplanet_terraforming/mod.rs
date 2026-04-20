//! Exoplanet Terraforming Module
//!
//! This module implements planetary engineering for exoplanets,
//! including atmosphere creation, surface modification, and habitability optimization.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Exoplanet terraforming operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExoplanetTerraforming {
    pub terraform_id: String,
    pub target_planets: Vec<TargetPlanet>,
    pub active_projects: Vec<TerraformProject>,
    pub resource_requirements: ResourceBudget,
    pub timeline_estimates: Vec<TimeEstimate>,
}

/// Target exoplanet for terraforming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetPlanet {
    pub planet_id: String,
    pub star_name: String,
    pub distance_ly: f64,
    pub mass: f64,
    pub radius: f64,
    pub surface_temperature: f64,
    pub current_atmosphere: Option<Atmosphere>,
    pub surface_water: f64,
    pub terraforming_potential: f64,
}

/// Atmosphere composition and properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Atmosphere {
    pub composition: HashMap<String, f64>,
    pub pressure: f64,
    pub temperature_gradient: TemperatureProfile,
    pub greenhouse_factor: f64,
    pub toxicity: f64,
    pub habitability: f64,
}

/// Temperature profile across atmosphere
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureProfile {
    pub surface_temp: f64,
    pub tropopause_temp: f64,
    pub stratopause_temp: f64,
    pub mesopause_temp: f64,
    pub gradient_type: GradientType,
}

/// Gradient type for temperature profile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GradientType {
    LapseRate,
    Inversion,
    Isothermal,
    Custom,
}

/// Active terraforming project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformProject {
    pub project_id: String,
    pub planet_id: String,
    pub phase: TerraformPhase,
    pub start_date: String,
    pub estimated_completion: String,
    pub operations: Vec<Operation>,
    pub progress_percentage: f64,
}

/// Phases of terraforming process
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TerraformPhase {
    Survey,
    Planning,
    AtmosphereCreation,
    TemperatureStabilization,
    WaterIntroduction,
    SoilCreation,
    AtmosphereBreathing,
    EcosystemIntroduction,
    FinalOptimization,
    Complete,
}

/// Individual terraforming operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub operation_id: String,
    pub operation_type: OperationType,
    pub description: String,
    pub energy_requirement: f64,
    pub material_requirement: f64,
    pub duration_days: f64,
    pub risk_level: RiskLevel,
}

/// Types of terraforming operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperationType {
    AtmosphericProcessing,
    GreenhouseGasInjection,
    IceCometDelivery,
    SolarMirrorDeployment,
    MagneticFieldGeneration,
    MicrobialSeeding,
    AlgaeCultivation,
    SoilGenesis,
    SeedPlanting,
    AnimalIntroduction,
    WaterExtraction,
    AsteroidRedirect,
}

/// Risk levels for operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Negligible,
    Low,
    Moderate,
    High,
    Catastrophic,
}

/// Resource budget for terraforming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceBudget {
    pub energy_budget: f64,
    pub material_budget: f64,
    pub workforce_budget: u64,
    pub technology_level: u32,
    pub funding_level: f64,
}

/// Timeline estimates for terraforming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEstimate {
    pub phase: TerraformPhase,
    pub minimum_duration: f64,
    pub maximum_duration: f64,
    pub average_duration: f64,
    pub acceleration_possible: f64,
}

impl ExoplanetTerraforming {
    /// Creates a new exoplanet terraforming system
    pub fn new() -> Self {
        Self {
            terraform_id: String::from("exoplanet_terraform_v1"),
            target_planets: Vec::new(),
            active_projects: Vec::new(),
            resource_requirements: ResourceBudget {
                energy_budget: 1e30,
                material_budget: 1e25,
                workforce_budget: 1e6,
                technology_level: 10,
                funding_level: 1e12,
            },
            timeline_estimates: Vec::new(),
        }
    }

    /// Registers a target exoplanet
    pub fn register_target(&mut self, planet: TargetPlanet) -> Result<&TargetPlanet> {
        self.target_planets.push(planet);
        Ok(self.target_planets.last().unwrap())
    }

    /// Initiates terraforming project
    pub fn initiate_project(&mut self, planet_id: &str) -> Result<&TerraformProject> {
        let project = TerraformProject {
            project_id: format!("project_{}", self.active_projects.len() + 1),
            planet_id: planet_id.to_string(),
            phase: TerraformPhase::Survey,
            start_date: String::from("2024-01-01"),
            estimated_completion: String::from("2200-01-01"),
            operations: Vec::new(),
            progress_percentage: 0.0,
        };
        self.active_projects.push(project);
        Ok(self.active_projects.last().unwrap())
    }

    /// Designs terraforming sequence
    pub fn design_sequence(&mut self, project_id: &str, target_atmosphere: &Atmosphere) -> Result<Vec<Operation>> {
        let operations = vec![
            Operation {
                operation_id: String::from("op_1"),
                operation_type: OperationType::AsteroidRedirect,
                description: String::from("Redirect ice asteroids for water delivery"),
                energy_requirement: 1e20,
                material_requirement: 1e15,
                duration_days: 36500.0,
                risk_level: RiskLevel::High,
            },
            Operation {
                operation_id: String::from("op_2"),
                operation_type: OperationType::GreenhouseGasInjection,
                description: String::from("Inject greenhouse gases for temperature increase"),
                energy_requirement: 1e18,
                material_requirement: 1e12,
                duration_days: 18250.0,
                risk_level: RiskLevel::Moderate,
            },
            Operation {
                operation_id: String::from("op_3"),
                operation_type: OperationType::AtmosphericProcessing,
                description: String::from("Process atmosphere for oxygen-nitrogen mix"),
                energy_requirement: 1e22,
                material_requirement: 1e14,
                duration_days: 36500.0,
                risk_level: RiskLevel::Moderate,
            },
            Operation {
                operation_id: String::from("op_4"),
                operation_type: OperationType::MicrobialSeeding,
                description: String::from("Seed planet with oxygen-producing microbes"),
                energy_requirement: 1e15,
                material_requirement: 1e10,
                duration_days: 7300.0,
                risk_level: RiskLevel::Low,
            },
            Operation {
                operation_id: String::from("op_5"),
                operation_type: OperationType::EcosystemIntroduction,
                description: String::from("Introduce plant and animal life"),
                energy_requirement: 1e17,
                material_requirement: 1e12,
                duration_days: 365000.0,
                risk_level: RiskLevel::High,
            },
        ];
        if let Some(project) = self.active_projects.iter_mut().find(|p| p.project_id == project_id) {
            project.operations = operations.clone();
        }
        Ok(operations)
    }

    /// Calculates terraforming timeline
    pub fn calculate_timeline(&self, planet: &TargetPlanet, target_type: &TerraformTarget) -> TerraformingTimeline {
        let base_time = match target_type {
            TerraformTarget::BasicHabitation => 500.0,
            TerraformTarget::Agricultural => 1000.0,
            TerraformTarget::Civilization => 2000.0,
            TerraformTarget::Paradise => 5000.0,
        };
        let difficulty_factor = (1.0 - planet.terraforming_potential).max(0.1);
        TerraformingTimeline {
            planet_id: planet.planet_id.clone(),
            target_type: target_type.clone(),
            estimated_years: base_time * difficulty_factor * (planet.distance_ly / 10.0).sqrt(),
            phases: 10,
            energy_total: 1e30 * difficulty_factor,
            materials_total: 1e25 * difficulty_factor,
        }
    }

    /// Evaluates terraforming viability
    pub fn evaluate_viability(&self, planet: &TargetPlanet) -> ViabilityAssessment {
        let mass_factor = if planet.mass > 0.5 && planet.mass < 10.0 { 1.0 } else { 0.5 };
        let distance_factor = if planet.distance_ly < 100.0 { 1.0 } else { 0.5 };
        let atmosphere_factor = if planet.current_atmosphere.is_some() { 1.2 } else { 0.8 };
        let water_factor = if planet.surface_water > 0.0 { 1.5 } else { 0.7 };
        let overall = (mass_factor + distance_factor + atmosphere_factor + water_factor) / 4.0;
        ViabilityAssessment {
            planet_id: planet.planet_id.clone(),
            viability_score: overall,
            recommendation: if overall > 0.7 {
                TerraformRecommendation::Recommended
            } else if overall > 0.4 {
                TerraformRecommendation::Consider
            } else {
                TerraformRecommendation::NotRecommended
            },
            challenges: vec![String::from("Resource transport"), String::from("Long timeline")],
            benefits: vec![String::from("New habitable world"), String::from("Scientific value")],
        }
    }

    /// Advances project to next phase
    pub fn advance_phase(&mut self, project_id: &str) -> Result<&TerraformPhase> {
        if let Some(project) = self.active_projects.iter_mut().find(|p| p.project_id == project_id) {
            let next = match project.phase {
                TerraformPhase::Survey => TerraformPhase::Planning,
                TerraformPhase::Planning => TerraformPhase::AtmosphereCreation,
                TerraformPhase::AtmosphereCreation => TerraformPhase::TemperatureStabilization,
                TerraformPhase::TemperatureStabilization => TerraformPhase::WaterIntroduction,
                TerraformPhase::WaterIntroduction => TerraformPhase::SoilCreation,
                TerraformPhase::SoilCreation => TerraformPhase::AtmosphereBreathing,
                TerraformPhase::AtmosphereBreathing => TerraformPhase::EcosystemIntroduction,
                TerraformPhase::EcosystemIntroduction => TerraformPhase::FinalOptimization,
                TerraformPhase::FinalOptimization => TerraformPhase::Complete,
                TerraformPhase::Complete => return Err(SbmumcError::InvalidInput(String::from("Project complete"))),
            };
            project.phase = next;
            project.progress_percentage = match &project.phase {
                TerraformPhase::Survey => 10.0,
                TerraformPhase::Planning => 20.0,
                TerraformPhase::AtmosphereCreation => 35.0,
                TerraformPhase::TemperatureStabilization => 50.0,
                TerraformPhase::WaterIntroduction => 65.0,
                TerraformPhase::SoilCreation => 75.0,
                TerraformPhase::AtmosphereBreathing => 85.0,
                TerraformPhase::EcosystemIntroduction => 95.0,
                TerraformPhase::FinalOptimization => 99.0,
                TerraformPhase::Complete => 100.0,
            };
            Ok(&project.phase)
        } else {
            Err(SbmumcError::NotFound(String::from("Project not found")))
        }
    }
}

/// Terraforming target levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TerraformTarget {
    BasicHabitation,
    Agricultural,
    Civilization,
    Paradise,
}

/// Timeline for terraforming project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformingTimeline {
    pub planet_id: String,
    pub target_type: TerraformTarget,
    pub estimated_years: f64,
    pub phases: usize,
    pub energy_total: f64,
    pub materials_total: f64,
}

/// Viability assessment for planet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViabilityAssessment {
    pub planet_id: String,
    pub viability_score: f64,
    pub recommendation: TerraformRecommendation,
    pub challenges: Vec<String>,
    pub benefits: Vec<String>,
}

/// Terraforming recommendations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TerraformRecommendation {
    HighlyRecommended,
    Recommended,
    Consider,
    NotRecommended,
    NotFeasible,
}

impl Default for ExoplanetTerraforming {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planet_registration() {
        let mut terraforming = ExoplanetTerraforming::new();
        let planet = TargetPlanet {
            planet_id: String::from("kepler_442b"),
            star_name: String::from("Kepler_442"),
            distance_ly: 112.0,
            mass: 2.3,
            radius: 1.4,
            surface_temperature: 273.0,
            current_atmosphere: None,
            surface_water: 0.3,
            terraforming_potential: 0.7,
        };
        let result = terraforming.register_target(planet);
        assert!(result.is_ok());
    }

    #[test]
    fn test_project_initiation() {
        let mut terraforming = ExoplanetTerraforming::new();
        let project = terraforming.initiate_project("kepler_442b");
        assert!(project.is_ok());
    }

    #[test]
    fn test_viability_evaluation() {
        let terraforming = ExoplanetTerraforming::new();
        let planet = TargetPlanet {
            planet_id: String::from("test"),
            star_name: String::from("test_star"),
            distance_ly: 50.0,
            mass: 1.0,
            radius: 1.0,
            surface_temperature: 300.0,
            current_atmosphere: None,
            surface_water: 0.5,
            terraforming_potential: 0.8,
        };
        let assessment = terraforming.evaluate_viability(&planet);
        assert!(assessment.viability_score > 0.0);
    }
}
