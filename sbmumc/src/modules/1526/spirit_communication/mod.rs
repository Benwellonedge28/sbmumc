//! # SBMUMC Module 1526: Spirit Communication
//!
//! Systems for spirit communication and mediumship.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpiritCommunicationTopic {
    MediumChannel,
    SpiritContact,
    Necromancy,
    GhostSummoning,
    SpiritSpeaking,
    MediumisticGift,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritCommunicationSystem {
    pub system_id: String,
    pub spirit_communication_topic: SpiritCommunicationTopic,
    pub mediumship: f64,
    pub spirit_contact: f64,
    pub spectral_communication: f64,
    pub other_side_connection: f64,
}

impl SpiritCommunicationSystem {
    pub fn new(spirit_communication_topic: SpiritCommunicationTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            spirit_communication_topic,
            mediumship: 0.0,
            spirit_contact: 0.0,
            spectral_communication: 0.0,
            other_side_connection: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.spirit_communication_topic {
            SpiritCommunicationTopic::MediumChannel => {
                self.mediumship = 0.95 + rand_simple() * 0.05;
                self.spirit_contact = 0.90 + rand_simple() * 0.10;
                self.spectral_communication = 0.85 + rand_simple() * 0.14;
            },
            SpiritCommunicationTopic::SpiritContact => {
                self.other_side_connection = 0.95 + rand_simple() * 0.05;
                self.spectral_communication = 0.90 + rand_simple() * 0.10;
                self.spirit_contact = 0.85 + rand_simple() * 0.14;
            },
            SpiritCommunicationTopic::Necromancy => {
                self.spirit_contact = 0.95 + rand_simple() * 0.05;
                self.mediumship = 0.90 + rand_simple() * 0.10;
                self.other_side_connection = 0.85 + rand_simple() * 0.14;
            },
            SpiritCommunicationTopic::GhostSummoning => {
                self.spectral_communication = 0.95 + rand_simple() * 0.05;
                self.other_side_connection = 0.90 + rand_simple() * 0.10;
                self.mediumship = 0.85 + rand_simple() * 0.14;
            },
            SpiritCommunicationTopic::SpiritSpeaking => {
                self.mediumship = 0.95 + rand_simple() * 0.05;
                self.spirit_contact = 0.90 + rand_simple() * 0.10;
                self.other_side_connection = 0.85 + rand_simple() * 0.14;
            },
            SpiritCommunicationTopic::MediumisticGift => {
                self.other_side_connection = 0.95 + rand_simple() * 0.05;
                self.spectral_communication = 0.90 + rand_simple() * 0.10;
                self.spirit_contact = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.spectral_communication == 0.0 {
            self.spectral_communication = (self.mediumship + self.spirit_contact) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_medium_channel() {
        let mut system = SpiritCommunicationSystem::new(SpiritCommunicationTopic::MediumChannel);
        system.analyze_system().unwrap();
        assert!(system.mediumship > 0.8);
    }
}