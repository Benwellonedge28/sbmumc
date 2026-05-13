//! # SBMUMC Module 1510: Nature Spirits
//!
//! Systems for nature spirits and plant consciousness.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NatureSpiritsTopic {
    TreeSpirits,
    PlantConsciousness,
    NatureDevas,
    ForestSpirits,
    GardenSpirits,
    EarthElementals,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatureSpiritsSystem {
    pub system_id: String,
    pub nature_spirits_topic: NatureSpiritsTopic,
    pub nature_connection: f64,
    pub plant_communication: f64,
    pub spirit_nature: f64,
    pub organic_living: f64,
}

impl NatureSpiritsSystem {
    pub fn new(nature_spirits_topic: NatureSpiritsTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            nature_spirits_topic,
            nature_connection: 0.0,
            plant_communication: 0.0,
            spirit_nature: 0.0,
            organic_living: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.nature_spirits_topic {
            NatureSpiritsTopic::TreeSpirits => {
                self.nature_connection = 0.95 + rand_simple() * 0.05;
                self.plant_communication = 0.90 + rand_simple() * 0.10;
                self.spirit_nature = 0.85 + rand_simple() * 0.14;
            },
            NatureSpiritsTopic::PlantConsciousness => {
                self.organic_living = 0.95 + rand_simple() * 0.05;
                self.spirit_nature = 0.90 + rand_simple() * 0.10;
                self.plant_communication = 0.85 + rand_simple() * 0.14;
            },
            NatureSpiritsTopic::NatureDevas => {
                self.plant_communication = 0.95 + rand_simple() * 0.05;
                self.nature_connection = 0.90 + rand_simple() * 0.10;
                self.organic_living = 0.85 + rand_simple() * 0.14;
            },
            NatureSpiritsTopic::ForestSpirits => {
                self.spirit_nature = 0.95 + rand_simple() * 0.05;
                self.organic_living = 0.90 + rand_simple() * 0.10;
                self.nature_connection = 0.85 + rand_simple() * 0.14;
            },
            NatureSpiritsTopic::GardenSpirits => {
                self.nature_connection = 0.95 + rand_simple() * 0.05;
                self.plant_communication = 0.90 + rand_simple() * 0.10;
                self.organic_living = 0.85 + rand_simple() * 0.14;
            },
            NatureSpiritsTopic::EarthElementals => {
                self.organic_living = 0.95 + rand_simple() * 0.05;
                self.spirit_nature = 0.90 + rand_simple() * 0.10;
                self.plant_communication = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.spirit_nature == 0.0 {
            self.spirit_nature = (self.nature_connection + self.plant_communication) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_tree_spirits() {
        let mut system = NatureSpiritsSystem::new(NatureSpiritsTopic::TreeSpirits);
        system.analyze_system().unwrap();
        assert!(system.nature_connection > 0.8);
    }
}