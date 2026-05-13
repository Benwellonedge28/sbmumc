//! # SBMUMC Module 1516: Alchemical Arts
//!
//! Systems for alchemical arts and transmutation.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlchemicalArtsTopic {
    AlchemicalTransmutation,
    PhilosopherStone,
    AlchemicalSymbols,
    MetallicTransmutation,
    Alchemical Stages,
    HermeticAlchemy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlchemicalArtsSystem {
    pub system_id: String,
    pub alchemical_arts_topic: AlchemicalArtsTopic,
    pub alchemical_mastery: f64,
    pub transmutation_power: f64,
    pub hermetic_wisdom: f64,
    pub prima_materia: f64,
}

impl AlchemicalArtsSystem {
    pub fn new(alchemical_arts_topic: AlchemicalArtsTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            alchemical_arts_topic,
            alchemical_mastery: 0.0,
            transmutation_power: 0.0,
            hermetic_wisdom: 0.0,
            prima_materia: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.alchemical_arts_topic {
            AlchemicalArtsTopic::AlchemicalTransmutation => {
                self.alchemical_mastery = 0.95 + rand_simple() * 0.05;
                self.transmutation_power = 0.90 + rand_simple() * 0.10;
                self.hermetic_wisdom = 0.85 + rand_simple() * 0.14;
            },
            AlchemicalArtsTopic::PhilosopherStone => {
                self.prima_materia = 0.95 + rand_simple() * 0.05;
                self.hermetic_wisdom = 0.90 + rand_simple() * 0.10;
                self.transmutation_power = 0.85 + rand_simple() * 0.14;
            },
            AlchemicalArtsTopic::AlchemicalSymbols => {
                self.transmutation_power = 0.95 + rand_simple() * 0.05;
                self.alchemical_mastery = 0.90 + rand_simple() * 0.10;
                self.prima_materia = 0.85 + rand_simple() * 0.14;
            },
            AlchemicalArtsTopic::MetallicTransmutation => {
                self.hermetic_wisdom = 0.95 + rand_simple() * 0.05;
                self.prima_materia = 0.90 + rand_simple() * 0.10;
                self.alchemical_mastery = 0.85 + rand_simple() * 0.14;
            },
            AlchemicalArtsTopic::Alchemical Stages => {
                self.alchemical_mastery = 0.95 + rand_simple() * 0.05;
                self.transmutation_power = 0.90 + rand_simple() * 0.10;
                self.prima_materia = 0.85 + rand_simple() * 0.14;
            },
            AlchemicalArtsTopic::HermeticAlchemy => {
                self.prima_materia = 0.95 + rand_simple() * 0.05;
                self.hermetic_wisdom = 0.90 + rand_simple() * 0.10;
                self.transmutation_power = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.hermetic_wisdom == 0.0 {
            self.hermetic_wisdom = (self.alchemical_mastery + self.transmutation_power) / 2.0 * (0.6 + rand_simple() * 0.3);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alchemical_transmutation() {
        let mut system = AlchemicalArtsSystem::new(AlchemicalArtsTopic::AlchemicalTransmutation);
        system.analyze_system().unwrap();
        assert!(system.alchemical_mastery > 0.8);
    }
}