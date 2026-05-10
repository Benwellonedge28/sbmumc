//! # SBMUMC Module 1326: Prefabrication
//!
//! Systems for prefabricated building construction.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrefabricationMethod {
    Panelized,
    Volumetric,
    Hybrid,
    3DPrinting,
    Prefab MEP,
    SmartPrefabrication,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefabricationSystem {
    pub system_id: String,
    pub prefabrication_method: PrefabricationMethod,
    pub production_efficiency: f64,
    pub site_impact: f64,
    pub quality_consistency: f64,
    pub logistics_complexity: f64,
}

impl PrefabricationSystem {
    pub fn new(prefabrication_method: PrefabricationMethod) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            prefabrication_method,
            production_efficiency: 0.0,
            site_impact: 0.0,
            quality_consistency: 0.0,
            logistics_complexity: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.prefabrication_method {
            PrefabricationMethod::Panelized => {
                self.production_efficiency = 0.90 + rand_simple() * 0.10;
                self.quality_consistency = 0.85 + rand_simple() * 0.14;
                self.site_impact = 0.80 + rand_simple() * 0.18;
            },
            PrefabricationMethod::Volumetric => {
                self.site_impact = 0.95 + rand_simple() * 0.05;
                self.production_efficiency = 0.85 + rand_simple() * 0.14;
                self.quality_consistency = 0.90 + rand_simple() * 0.10;
            },
            PrefabricationMethod::Hybrid => {
                self.production_efficiency = 0.85 + rand_simple() * 0.14;
                self.quality_consistency = 0.80 + rand_simple() * 0.18;
                self.site_impact = 0.85 + rand_simple() * 0.14;
            },
            PrefabricationMethod::3DPrinting => {
                self.quality_consistency = 0.80 + rand_simple() * 0.18;
                self.production_efficiency = 0.85 + rand_simple() * 0.14;
                self.logistics_complexity = 0.90 + rand_simple() * 0.10;
            },
            PrefabricationMethod::PrefabMEP => {
                self.quality_consistency = 0.90 + rand_simple() * 0.10;
                self.site_impact = 0.85 + rand_simple() * 0.14;
                self.production_efficiency = 0.80 + rand_simple() * 0.18;
            },
            PrefabricationMethod::SmartPrefabrication => {
                self.production_efficiency = 0.95 + rand_simple() * 0.05;
                self.quality_consistency = 0.90 + rand_simple() * 0.10;
                self.logistics_complexity = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.logistics_complexity == 0.0 {
            self.logistics_complexity = (1.0 - self.site_impact) * (0.5 + rand_simple() * 0.5);
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
    fn test_volumetric() {
        let mut system = PrefabricationSystem::new(PrefabricationMethod::Volumetric);
        system.analyze_system().unwrap();
        assert!(system.site_impact > 0.8);
    }
}
