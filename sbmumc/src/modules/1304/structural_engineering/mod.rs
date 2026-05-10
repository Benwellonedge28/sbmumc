//! # SBMUMC Module 1304: Structural Engineering
//!
//! Systems for structural design and load analysis.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructuralSystem {
    SteelFrame,
    ConcreteFrame,
    TimberConstruction,
    Masonry,
    TensileStructure,
    CompositeSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralEngineeringSystem {
    pub system_id: String,
    pub structural_system: StructuralSystem,
    pub load_capacity: f64,
    pub seismic_resistance: f64,
    pub construction_efficiency: f64,
    pub material_cost: f64,
}

impl StructuralEngineeringSystem {
    pub fn new(structural_system: StructuralSystem) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            structural_system,
            load_capacity: 0.0,
            seismic_resistance: 0.0,
            construction_efficiency: 0.0,
            material_cost: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.structural_system {
            StructuralSystem::SteelFrame => {
                self.load_capacity = 0.95 + rand_simple() * 0.05;
                self.seismic_resistance = 0.90 + rand_simple() * 0.10;
                self.construction_efficiency = 0.80 + rand_simple() * 0.18;
            },
            StructuralSystem::ConcreteFrame => {
                self.load_capacity = 0.90 + rand_simple() * 0.10;
                self.seismic_resistance = 0.85 + rand_simple() * 0.14;
                self.material_cost = 0.75 + rand_simple() * 0.22;
            },
            StructuralSystem::TimberConstruction => {
                self.construction_efficiency = 0.90 + rand_simple() * 0.10;
                self.seismic_resistance = 0.85 + rand_simple() * 0.14;
                self.load_capacity = 0.70 + rand_simple() * 0.25;
            },
            StructuralSystem::Masonry => {
                self.load_capacity = 0.85 + rand_simple() * 0.14;
                self.material_cost = 0.90 + rand_simple() * 0.10;
                self.seismic_resistance = 0.55 + rand_simple() * 0.40;
            },
            StructuralSystem::TensileStructure => {
                self.load_capacity = 0.80 + rand_simple() * 0.18;
                self.construction_efficiency = 0.85 + rand_simple() * 0.14;
                self.seismic_resistance = 0.90 + rand_simple() * 0.10;
            },
            StructuralSystem::CompositeSystem => {
                self.load_capacity = 0.90 + rand_simple() * 0.10;
                self.seismic_resistance = 0.90 + rand_simple() * 0.10;
                self.construction_efficiency = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.material_cost == 0.0 {
            self.material_cost = (1.0 - self.construction_efficiency) * (0.5 + rand_simple() * 0.5);
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
    fn test_steel_frame() {
        let mut system = StructuralEngineeringSystem::new(StructuralSystem::SteelFrame);
        system.analyze_system().unwrap();
        assert!(system.load_capacity > 0.8);
    }
}
