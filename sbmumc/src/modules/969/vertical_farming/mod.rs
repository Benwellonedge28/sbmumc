//! # SBMUMC Module 969: Vertical Farming
//! 
//! Vertical farming frameworks for sustainable food production.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FarmingTechnology {
    Hydroponics,
    Aeroponics,
    Aquaponics,
    LEDLighting,
    ClimateControl,
    AutomatedHarvesting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerticalFarm {
    pub farm_id: String,
    pub location: String,
    pub area_m2: f64,
    pub layers: u32,
    pub crops_produced_tons: f64,
    pub water_usage_liters: f64,
    pub energy_usage_kwh: f64,
    pub carbon_footprint_tco2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerticalFarmingPortfolio {
    pub portfolio_id: String,
    pub farms: Vec<VerticalFarm>,
    pub total_production: f64,
    pub total_water_saved: f64,
    pub carbon_intensity: f64,
}

impl VerticalFarm {
    pub fn new(location: &str, area: f64, layers: u32) -> Self {
        Self {
            farm_id: format!("vf_{}", uuid_simple()),
            location: location.to_string(),
            area_m2: area,
            layers,
            crops_produced_tons: 0.0,
            water_usage_liters: 0.0,
            energy_usage_kwh: 0.0,
            carbon_footprint_tco2: 0.0,
        }
    }

    pub fn configure(&mut self, production: f64, water: f64, energy: f64) {
        self.crops_produced_tons = production;
        self.water_usage_liters = water;
        self.energy_usage_kwh = energy;
        self.carbon_footprint_tco2 = energy * 0.0004;
    }

    pub fn water_efficiency(&self) -> f64 {
        if self.crops_produced_tons == 0.0 { 0.0 }
        else { self.water_usage_liters / self.crops_produced_tons }
    }
}

impl VerticalFarmingPortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("vfport_{}", uuid_simple()),
            farms: Vec::new(),
            total_production: 0.0,
            total_water_saved: 0.0,
            carbon_intensity: 0.0,
        }
    }

    pub fn add_farm(&mut self, farm: VerticalFarm) {
        self.farms.push(farm);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_production = self.farms.iter()
            .map(|f| f.crops_produced_tons)
            .sum();
        let total_water: f64 = self.farms.iter()
            .map(|f| f.water_usage_liters)
            .sum();
        let total_carbon: f64 = self.farms.iter()
            .map(|f| f.carbon_footprint_tco2)
            .sum();
        if self.total_production > 0.0 {
            self.carbon_intensity = total_carbon / self.total_production;
        }
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
    fn test_vertical_farm() {
        let mut farm = VerticalFarm::new("Singapore", 5000.0, 10);
        farm.configure(100.0, 500000.0, 500000.0);
        assert!(farm.crops_produced_tons > 0.0);
    }

    #[test]
    fn test_vertical_farming_portfolio() {
        let mut portfolio = VerticalFarmingPortfolio::new();
        portfolio.add_farm(VerticalFarm::new("Tokyo", 3000.0, 8));
        assert!(portfolio.total_production >= 0.0);
    }
}
