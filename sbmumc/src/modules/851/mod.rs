//! # SBMUMC Module 851: Bicycle Technology
//! 
//! Bicycle design, components, and cycling infrastructure.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// Bicycle frame types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FrameType {
    Diamond,
    StepThrough,
    Folding,
    Recumbent,
    Triathlon,
}

/// Wheel sizes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WheelSize {
    TwentyInch,
    TwentySixInch,
    TwentySevenInch,
    TwentyNineInch,
    SevenHundredC,
    EighteenInch,
}

/// Drivetrain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drivetrain {
    pub chainring_teeth: u32,
    pub cassette_range: (u32, u32),
    pub crank_length: f64,
    pub gear_count: u32,
}

/// Cycling surface type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SurfaceType {
    Road,
    Trail,
    Gravel,
    Mountain,
}

/// Bicycle fit geometry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BikeFit {
    pub stack_height: f64,
    pub reach: f64,
    pub saddle_height: f64,
    pub handlebar_width: f64,
}

impl BicycleTechnology {
    /// Create new bicycle technology system
    pub fn new() -> Self {
        Self
    }

    /// Calculate gear ratios
    pub fn calculate_gear_ratios(&self, drivetrain: &Drivetrain) -> Result<Vec<f64>> {
        let ratios: Vec<f64> = (drivetrain.cassette_range.0..=drivetrain.cassette_range.1)
            .map(|t| drivetrain.chainring_teeth as f64 / t as f64)
            .collect();
        Ok(ratios)
    }

    /// Estimate rolling resistance
    pub fn estimate_rolling_resistance(&self, surface: SurfaceType) -> Result<f64> {
        let coefficient = match surface {
            SurfaceType::Road => 0.005,
            SurfaceType::Gravel => 0.015,
            SurfaceType::Trail => 0.025,
            SurfaceType::Mountain => 0.035,
        };
        Ok(coefficient)
    }

    /// Optimize fit position
    pub fn optimize_fit(&self, rider_height: f64, riding_style: &str) -> Result<BikeFit> {
        Ok(BikeFit {
            stack_height: rider_height * 0.55,
            reach: rider_height * 0.38,
            saddle_height: rider_height * 0.88,
            handlebar_width: 42.0,
        })
    }
}

impl Default for BicycleTechnology {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BicycleTechnology;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_ratios() {
        let system = BicycleTechnology::new();
        let drivetrain = Drivetrain {
            chainring_teeth: 50,
            cassette_range: (11, 32),
            crank_length: 172.5,
            gear_count: 10,
        };
        let ratios = system.calculate_gear_ratios(&drivetrain);
        assert!(ratios.is_ok());
        assert_eq!(ratios.unwrap().len(), 22);
    }

    #[test]
    fn test_rolling_resistance() {
        let system = BicycleTechnology::new();
        let resistance = system.estimate_rolling_resistance(SurfaceType::Road);
        assert!(resistance.is_ok());
        assert_eq!(resistance.unwrap(), 0.005);
    }
}
