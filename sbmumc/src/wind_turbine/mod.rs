//! Wind Turbine Module (754)
//!
//! Wind turbine design, aerodynamics, and turbine performance.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TurbineClass {
    IECClassI,
    IECClassII,
    IECClassIII,
    Offshore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindTurbine {
    pub turbine_id: String,
    pub rated_power_mw: f64,
    pub rotor_diameter_m: f64,
    pub hub_height_m: f64,
    pub turbine_class: TurbineClass,
    pub cut_in_speed_ms: f64,
    pub cut_out_speed_ms: f64,
    pub capacity_factor: f64,
}

impl WindTurbine {
    pub fn new(turbine_id: String) -> Self {
        Self {
            turbine_id,
            rated_power_mw: 0.0,
            rotor_diameter_m: 0.0,
            hub_height_m: 0.0,
            turbine_class: TurbineClass::IECClassII,
            cut_in_speed_ms: 3.0,
            cut_out_speed_ms: 25.0,
            capacity_factor: 0.0,
        }
    }

    pub fn specific_power(&self) -> f64 {
        let swept_area = std::f64::consts::PI * (self.rotor_diameter_m / 2.0).powi(2);
        if swept_area <= 0.0 { return 0.0; }
        self.rated_power_mw / swept_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wind_turbine() {
        let turbine = WindTurbine::new("WT-001".into());
        assert_eq!(turbine.turbine_id, "WT-001");
    }
}
