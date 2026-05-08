//! # SBMUMC Module 890: Passenger Experience
//! 
//! Transportation passenger services and experience optimization.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// Service quality dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceQualityDim {
    Reliability,
    Comfort,
    Information,
    Safety,
    Cleanliness,
    Staff,
}

/// Passenger satisfaction score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatisfactionScore {
    pub overall_score: f64,
    pub dimension_scores: Vec<(ServiceQualityDim, f64)>,
    pub net_promoter_score: i32,
    pub complaint_rate: f64,
}

/// Journey experience metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyExperience {
    pub wait_time_min: f64,
    pub journey_time_min: f64,
    pub transfer_time_min: f64,
    pub crowding_level: f64,
    pub information_availability: f64,
    pub accessibility_score: f64,
}

/// Passenger flow analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassengerFlow {
    pub station_id: String,
    pub boardings: u32,
    pub alightings: u32,
    pub peak_hour_volume: u32,
    pub platform_capacity: u32,
    pub congestion_index: f64,
}

impl PassengerExperience {
    /// Create new passenger experience system
    pub fn new() -> Self {
        Self
    }

    /// Calculate satisfaction score
    pub fn calculate_satisfaction(&self, dimensions: &[(ServiceQualityDim, f64)]) -> Result<SatisfactionScore> {
        let total: f64 = dimensions.iter().map(|(_, s)| s).sum();
        let overall = total / dimensions.len() as f64;
        let nps = if overall > 8.0 { 50 } else if overall > 6.0 { 20 } else { -30 };
        Ok(SatisfactionScore {
            overall_score: overall,
            dimension_scores: dimensions.to_vec(),
            net_promoter_score: nps,
            complaint_rate: (10.0 - overall) * 0.5,
        })
    }

    /// Assess journey experience
    pub fn assess_journey(&self, journey: &JourneyExperience) -> Result<f64> {
        let wait_score = (15.0 - journey.wait_time_min).max(0.0) / 15.0;
        let comfort_score = (1.0 - journey.crowding_level * 0.5).max(0.3);
        let info_score = journey.information_availability;
        let experience_score = (wait_score * 0.3 + comfort_score * 0.3 + info_score * 0.2 + 0.2) * 10.0;
        Ok(experience_score)
    }

    /// Analyze passenger flow
    pub fn analyze_flow(&self, flows: &[PassengerFlow]) -> Result<PassengerFlowAnalysis> {
        let total_boardings: u32 = flows.iter().map(|f| f.boardings).sum();
        let peak_total: u32 = flows.iter().map(|f| f.peak_hour_volume).max().unwrap_or(0);
        let avg_congestion = flows.iter().map(|f| f.congestion_index).sum::<f64>() / flows.len() as f64;
        Ok(PassengerFlowAnalysis {
            total_daily_boardings: total_boardings,
            peak_hour_total: peak_total,
            avg_congestion_index: avg_congestion,
            hotspots: vec!["Station_A".to_string()],
        })
    }

    /// Optimize service frequency
    pub fn optimize_frequency(&self, demand: f64, capacity_per_vehicle: u32) -> Result<u32> {
        let vehicles_per_hour = (demand / capacity_per_vehicle as f64).ceil() as u32;
        Ok(vehicles_per_hour.max(4))
    }
}

impl Default for PassengerExperience {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PassengerExperience;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassengerFlowAnalysis {
    pub total_daily_boardings: u32,
    pub peak_hour_total: u32,
    pub avg_congestion_index: f64,
    pub hotspots: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_satisfaction_calculation() {
        let system = PassengerExperience::new();
        let dimensions = vec![
            (ServiceQualityDim::Reliability, 8.5),
            (ServiceQualityDim::Comfort, 7.0),
            (ServiceQualityDim::Information, 6.5),
        ];
        let score = system.calculate_satisfaction(&dimensions);
        assert!(score.is_ok());
    }

    #[test]
    fn test_journey_assessment() {
        let system = PassengerExperience::new();
        let journey = JourneyExperience {
            wait_time_min: 8.0,
            journey_time_min: 35.0,
            transfer_time_min: 5.0,
            crowding_level: 0.6,
            information_availability: 0.8,
            accessibility_score: 0.9,
        };
        let exp = system.assess_journey(&journey);
        assert!(exp.is_ok());
    }
}
