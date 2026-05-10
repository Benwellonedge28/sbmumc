//! # SBMUMC Module 1274: Ocean Energy
//!
//! Systems for harnessing energy from ocean resources.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OceanEnergySource {
    TidalEnergy,
    WaveEnergy,
    OceanThermal,
    OffshoreWind,
    MarineBiomass,
    CurrentEnergy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OceanEnergySystem {
    pub system_id: String,
    pub energy_source: OceanEnergySource,
    pub energy_output: f64,
    pub conversion_efficiency: f64,
    pub environmental_friendly: f64,
    pub cost_effectiveness: f64,
}

impl OceanEnergySystem {
    pub fn new(energy_source: OceanEnergySource) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            energy_source,
            energy_output: 0.0,
            conversion_efficiency: 0.0,
            environmental_friendly: 0.0,
            cost_effectiveness: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.energy_source {
            OceanEnergySource::TidalEnergy => {
                self.conversion_efficiency = 0.80 + rand_simple() * 0.18;
                self.energy_output = 0.70 + rand_simple() * 0.25;
                self.environmental_friendly = 0.85 + rand_simple() * 0.14;
            },
            OceanEnergySource::WaveEnergy => {
                self.energy_output = 0.65 + rand_simple() * 0.30;
                self.conversion_efficiency = 0.60 + rand_simple() * 0.35;
                self.environmental_friendly = 0.90 + rand_simple() * 0.10;
            },
            OceanEnergySource::OceanThermal => {
                self.conversion_efficiency = 0.55 + rand_simple() * 0.40;
                self.energy_output = 0.60 + rand_simple() * 0.35;
                self.cost_effectiveness = 0.50 + rand_simple() * 0.40;
            },
            OceanEnergySource::OffshoreWind => {
                self.energy_output = 0.85 + rand_simple() * 0.14;
                self.conversion_efficiency = 0.75 + rand_simple() * 0.22;
                self.cost_effectiveness = 0.70 + rand_simple() * 0.25;
            },
            OceanEnergySource::MarineBiomass => {
                self.environmental_friendly = 0.80 + rand_simple() * 0.18;
                self.energy_output = 0.55 + rand_simple() * 0.40;
                self.cost_effectiveness = 0.60 + rand_simple() * 0.35;
            },
            OceanEnergySource::CurrentEnergy => {
                self.conversion_efficiency = 0.70 + rand_simple() * 0.25;
                self.energy_output = 0.65 + rand_simple() * 0.30;
                self.environmental_friendly = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.cost_effectiveness == 0.0 {
            self.cost_effectiveness = (self.energy_output + self.conversion_efficiency) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_offshore_wind() {
        let mut system = OceanEnergySystem::new(OceanEnergySource::OffshoreWind);
        system.analyze_system().unwrap();
        assert!(system.energy_output > 0.7);
    }
}
