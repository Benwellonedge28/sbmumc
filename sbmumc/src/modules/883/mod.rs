//! # SBMUMC Module 883: Electric Propulsion
//! 
//! Electric motor systems for transportation.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// Motor types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MotorType {
    PMSM,
    Induction,
    SRM,
    AxialFlux,
}

/// Power electronics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerElectronicsConfig {
    pub inverter_type: String,
    pub switching_frequency_hz: f64,
    pub modulation_type: String,
    pub dc_voltage_v: f64,
}

/// Motor performance map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorPerformanceMap {
    pub torque_points: Vec<(f64, f64, f64)>,
    pub efficiency_grid: Vec<Vec<f64>>,
    pub max_torque_nm: f64,
    pub max_speed_rpm: u32,
    pub peak_power_kw: f64,
    pub continuous_power_kw: f64,
}

/// Drive cycle analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveCycleAnalysis {
    pub cycle_name: String,
    pub energy_consumption_wh_km: f64,
    pub avg_efficiency: f64,
    pub regeneration_benefit: f64,
    pub thermal_peak_temp_c: f64,
}

impl ElectricPropulsion {
    /// Create new electric propulsion system
    pub fn new() -> Self {
        Self
    }

    /// Calculate motor efficiency
    pub fn calculate_motor_efficiency(&self, speed: f64, torque: f64, motor: &MotorPerformanceMap) -> Result<f64> {
        let speed_ratio = speed / motor.max_speed_rpm as f64;
        let torque_ratio = torque / motor.max_torque_nm;
        let base_efficiency = 0.95;
        let load_factor = speed_ratio * torque_ratio;
        Ok(base_efficiency * load_factor.min(1.0) * 0.95.max(0.85))
    }

    /// Size motor for application
    pub fn size_motor(&self, vehicle_mass: f64, max_speed_kmh: f64, gradeability: f64) -> Result<MotorPerformanceMap> {
        let required_power = vehicle_mass * 9.81 * max_speed_kmh / 3.6 / 1000.0 * 1.2;
        let required_torque = vehicle_mass * 9.81 * 0.5 / 0.3;
        Ok(MotorPerformanceMap {
            torque_points: vec![],
            efficiency_grid: vec![],
            max_torque_nm: required_torque,
            max_speed_rpm: (max_speed_kmh / 3.6 * 60.0 / 0.3) as u32,
            peak_power_kw: required_power * 2.0,
            continuous_power_kw: required_power,
        })
    }

    /// Analyze drive cycle
    pub fn analyze_drive_cycle(&self, cycle_data: &[DrivePoint]) -> Result<DriveCycleAnalysis> {
        let total_energy: f64 = cycle_data.iter().map(|p| p.power_w * p.duration_s).sum();
        let distance: f64 = cycle_data.iter().map(|p| p.speed_kmh * p.duration_s / 3600.0).sum();
        Ok(DriveCycleAnalysis {
            cycle_name: "WLTP".to_string(),
            energy_consumption_wh_km: total_energy / distance,
            avg_efficiency: 0.90,
            regeneration_benefit: 0.15,
            thermal_peak_temp_c: 120.0,
        })
    }
}

impl Default for ElectricPropulsion {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ElectricPropulsion;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrivePoint {
    pub speed_kmh: f64,
    pub power_w: f64,
    pub duration_s: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motor_sizing() {
        let system = ElectricPropulsion::new();
        let motor = system.size_motor(1500.0, 150.0, 0.05);
        assert!(motor.is_ok());
    }

    #[test]
    fn test_efficiency_calculation() {
        let system = ElectricPropulsion::new();
        let perf = MotorPerformanceMap {
            torque_points: vec![],
            efficiency_grid: vec![],
            max_torque_nm: 300.0,
            max_speed_rpm: 12000,
            peak_power_kw: 150.0,
            continuous_power_kw: 75.0,
        };
        let eff = system.calculate_motor_efficiency(6000.0, 200.0, &perf);
        assert!(eff.is_ok());
    }
}
