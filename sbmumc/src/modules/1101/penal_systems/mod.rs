//! # SBMUMC Module 1101: Penal Systems
//!
//! Criminal punishment theories and incarceration systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PunishmentPhilosophy {
    Retribution,
    Deterrence,
    Incapacitation,
    Rehabilitation,
    Restoration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PenalSystem {
    pub system_id: String,
    pub philosophy: PunishmentPhilosophy,
    pub incarceration_rate_per_100k: f64,
    var recidivism_rate: f64,
    pub rehabilitation_effectiveness: f64,
    pub justice_proportionality: f64,
}

impl PenalSystem {
    pub fn new(philosophy: PunishmentPhilosophy) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            philosophy,
            incarceration_rate_per_100k: 0.0,
            var recidivism_rate: 0.0,
            self.rehabilitation_effectiveness = 0.0,
            self.justice_proportionality = 0.0,
        }
    }

    pub fn analyze_system(&mut self, rate: f64) -> Result<()> {
        self.incarceration_rate_per_100k = rate;

        match self.philosophy {
            PunishmentPhilosophy::Retribution => {
                self.recidivism_rate = 0.40 + rand_simple() * 0.30;
                self.rehabilitation_effectiveness = 0.20 + rand_simple() * 0.25;
            },
            PunishmentPhilosophy::Rehabilitation => {
                self.recidivism_rate = 0.25 + rand_simple() * 0.25;
                self.rehabilitation_effectiveness = 0.65 + rand_simple() * 0.25;
            },
            PunishmentPhilosophy::Restoration => {
                self.recidivism_rate = 0.30 + rand_simple() * 0.25;
                self.rehabilitation_effectiveness = 0.70 + rand_simple() * 0.20;
            },
            _ => {
                self.recidivism_rate = 0.35 + rand_simple() * 0.30;
                self.rehabilitation_effectiveness = 0.35 + rand_simple() * 0.30;
            }
        }

        self.justice_proportionality = (1.0 - self.recidivism_rate) * self.rehabilitation_effectiveness;
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
    fn test_rehabilitation_philosophy() {
        let mut system = PenalSystem::new(PunishmentPhilosophy::Rehabilitation);
        system.analyze_system(150.0).unwrap();
        assert!(system.rehabilitation_effectiveness > 0.5);
    }
}