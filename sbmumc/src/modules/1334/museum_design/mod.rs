//! # SBMUMC Module 1334: Museum Design
//!
//! Systems for museum and exhibition space design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MuseumType {
    ArtMuseum,
    ScienceMuseum,
    HistoryMuseum,
    NaturalHistory,
    Children'sMuseum,
    VirtualMuseum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuseumDesignSystem {
    pub system_id: String,
    pub museum_type: MuseumType,
    pub visitor_experience: f64,
    pub preservation_conditions: f64,
    pub educational_value: f64,
    pub exhibition_flexibility: f64,
}

impl MuseumDesignSystem {
    pub fn new(museum_type: MuseumType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            museum_type,
            visitor_experience: 0.0,
            preservation_conditions: 0.0,
            educational_value: 0.0,
            exhibition_flexibility: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.museum_type {
            MuseumType::ArtMuseum => {
                self.visitor_experience = 0.95 + rand_simple() * 0.05;
                self.exhibition_flexibility = 0.90 + rand_simple() * 0.10;
                self.preservation_conditions = 0.85 + rand_simple() * 0.14;
            },
            MuseumType::ScienceMuseum => {
                self.educational_value = 0.95 + rand_simple() * 0.05;
                self.visitor_experience = 0.90 + rand_simple() * 0.10;
                self.exhibition_flexibility = 0.85 + rand_simple() * 0.14;
            },
            MuseumType::HistoryMuseum => {
                self.educational_value = 0.90 + rand_simple() * 0.10;
                self.preservation_conditions = 0.85 + rand_simple() * 0.14;
                self.visitor_experience = 0.80 + rand_simple() * 0.18;
            },
            MuseumType::NaturalHistory => {
                self.visitor_experience = 0.90 + rand_simple() * 0.10;
                self.educational_value = 0.90 + rand_simple() * 0.10;
                self.preservation_conditions = 0.85 + rand_simple() * 0.14;
            },
            MuseumType::ChildrenMuseum => {
                self.visitor_experience = 0.95 + rand_simple() * 0.05;
                self.educational_value = 0.90 + rand_simple() * 0.10;
                self.exhibition_flexibility = 0.85 + rand_simple() * 0.14;
            },
            MuseumType::VirtualMuseum => {
                self.exhibition_flexibility = 0.95 + rand_simple() * 0.05;
                self.educational_value = 0.90 + rand_simple() * 0.10;
                self.visitor_experience = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.preservation_conditions == 0.0 {
            self.preservation_conditions = (self.visitor_experience + self.exhibition_flexibility) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_science_museum() {
        let mut system = MuseumDesignSystem::new(MuseumType::ScienceMuseum);
        system.analyze_system().unwrap();
        assert!(system.educational_value > 0.8);
    }
}