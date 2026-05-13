//! # SBMUMC Module 1517: Kabbalah Study
//!
//! Systems for Kabbalah study and tree of life.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KabbalahStudyTopic {
    TreeOfLife,
    Sephiroth,
    KabbalisticSymbolism,
    EinSof,
    KabbalisticMysticism,
    Gematria,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KabbalahStudySystem {
    pub system_id: String,
    pub kabbalah_study_topic: KabbalahStudyTopic,
    pub kabbalistic_wisdom: f64,
    pub sephiroth_path: f64,
    pub divine_structure: f64,
    pub mystical_kabbalist: f64,
}

impl KabbalahStudySystem {
    pub fn new(kabbalah_study_topic: KabbalahStudyTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            kabbalah_study_topic,
            kabbalistic_wisdom: 0.0,
            sephiroth_path: 0.0,
            divine_structure: 0.0,
            mystical_kabbalist: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.kabbalah_study_topic {
            KabbalahStudyTopic::TreeOfLife => {
                self.kabbalistic_wisdom = 0.95 + rand_simple() * 0.05;
                self.sephiroth_path = 0.90 + rand_simple() * 0.10;
                self.divine_structure = 0.85 + rand_simple() * 0.14;
            },
            KabbalahStudyTopic::Sephiroth => {
                self.mystical_kabbalist = 0.95 + rand_simple() * 0.05;
                self.divine_structure = 0.90 + rand_simple() * 0.10;
                self.sephiroth_path = 0.85 + rand_simple() * 0.14;
            },
            KabbalahStudyTopic::KabbalisticSymbolism => {
                self.sephiroth_path = 0.95 + rand_simple() * 0.05;
                self.kabbalistic_wisdom = 0.90 + rand_simple() * 0.10;
                self.mystical_kabbalist = 0.85 + rand_simple() * 0.14;
            },
            KabbalahStudyTopic::EinSof => {
                self.divine_structure = 0.95 + rand_simple() * 0.05;
                self.mystical_kabbalist = 0.90 + rand_simple() * 0.10;
                self.kabbalistic_wisdom = 0.85 + rand_simple() * 0.14;
            },
            KabbalahStudyTopic::KabbalisticMysticism => {
                self.kabbalistic_wisdom = 0.95 + rand_simple() * 0.05;
                self.sephiroth_path = 0.90 + rand_simple() * 0.10;
                self.mystical_kabbalist = 0.85 + rand_simple() * 0.14;
            },
            KabbalahStudyTopic::Gematria => {
                self.mystical_kabbalist = 0.95 + rand_simple() * 0.05;
                self.divine_structure = 0.90 + rand_simple() * 0.10;
                self.sephiroth_path = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.divine_structure == 0.0 {
            self.divine_structure = (self.kabbalistic_wisdom + self.sephiroth_path) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_tree_of_life() {
        let mut system = KabbalahStudySystem::new(KabbalahStudyTopic::TreeOfLife);
        system.analyze_system().unwrap();
        assert!(system.kabbalistic_wisdom > 0.8);
    }
}