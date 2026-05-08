//! # SBMUMC Module 992: Climate Hope & Resilience
//! 
//! Frameworks for building climate hope and community resilience.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResilienceComponent {
    Social,
    Economic,
    Infrastructure,
    Environmental,
    Institutional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceIndicator {
    pub indicator_id: String,
    pub component: ResilienceComponent,
    pub name: String,
    pub current_value: f64,
    pub target_value: f64,
    pub trend_direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HopeResilienceFramework {
    pub framework_id: String,
    pub indicators: Vec<ResilienceIndicator>,
    pub overall_resilience_score: f64,
    pub hope_index: f64,
    pub community_strengths: Vec<String>,
}

impl ResilienceIndicator {
    pub fn new(component: ResilienceComponent, name: &str) -> Self {
        Self {
            indicator_id: format!("ri_{}", uuid_simple()),
            component,
            name: name.to_string(),
            current_value: 0.0,
            target_value: 0.0,
            trend_direction: "Stable".to_string(),
        }
    }

    pub fn configure(&mut self, current: f64, target: f64, trend: &str) {
        self.current_value = current;
        self.target_value = target;
        self.trend_direction = trend.to_string();
    }

    pub fn progress_score(&self) -> f64 {
        if self.target_value == 0.0 { 0.0 }
        else { (self.current_value / self.target_value).min(1.0) }
    }
}

impl HopeResilienceFramework {
    pub fn new() -> Self {
        Self {
            framework_id: format!("hrf_{}", uuid_simple()),
            indicators: Vec::new(),
            overall_resilience_score: 0.0,
            hope_index: 0.0,
            community_strengths: Vec::new(),
        }
    }

    pub fn add_indicator(&mut self, indicator: ResilienceIndicator) {
        self.indicators.push(indicator);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.overall_resilience_score = self.indicators.iter()
            .map(|i| i.progress_score())
            .sum::<f64>() / self.indicators.len().max(1) as f64;
        self.hope_index = self.overall_resilience_score * 0.8 + 0.2;
    }

    pub fn add_strength(&mut self, strength: &str) {
        self.community_strengths.push(strength.to_string());
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
    fn test_resilience_indicator() {
        let mut indicator = ResilienceIndicator::new(
            ResilienceComponent::Social,
            "Community cohesion",
        );
        indicator.configure(0.7, 1.0, "Improving");
        assert!(indicator.progress_score() > 0.0);
    }

    #[test]
    fn test_hope_resilience_framework() {
        let mut framework = HopeResilienceFramework::new();
        framework.add_indicator(ResilienceIndicator::new(
            ResilienceComponent::Infrastructure,
            "Renewable energy adoption",
        ));
        framework.add_strength("Strong community networks");
        assert!(framework.hope_index >= 0.0);
    }
}
