//! # SBMUMC Module 1280: Ocean Chemistry
//!
//! Systems for studying ocean chemical processes.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OceanChemicalProcess {
    Salinity,
    NutrientCycles,
    CarbonateChemistry,
    TraceMetals,
    DissolvedGases,
    RedoxProcesses,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OceanChemistrySystem {
    pub system_id: String,
    pub chemical_process: OceanChemicalProcess,
    pub chemical_balance: f64,
    pub cycling_rate: f64,
    pub analysis_precision: f64,
    pub ecosystem_interaction: f64,
}

impl OceanChemistrySystem {
    pub fn new(chemical_process: OceanChemicalProcess) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            chemical_process,
            chemical_balance: 0.0,
            cycling_rate: 0.0,
            analysis_precision: 0.0,
            ecosystem_interaction: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.chemical_process {
            OceanChemicalProcess::Salinity => {
                self.chemical_balance = 0.90 + rand_simple() * 0.10;
                self.analysis_precision = 0.95 + rand_simple() * 0.05;
                self.cycling_rate = 0.60 + rand_simple() * 0.35;
            },
            OceanChemicalProcess::NutrientCycles => {
                self.cycling_rate = 0.85 + rand_simple() * 0.14;
                self.ecosystem_interaction = 0.90 + rand_simple() * 0.10;
                self.chemical_balance = 0.75 + rand_simple() * 0.22;
            },
            OceanChemicalProcess::CarbonateChemistry => {
                self.chemical_balance = 0.80 + rand_simple() * 0.18;
                self.analysis_precision = 0.85 + rand_simple() * 0.14;
                self.ecosystem_interaction = 0.85 + rand_simple() * 0.14;
            },
            OceanChemicalProcess::TraceMetals => {
                self.analysis_precision = 0.70 + rand_simple() * 0.25;
                self.ecosystem_interaction = 0.75 + rand_simple() * 0.22;
                self.cycling_rate = 0.50 + rand_simple() * 0.40;
            },
            OceanChemicalProcess::DissolvedGases => {
                self.cycling_rate = 0.80 + rand_simple() * 0.18;
                self.chemical_balance = 0.85 + rand_simple() * 0.14;
                self.ecosystem_interaction = 0.80 + rand_simple() * 0.18;
            },
            OceanChemicalProcess::RedoxProcesses => {
                self.ecosystem_interaction = 0.75 + rand_simple() * 0.22;
                self.cycling_rate = 0.70 + rand_simple() * 0.25;
                self.analysis_precision = 0.65 + rand_simple() * 0.30;
            },
        }

        if self.ecosystem_interaction == 0.0 {
            self.ecosystem_interaction = (self.chemical_balance + self.cycling_rate) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_nutrient_cycles() {
        let mut system = OceanChemistrySystem::new(OceanChemicalProcess::NutrientCycles);
        system.analyze_system().unwrap();
        assert!(system.cycling_rate > 0.7);
    }
}
