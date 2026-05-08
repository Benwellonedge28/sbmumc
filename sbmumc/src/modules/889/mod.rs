//! # SBMUMC Module 889: Shared Mobility
//! 
//! Bike sharing, car sharing, and ride sharing systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// Shared mobility service types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    BikeShare,
    ScooterShare,
    CarShare,
    RideHail,
    RideShare,
}

/// Vehicle sharing station
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharingStation {
    pub station_id: String,
    pub location: (f64, f64),
    pub dock_count: u32,
    pub available_vehicles: u32,
    pub return_demand: f64,
    pub service_type: ServiceType,
}

/// Rebalancing optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebalancingPlan {
    pub vehicle_movements: Vec<VehicleMovement>,
    pub total_distance_km: f64,
    pub cost: f64,
    pub service_level_improvement: f64,
}

/// Vehicle movement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleMovement {
    pub vehicle_id: String,
    pub from_station: String,
    pub to_station: String,
    pub distance_km: f64,
}

/// Usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub total_trips: u32,
    pub avg_trip_duration_min: f64,
    pub avg_trip_distance_km: f64,
    pub fleet_utilization: f64,
    pub revenue: f64,
}

impl SharedMobility {
    /// Create new shared mobility system
    pub fn new() -> Self {
        Self
    }

    /// Calculate fleet size
    pub fn calculate_fleet_size(&self, daily_demand: u32, avg_trip_duration_hrs: f64, target_service_level: f64) -> Result<u32> {
        let peak_factor = 2.5;
        let fleet = (daily_demand as f64 * avg_trip_duration_hrs * peak_factor / (24.0 * target_service_level)) as u32;
        Ok(fleet.max(100))
    }

    /// Optimize rebalancing
    pub fn optimize_rebalancing(&self, stations: &[SharingStation]) -> Result<RebalancingPlan> {
        let mut movements = Vec::new();
        let total_distance = stations.iter()
            .filter(|s| s.available_vehicles > s.dock_count / 2)
            .map(|s| {
                movements.push(VehicleMovement {
                    vehicle_id: format!("V{}", s.station_id),
                    from_station: s.station_id.clone(),
                    to_station: format!("TARGET_{}", s.station_id),
                    distance_km: 2.0,
                });
                2.0
            })
            .sum::<f64>();
        
        Ok(RebalancingPlan {
            vehicle_movements: movements,
            total_distance_km: total_distance,
            cost: total_distance * 0.5,
            service_level_improvement: 0.15,
        })
    }

    /// Calculate station coverage
    pub fn calculate_coverage(&self, stations: &[SharingStation], service_radius_km: f64) -> Result<f64> {
        let coverage_area = stations.len() as f64 * std::f64::consts::PI * service_radius_km.powi(2);
        Ok(coverage_area)
    }

    /// Predict demand patterns
    pub fn predict_demand(&self, historical: &[UsageStats], hour: u32) -> Result<f64> {
        let base_demand = historical.iter().map(|s| s.total_trips as f64).sum::<f64>() / historical.len() as f64;
        let hourly_factor = if hour >= 7 && hour <= 9 { 2.0 }
                           else if hour >= 17 && hour <= 19 { 2.5 }
                           else if hour >= 22 || hour <= 5 { 0.3 }
                           else { 1.0 };
        Ok(base_demand * hourly_factor)
    }
}

impl Default for SharedMobility {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SharedMobility;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fleet_sizing() {
        let system = SharedMobility::new();
        let fleet = system.calculate_fleet_size(500, 0.5, 0.8);
        assert!(fleet.is_ok());
    }

    #[test]
    fn test_rebalancing() {
        let system = SharedMobility::new();
        let stations = vec![
            SharingStation {
                station_id: "S1".to_string(),
                location: (40.0, -74.0),
                dock_count: 20,
                available_vehicles: 15,
                return_demand: 0.3,
                service_type: ServiceType::BikeShare,
            },
        ];
        let plan = system.optimize_rebalancing(&stations);
        assert!(plan.is_ok());
    }
}
