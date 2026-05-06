//! Space Manufacturing Module (663)
//!
//! In-space manufacturing, zero-g production, and advanced fabrication systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManufacturingType {
    AdditiveManufacturing,
    SubtractiveManufacturing,
    PowderBed,
    ContinuousFlow,
    ContainerlessProcessing,
    BioManufacturing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceManufacturing {
    pub facility_name: String,
    pub manufacturing_type: ManufacturingType,
    pub location: String,
    pub build_volume: f64,           // m^3
    pub power_consumption: f64,       // kW
    pub precision: f64,              // microns
    pub throughput: f64,            // kg/day
    pub materials_compatible: Vec<String>,
    pub quality_assurance_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturingProcess {
    pub process_name: String,
    pub temperature_range: (f64, f64), // C
    pub pressure_range: (f64, f64),     // Pa
    pub atmosphere_required: String,
    pub cycle_time: f64,                // hours
    pub material_usage_efficiency: f64, // percent
}

impl SpaceManufacturing {
    pub fn new(facility_name: String, manufacturing_type: ManufacturingType) -> Self {
        Self {
            facility_name,
            manufacturing_type,
            location: "Orbital".into(),
            build_volume: 1.0,
            power_consumption: 0.0,
            precision: 50.0,
            throughput: 0.0,
            materials_compatible: vec!["Titanium".into(), "Aluminum".into()],
            quality_assurance_level: 3,
        }
    }

    pub fn production_cost(&self, material_cost: f64) -> f64 {
        material_cost * (100.0 / self.material_usage_efficiency)
    }
}

pub struct ManufacturingAnalysis;

impl ManufacturingAnalysis {
    pub fn layer_thickness_build_time(layers: u32, layer_time: f64) -> f64 {
        layers as f64 * layer_time
    }

    pub fn powder_utilization(initial_powder: f64, final_powder: f64) -> f64 {
        ((initial_powder - final_powder) / initial_powder) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_manufacturing() {
        let facility = SpaceManufacturing::new("Orbital Fab Lab".into(), ManufacturingType::AdditiveManufacturing);
        assert_eq!(facility.facility_name, "Orbital Fab Lab");
    }
}
