//! # SBMUMC Module 1061: Economic Indicators
//!
//! Key economic metrics and measurement systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndicatorCategory {
    Leading,
    Coincident,
    Lagging,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicIndicator {
    pub indicator_id: String,
    pub indicator_name: String,
    pub category: IndicatorCategory,
    pub current_value: f64,
    pub previous_value: f64,
    pub change_percent: f64,
    pub trend_direction: String,
}

impl EconomicIndicator {
    pub fn new(name: String, category: IndicatorCategory) -> Self {
        Self {
            indicator_id: crate::core::uuid_simple(),
            indicator_name: name,
            category,
            current_value: 0.0,
            previous_value: 0.0,
            change_percent: 0.0,
            trend_direction: String::new(),
        }
    }

    pub fn update_indicator(&mut self, new_value: f64) -> Result<()> {
        self.previous_value = self.current_value;
        self.current_value = new_value;
        self.change_percent = if self.previous_value != 0.0 {
            ((self.current_value - self.previous_value) / self.previous_value) * 100.0
        } else {
            0.0
        };

        self.trend_direction = if self.change_percent > 0.5 {
            "Upward".to_string()
        } else if self.change_percent < -0.5 {
            "Downward".to_string()
        } else {
            "Stable".to_string()
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeIndicator {
    pub composite_id: String,
    pub component_indicators: Vec<String>,
    pub composite_value: f64,
    pub diffusion_index: f64,
    pub economic_health_score: f64,
}

impl CompositeIndicator {
    pub fn new(components: Vec<String>) -> Self {
        Self {
            composite_id: crate::core::uuid_simple(),
            component_indicators: components,
            composite_value: 0.0,
            diffusion_index: 0.0,
            economic_health_score: 0.0,
        }
    }

    pub fn calculate_composite(&mut self) -> Result<()> {
        let base_value = 50.0 + rand_simple() * 50.0;
        self.composite_value = base_value;

        self.diffusion_index = self.component_indicators.iter()
            .map(|_| if rand_simple() > 0.5 { 1.0 } else { 0.0 })
            .sum::<f64>()
            / self.component_indicators.len() as f64;

        self.economic_health_score = (self.composite_value / 100.0) * self.diffusion_index;
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

pub fn compute_indicator_aggregate(indicators: &[f64]) -> Result<f64> {
    let sum: f64 = indicators.iter().sum();
    Ok(sum / indicators.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdp_growth() {
        let mut indicator = EconomicIndicator::new(
            "Real_GDP_Growth".to_string(),
            IndicatorCategory::Coincident,
        );
        indicator.update_indicator(2.5).unwrap();
        assert!(indicator.change_percent >= 0.0);
    }

    #[test]
    fn test_pmi_composite() {
        let mut composite = CompositeIndicator::new(vec![
            "PMI_Manufacturing".to_string(),
            "PMI_Services".to_string(),
            "PMI_Construction".to_string(),
        ]);
        composite.calculate_composite().unwrap();
        assert!(composite.diffusion_index >= 0.0 && composite.diffusion_index <= 1.0);
    }
}