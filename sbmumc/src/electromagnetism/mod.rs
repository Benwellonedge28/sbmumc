//! Electromagnetism Module
//!
//! This module implements electromagnetism, electromagnetic fields,
//! and electromagnetic phenomena for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Electromagnetism {
    pub em_id: String,
    pub maxwell_equations: MaxwellEquations,
    pub electromagnetic_waves: Vec<EMWave>,
    pub electromagnetic_fields: Vec<EMField>,
    pub circuit_theory: CircuitTheory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxwellEquations { pub gauss_law: String, pub gauss_magnetism: String, pub faraday_law: String, pub ampere_law: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMWave { pub wave_id: String, pub wavelength_m: f64, pub frequency_hz: f64, pub speed_m_s: f64, pub polarization: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMField { pub field_id: String, pub field_type: FieldType, pub magnitude: f64, pub direction: [f64; 3] }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldType { Electric, Magnetic, Electromagnetic }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitTheory { pub circuits: Vec<Circuit>, pub components: Vec<Component> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit { pub circuit_id: String, pub nodes: u32, pub elements: Vec<String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component { pub component_id: String, pub component_type: ComponentType, pub value: f64, pub units: String }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComponentType { Resistor, Capacitor, Inductor, Diode, Transistor }

impl Electromagnetism {
    pub fn new() -> Self {
        Self {
            em_id: String::from("electromagnetism_v1"),
            maxwell_equations: MaxwellEquations { gauss_law: String::from("div E = rho/eps0"), gauss_magnetism: String::from("div B = 0"), faraday_law: String::from("curl E = -dB/dt"), ampere_law: String::from("curl B = mu0 J + mu0 eps0 dE/dt") },
            electromagnetic_waves: vec![EMWave { wave_id: String::from("wave_1"), wavelength_m: 500e-9, frequency_hz: 6e14, speed_m_s: 3e8, polarization: String::from("Linear") }],
            electromagnetic_fields: vec![EMField { field_id: String::from("field_1"), field_type: FieldType::Electric, magnitude: 1e6, direction: [0.0, 0.0, 1.0] }],
            circuit_theory: CircuitTheory { circuits: vec![Circuit { circuit_id: String::from("ckt_1"), nodes: 4, elements: vec![String::from("R1"), String::from("C1")] }], components: vec![Component { component_id: String::from("comp_1"), component_type: ComponentType::Resistor, value: 100.0, units: String::from("Ohm") }] },
        }
    }

    pub fn compute_electric_field(&self, q: f64, r: f64, eps_r: f64) -> f64 {
        let eps0 = 8.854e-12;
        q / (4.0 * 3.14159 * eps0 * eps_r * r.powi(2))
    }

    pub fn compute_magnetic_field(&self, i: f64, r: f64) -> f64 {
        let mu0 = 1.256e-6;
        mu0 * i / (2.0 * 3.14159 * r)
    }

    pub fn compute_wave_impedance(&self, mu_r: f64, eps_r: f64) -> f64 {
        let mu0 = 1.256e-6;
        let eps0 = 8.854e-12;
        (mu0 * mu_r / (eps0 * eps_r)).sqrt()
    }
}

impl Default for Electromagnetism { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_electric_field() { let em = Electromagnetism::new(); assert!(em.compute_electric_field(1e-9, 0.1, 1.0) > 0.0); } }
