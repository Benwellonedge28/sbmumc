//! Rocket Engine Design Module (646)
//!
//! Detailed rocket engine design and simulation for chemical and advanced propulsion.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombustionCycle {
    GasGenerator,
    StagedCombustion,
    ExpanderCycle,
    TapOffCycle,
    PressureFed,
    ElectricPump,
    FullFlowStaged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RocketEngine {
    pub name: String,
    pub cycle: CombustionCycle,
    pub thrust_vacuum: f64,      // kN
    pub thrust_sea_level: f64,     // kN
    pub isp_vacuum: f64,          // s
    pub isp_sea_level: f64,       // s
    pub chamber_pressure: f64,    // MPa
    pub expansion_ratio: f64,
    pub propellant_flow: f64,      // kg/s
    pub mixture_ratio: f64,       // O/F
    pub dry_mass: f64,            // kg
    pub engine_length: f64,       // m
    pub nozzle_exit_diameter: f64, // m
    pub combustion_temperature: f64, // K
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombustionChamber {
    pub diameter: f64,           // m
    pub length: f64,              // m
    pub volume: f64,              // m^3
    pub wall_thickness: f64,      // m
    pub material: String,
    pub cooling_type: String,
    pub max_temperature: f64,    // K
    pub thermal_conductivity: f64,
}

impl RocketEngine {
    pub fn new(name: String, cycle: CombustionCycle) -> Self {
        Self {
            name,
            cycle,
            thrust_vacuum: 0.0,
            thrust_sea_level: 0.0,
            isp_vacuum: 0.0,
            isp_sea_level: 0.0,
            chamber_pressure: 0.0,
            expansion_ratio: 0.0,
            propellant_flow: 0.0,
            mixture_ratio: 0.0,
            dry_mass: 0.0,
            engine_length: 0.0,
            nozzle_exit_diameter: 0.0,
            combustion_temperature: 0.0,
        }
    }

    pub fn calculate_thrust(&self, ambient_pressure: f64) -> f64 {
        let pc = self.chamber_pressure * 1e6; // Pa
        let ae = std::f64::consts::PI * (self.nozzle_exit_diameter/2.0).powi(2);
        let pe = ambient_pressure;
        let thrust = pc * ae * self.expansion_ratio + (pe - ambient_pressure) * ae * 1e-3;
        thrust.max(0.0)
    }

    pub fn calculate_isp(&self, ambient_pressure: f64) -> f64 {
        let thrust = self.calculate_thrust(ambient_pressure);
        let g0 = 9.80665;
        thrust / (self.propellant_flow * g0)
    }

    pub fn optimize_expansion_ratio(&self, target_altitude: f64) -> f64 {
        let p0 = 101325.0;
        let pc = self.chamber_pressure * 1e6;
        let gamma = 1.2;
        (2.0 / (gamma - 1.0) * (pc / p0).powf((gamma - 1.0) / (2.0 * gamma) - 1.0)).sqrt()
    }
}

impl CombustionChamber {
    pub fn new(diameter: f64, length: f64) -> Self {
        Self {
            diameter,
            length,
            volume: std::f64::consts::PI * (diameter/2.0).powi(2) * length,
            wall_thickness: 0.0,
            material: "N/A".into(),
            cooling_type: "Regenerative".into(),
            max_temperature: 0.0,
            thermal_conductivity: 0.0,
        }
    }

    pub fn calculate_heat_flux(&self, q_combustion: f64) -> f64 {
        q_combustion / self.volume
    }

    pub fn cooling_requirement(&self, dt: f64) -> f64 {
        self.max_temperature * dt * self.thermal_conductivity
    }
}

pub struct EngineAnalysis;

impl EngineAnalysis {
    pub fn calculate_specific_impulse(thrust: f64, mass_flow: f64, g0: f64) -> f64 {
        thrust / (mass_flow * g0)
    }

    pub fn engine_efficiency(isp_actual: f64, isp_theoretical: f64) -> f64 {
        (isp_actual / isp_theoretical.max(1.0)).min(1.0)
    }

    pub fn mass_flow_rate(thrust: f64, isp: f64, g0: f64) -> f64 {
        thrust / (isp * g0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rocket_engine() {
        let engine = RocketEngine::new(" Merlin".into(), CombustionCycle::GasGenerator);
        assert_eq!(engine.name, " Merlin");
    }
}
