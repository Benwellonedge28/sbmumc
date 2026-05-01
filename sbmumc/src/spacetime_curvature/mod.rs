//! Spacetime Curvature Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacetimeCurvature {
    pub sc_id: String,
    pub curvature_type: CurvatureType,
    pub einstein_field_equations: EinsteinEquations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurvatureType {
    RiemannCurvature,
    RicciCurvature,
    ScalarCurvature,
    WeylCurvature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EinsteinEquations {
    pub cosmological_constant: f64,
    pub energy_momentum_trace: f64,
}

impl SpacetimeCurvature {
    pub fn new() -> Self {
        Self {
            sc_id: String::from("spacetime_curvature_v1"),
            curvature_type: CurvatureType::RiemannCurvature,
            einstein_field_equations: EinsteinEquations {
                cosmological_constant: 0.0,
                energy_momentum_trace: 0.0,
            },
        }
    }
}

impl Default for SpacetimeCurvature {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacetime_curvature_creation() {
        let curvature = SpacetimeCurvature::new();
        assert!(matches!(curvature.curvature_type, CurvatureType::RiemannCurvature));
    }
}
