//! Ion Drive Module (647)
//!
//! Ion propulsion system design, simulation and optimization.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IonThrusterType {
    Gridded,
    HallEffect,
    Electrospray,
    Colloid,
    Arcjet,
    PulsedInductive,
    VASIMR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonThruster {
    pub thruster_type: IonThrusterType,
    pub thrust: f64,             // mN
    pub specific_impulse: f64,   // s
    pub power: f64,              // kW
    pub efficiency: f64,
    pub propellant: String,
    pub mass_flow_rate: f64,     // mg/s
    pub beam_voltage: f64,       // V
    pub beam_current: f64,       // A
    pub discharge_voltage: f64,  // V
    pub specific_charge: f64,    // C/kg
    pub lifetime: f64,           // hours
    pub dry_mass: f64,           // kg
}

impl IonThruster {
    pub fn new(thruster_type: IonThrusterType) -> Self {
        Self {
            thruster_type,
            thrust: 0.0,
            specific_impulse: 0.0,
            power: 0.0,
            efficiency: 0.0,
            propellant: "Xenon".into(),
            mass_flow_rate: 0.0,
            beam_voltage: 0.0,
            beam_current: 0.0,
            discharge_voltage: 0.0,
            specific_charge: 0.0,
            lifetime: 0.0,
            dry_mass: 0.0,
        }
    }

    pub fn calculate_thrust(&self) -> f64 {
        let g0 = 9.80665;
        2.0 * self.beam_current * self.beam_voltage.sqrt() * self.efficiency / (self.specific_impulse * g0).sqrt()
    }

    pub fn calculate_isp(&self) -> f64 {
        let g0 = 9.80665;
        0.5 * self.beam_voltage / (g0 * self.specific_charge).sqrt() * self.efficiency.sqrt()
    }

    pub fn calculate_mass_flow(&self) -> f64 {
        self.beam_current / self.specific_charge / 1000.0
    }

    pub fn power_to_thrust_ratio(&self) -> f64 {
        self.power / self.calculate_thrust().max(0.001) * 1000.0
    }
}

pub struct IonDriveSimulation;

impl IonDriveSimulation {
    pub fn grid_erosion_rate(beam_current: f64, energy: f64) -> f64 {
        beam_current * energy.powi(2) * 1e-6
    }

    pub fn discharge_loss(voltage: f64, current: f64, propellant_utilization: f64) -> f64 {
        voltage * current * (1.0 - propellant_utilization)
    }

    pub fn divergence_loss(divergence_angle: f64) -> f64 {
        (divergence_angle * std::f64::consts::PI / 180.0).cos().powi(2)
    }

    pub fn ion_thrust(beam_current: f64, beam_voltage: f64, isp: f64) -> f64 {
        let g0 = 9.80665;
        2.0 * beam_current * (beam_voltage * isp * g0 / 2.0).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ion_thruster() {
        let thruster = IonThruster::new(IonThrusterType::Gridded);
        assert!(matches!(thruster.thruster_type, IonThrusterType::Gridded));
    }
}
