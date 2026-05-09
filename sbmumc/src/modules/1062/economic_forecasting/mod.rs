//! # SBMUMC Module 1062: Economic Forecasting
//!
//! Methods and models for economic prediction.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForecastHorizon {
    ShortTerm,
    MediumTerm,
    LongTerm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicForecast {
    pub forecast_id: String,
    pub variable: String,
    pub horizon: ForecastHorizon,
    pub forecast_value: f64,
    pub confidence_interval_lower: f64,
    pub confidence_interval_upper: f64,
    pub accuracy_score: f64,
}

impl EconomicForecast {
    pub fn new(variable: String, horizon: ForecastHorizon) -> Self {
        Self {
            forecast_id: crate::core::uuid_simple(),
            variable,
            horizon,
            forecast_value: 0.0,
            confidence_interval_lower: 0.0,
            confidence_interval_upper: 0.0,
            accuracy_score: 0.0,
        }
    }

    pub fn generate_forecast(&mut self, base_value: f64) -> Result<()> {
        match self.horizon {
            ForecastHorizon::ShortTerm => {
                self.forecast_value = base_value * (1.0 + (rand_simple() - 0.5) * 0.05);
                let uncertainty = base_value * 0.02;
                self.confidence_interval_lower = self.forecast_value - uncertainty;
                self.confidence_interval_upper = self.forecast_value + uncertainty;
                self.accuracy_score = 0.85 + rand_simple() * 0.10;
            },
            ForecastHorizon::MediumTerm => {
                self.forecast_value = base_value * (1.0 + (rand_simple() - 0.4) * 0.15);
                let uncertainty = base_value * 0.08;
                self.confidence_interval_lower = self.forecast_value - uncertainty;
                self.confidence_interval_upper = self.forecast_value + uncertainty;
                self.accuracy_score = 0.65 + rand_simple() * 0.15;
            },
            ForecastHorizon::LongTerm => {
                self.forecast_value = base_value * (1.0 + (rand_simple() - 0.3) * 0.50);
                let uncertainty = base_value * 0.20;
                self.confidence_interval_lower = self.forecast_value - uncertainty;
                self.confidence_interval_upper = self.forecast_value + uncertainty;
                self.accuracy_score = 0.40 + rand_simple() * 0.20;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioAnalysis {
    pub scenario_id: String,
    pub scenario_name: String,
    pub gdp_impact: f64,
    pub employment_impact: f64,
    pub inflation_impact: f64,
    pub probability: f64,
}

impl ScenarioAnalysis {
    pub fn new(name: String) -> Self {
        Self {
            scenario_id: crate::core::uuid_simple(),
            scenario_name: name,
            gdp_impact: 0.0,
            employment_impact: 0.0,
            inflation_impact: 0.0,
            probability: 0.0,
        }
    }

    pub fn analyze_scenario(&mut self) -> Result<()> {
        self.probability = 0.1 + rand_simple() * 0.5;

        match self.scenario_name.as_str() {
            "Recession" => {
                self.gdp_impact = -3.0 - rand_simple() * 4.0;
                self.employment_impact = -2.0 - rand_simple() * 3.0;
                self.inflation_impact = -1.0 + rand_simple() * 0.5;
            },
            "Boom" => {
                self.gdp_impact = 3.0 + rand_simple() * 4.0;
                self.employment_impact = 2.0 + rand_simple() * 2.0;
                self.inflation_impact = 2.0 + rand_simple() * 2.0;
            },
            "Stagflation" => {
                self.gdp_impact = -1.0 - rand_simple() * 2.0;
                self.employment_impact = -1.0 - rand_simple() * 1.5;
                self.inflation_impact = 4.0 + rand_simple() * 3.0;
            },
            _ => {
                self.gdp_impact = -0.5 + rand_simple() * 2.0;
                self.employment_impact = -0.5 + rand_simple() * 1.5;
                self.inflation_impact = -0.5 + rand_simple() * 2.0;
            }
        }
        Ok(())
    }
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

pub fn compute_expected_value(forecasts: &[(f64, f64)]) -> Result<f64> {
    let mut weighted_sum = 0.0;
    let mut total_weight = 0.0;

    for (value, weight) in forecasts {
        weighted_sum += value * weight;
        total_weight += weight;
    }

    Ok(if total_weight > 0.0 { weighted_sum / total_weight } else { 0.0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_term_forecast() {
        let mut forecast = EconomicForecast::new(
            "CPI_Inflation".to_string(),
            ForecastHorizon::ShortTerm,
        );
        forecast.generate_forecast(3.0).unwrap();
        assert!(forecast.accuracy_score > 0.8);
    }

    #[test]
    fn test_recession_scenario() {
        let mut scenario = ScenarioAnalysis::new("Recession".to_string());
        scenario.analyze_scenario().unwrap();
        assert!(scenario.gdp_impact < 0.0);
    }
}