//! Parsec Calculations Module (690)
//!
//! Parsec-based astronomical calculations, coordinate transformations, and distance ladder.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsecCalculation {
    pub calculation_id: String,
    pub distance_pc: f64,
    pub distance_ly: f64,
    pub distance_au: f64,
    pub distance_m: f64,
    pub parallax_arcsec: f64,
    pub hubble_constant: f64,
}

impl ParsecCalculation {
    pub fn new(distance_pc: f64) -> Self {
        let ly = distance_pc * 3.26156;
        let au = distance_pc * 206264.806;
        let m = au * 1.496e11;
        Self {
            calculation_id: "PC-001".into(),
            distance_pc,
            distance_ly: ly,
            distance_au: au,
            distance_m: m,
            parallax_arcsec: 1.0 / distance_pc,
            hubble_constant: 70.0,
        }
    }

    pub fn from_parallax(parallax_arcsec: f64) -> Self {
        if parallax_arcsec <= 0.0 {
            return Self::new(f64::MAX);
        }
        Self::new(1.0 / parallax_arcsec)
    }

    pub fn recession_velocity(&self) -> f64 {
        self.hubble_constant * self.distance_m / 1000.0
    }
}

pub struct ParsecUtils;

impl ParsecUtils {
    pub fn pc_to_ly(pc: f64) -> f64 {
        pc * 3.26156
    }

    pub fn ly_to_pc(ly: f64) -> f64 {
        ly / 3.26156
    }

    pub fn pc_to_meters(pc: f64) -> f64 {
        pc * 3.0857e16
    }

    pub fn angstrom_to_pc(angstrom: f64) -> f64 {
        angstrom * 1e-10 / 3.0857e16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsec_calculation() {
        let calc = ParsecCalculation::new(10.0);
        assert!(calc.distance_ly > 0.0);
    }
}
