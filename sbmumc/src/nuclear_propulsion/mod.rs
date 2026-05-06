//! Nuclear Propulsion Module (648)
//!
//! Nuclear thermal and nuclear pulse propulsion system design and simulation.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NuclearPropulsionType {
    NTR,           // Nuclear Thermal Rocket
    NEP,           // Nuclear Electric Propulsion
    NuclearPulse,  // Orion-style pulse propulsion
    FusionDrive,
    AntimatterCatalyzed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearPropulsionSystem {
    pub propulsion_type: NuclearPropulsionType,
    pub thrust: f64,             // kN
    pub isp: f64,                // s
    pub power_output: f64,       // MW
    pub reactor_power: f64,       // MW thermal
    pub fuel_type: String,
    pub enrichment: f64,          // percent
    pub core_mass: f64,          // kg
    pub shield_mass: f64,        // kg
    pub dry_mass: f64,           // kg
    pub operational_lifetime: f64, // years
    pub safety_factor: f64,
}

impl NuclearPropulsionSystem {
    pub fn new(propulsion_type: NuclearPropulsionType) -> Self {
        Self {
            propulsion_type,
            thrust: 0.0,
            isp: 0.0,
            power_output: 0.0,
            reactor_power: 0.0,
            fuel_type: "UH3".into(),
            enrichment: 0.0,
            core_mass: 0.0,
            shield_mass: 0.0,
            dry_mass: 0.0,
            operational_lifetime: 0.0,
            safety_factor: 0.0,
        }
    }

    pub fn calculate_isp_ntr(&self, exhaust_temperature: f64, molecular_weight: f64) -> f64 {
        let r = 8.31446;
        let g0 = 9.80665;
        (exhaust_temperature * r / (molecular_weight * g0)).sqrt()
    }

    pub fn calculate_thrust_to_power(&self) -> f64 {
        self.thrust / self.power_output.max(0.001)
    }

    pub fn radiation_shielding_requirement(&self) -> f64 {
        let dose_rate = self.reactor_power * 1e6 / (4.0 * std::f64::consts::PI * 10.0_f64.powi(2));
        100.0 / dose_rate.max(1.0)
    }
}

pub struct NuclearPropulsionAnalysis;

impl NuclearPropulsionAnalysis {
    pub fn ntr_specific_impulse(temperature: f64, mw: f64) -> f64 {
        let r = 8.31446;
        let g0 = 9.80665;
        (2.0 * r * temperature / (mw * g0) * 1000.0).sqrt()
    }

    pub fn nuclear_rocket_mass_ratio(thrust: f64, isp: f64, dry_mass: f64) -> f64 {
        let g0 = 9.80665;
        let ve = isp * g0;
        let propellant = thrust * 1000.0 / ve;
        (dry_mass + propellant) / dry_mass
    }

    pub fn critical_mass(isotope: &str, density: f64) -> f64 {
        match isotope {
            "U235" => 52.0 / density,
            "Pu239" => 10.0 / density,
            _ => 100.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nuclear_propulsion() {
        let system = NuclearPropulsionSystem::new(NuclearPropulsionType::NTR);
        assert!(matches!(system.propulsion_type, NuclearPropulsionType::NTR));
    }
}
