//! Quantum Chromodynamics Module (545)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumChromodynamics {
    pub qcd_id: String,
    pub color_charge: Vec<ColorCharge>,
    pub quark_flavor: Vec<QuarkFlavor>,
    pub strong_coupling: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorCharge {
    Red,
    Green,
    Blue,
    AntiRed,
    AntiGreen,
    AntiBlue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuarkFlavor {
    Up,
    Down,
    Charm,
    Strange,
    Top,
    Bottom,
}

impl QuantumChromodynamics {
    pub fn new() -> Self {
        Self {
            qcd_id: String::from("quantum_chromodynamics_v1"),
            color_charge: vec![ColorCharge::Red, ColorCharge::Green, ColorCharge::Blue],
            quark_flavor: vec![QuarkFlavor::Up, QuarkFlavor::Down],
            strong_coupling: 0.118,
        }
    }
}

impl Default for QuantumChromodynamics {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_qcd() {
        let qcd = QuantumChromodynamics::new();
        assert!(qcd.strong_coupling < 1.0);
    }
}
