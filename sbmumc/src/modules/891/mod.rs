//! # SBMUMC Module 891: Freight Forwarding
//! 
//! International freight logistics and forwarding operations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// Incoterms types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Incoterm {
    EXW,
    FOB,
    CIF,
    DDP,
    DAP,
    FCA,
    CPT,
    CIP,
}

/// Shipping container types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerType {
    TwentyFoot,
    FortyFoot,
    FortyHighCube,
    FortyFiveHighCube,
    Reefer,
}

/// Freight forwarding quote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardingQuote {
    pub quote_id: String,
    pub origin_port: String,
    pub destination_port: String,
    pub container_type: ContainerType,
    pub rate_per_container: f64,
    pub transit_days: u32,
    pub carrier: String,
    pub valid_until: u64,
}

/// Customs documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsDocumentation {
    pub bl_number: String,
    pub commercial_invoice: bool,
    pub packing_list: bool,
    pub certificate_of_origin: bool,
    pub customs_declaration: bool,
    pub dangerous_goods_declaration: bool,
}

/// Multi-modal transport route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiModalRoute {
    pub leg_count: u32,
    pub legs: Vec<TransportLeg>,
    pub total_transit_days: u32,
    pub total_cost: f64,
    pub carbon_footprint_kg: f64,
}

/// Transport leg
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportLeg {
    pub leg_number: u32,
    pub transport_mode: String,
    pub origin: String,
    pub destination: String,
    pub departure_date: u64,
    pub arrival_date: u64,
    pub cost: f64,
}

impl FreightForwarding {
    /// Create new freight forwarding system
    pub fn new() -> Self {
        Self
    }

    /// Calculate freight rate
    pub fn calculate_rate(&self, origin: &str, dest: &str, container: ContainerType, weight_kg: f64) -> Result<f64> {
        let base_rate = match container {
            ContainerType::TwentyFoot => 1500.0,
            ContainerType::FortyFoot => 2800.0,
            ContainerType::FortyHighCube => 3000.0,
            ContainerType::FortyFiveHighCube => 3500.0,
            ContainerType::Reefer => 4000.0,
        };
        let weight_surcharge = if weight_kg > 20000.0 { 1.15 } else { 1.0 };
        let destination_factor = if dest.contains("US") { 1.2 } else { 1.0 };
        Ok(base_rate * weight_surcharge * destination_factor)
    }

    /// Plan multi-modal route
    pub fn plan_multimodal_route(&self, origin: &str, dest: &str, cargo_type: &str) -> Result<MultiModalRoute> {
        let legs = if cargo_type == "container" {
            vec![
                TransportLeg {
                    leg_number: 1,
                    transport_mode: "truck".to_string(),
                    origin: origin.to_string(),
                    destination: "Port_Origin".to_string(),
                    departure_date: 0,
                    arrival_date: 86400,
                    cost: 500.0,
                },
                TransportLeg {
                    leg_number: 2,
                    transport_mode: "sea".to_string(),
                    origin: "Port_Origin".to_string(),
                    destination: "Port_Dest".to_string(),
                    departure_date: 86400,
                    arrival_date: 86400 * 21,
                    cost: 2500.0,
                },
                TransportLeg {
                    leg_number: 3,
                    transport_mode: "truck".to_string(),
                    origin: "Port_Dest".to_string(),
                    destination: dest.to_string(),
                    departure_date: 86400 * 21,
                    arrival_date: 86400 * 22,
                    cost: 600.0,
                },
            ]
        } else {
            vec![TransportLeg {
                leg_number: 1,
                transport_mode: "air".to_string(),
                origin: origin.to_string(),
                destination: dest.to_string(),
                departure_date: 0,
                arrival_date: 86400,
                cost: 5000.0,
            }]
        };
        
        let total_cost: f64 = legs.iter().map(|l| l.cost).sum();
        Ok(MultiModalRoute {
            leg_count: legs.len() as u32,
            legs,
            total_transit_days: 22,
            total_cost,
            carbon_footprint_kg: total_cost * 0.5,
        })
    }

    /// Verify customs documentation
    pub fn verify_documentation(&self, docs: &CustomsDocumentation) -> Result<bool> {
        let required = docs.commercial_invoice && docs.packing_list && docs.customs_declaration;
        if !required {
            return Err(SbmumcError::InvalidInput("Missing required documents".into()));
        }
        Ok(true)
    }
}

impl Default for FreightForwarding {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FreightForwarding;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_calculation() {
        let system = FreightForwarding::new();
        let rate = system.calculate_rate("CN", "US", ContainerType::FortyFoot, 18000.0);
        assert!(rate.is_ok());
    }

    #[test]
    fn test_multimodal_planning() {
        let system = FreightForwarding::new();
        let route = system.plan_multimodal_route("Shanghai", "Los_Angeles", "container");
        assert!(route.is_ok());
    }
}
