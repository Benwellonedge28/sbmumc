//! Light Year Measurement Module (689)
//!
//! Astronomical distance measurement, light-year calculations, and cosmic distance ladder.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistanceMethod {
    Radar,
    Parallax,
    StandardCandle,
    Redshift,
    TullyFisher,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightYearMeasurement {
    pub measurement_id: String,
    pub object_name: String,
    pub distance_ly: f64,
    pub measurement_method: DistanceMethod,
    pub uncertainty_percent: f64,
    pub object_type: String,
    pub redshift: f64,
    pub apparent_magnitude: f64,
    pub absolute_magnitude: f64,
}

impl LightYearMeasurement {
    pub fn new(measurement_id: String, object_name: String) -> Self {
        Self {
            measurement_id,
            object_name,
            distance_ly: 0.0,
            measurement_method: DistanceMethod::Parallax,
            uncertainty_percent: 0.0,
            object_type: "Star".into(),
            redshift: 0.0,
            apparent_magnitude: 0.0,
            absolute_magnitude: 0.0,
        }
    }

    pub fn distance_from_parallax(mas: f64) -> f64 {
        1.0 / (mas / 1000.0) * 3.26
    }

    pub fn distance_modulus(&self) -> f64 {
        self.apparent_magnitude - self.absolute_magnitude
    }
}

pub struct LightYearCalculations;

impl LightYearCalculations {
    pub fn ly_to_meters(light_years: f64) -> f64 {
        light_years * 9.461e15
    }

    pub fn meters_to_ly(meters: f64) -> f64 {
        meters / 9.461e15
    }

    pub fn light_travel_time(distance_m: f64) -> f64 {
        distance_m / 299792458.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_year() {
        let measurement = LightYearMeasurement::new("LY-001".into(), "Proxima Centauri".into());
        assert_eq!(measurement.object_name, "Proxima Centauri");
    }
}
