//! # SBMUMC Module 1512: Ancestral Worship
//!
//! Systems for ancestral worship and spirit veneration.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AncestralWorshipTopic {
    AncestorVeneration,
    LineageSpirits,
    FamilySpirits,
    DeceasedHonor,
    SpiritAltars,
    AncestralGuidance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncestralWorshipSystem {
    pub system_id: String,
    pub ancestral_worship_topic: AncestralWorshipTopic,
    pub ancestral_connection: f64,
    pub lineage_wisdom: f64,
    pub spirit_veneration: f64,
    pub ancestral_blessing: f64,
}

impl AncestralWorshipSystem {
    pub fn new(ancestral_worship_topic: AncestralWorshipTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            ancestral_worship_topic,
            ancestral_connection: 0.0,
            lineage_wisdom: 0.0,
            spirit_veneration: 0.0,
            ancestral_blessing: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.ancestral_worship_topic {
            AncestralWorshipTopic::AncestorVeneration => {
                self.ancestral_connection = 0.95 + rand_simple() * 0.05;
                self.lineage_wisdom = 0.90 + rand_simple() * 0.10;
                self.spirit_veneration = 0.85 + rand_simple() * 0.14;
            },
            AncestralWorshipTopic::LineageSpirits => {
                self.ancestral_blessing = 0.95 + rand_simple() * 0.05;
                self.spirit_veneration = 0.90 + rand_simple() * 0.10;
                self.lineage_wisdom = 0.85 + rand_simple() * 0.14;
            },
            AncestralWorshipTopic::FamilySpirits => {
                self.lineage_wisdom = 0.95 + rand_simple() * 0.05;
                self.ancestral_connection = 0.90 + rand_simple() * 0.10;
                self.ancestral_blessing = 0.85 + rand_simple() * 0.14;
            },
            AncestralWorshipTopic::DeceasedHonor => {
                self.spirit_veneration = 0.95 + rand_simple() * 0.05;
                self.ancestral_blessing = 0.90 + rand_simple() * 0.10;
                self.ancestral_connection = 0.85 + rand_simple() * 0.14;
            },
            AncestralWorshipTopic::SpiritAltars => {
                self.ancestral_connection = 0.95 + rand_simple() * 0.05;
                self.lineage_wisdom = 0.90 + rand_simple() * 0.10;
                self.ancestral_blessing = 0.85 + rand_simple() * 0.14;
            },
            AncestralWorshipTopic::AncestralGuidance => {
                self.ancestral_blessing = 0.95 + rand_simple() * 0.05;
                self.spirit_veneration = 0.90 + rand_simple() * 0.10;
                self.lineage_wisdom = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.spirit_veneration == 0.0 {
            self.spirit_veneration = (self.ancestral_connection + self.lineage_wisdom) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_ancestor_veneration() {
        let mut system = AncestralWorshipSystem::new(AncestralWorshipTopic::AncestorVeneration);
        system.analyze_system().unwrap();
        assert!(system.ancestral_connection > 0.8);
    }
}