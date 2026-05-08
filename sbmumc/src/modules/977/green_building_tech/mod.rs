//! # SBMUMC Module 977: Green Building Tech
//! 
//! Technologies and frameworks for sustainable building design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildingTech {
    PassiveHouse,
    NetZero,
    LEED,
    BREEAM,
    LivingBuilding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreenBuilding {
    pub building_id: String,
    pub name: String,
    pub technology: BuildingTech,
    pub certification_level: String,
    pub energy_savings_percent: f64,
    pub water_savings_percent: f64,
    pub carbon_footprint_tco2: f64,
    pub construction_cost_premium: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreenBuildingPortfolio {
    pub portfolio_id: String,
    pub buildings: Vec<GreenBuilding>,
    pub total_energy_savings: f64,
    pub total_carbon_savings: f64,
    pub average_certification_level: f64,
}

impl GreenBuilding {
    pub fn new(name: &str, technology: BuildingTech) -> Self {
        Self {
            building_id: format!("gb_{}", uuid_simple()),
            name: name.to_string(),
            technology,
            certification_level: "Pending".to_string(),
            energy_savings_percent: 0.0,
            water_savings_percent: 0.0,
            carbon_footprint_tco2: 0.0,
            construction_cost_premium: 0.0,
        }
    }

    pub fn configure(&mut self, cert_level: &str, energy: f64, water: f64, carbon: f64, premium: f64) {
        self.certification_level = cert_level.to_string();
        self.energy_savings_percent = energy;
        self.water_savings_percent = water;
        self.carbon_footprint_tco2 = carbon;
        self.construction_cost_premium = premium;
    }

    pub fn roi_period_years(&self) -> f64 {
        if self.construction_cost_premium == 0.0 { 0.0 }
        else { self.construction_cost_premium / (self.carbon_footprint_tco2 * 50.0 + 10000.0) }
    }
}

impl GreenBuildingPortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("gbport_{}", uuid_simple()),
            buildings: Vec::new(),
            total_energy_savings: 0.0,
            total_carbon_savings: 0.0,
            average_certification_level: 0.0,
        }
    }

    pub fn add_building(&mut self, building: GreenBuilding) {
        self.buildings.push(building);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_energy_savings = self.buildings.iter()
            .map(|b| b.energy_savings_percent)
            .sum();
        self.total_carbon_savings = self.buildings.iter()
            .map(|b| b.carbon_footprint_tco2)
            .sum();
        self.average_certification_level = (self.buildings.len() as f64 * 0.3).min(1.0);
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
    fn test_green_building() {
        let mut building = GreenBuilding::new(
            "Corporate Headquarters",
            BuildingTech::LEED,
        );
        building.configure("Platinum", 60.0, 40.0, 500.0, 2000000.0);
        assert!(building.energy_savings_percent > 0.0);
    }

    #[test]
    fn test_green_building_portfolio() {
        let mut portfolio = GreenBuildingPortfolio::new();
        portfolio.add_building(GreenBuilding::new("Office Complex", BuildingTech::NetZero));
        assert!(portfolio.buildings.len() == 1);
    }
}
