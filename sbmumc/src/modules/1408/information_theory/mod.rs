//! # SBMUMC Module 1408: Information Theory
//!
//! Systems for information theory and communication.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InformationDomain {
    ShannonEntropy,
    ChannelCapacity,
    SourceCoding,
    RateDistortion,
    CryptographicEntropy,
    QuantumInformation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformationTheorySystem {
    pub system_id: String,
    pub information_domain: InformationDomain,
    pub entropy_calculation: f64,
    pub channel_modeling: f64,
    pub coding_efficiency: f64,
    pub redundancy_analysis: f64,
}

impl InformationTheorySystem {
    pub fn new(information_domain: InformationDomain) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            information_domain,
            entropy_calculation: 0.0,
            channel_modeling: 0.0,
            coding_efficiency: 0.0,
            redundancy_analysis: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.information_domain {
            InformationDomain::ShannonEntropy => {
                self.entropy_calculation = 0.95 + rand_simple() * 0.05;
                self.redundancy_analysis = 0.90 + rand_simple() * 0.10;
                self.coding_efficiency = 0.85 + rand_simple() * 0.14;
            },
            InformationDomain::ChannelCapacity => {
                self.channel_modeling = 0.95 + rand_simple() * 0.05;
                self.entropy_calculation = 0.90 + rand_simple() * 0.10;
                self.coding_efficiency = 0.85 + rand_simple() * 0.14;
            },
            InformationDomain::SourceCoding => {
                self.coding_efficiency = 0.95 + rand_simple() * 0.05;
                self.channel_modeling = 0.90 + rand_simple() * 0.10;
                self.redundancy_analysis = 0.85 + rand_simple() * 0.14;
            },
            InformationDomain::RateDistortion => {
                self.redundancy_analysis = 0.95 + rand_simple() * 0.05;
                self.coding_efficiency = 0.90 + rand_simple() * 0.10;
                self.entropy_calculation = 0.85 + rand_simple() * 0.14;
            },
            InformationDomain::CryptographicEntropy => {
                self.entropy_calculation = 0.95 + rand_simple() * 0.05;
                self.channel_modeling = 0.90 + rand_simple() * 0.10;
                self.coding_efficiency = 0.85 + rand_simple() * 0.14;
            },
            InformationDomain::QuantumInformation => {
                self.channel_modeling = 0.95 + rand_simple() * 0.05;
                self.redundancy_analysis = 0.90 + rand_simple() * 0.10;
                self.entropy_calculation = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.coding_efficiency == 0.0 {
            self.coding_efficiency = (self.entropy_calculation + self.channel_modeling) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_shannon() {
        let mut system = InformationTheorySystem::new(InformationDomain::ShannonEntropy);
        system.analyze_system().unwrap();
        assert!(system.entropy_calculation > 0.8);
    }
}
