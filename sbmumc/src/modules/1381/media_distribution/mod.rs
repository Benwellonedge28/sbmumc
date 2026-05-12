//! # SBMUMC Module 1381: Media Distribution
//!
//! Systems for media content distribution.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributionChannel {
    Theatrical,
    Streaming,
    Television,
    HomeVideo,
    Syndication,
    DigitalTheatrical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDistributionSystem {
    pub system_id: String,
    pub distribution_channel: DistributionChannel,
    pub market_reach: f64,
    pub revenue_optimization: f64,
    pub release_strategy: f64,
    pub platform_relationships: f64,
}

impl MediaDistributionSystem {
    pub fn new(distribution_channel: DistributionChannel) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            distribution_channel,
            market_reach: 0.0,
            revenue_optimization: 0.0,
            release_strategy: 0.0,
            platform_relationships: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.distribution_channel {
            DistributionChannel::Theatrical => {
                self.market_reach = 0.95 + rand_simple() * 0.05;
                self.release_strategy = 0.90 + rand_simple() * 0.10;
                self.revenue_optimization = 0.85 + rand_simple() * 0.14;
            },
            DistributionChannel::Streaming => {
                self.platform_relationships = 0.95 + rand_simple() * 0.05;
                self.revenue_optimization = 0.90 + rand_simple() * 0.10;
                self.market_reach = 0.85 + rand_simple() * 0.14;
            },
            DistributionChannel::Television => {
                self.market_reach = 0.95 + rand_simple() * 0.05;
                self.platform_relationships = 0.90 + rand_simple() * 0.10;
                self.revenue_optimization = 0.85 + rand_simple() * 0.14;
            },
            DistributionChannel::HomeVideo => {
                self.revenue_optimization = 0.95 + rand_simple() * 0.05;
                self.market_reach = 0.85 + rand_simple() * 0.14;
                self.release_strategy = 0.90 + rand_simple() * 0.10;
            },
            DistributionChannel::Syndication => {
                self.revenue_optimization = 0.95 + rand_simple() * 0.05;
                self.market_reach = 0.90 + rand_simple() * 0.10;
                self.platform_relationships = 0.85 + rand_simple() * 0.14;
            },
            DistributionChannel::DigitalTheatrical => {
                self.release_strategy = 0.95 + rand_simple() * 0.05;
                self.platform_relationships = 0.90 + rand_simple() * 0.10;
                self.revenue_optimization = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.release_strategy == 0.0 {
            self.release_strategy = (self.market_reach + self.revenue_optimization) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_streaming() {
        let mut system = MediaDistributionSystem::new(DistributionChannel::Streaming);
        system.analyze_system().unwrap();
        assert!(system.platform_relationships > 0.8);
    }
}