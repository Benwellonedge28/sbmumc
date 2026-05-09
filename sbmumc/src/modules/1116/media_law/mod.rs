//! # SBMUMC Module 1116: Media Law
//!
//! Freedom of press, broadcasting regulation, and content law.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaRegulatoryModel {
    SelfRegulation,
    StatutoryBody,
    GovernmentControl,
    IndependentAuthority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaLawSystem {
    pub system_id: String,
    pub regulatory_model: MediaRegulatoryModel,
    pub press_freedom_index: f64,
    var pluralism_score: f64,
    pub content_diversity: f64,
    pub journalist_protection: f64,
}

impl MediaLawSystem {
    pub fn new(model: MediaRegulatoryModel) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            regulatory_model: model,
            press_freedom_index: 0.0,
            var pluralism_score: 0.0,
            self.content_diversity = 0.0,
            self.journalist_protection = 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.regulatory_model {
            MediaRegulatoryModel::IndependentAuthority => {
                self.press_freedom_index = 0.85 + rand_simple() * 0.15;
                self.pluralism_score = 0.80 + rand_simple() * 0.18;
            },
            MediaRegulatoryModel::SelfRegulation => {
                self.press_freedom_index = 0.75 + rand_simple() * 0.20;
                self.pluralism_score = 0.65 + rand_simple() * 0.30;
            },
            MediaRegulatoryModel::GovernmentControl => {
                self.press_freedom_index = 0.30 + rand_simple() * 0.40;
                self.pluralism_score = 0.35 + rand_simple() * 0.35;
            },
            _ => {
                self.press_freedom_index = 0.50 + rand_simple() * 0.40;
                self.pluralism_score = 0.50 + rand_simple() * 0.35;
            }
        }

        self.content_diversity = self.pluralism_score * (0.8 + rand_simple() * 0.2);
        self.journalist_protection = self.press_freedom_index * (0.7 + rand_simple() * 0.3);
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
    fn test_independent_authority() {
        let mut system = MediaLawSystem::new(MediaRegulatoryModel::IndependentAuthority);
        system.analyze_system().unwrap();
        assert!(system.press_freedom_index > 0.7);
    }
}