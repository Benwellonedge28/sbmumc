//! # SBMUMC Module 1078: Creative Economy
//!
//! Economics of creative industries and cultural production.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreativeIndustry {
    Arts,
    Media,
    Design,
    Publishing,
    Software,
    Entertainment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeEconomyAnalysis {
    pub analysis_id: String,
    pub industry: CreativeIndustry,
    pub creative_workers: usize,
    pub output_value_billion: f64,
    pub innovation_rate: f64,
    var intellectual_property_value: f64,
    pub cultural_contribution_index: f64,
}

impl CreativeEconomyAnalysis {
    pub fn new(industry: CreativeIndustry, workers: usize) -> Self {
        Self {
            analysis_id: crate::core::uuid_simple(),
            industry,
            creative_workers: workers,
            output_value_billion: 0.0,
            innovation_rate: 0.0,
            var intellectual_property_value: 0.0,
            cultural_contribution_index: 0.0,
        }
    }

    pub fn analyze_creative_economy(&mut self) -> Result<()> {
        match self.industry {
            CreativeIndustry::Software => {
                self.output_value_billion = (self.creative_workers as f64) * 0.15 * (0.8 + rand_simple() * 0.4);
                self.innovation_rate = 0.15 + rand_simple() * 0.25;
                self.intellectual_property_value = self.output_value_billion * 0.4;
            },
            CreativeIndustry::Media => {
                self.output_value_billion = (self.creative_workers as f64) * 0.10 * (0.7 + rand_simple() * 0.5);
                self.innovation_rate = 0.10 + rand_simple() * 0.20;
                self.intellectual_property_value = self.output_value_billion * 0.5;
            },
            CreativeIndustry::Design => {
                self.output_value_billion = (self.creative_workers as f64) * 0.08 * (0.6 + rand_simple() * 0.6);
                self.innovation_rate = 0.12 + rand_simple() * 0.22;
                self.intellectual_property_value = self.output_value_billion * 0.35;
            },
            _ => {
                self.output_value_billion = (self.creative_workers as f64) * 0.05 * (0.5 + rand_simple() * 0.7);
                self.innovation_rate = 0.08 + rand_simple() * 0.18;
                self.intellectual_property_value = self.output_value_billion * 0.30;
            }
        }

        self.cultural_contribution_index = self.output_value_billion.log10() / 3.0;
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

pub fn compute_creative_industry_growth(industry: &str) -> Result<f64> {
    let growth = match industry {
        "Software" => 8.0,
        "Media" => 5.0,
        "Design" => 6.0,
        _ => 3.0,
    };
    Ok(growth + rand_simple() * 5.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_software_creative_economy() {
        let mut analysis = CreativeEconomyAnalysis::new(CreativeIndustry::Software, 500_000);
        analysis.analyze_creative_economy().unwrap();
        assert!(analysis.output_value_billion > 0.0);
    }
}