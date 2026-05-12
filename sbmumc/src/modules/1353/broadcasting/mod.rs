//! # SBMUMC Module 1353: Broadcasting
//!
//! Systems for broadcast media production.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BroadcastType {
    Television,
    Radio,
    NewsBroadcast,
    SportsBroadcast,
    LiveEvent,
    OnDemand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastingSystem {
    pub system_id: String,
    pub broadcast_type: BroadcastType,
    pub production_quality: f64,
    pub audience_reach: f64,
    pub technical_reliability: f64,
    pub content_engagement: f64,
}

impl BroadcastingSystem {
    pub fn new(broadcast_type: BroadcastType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            broadcast_type,
            production_quality: 0.0,
            audience_reach: 0.0,
            technical_reliability: 0.0,
            content_engagement: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.broadcast_type {
            BroadcastType::Television => {
                self.production_quality = 0.95 + rand_simple() * 0.05;
                self.audience_reach = 0.90 + rand_simple() * 0.10;
                self.content_engagement = 0.85 + rand_simple() * 0.14;
            },
            BroadcastType::Radio => {
                self.content_engagement = 0.95 + rand_simple() * 0.05;
                self.audience_reach = 0.90 + rand_simple() * 0.10;
                self.technical_reliability = 0.85 + rand_simple() * 0.14;
            },
            BroadcastType::NewsBroadcast => {
                self.technical_reliability = 0.95 + rand_simple() * 0.05;
                self.audience_reach = 0.90 + rand_simple() * 0.10;
                self.production_quality = 0.85 + rand_simple() * 0.14;
            },
            BroadcastType::SportsBroadcast => {
                self.production_quality = 0.95 + rand_simple() * 0.05;
                self.technical_reliability = 0.90 + rand_simple() * 0.10;
                self.audience_reach = 0.85 + rand_simple() * 0.14;
            },
            BroadcastType::LiveEvent => {
                self.technical_reliability = 0.95 + rand_simple() * 0.05;
                self.production_quality = 0.90 + rand_simple() * 0.10;
                self.content_engagement = 0.85 + rand_simple() * 0.14;
            },
            BroadcastType::OnDemand => {
                self.audience_reach = 0.95 + rand_simple() * 0.05;
                self.content_engagement = 0.90 + rand_simple() * 0.10;
                self.production_quality = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.content_engagement == 0.0 {
            self.content_engagement = (self.production_quality + self.audience_reach) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_live_event() {
        let mut system = BroadcastingSystem::new(BroadcastType::LiveEvent);
        system.analyze_system().unwrap();
        assert!(system.technical_reliability > 0.8);
    }
}