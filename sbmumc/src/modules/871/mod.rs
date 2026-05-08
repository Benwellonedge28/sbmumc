//! # SBMUMC Module 871: Charging Infrastructure
//! 
//! Electric vehicle charging station networks.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// Charging connector types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectorType {
    Type1,
    Type2,
    CCS,
    CHAdeMO,
    Tesla,
    GB_T,
}

/// Charging station status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StationStatus {
    Available,
    Occupied,
    OutOfService,
    Reserved,
}

/// Charging station
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingStation {
    pub station_id: String,
    pub location: (f64, f64),
    pub connectors: Vec<ConnectorConfig>,
    pub power_kw: f64,
    pub status: StationStatus,
    pub pricing: PricingInfo,
}

/// Connector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorConfig {
    pub connector_type: ConnectorType,
    pub power_kw: f64,
    pub available: bool,
}

/// Pricing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingInfo {
    pub per_kwh: f64,
    pub per_minute: Option<f64>,
    pub session_fee: f64,
}

/// Charging session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingSession {
    pub session_id: String,
    pub station_id: String,
    pub start_time: u64,
    pub energy_delivered_kwh: f64,
    pub cost: f64,
    pub duration_min: f64,
}

impl ChargingInfrastructure {
    /// Create new charging infrastructure system
    pub fn new() -> Self {
        Self
    }

    /// Find nearby stations
    pub fn find_nearby_stations(&self, location: (f64, f64), radius_km: f64) -> Result<Vec<ChargingStation>> {
        Ok(vec![ChargingStation {
            station_id: "CS001".to_string(),
            location: (location.0 + 0.01, location.1 + 0.01),
            connectors: vec![ConnectorConfig {
                connector_type: ConnectorType::CCS,
                power_kw: 150.0,
                available: true,
            }],
            power_kw: 150.0,
            status: StationStatus::Available,
            pricing: PricingInfo {
                per_kwh: 0.35,
                per_minute: Some(0.10),
                session_fee: 0.0,
            },
        }])
    }

    /// Estimate charging time
    pub fn estimate_charging_time(&self, target_soc: f64, battery_kwh: f64, power_kw: f64) -> Result<f64> {
        let energy_needed = battery_kwh * target_soc;
        Ok(energy_needed / power_kw * 60.0) // minutes
    }

    /// Calculate charging cost
    pub fn calculate_charging_cost(&self, station: &ChargingStation, energy_kwh: f64) -> Result<f64> {
        let energy_cost = energy_kwh * station.pricing.per_kwh;
        Ok(energy_cost + station.pricing.session_fee)
    }

    /// Check station availability
    pub fn check_availability(&self, station: &ChargingStation) -> Result<bool> {
        Ok(station.status == StationStatus::Available && 
           station.connectors.iter().any(|c| c.available))
    }
}

impl Default for ChargingInfrastructure {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ChargingInfrastructure;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_nearby_stations() {
        let system = ChargingInfrastructure::new();
        let stations = system.find_nearby_stations((40.0, -74.0), 5.0);
        assert!(stations.is_ok());
    }

    #[test]
    fn test_charging_time() {
        let system = ChargingInfrastructure::new();
        let time = system.estimate_charging_time(0.8, 75.0, 150.0);
        assert!(time.is_ok());
    }
}
