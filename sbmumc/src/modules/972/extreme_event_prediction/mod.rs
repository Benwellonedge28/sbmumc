//! # SBMUMC Module 972: Extreme Event Prediction
//! 
//! Frameworks for predicting climate-related extreme events.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtremeEventType {
    Heatwave,
    Drought,
    Flood,
    Hurricane,
    Wildfire,
    ColdSnap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPrediction {
    pub prediction_id: String,
    pub event_type: ExtremeEventType,
    pub region: String,
    pub probability: f64,
    pub severity: f64,
    pub lead_time_days: u32,
    pub accuracy_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionSystem {
    pub system_id: String,
    pub predictions: Vec<EventPrediction>,
    pub overall_accuracy: f64,
    pub false_positive_rate: f64,
    pub coverage_regions: Vec<String>,
}

impl EventPrediction {
    pub fn new(event_type: ExtremeEventType, region: &str) -> Self {
        Self {
            prediction_id: format!("ep_{}", uuid_simple()),
            event_type,
            region: region.to_string(),
            probability: 0.0,
            severity: 0.0,
            lead_time_days: 0,
            accuracy_score: 0.0,
        }
    }

    pub fn configure(&mut self, probability: f64, severity: f64, lead: u32) {
        self.probability = probability.clamp(0.0, 1.0);
        self.severity = severity.clamp(0.0, 1.0);
        self.lead_time_days = lead;
    }

    pub fn risk_score(&self) -> f64 {
        self.probability * self.severity
    }
}

impl PredictionSystem {
    pub fn new() -> Self {
        Self {
            system_id: format!("ps_{}", uuid_simple()),
            predictions: Vec::new(),
            overall_accuracy: 0.0,
            false_positive_rate: 0.0,
            coverage_regions: Vec::new(),
        }
    }

    pub fn add_prediction(&mut self, prediction: EventPrediction) {
        self.predictions.push(prediction);
        if !self.coverage_regions.contains(&prediction.region) {
            self.coverage_regions.push(prediction.region.clone());
        }
    }

    pub fn compute_accuracy(&mut self) {
        if self.predictions.is_empty() {
            self.overall_accuracy = 0.0;
            return;
        }
        self.overall_accuracy = self.predictions.iter()
            .map(|p| p.accuracy_score)
            .sum::<f64>() / self.predictions.len() as f64;
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
    fn test_event_prediction() {
        let mut prediction = EventPrediction::new(
            ExtremeEventType::Heatwave,
            "Southern Europe",
        );
        prediction.configure(0.8, 0.9, 7);
        assert!(prediction.risk_score() > 0.7);
    }

    #[test]
    fn test_prediction_system() {
        let mut system = PredictionSystem::new();
        system.add_prediction(EventPrediction::new(
            ExtremeEventType::Hurricane,
            "Caribbean",
        ));
        system.compute_accuracy();
        assert!(system.predictions.len() == 1);
    }
}
