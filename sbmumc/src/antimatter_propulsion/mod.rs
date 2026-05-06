//! Antimatter Propulsion Module (649)
//!
//! Antimatter-catalyzed and matter-antimatter annihilation propulsion systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AntimatterPropulsionType {
    AntimatterCatalyzed,
    MatterAntimatterAnnihilation,
    AntimatterHeated,
    AntimatterFusion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntimatterStorage {
    pub storage_type: String,    // Penning trap, MAGNEL
    pub capacity: f64,           // kg
    pub containment_field: f64,  // T
    pub temperature: f64,       // K
    pub vacuum_level: f64,       // Pa
    pub half_life: f64,          // years
    pub efficiency: f64,         // percent
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntimatterPropulsionSystem {
    pub propulsion_type: AntimatterPropulsionType,
    pub thrust: f64,             // N
    pub isp: f64,                // s
    pub antimatter_mass: f64,    // micrograms
    pub matter_mass: f64,        // kg
    pub storage: AntimatterStorage,
    pub annihilation_rate: f64,   // kg/s
    pub specific_impulse: f64,   // s
    pub power_output: f64,       // GW
    pub dry_mass: f64,           // kg
    pub safety_rating: String,
}

impl AntimatterPropulsionSystem {
    pub fn new(propulsion_type: AntimatterPropulsionType) -> Self {
        Self {
            propulsion_type,
            thrust: 0.0,
            isp: 0.0,
            antimatter_mass: 0.0,
            matter_mass: 0.0,
            storage: AntimatterStorage {
                storage_type: "Penning Trap".into(),
                capacity: 0.0,
                containment_field: 0.0,
                temperature: 4.0,
                vacuum_level: 1e-10,
                half_life: 0.0,
                efficiency: 0.0,
            },
            annihilation_rate: 0.0,
            specific_impulse: 0.0,
            power_output: 0.0,
            dry_mass: 0.0,
            safety_rating: "N/A".into(),
        }
    }

    pub fn energy_from_annihilation(&self) -> f64 {
        let c = 299792458.0; // m/s
        let e_per_kg = c * c * 1e6; // Joules
        (self.antimatter_mass * 1e-6 + self.matter_mass) * e_per_kg
    }

    pub fn thrust_from_antimatter(&self) -> f64 {
        let ve = self.isp * 9.80665;
        let mdot = self.annihilation_rate;
        mdot * ve
    }

    pub fn calculate_cost(&self, cost_per_gram: f64) -> f64 {
        self.antimatter_mass * cost_per_gram * 1e6
    }
}

pub struct AntimatterAnalysis;

impl AntimatterAnalysis {
    pub fn annihilation_energy(mass: f64) -> f64 {
        let c = 299792458.0_f64.powi(2);
        mass * c * 1e6 // Joules
    }

    pub fn antiproton_production_rate(beam_power: f64) -> f64 {
        beam_power / 6e13 // protons per second per watt
    }

    pub fn storage_half_life(field_strength: f64, temperature: f64) -> f64 {
        (field_strength / temperature).exp() * 1e6 // seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antimatter_propulsion() {
        let system = AntimatterPropulsionSystem::new(AntimatterPropulsionType::MatterAntimatterAnnihilation);
        assert!(matches!(system.propulsion_type, AntimatterPropulsionType::MatterAntimatterAnnihilation));
    }
}
