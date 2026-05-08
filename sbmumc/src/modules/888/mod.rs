//! # SBMUMC Module 888: Transport Demand Management
//! 
//! Transportation demand forecasting and management systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// Demand modeling approaches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DemandModelType {
    FourStep,
    ActivityBased,
    TourBased,
}

/// Trip generation rates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripGenerationRates {
    pub home_based_work: f64,
    pub home_based_nonwork: f64,
    pub non_home_based: f64,
    pub commercial: f64,
}

/// Demand forecasting result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandForecast {
    pub horizon_year: u32,
    pub total_trips: f64,
    pub vmt_forecast: f64,
    pub peak_hour_demand: f64,
    pub mode_share: Vec<(String, f64)>,
}

/// TDM strategy effectiveness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TDMEffectiveness {
    pub strategy_name: String,
    pub demand_reduction_pct: f64,
    pub implementation_cost: f64,
    pub annual_benefit: f64,
    pub payback_years: f64,
}

/// Land use transport interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandUseTransport {
    pub population_density: f64,
    pub employment_density: f64,
    pub transit_accessibility: f64,
    pub walkability_index: f64,
    pub parking_supply: f64,
}

impl TransportDemandManagement {
    /// Create new TDM system
    pub fn new() -> Self {
        Self
    }

    /// Forecast travel demand
    pub fn forecast_demand(&self, land_use: &LandUseTransport, growth_rate: f64) -> Result<DemandForecast> {
        let base_trips = land_use.population_density * land_use.employment_density * 0.001;
        let growth_factor = (1.0 + growth_rate).powi(10);
        Ok(DemandForecast {
            horizon_year: 2035,
            total_trips: base_trips * growth_factor,
            vmt_forecast: base_trips * growth_factor * 15.0,
            peak_hour_demand: base_trips * growth_factor * 0.1,
            mode_share: vec![
                ("car".to_string(), 0.6),
                ("transit".to_string(), 0.25),
                ("active".to_string(), 0.15),
            ],
        })
    }

    /// Calculate mode choice probability
    pub fn calculate_mode_choice(&self, attributes: &ModeAttributes) -> Result<Vec<f64>> {
        let car_util = -0.01 * attributes.car_travel_time - 0.5 * attributes.car_cost;
        let transit_util = -0.02 * attributes.transit_travel_time - 0.3 * attributes.transit_cost + 0.5;
        let bike_util = -0.03 * attributes.bike_travel_time + 0.3;
        let walk_util = -0.04 * attributes.walk_travel_time + 0.2;
        
        let exp_car = car_util.exp();
        let exp_transit = transit_util.exp();
        let exp_bike = bike_util.exp();
        let exp_walk = walk_util.exp();
        let sum = exp_car + exp_transit + exp_bike + exp_walk;
        
        Ok(vec![
            exp_car / sum,
            exp_transit / sum,
            exp_bike / sum,
            exp_walk / sum,
        ])
    }

    /// Evaluate TDM strategy
    pub fn evaluate_tdm_strategy(&self, strategy: &str, baseline_demand: f64) -> Result<TDMEffectiveness> {
        let (reduction, cost, benefit) = match strategy {
            "congestion_pricing" => (0.15, 5000000.0, 800000.0),
            "parking_management" => (0.10, 1000000.0, 300000.0),
            "transit_improvement" => (0.12, 20000000.0, 600000.0),
            _ => (0.05, 500000.0, 100000.0),
        };
        Ok(TDMEffectiveness {
            strategy_name: strategy.to_string(),
            demand_reduction_pct: reduction,
            implementation_cost: cost,
            annual_benefit: benefit,
            payback_years: cost / benefit,
        })
    }
}

impl Default for TransportDemandManagement {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TransportDemandManagement;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeAttributes {
    pub car_travel_time: f64,
    pub car_cost: f64,
    pub transit_travel_time: f64,
    pub transit_cost: f64,
    pub bike_travel_time: f64,
    pub walk_travel_time: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demand_forecast() {
        let system = TransportDemandManagement::new();
        let land_use = LandUseTransport {
            population_density: 5000.0,
            employment_density: 3000.0,
            transit_accessibility: 0.7,
            walkability_index: 0.6,
            parking_supply: 0.5,
        };
        let forecast = system.forecast_demand(&land_use, 0.02);
        assert!(forecast.is_ok());
    }

    #[test]
    fn test_mode_choice() {
        let system = TransportDemandManagement::new();
        let attrs = ModeAttributes {
            car_travel_time: 30.0,
            car_cost: 10.0,
            transit_travel_time: 45.0,
            transit_cost: 3.0,
            bike_travel_time: 40.0,
            walk_travel_time: 60.0,
        };
        let probs = system.calculate_mode_choice(&attrs);
        assert!(probs.is_ok());
    }
}
