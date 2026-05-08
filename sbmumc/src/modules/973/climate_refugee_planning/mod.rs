//! # SBMUMC Module 973: Climate Refugee Planning
//! 
//! Frameworks for planning and managing climate-related migration.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationDriver {
    SeaLevelRise,
    Desertification,
    ExtremeHeat,
    WaterScarcity,
    StormSeverity,
    AgriculturalFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateMigrationProjection {
    pub projection_id: String,
    pub driver: MigrationDriver,
    pub origin_region: String,
    pub estimated_population: u64,
    pub timeline_years: u32,
    pub confidence_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPlan {
    pub plan_id: String,
    pub projections: Vec<ClimateMigrationProjection>,
    pub total_displaced: u64,
    pub receiving_capacity: u64,
    pub resource_requirements: f64,
}

impl ClimateMigrationProjection {
    pub fn new(driver: MigrationDriver, origin: &str) -> Self {
        Self {
            projection_id: format!("cmp_{}", uuid_simple()),
            driver,
            origin_region: origin.to_string(),
            estimated_population: 0,
            timeline_years: 0,
            confidence_level: 0.0,
        }
    }

    pub fn configure(&mut self, population: u64, timeline: u32, confidence: f64) {
        self.estimated_population = population;
        self.timeline_years = timeline;
        self.confidence_level = confidence.clamp(0.0, 1.0);
    }
}

impl MigrationPlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("mp_{}", uuid_simple()),
            projections: Vec::new(),
            total_displaced: 0,
            receiving_capacity: 0,
            resource_requirements: 0.0,
        }
    }

    pub fn add_projection(&mut self, projection: ClimateMigrationProjection) {
        self.projections.push(projection);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_displaced = self.projections.iter()
            .map(|p| p.estimated_population)
            .sum();
        self.receiving_capacity = self.total_displaced / 2;
        self.resource_requirements = self.total_displaced as f64 * 5000.0;
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
    fn test_migration_projection() {
        let mut projection = ClimateMigrationProjection::new(
            MigrationDriver::SeaLevelRise,
            "Bangladesh Delta",
        );
        projection.configure(2000000, 30, 0.7);
        assert!(projection.estimated_population > 0);
    }

    #[test]
    fn test_migration_plan() {
        let mut plan = MigrationPlan::new();
        plan.add_projection(ClimateMigrationProjection::new(
            MigrationDriver::Desertification,
            "Sahel Region",
        ));
        assert!(plan.total_displaced >= 0);
    }
}
