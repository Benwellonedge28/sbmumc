//! # SBMUMC Module 1490: Pragmatist Tradition
//!
//! Systems for pragmatist tradition and pragmatic philosophy.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PragmatistTraditionTopic {
    ClassicalPragmatism,
    Instrumentalism,
    TruthPragmatism,
    ExperimentalInquiry,
    PragmaticMaxim,
    CommunityInquiry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PragmatistTraditionSystem {
    pub system_id: String,
    pub pragmatist_tradition_topic: PragmatistTraditionTopic,
    pub practical_consequences: f64,
    pub inquiry_method: f64,
    pub truth_utility: f64,
    pub community_practices: f64,
}

impl PragmatistTraditionSystem {
    pub fn new(pragmatist_tradition_topic: PragmatistTraditionTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            pragmatist_tradition_topic,
            practical_consequences: 0.0,
            inquiry_method: 0.0,
            truth_utility: 0.0,
            community_practices: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.pragmatist_tradition_topic {
            PragmatistTraditionTopic::ClassicalPragmatism => {
                self.practical_consequences = 0.95 + rand_simple() * 0.05;
                self.inquiry_method = 0.90 + rand_simple() * 0.10;
                self.truth_utility = 0.85 + rand_simple() * 0.14;
            },
            PragmatistTraditionTopic::Instrumentalism => {
                self.community_practices = 0.95 + rand_simple() * 0.05;
                self.practical_consequences = 0.90 + rand_simple() * 0.10;
                self.inquiry_method = 0.85 + rand_simple() * 0.14;
            },
            PragmatistTraditionTopic::TruthPragmatism => {
                self.truth_utility = 0.95 + rand_simple() * 0.05;
                self.community_practices = 0.90 + rand_simple() * 0.10;
                self.practical_consequences = 0.85 + rand_simple() * 0.14;
            },
            PragmatistTraditionTopic::ExperimentalInquiry => {
                self.inquiry_method = 0.95 + rand_simple() * 0.05;
                self.truth_utility = 0.90 + rand_simple() * 0.10;
                self.community_practices = 0.85 + rand_simple() * 0.14;
            },
            PragmatistTraditionTopic::PragmaticMaxim => {
                self.practical_consequences = 0.95 + rand_simple() * 0.05;
                self.inquiry_method = 0.90 + rand_simple() * 0.10;
                self.community_practices = 0.85 + rand_simple() * 0.14;
            },
            PragmatistTraditionTopic::CommunityInquiry => {
                self.truth_utility = 0.95 + rand_simple() * 0.05;
                self.practical_consequences = 0.90 + rand_simple() * 0.10;
                self.inquiry_method = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.truth_utility == 0.0 {
            self.truth_utility = (self.practical_consequences + self.inquiry_method) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_classical() {
        let mut system = PragmatistTraditionSystem::new(PragmatistTraditionTopic::ClassicalPragmatism);
        system.analyze_system().unwrap();
        assert!(system.practical_consequences > 0.8);
    }
}