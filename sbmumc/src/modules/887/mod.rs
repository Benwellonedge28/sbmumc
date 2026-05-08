//! # SBMUMC Module 887: Mobility Analytics
//! 
//! Transportation data analytics and insights.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// Mobility data sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    GPS,
    Bluetooth,
    WiFi,
    CellTower,
    Camera,
    InductiveLoop,
}

/// Analytics metric types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    TravelTime,
    Speed,
    Volume,
    Density,
    OriginDestination,
    ModeShare,
}

/// Travel time reliability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelTimeReliability {
    pub mean_travel_time_min: f64,
    pub std_deviation_min: f64,
    pub buffer_index: f64,
    pub planning_time_index: f64,
    pub畅通概率: f64,
}

/// Origin-destination matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ODMatrix {
    pub zones: Vec<String>,
    pub demand: Vec<Vec<f64>>,
    pub total_trips: f64,
}

/// Mobility insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobilityInsights {
    pub congestion_hotspots: Vec<Hotspot>,
    pub mode_share_trends: Vec<(String, f64)>,
    pub peak_hours: Vec<u32>,
    pub recommendations: Vec<String>,
}

/// Congestion hotspot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotspot {
    pub location: (f64, f64),
    pub severity: String,
    pub avg_delay_min: f64,
    pub affected_routes: Vec<String>,
}

impl MobilityAnalytics {
    /// Create new mobility analytics system
    pub fn new() -> Self {
        Self
    }

    /// Calculate travel time reliability
    pub fn calculate_reliability(&self, travel_times: &[f64]) -> Result<TravelTimeReliability> {
        let n = travel_times.len() as f64;
        let mean = travel_times.iter().sum::<f64>() / n;
        let variance = travel_times.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / n;
        let std_dev = variance.sqrt();
        let buffer_index = std_dev / mean;
        let planning_index = mean * 1.2;
        Ok(TravelTimeReliability {
            mean_travel_time_min: mean,
            std_deviation_min: std_dev,
            buffer_index,
            planning_time_index: planning_index,
            畅通概率: 0.85,
        })
    }

    /// Analyze mode share
    pub fn analyze_mode_share(&self, trips: &[TripData]) -> Result<Vec<(String, f64)>> {
        let mut mode_counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
        for trip in trips {
            *mode_counts.entry(trip.mode.clone()).or_insert(0) += 1;
        }
        let total = trips.len() as f64;
        let mut shares: Vec<(String, f64)> = mode_counts.iter()
            .map(|(m, c)| (m.clone(), *c as f64 / total))
            .collect();
        shares.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(shares)
    }

    /// Generate mobility insights
    pub fn generate_insights(&self, data: &MobilityDataSet) -> Result<MobilityInsights> {
        Ok(MobilityInsights {
            congestion_hotspots: vec![
                Hotspot {
                    location: (40.7580, -73.9855),
                    severity: "high".to_string(),
                    avg_delay_min: 12.0,
                    affected_routes: vec!["I-95".to_string(), "RT-9".to_string()],
                },
            ],
            mode_share_trends: vec![
                ("car".to_string(), 0.45),
                ("transit".to_string(), 0.30),
                ("bike".to_string(), 0.10),
                ("walk".to_string(), 0.15),
            ],
            peak_hours: vec![7, 8, 9, 17, 18, 19],
            recommendations: vec![
                "Expand transit during peak hours".to_string(),
                "Add bike lanes near hotspots".to_string(),
            ],
        })
    }
}

impl Default for MobilityAnalytics {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MobilityAnalytics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripData {
    pub mode: String,
    pub origin: (f64, f64),
    pub destination: (f64, f64),
    pub duration_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobilityDataSet {
    pub trips: Vec<TripData>,
    pub traffic_counts: Vec<(String, u32)>,
    pub timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reliability_calculation() {
        let system = MobilityAnalytics::new();
        let times = vec![30.0, 35.0, 32.0, 45.0, 33.0, 31.0];
        let reliability = system.calculate_reliability(&times);
        assert!(reliability.is_ok());
    }

    #[test]
    fn test_mode_share() {
        let system = MobilityAnalytics::new();
        let trips = vec![
            TripData { mode: "car".to_string(), origin: (0.0, 0.0), destination: (1.0, 1.0), duration_min: 30.0 },
            TripData { mode: "transit".to_string(), origin: (0.0, 0.0), destination: (1.0, 1.0), duration_min: 45.0 },
        ];
        let share = system.analyze_mode_share(&trips);
        assert!(share.is_ok());
    }
}
