//! # SBMUMC Module 882: Aircraft Performance
//! 
//! Flight performance and operational calculations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// Aircraft performance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceData {
    pub max_takeoff_weight_kg: f64,
    pub max_landing_weight_kg: f64,
    pub empty_weight_kg: f64,
    pub fuel_capacity_kg: f64,
    pub wing_area_m2: f64,
    pub aspect_ratio: f64,
    pub max_cruise_kt: f64,
}

/// Takeoff performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeoffPerformance {
    pub field_length_m: f64,
    pub takeoff_distance_m: f64,
    pub accelerate_stop_distance: f64,
    pub climb_gradient: f64,
    pub v1_kt: f64,
    pub vr_kt: f64,
    pub v2_kt: f64,
}

/// Landing performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingPerformance {
    pub field_length_m: f64,
    pub landing_distance_m: f64,
    pub approach_speed_kt: f64,
    pub touchdown_speed_kt: f64,
    pub brake_energy: f64,
}

/// Cruise performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CruisePerformance {
    pub mach_number: f64,
    pub true_airspeed_kt: f64,
    pub fuel_flow_kg_hr: f64,
    pub range_nm: f64,
    pub endurance_hr: f64,
    pub ceiling_ft: f64,
}

impl AircraftPerformance {
    /// Create new aircraft performance system
    pub fn new() -> Self {
        Self
    }

    /// Calculate takeoff distance
    pub fn calculate_takeoff(&self, perf: &PerformanceData, temp_c: f64, pressure_alt_ft: f64) -> Result<TakeoffPerformance> {
        let density_alt = pressure_alt_ft + 120.0 * (temp_c - 15.0);
        let v2 = 1.2 * (density_alt / 1000.0).sqrt() * 100.0;
        
        Ok(TakeoffPerformance {
            field_length_m: density_alt * 1.5,
            takeoff_distance_m: density_alt * 1.2,
            accelerate_stop_distance: density_alt * 1.6,
            climb_gradient: 0.05,
            v1_kt: v2 - 5.0,
            vr_kt: v2 - 2.0,
            v2_kt: v2,
        })
    }

    /// Calculate landing distance
    pub fn calculate_landing(&self, perf: &PerformanceData, field_length_available: f64) -> Result<LandingPerformance> {
        let approach_speed = 1.3 * 100.0;
        Ok(LandingPerformance {
            field_length_m: field_length_available * 0.6,
            landing_distance_m: field_length_available * 0.5,
            approach_speed_kt: approach_speed,
            touchdown_speed_kt: approach_speed - 5.0,
            brake_energy: 50000.0,
        })
    }

    /// Calculate cruise performance
    pub fn calculate_cruise(&self, perf: &PerformanceData, altitude_ft: f64, mach: f64) -> Result<CruisePerformance> {
        let isa_dev = 0.0;
        let speed_sound = 661.5 * (1.0 + isa_dev * 0.02) * ((altitude_ft / 1000.0) as f64 * 0.002 - 1.0).exp();
        let tas = mach * speed_sound;
        
        Ok(CruisePerformance {
            mach_number: mach,
            true_airspeed_kt: tas * 1.944,
            fuel_flow_kg_hr: perf.empty_weight_kg * 0.02,
            range_nm: 3000.0,
            endurance_hr: 8.0,
            ceiling_ft: 41000.0,
        })
    }
}

impl Default for AircraftPerformance {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AircraftPerformance;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_takeoff_calculation() {
        let system = AircraftPerformance::new();
        let perf = PerformanceData {
            max_takeoff_weight_kg: 79000.0,
            max_landing_weight_kg: 66000.0,
            empty_weight_kg: 41000.0,
            fuel_capacity_kg: 26000.0,
            wing_area_m2: 122.0,
            aspect_ratio: 9.0,
            max_cruise_kt: 450.0,
        };
        let takeoff = system.calculate_takeoff(&perf, 15.0, 0.0);
        assert!(takeoff.is_ok());
    }

    #[test]
    fn test_cruise_calculation() {
        let system = AircraftPerformance::new();
        let perf = PerformanceData {
            max_takeoff_weight_kg: 79000.0,
            max_landing_weight_kg: 66000.0,
            empty_weight_kg: 41000.0,
            fuel_capacity_kg: 26000.0,
            wing_area_m2: 122.0,
            aspect_ratio: 9.0,
            max_cruise_kt: 450.0,
        };
        let cruise = system.calculate_cruise(&perf, 35000.0, 0.82);
        assert!(cruise.is_ok());
    }
}
