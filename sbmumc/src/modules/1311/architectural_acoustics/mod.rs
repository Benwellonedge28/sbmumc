//! # SBMUMC Module 1311: Architectural Acoustics
//!
//! Systems for acoustic design in buildings.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AcousticApplication {
    ConcertHalls,
    RecordingStudios,
    Classrooms,
    OpenOffices,
    theaters,
    Healthcare,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitecturalAcousticsSystem {
    pub system_id: String,
    pub acoustic_application: AcousticApplication,
    pub sound_quality: f64,
    pub noise_control: f64,
    pub speech_clarity: f64,
    pub privacy_level: f64,
}

impl ArchitecturalAcousticsSystem {
    pub fn new(acoustic_application: AcousticApplication) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            acoustic_application,
            sound_quality: 0.0,
            noise_control: 0.0,
            speech_clarity: 0.0,
            privacy_level: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.acoustic_application {
            AcousticApplication::ConcertHalls => {
                self.sound_quality = 0.95 + rand_simple() * 0.05;
                self.noise_control = 0.90 + rand_simple() * 0.10;
                self.speech_clarity = 0.85 + rand_simple() * 0.14;
            },
            AcousticApplication::RecordingStudios => {
                self.sound_quality = 0.95 + rand_simple() * 0.05;
                self.noise_control = 0.95 + rand_simple() * 0.05;
                self.privacy_level = 0.90 + rand_simple() * 0.10;
            },
            AcousticApplication::Classrooms => {
                self.speech_clarity = 0.95 + rand_simple() * 0.05;
                self.noise_control = 0.85 + rand_simple() * 0.14;
                self.sound_quality = 0.80 + rand_simple() * 0.18;
            },
            AcousticApplication::OpenOffices => {
                self.noise_control = 0.80 + rand_simple() * 0.18;
                self.speech_clarity = 0.75 + rand_simple() * 0.22;
                self.privacy_level = 0.60 + rand_simple() * 0.35;
            },
            AcousticApplication::theaters => {
                self.sound_quality = 0.90 + rand_simple() * 0.10;
                self.speech_clarity = 0.85 + rand_simple() * 0.14;
                self.noise_control = 0.90 + rand_simple() * 0.10;
            },
            AcousticApplication::Healthcare => {
                self.noise_control = 0.90 + rand_simple() * 0.10;
                self.privacy_level = 0.85 + rand_simple() * 0.14;
                self.speech_clarity = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.privacy_level == 0.0 {
            self.privacy_level = (self.noise_control + self.sound_quality) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_recording_studios() {
        let mut system = ArchitecturalAcousticsSystem::new(AcousticApplication::RecordingStudios);
        system.analyze_system().unwrap();
        assert!(system.sound_quality > 0.8);
    }
}
