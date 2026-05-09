//! # SBMUMC Module 1029: Quantum Ion Channels
//!
//! Quantum effects in ion channel function and neural signaling.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IonChannelType {
    VoltageGatedSodium,
    VoltageGatedPotassium,
    VoltageGatedCalcium,
    LigandGated,
    Mechanosensitive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumIonChannel {
    pub channel_id: String,
    pub channel_type: IonChannelType,
    pub ion_species: String,
    pub conductance_pS: f64,
    pub quantum_coherence_conductance: f64,
    pub selectivity_filter_coherence: f64,
}

impl QuantumIonChannel {
    pub fn new(channel_type: IonChannelType, ion: String) -> Self {
        Self {
            channel_id: crate::core::uuid_simple(),
            channel_type,
            ion_species: ion,
            conductance_pS: 0.0,
            quantum_coherence_conductance: 0.0,
            selectivity_filter_coherence: 0.0,
        }
    }

    pub fn compute_quantum_conductance(&mut self) -> Result<()> {
        match self.channel_type {
            IonChannelType::VoltageGatedSodium => {
                self.conductance_pS = 15.0 + rand_simple() * 5.0;
            },
            IonChannelType::VoltageGatedPotassium => {
                self.conductance_pS = 10.0 + rand_simple() * 8.0;
            },
            IonChannelType::VoltageGatedCalcium => {
                self.conductance_pS = 8.0 + rand_simple() * 4.0;
            },
            IonChannelType::LigandGated => {
                self.conductance_pS = 20.0 + rand_simple() * 10.0;
            },
            IonChannelType::Mechanosensitive => {
                self.conductance_pS = 5.0 + rand_simple() * 15.0;
            }
        }

        self.quantum_coherence_conductance = self.conductance_pS * (1.05 + rand_simple() * 0.15);
        self.selectivity_filter_coherence = 0.85 + rand_simple() * 0.15;
        Ok(())
    }

    pub fn simulate_ion_flow(&self, duration_ms: f64, membrane_potential_mV: f64) -> f64 {
        let driving_force = membrane_potential_mV / 60.0;
        self.quantum_coherence_conductance * driving_force * duration_ms / 1000.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSelectivityFilter {
    pub filter_id: String,
    pub channel_id: String,
    pub ion_type: String,
    pub filter_diameter_angstrom: f64,
    pub quantum_selectivity_ratio: f64,
    pub binding_site_coherence: f64,
}

impl QuantumSelectivityFilter {
    pub fn new(channel: String, ion: String) -> Self {
        Self {
            filter_id: crate::core::uuid_simple(),
            channel_id: channel,
            ion_type: ion,
            filter_diameter_angstrom: 0.0,
            quantum_selectivity_ratio: 0.0,
            binding_site_coherence: 0.0,
        }
    }

    pub fn analyze_selectivity(&mut self) -> Result<()> {
        match self.ion_type.as_str() {
            "Na+" => self.filter_diameter_angstrom = 3.0 + rand_simple() * 0.5,
            "K+" => self.filter_diameter_angstrom = 4.0 + rand_simple() * 0.5,
            "Ca2+" => self.filter_diameter_angstrom = 4.5 + rand_simple() * 0.5,
            _ => self.filter_diameter_angstrom = 3.5 + rand_simple() * 0.5,
        }
        self.quantum_selectivity_ratio = 50.0 + rand_simple() * 450.0;
        self.binding_site_coherence = 0.8 + rand_simple() * 0.2;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumChannelGating {
    pub gating_id: String,
    pub channel_id: String,
    pub gate_type: String,
    pub activation_threshold_mV: f64,
    pub quantum_gate_speed_ms: f64,
    pub coherence_enhanced_gating: f64,
}

impl QuantumChannelGating {
    pub fn new(channel: String, gate: String) -> Self {
        Self {
            gating_id: crate::core::uuid_simple(),
            channel_id: channel,
            gate_type: gate,
            activation_threshold_mV: 0.0,
            quantum_gate_speed_ms: 0.0,
            coherence_enhanced_gating: 0.0,
        }
    }

    pub fn simulate_gating(&mut self) -> Result<()> {
        match self.gate_type.as_str() {
            "Voltage" => {
                self.activation_threshold_mV = -50.0 + rand_simple() * -20.0;
            },
            "Ligand" => {
                self.activation_threshold_mV = 0.0;
            },
            _ => {
                self.activation_threshold_mV = -40.0 + rand_simple() * -30.0;
            }
        }
        self.quantum_gate_speed_ms = 0.1 + rand_simple() * 0.9;
        self.coherence_enhanced_gating = 1.2 + rand_simple() * 0.5;
        Ok(())
    }
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

pub fn simulate_quantum_ion_flow(channel_type: IonChannelType, duration: f64) -> Result<f64> {
    let mut channel = QuantumIonChannel::new(channel_type, "Generic".to_string());
    channel.compute_quantum_conductance()?;
    Ok(channel.simulate_ion_flow(duration, -70.0))
}

pub fn compute_channel_coherence(channel_id: &str) -> Result<f64> {
    Ok(0.85 + rand_simple() * 0.15)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_potassium_channel() {
        let mut channel = QuantumIonChannel::new(
            IonChannelType::VoltageGatedPotassium,
            "K+".to_string(),
        );
        channel.compute_quantum_conductance().unwrap();
        assert!(channel.quantum_coherence_conductance > 0.0);
    }

    #[test]
    fn test_selectivity_filter() {
        let mut filter = QuantumSelectivityFilter::new("KcsA_Channel".to_string(), "K+".to_string());
        filter.analyze_selectivity().unwrap();
        assert!(filter.quantum_selectivity_ratio > 1.0);
    }

    #[test]
    fn test_gating_simulation() {
        let mut gating = QuantumChannelGating::new("Nav1_5".to_string(), "Voltage".to_string());
        gating.simulate_gating().unwrap();
        assert!(gating.activation_threshold_mV < 0.0);
    }
}