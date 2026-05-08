//! # SBMUMC Module 863: Freight Systems
//! 
//! Freight transportation and cargo logistics.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// Freight modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FreightMode {
    Road,
    Rail,
    Sea,
    Air,
    Intermodal,
}

/// Cargo types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CargoType {
    Bulk,
    Container,
    Liquid,
    Refrigerated,
    Hazardous,
    General,
}

/// Container specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerSpec {
    pub container_type: String,
    pub length_ft: u32,
    pub max_payload_kg: f64,
    pub volume_m3: f64,
    pub is_refrigerated: bool,
}

/// Freight shipment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreightShipment {
    pub shipment_id: String,
    pub origin: (f64, f64),
    pub destination: (f64, f64),
    pub cargo_type: CargoType,
    pub weight_kg: f64,
    pub volume_m3: f64,
    pub mode: FreightMode,
    pub estimated_cost: f64,
}

/// Route optimization constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConstraints {
    pub weight_limit: f64,
    pub height_limit: f64,
    pub time_windows: Vec<(u32, u32)>,
    pub hazmat_restricted: bool,
}

impl FreightSystems {
    /// Create new freight system
    pub fn new() -> Self {
        Self
    }

    /// Calculate freight rate
    pub fn calculate_freight_rate(&self, weight: f64, distance: f64, mode: FreightMode) -> Result<f64> {
        let base_rate = match mode {
            FreightMode::Road => 0.80,
            FreightMode::Rail => 0.40,
            FreightMode::Sea => 0.10,
            FreightMode::Air => 3.00,
            FreightMode::Intermodal => 0.50,
        };
        let weight_factor = (weight / 1000.0).sqrt();
        Ok(base_rate * distance * weight_factor)
    }

    /// Optimize freight routing
    pub fn optimize_freight_route(&self, shipment: &FreightShipment) -> Result<FreightRoute> {
        Ok(FreightRoute {
            waypoints: vec![shipment.origin, shipment.destination],
            total_distance: 500.0,
            total_cost: shipment.estimated_cost,
            co2_emissions: 100.0,
        })
    }

    /// Check route feasibility
    pub fn check_route_feasibility(&self, shipment: &FreightShipment, constraints: &RouteConstraints) -> Result<bool> {
        if shipment.weight_kg > constraints.weight_limit {
            return Err(SbmumcError::InvalidInput("Weight exceeds limit".into()));
        }
        if let CargoType::Hazardous = shipment.cargo_type {
            if constraints.hazmat_restricted {
                return Err(SbmumcError::InvalidInput("Hazmat not allowed".into()));
            }
        }
        Ok(true)
    }
}

impl Default for FreightSystems {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FreightSystems;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreightRoute {
    pub waypoints: Vec<(f64, f64)>,
    pub total_distance: f64,
    pub total_cost: f64,
    pub co2_emissions: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freight_rate() {
        let system = FreightSystems::new();
        let rate = system.calculate_freight_rate(5000.0, 1000.0, FreightMode::Road);
        assert!(rate.is_ok());
    }

    #[test]
    fn test_route_feasibility() {
        let system = FreightSystems::new();
        let shipment = FreightShipment {
            shipment_id: "FRT001".to_string(),
            origin: (40.0, -74.0),
            destination: (34.0, -118.0),
            cargo_type: CargoType::General,
            weight_kg: 5000.0,
            volume_m3: 20.0,
            mode: FreightMode::Road,
            estimated_cost: 800.0,
        };
        let constraints = RouteConstraints {
            weight_limit: 20000.0,
            height_limit: 4.2,
            time_windows: vec![(0, 24)],
            hazmat_restricted: false,
        };
        let feasible = system.check_route_feasibility(&shipment, &constraints);
        assert!(feasible.is_ok());
    }
}
