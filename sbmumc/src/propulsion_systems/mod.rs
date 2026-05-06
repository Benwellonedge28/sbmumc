//! Propulsion Systems Module (645)
//!
//! Advanced propulsion system design and simulation for spacecraft.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropulsionType {
    Chemical,
    Electric,
    Nuclear,
    Antimatter,
    SolarSail,
    LaserPropulsion,
    Fusion,
    AntimatterCatalyzed,
    MatterAntimatter,
    WarpDrive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropulsionSystem {
    pub propulsion_type: PropulsionType,
    pub thrust: f64,             // N
    pub specific_impulse: f64,   // s
    pub power_consumption: f64,  // kW
    pub dry_mass: f64,           // kg
    pub propellant_mass: f64,    // kg
    pub total_mass: f64,         // kg
    pub efficiency: f64,
    pub engine_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrustProfile {
    pub time: Vec<f64>,          // s
    pub thrust: Vec<f64>,        // N
    pub mass_flow_rate: Vec<f64>, // kg/s
    pub specific_impulse: Vec<f64>, // s
}

impl PropulsionSystem {
    pub fn new(propulsion_type: PropulsionType) -> Self {
        Self {
            propulsion_type,
            thrust: 0.0,
            specific_impulse: 0.0,
            power_consumption: 0.0,
            dry_mass: 0.0,
            propellant_mass: 0.0,
            total_mass: 0.0,
            efficiency: 0.0,
            engine_type: String::new(),
        }
    }

    pub fn calculate_delta_v(&self) -> f64 {
        let isp = self.specific_impulse;
        let g0 = 9.80665;
        self.total_mass / (self.total_mass - self.propellant_mass) * isp * g0 * (1.0 - self.efficiency).max(0.001).ln()
    }

    pub fn calculate_mass_ratio(&self) -> f64 {
        (self.delta_v() / (self.specific_impulse * 9.80665) + 1.0).exp()
    }

    pub fn set_parameters(&mut self, thrust: f64, isp: f64, dry_mass: f64, propellant_mass: f64) {
        self.thrust = thrust;
        self.specific_impulse = isp;
        self.dry_mass = dry_mass;
        self.propellant_mass = propellant_mass;
        self.total_mass = dry_mass + propellant_mass;
    }
}

impl ThrustProfile {
    pub fn new() -> Self {
        Self {
            time: Vec::new(),
            thrust: Vec::new(),
            mass_flow_rate: Vec::new(),
            specific_impulse: Vec::new(),
        }
    }

    pub fn add_data_point(&mut self, time: f64, thrust: f64, mass_flow: f64, isp: f64) {
        self.time.push(time);
        self.thrust.push(thrust);
        self.mass_flow_rate.push(mass_flow);
        self.specific_impulse.push(isp);
    }

    pub fn total_impulse(&self) -> f64 {
        self.thrust.iter().zip(self.time.iter())
            .map(|(t, dt)| t * dt)
            .sum()
    }
}

pub struct PropulsionCalculator;

impl PropulsionCalculator {
    pub fn rocket_equation(m0: f64, m1: f64, ve: f64) -> f64 {
        ve * ((m0 / m1).max(1.001).ln())
    }

    pub fn tsiolkovsky_delta_v(initial_mass: f64, final_mass: f64, isp: f64) -> f64 {
        let g0 = 9.80665;
        isp * g0 * (initial_mass / final_mass.max(1.0)).ln()
    }

    pub fn calculate_power_to_thrust(power: f64, efficiency: f64, isp: f64) -> f64 {
        let g0 = 9.80665;
        2.0 * efficiency * power / (isp * g0)
    }

    pub fn specific_mass(power: f64, thrust: f64) -> f64 {
        power / thrust.max(0.001)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_propulsion_system() {
        let mut prop = PropulsionSystem::new(PropulsionType::Chemical);
        prop.set_parameters(1000.0, 300.0, 100.0, 500.0);
        assert!(prop.thrust > 0.0);
    }

    #[test]
    fn test_delta_v_calculation() {
        let mut prop = PropulsionSystem::new(PropulsionType::Electric);
        prop.set_parameters(100.0, 3000.0, 500.0, 200.0);
        assert!(prop.calculate_delta_v() > 0.0);
    }
}
