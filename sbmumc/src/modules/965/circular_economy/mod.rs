//! # SBMUMC Module 965: Circular Economy
//! 
//! Circular economy frameworks for sustainable production and consumption.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CircularStrategy {
    Reduce,
    Reuse,
    Recycle,
    Remanufacture,
    Refurbish,
    Recover,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircularInitiative {
    pub initiative_id: String,
    pub strategy: CircularStrategy,
    pub sector: String,
    pub materials_diverted_tons: f64,
    pub emissions_avoided_tco2: f64,
    pub economic_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircularEconomyPlan {
    pub plan_id: String,
    pub initiatives: Vec<CircularInitiative>,
    pub total_waste_diverted: f64,
    pub total_emissions_avoided: f64,
    pub circularity_index: f64,
}

impl CircularInitiative {
    pub fn new(strategy: CircularStrategy, sector: &str) -> Self {
        Self {
            initiative_id: format!("ci_{}", uuid_simple()),
            strategy,
            sector: sector.to_string(),
            materials_diverted_tons: 0.0,
            emissions_avoided_tco2: 0.0,
            economic_value: 0.0,
        }
    }

    pub fn configure(&mut self, materials: f64, emissions: f64, value: f64) {
        self.materials_diverted_tons = materials;
        self.emissions_avoided_tco2 = emissions;
        self.economic_value = value;
    }
}

impl CircularEconomyPlan {
    pub fn new() -> Self {
        Self {
            plan_id: format!("cep_{}", uuid_simple()),
            initiatives: Vec::new(),
            total_waste_diverted: 0.0,
            total_emissions_avoided: 0.0,
            circularity_index: 0.0,
        }
    }

    pub fn add_initiative(&mut self, initiative: CircularInitiative) {
        self.initiatives.push(initiative);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_waste_diverted = self.initiatives.iter()
            .map(|i| i.materials_diverted_tons)
            .sum();
        self.total_emissions_avoided = self.initiatives.iter()
            .map(|i| i.emissions_avoided_tco2)
            .sum();
        let total_material_value: f64 = self.initiatives.iter()
            .map(|i| i.economic_value)
            .sum();
        self.circularity_index = (self.total_waste_diverted / 1000.0 + total_material_value / 1000000.0).min(1.0);
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
    fn test_circular_initiative() {
        let mut initiative = CircularInitiative::new(
            CircularStrategy::Recycle,
            "Electronics",
        );
        initiative.configure(5000.0, 15000.0, 2000000.0);
        assert!(initiative.materials_diverted_tons > 0.0);
    }

    #[test]
    fn test_circular_economy_plan() {
        let mut plan = CircularEconomyPlan::new();
        plan.add_initiative(CircularInitiative::new(
            CircularStrategy::Reuse,
            "Packaging",
        ));
        assert!(plan.total_waste_diverted >= 0.0);
    }
}
