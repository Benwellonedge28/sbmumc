//! Spacetime Geometry Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacetimeGeometry {
    pub sg_id: String,
    pub metric_structure: MetricStructure,
    pub coordinate_systems: Vec<CoordinateSystem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStructure {
    pub metric_type: MetricType,
    pub signature: [i32; 4],
    pub curvature_components: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Minkowski,
    Schwarzschild,
    Kerr,
    FLRW,
    ReissnerNordstrom,
    KerrNewman,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinateSystem {
    pub system_name: String,
    pub coordinates: Vec<Coordinate>,
    pub transformation_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinate {
    pub name: String,
    pub range_min: f64,
    pub range_max: f64,
}

impl SpacetimeGeometry {
    pub fn new() -> Self {
        Self {
            sg_id: String::from("spacetime_geometry_v1"),
            metric_structure: MetricStructure {
                metric_type: MetricType::Minkowski,
                signature: [1, -1, -1, -1],
                curvature_components: vec![],
            },
            coordinate_systems: vec![],
        }
    }
}

impl Default for SpacetimeGeometry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacetime_geometry_creation() {
        let geometry = SpacetimeGeometry::new();
        assert!(matches!(geometry.metric_structure.metric_type, MetricType::Minkowski));
    }
}
