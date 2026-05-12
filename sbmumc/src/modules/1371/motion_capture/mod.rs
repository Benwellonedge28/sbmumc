//! # SBMUMC Module 1371: Motion Capture
//!
//! Systems for motion capture technology and application.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MotionCaptureApplication {
    FilmPerformance,
    VideoGameCharacter,
    SportsAnalysis,
    MedicalRehabilitation,
    VirtualProduction,
    BiomechanicalResearch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionCaptureSystem {
    pub system_id: String,
    pub mocap_application: MotionCaptureApplication,
    pub capture_accuracy: f64,
    pub animation_quality: f64,
    pub processing_speed: f64,
    pub data_integration: f64,
}

impl MotionCaptureSystem {
    pub fn new(mocap_application: MotionCaptureApplication) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            mocap_application,
            capture_accuracy: 0.0,
            animation_quality: 0.0,
            processing_speed: 0.0,
            data_integration: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.mocap_application {
            MotionCaptureApplication::FilmPerformance => {
                self.capture_accuracy = 0.95 + rand_simple() * 0.05;
                self.animation_quality = 0.90 + rand_simple() * 0.10;
                self.data_integration = 0.85 + rand_simple() * 0.14;
            },
            MotionCaptureApplication::VideoGameCharacter => {
                self.animation_quality = 0.95 + rand_simple() * 0.05;
                self.capture_accuracy = 0.90 + rand_simple() * 0.10;
                self.processing_speed = 0.85 + rand_simple() * 0.14;
            },
            MotionCaptureApplication::SportsAnalysis => {
                self.processing_speed = 0.95 + rand_simple() * 0.05;
                self.capture_accuracy = 0.90 + rand_simple() * 0.10;
                self.data_integration = 0.85 + rand_simple() * 0.14;
            },
            MotionCaptureApplication::MedicalRehabilitation => {
                self.capture_accuracy = 0.95 + rand_simple() * 0.05;
                self.data_integration = 0.90 + rand_simple() * 0.10;
                self.processing_speed = 0.85 + rand_simple() * 0.14;
            },
            MotionCaptureApplication::VirtualProduction => {
                self.processing_speed = 0.95 + rand_simple() * 0.05;
                self.animation_quality = 0.90 + rand_simple() * 0.10;
                self.capture_accuracy = 0.85 + rand_simple() * 0.14;
            },
            MotionCaptureApplication::BiomechanicalResearch => {
                self.capture_accuracy = 0.95 + rand_simple() * 0.05;
                self.data_integration = 0.90 + rand_simple() * 0.10;
                self.processing_speed = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.data_integration == 0.0 {
            self.data_integration = (self.capture_accuracy + self.processing_speed) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_film_performance() {
        let mut system = MotionCaptureSystem::new(MotionCaptureApplication::FilmPerformance);
        system.analyze_system().unwrap();
        assert!(system.capture_accuracy > 0.8);
    }
}