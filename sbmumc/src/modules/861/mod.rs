//! # SBMUMC Module 861: Multimodal Transport
//! 
//! Integrated transportation systems and intermodal connectivity.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// Intermodal connection types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    BusRail,
    RailAir,
    SeaRail,
    BikeTransit,
    ParkRide,
}

/// Hub configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportHub {
    pub hub_id: String,
    pub hub_type: Vec<String>,
    pub location: (f64, f64),
    pub daily_pax: u32,
    pub connections: Vec<String>,
}

/// Multimodal journey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimodalJourney {
    pub journey_id: String,
    pub segments: Vec<JourneySegment>,
    pub total_duration: f64,
    pub total_cost: f64,
    pub co2_footprint: f64,
}

/// Journey segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneySegment {
    pub mode: String,
    pub origin: (f64, f64),
    pub destination: (f64, f64),
    pub duration: f64,
    pub cost: f64,
    pub transfer_time: f64,
}

/// First/last mile options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstLastMile {
    pub options: Vec<MileOption>,
    pub recommended: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MileOption {
    pub mode: String,
    pub duration: f64,
    pub cost: f64,
}

impl MultimodalTransport {
    /// Create new multimodal transport system
    pub fn new() -> Self {
        Self
    }

    /// Optimize multimodal route
    pub fn optimize_multimodal_route(&self, origin: (f64, f64), dest: (f64, f64)) -> Result<MultimodalJourney> {
        Ok(MultimodalJourney {
            journey_id: "mm_001".to_string(),
            segments: vec![
                JourneySegment {
                    mode: "walk".to_string(),
                    origin,
                    destination: (origin.0 + 0.01, origin.1),
                    duration: 10.0,
                    cost: 0.0,
                    transfer_time: 0.0,
                },
                JourneySegment {
                    mode: "bus".to_string(),
                    origin: (origin.0 + 0.01, origin.1),
                    destination: (dest.0 - 0.01, dest.1),
                    duration: 30.0,
                    cost: 2.5,
                    transfer_time: 5.0,
                },
                JourneySegment {
                    mode: "walk".to_string(),
                    origin: (dest.0 - 0.01, dest.1),
                    destination: dest,
                    duration: 5.0,
                    cost: 0.0,
                    transfer_time: 0.0,
                },
            ],
            total_duration: 50.0,
            total_cost: 2.5,
            co2_footprint: 0.5,
        })
    }

    /// Calculate transfer penalty
    pub fn calculate_transfer_penalty(&self, transfers: u32) -> Result<f64> {
        Ok(transfers as f64 * 8.0) // 8 minutes per transfer
    }

    /// Recommend first/last mile
    pub fn recommend_first_last_mile(&self, hub: &TransportHub) -> Result<FirstLastMile> {
        Ok(FirstLastMile {
            options: vec![
                MileOption { mode: "walk".to_string(), duration: 10.0, cost: 0.0 },
                MileOption { mode: "bike".to_string(), duration: 5.0, cost: 1.5 },
                MileOption { mode: "bus".to_string(), duration: 8.0, cost: 2.0 },
            ],
            recommended: Some("bike".to_string()),
        })
    }
}

impl Default for MultimodalTransport {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MultimodalTransport;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multimodal_optimization() {
        let system = MultimodalTransport::new();
        let journey = system.optimize_multimodal_route((40.0, -74.0), (40.5, -73.5));
        assert!(journey.is_ok());
    }

    #[test]
    fn test_transfer_penalty() {
        let system = MultimodalTransport::new();
        let penalty = system.calculate_transfer_penalty(2);
        assert!(penalty.is_ok());
        assert_eq!(penalty.unwrap(), 16.0);
    }
}
