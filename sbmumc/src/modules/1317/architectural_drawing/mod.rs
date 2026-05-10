//! # SBMUMC Module 1317: Architectural Drawing
//!
//! Systems for architectural visualization and documentation.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DrawingType {
    FloorPlans,
    Elevations,
    Sections,
    Details,
    axonometric,
    Perspectives,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitecturalDrawingSystem {
    pub system_id: String,
    pub drawing_type: DrawingType,
    pub drawing_accuracy: f64,
    pub visualization_quality: f64,
    pub documentation_completeness: f64,
    pub communication_clarity: f64,
}

impl ArchitecturalDrawingSystem {
    pub fn new(drawing_type: DrawingType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            drawing_type,
            drawing_accuracy: 0.0,
            visualization_quality: 0.0,
            documentation_completeness: 0.0,
            communication_clarity: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.drawing_type {
            DrawingType::FloorPlans => {
                self.drawing_accuracy = 0.95 + rand_simple() * 0.05;
                self.documentation_completeness = 0.90 + rand_simple() * 0.10;
                self.communication_clarity = 0.85 + rand_simple() * 0.14;
            },
            DrawingType::Elevations => {
                self.visualization_quality = 0.90 + rand_simple() * 0.10;
                self.drawing_accuracy = 0.85 + rand_simple() * 0.14;
                self.communication_clarity = 0.90 + rand_simple() * 0.10;
            },
            DrawingType::Sections => {
                self.drawing_accuracy = 0.90 + rand_simple() * 0.10;
                self.documentation_completeness = 0.85 + rand_simple() * 0.14;
                self.communication_clarity = 0.85 + rand_simple() * 0.14;
            },
            DrawingType::Details => {
                self.drawing_accuracy = 0.95 + rand_simple() * 0.05;
                self.documentation_completeness = 0.95 + rand_simple() * 0.05;
                self.communication_clarity = 0.90 + rand_simple() * 0.10;
            },
            DrawingType::axonometric => {
                self.visualization_quality = 0.95 + rand_simple() * 0.05;
                self.communication_clarity = 0.85 + rand_simple() * 0.14;
                self.drawing_accuracy = 0.80 + rand_simple() * 0.18;
            },
            DrawingType::Perspectives => {
                self.visualization_quality = 0.95 + rand_simple() * 0.05;
                self.communication_clarity = 0.90 + rand_simple() * 0.10;
                self.drawing_accuracy = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.communication_clarity == 0.0 {
            self.communication_clarity = (self.drawing_accuracy + self.visualization_quality) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_details() {
        let mut system = ArchitecturalDrawingSystem::new(DrawingType::Details);
        system.analyze_system().unwrap();
        assert!(system.drawing_accuracy > 0.8);
    }
}
