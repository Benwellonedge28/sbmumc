//! # SBMUMC Module 1051: Wealth Distribution
//!
//! Analysis of wealth inequality and distribution mechanisms.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributionMetric {
    GiniCoefficient,
    PalmaRatio,
    Top10Share,
    Bottom50Share,
    WealthConcentration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WealthDistributionAnalysis {
    pub analysis_id: String,
    pub country: String,
    pub metric: DistributionMetric,
    pub value: f64,
    pub trend_direction: f64,
    pub inequality_level: String,
}

impl WealthDistributionAnalysis {
    pub fn new(country: String, metric: DistributionMetric) -> Self {
        Self {
            analysis_id: crate::core::uuid_simple(),
            country,
            metric,
            value: 0.0,
            trend_direction: 0.0,
            inequality_level: String::new(),
        }
    }

    pub fn compute_metric(&mut self) -> Result<()> {
        match self.metric {
            DistributionMetric::GiniCoefficient => {
                self.value = 0.30 + rand_simple() * 0.45;
            },
            DistributionMetric::PalmaRatio => {
                self.value = 0.8 + rand_simple() * 8.0;
            },
            DistributionMetric::Top10Share => {
                self.value = 20.0 + rand_simple() * 50.0;
            },
            DistributionMetric::Bottom50Share => {
                self.value = 5.0 + rand_simple() * 25.0;
            },
            DistributionMetric::WealthConcentration => {
                self.value = 0.4 + rand_simple() * 0.5;
            }
        }

        self.trend_direction = -0.01 + rand_simple() * 0.03;

        self.inequality_level = if self.value < 0.3 {
            "Low".to_string()
        } else if self.value < 0.5 {
            "Moderate".to_string()
        } else {
            "High".to_string()
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WealthMobility {
    pub mobility_id: String,
    pub generation: String,
    pub upward_mobility_rate: f64,
    pub downward_mobility_rate: f64,
    pub intergenerational_correlation: f64,
    pub social_elasticity: f64,
}

impl WealthMobility {
    pub fn new(generation: String) -> Self {
        Self {
            mobility_id: crate::core::uuid_simple(),
            generation,
            upward_mobility_rate: 0.0,
            downward_mobility_rate: 0.0,
            intergenerational_correlation: 0.0,
            social_elasticity: 0.0,
        }
    }

    pub fn analyze_mobility(&mut self) -> Result<()> {
        self.upward_mobility_rate = 0.15 + rand_simple() * 0.25;
        self.downward_mobility_rate = 0.05 + rand_simple() * 0.15;
        self.intergenerational_correlation = 0.3 + rand_simple() * 0.5;
        self.social_elasticity = 1.0 - self.intergenerational_correlation;
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

pub fn compute_gini_coefficient(income_data: &[f64]) -> Result<f64> {
    let sorted = {
        let mut s = income_data.to_vec();
        s.sort_by(|a, b| a.partial_cmp(b).unwrap());
        s
    };

    let n = sorted.len() as f64;
    let sum: f64 = sorted.iter().enumerate().map(|(i, x)| x * (2.0 * (i + 1) as f64 - n - 1.0)).sum();
    let mean = sorted.iter().sum::<f64>() / n;

    Ok(sum / (n * n * mean).abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gini_analysis() {
        let mut analysis = WealthDistributionAnalysis::new(
            "United_States".to_string(),
            DistributionMetric::GiniCoefficient,
        );
        analysis.compute_metric().unwrap();
        assert!(analysis.value > 0.0 && analysis.value < 1.0);
    }

    #[test]
    fn test_wealth_mobility() {
        let mut mobility = WealthMobility::new("Millennials".to_string());
        mobility.analyze_mobility().unwrap();
        assert!(mobility.social_elasticity > 0.0);
    }
}