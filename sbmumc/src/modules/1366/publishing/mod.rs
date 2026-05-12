//! # SBMUMC Module 1366: Publishing
//!
//! Systems for publishing industry operations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublishingFormat {
    Books,
    Magazines,
    Newspapers,
    DigitalPublishing,
    AcademicPublishing,
    SelfPublishing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishingSystem {
    pub system_id: String,
    pub publishing_format: PublishingFormat,
    pub editorial_excellence: f64,
    pub distribution_reach: f64,
    pub reader_engagement: f64,
    pub content_diversity: f64,
}

impl PublishingSystem {
    pub fn new(publishing_format: PublishingFormat) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            publishing_format,
            editorial_excellence: 0.0,
            distribution_reach: 0.0,
            reader_engagement: 0.0,
            content_diversity: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.publishing_format {
            PublishingFormat::Books => {
                self.editorial_excellence = 0.95 + rand_simple() * 0.05;
                self.reader_engagement = 0.90 + rand_simple() * 0.10;
                self.distribution_reach = 0.85 + rand_simple() * 0.14;
            },
            PublishingFormat::Magazines => {
                self.reader_engagement = 0.95 + rand_simple() * 0.05;
                self.editorial_excellence = 0.90 + rand_simple() * 0.10;
                self.content_diversity = 0.85 + rand_simple() * 0.14;
            },
            PublishingFormat::Newspapers => {
                self.distribution_reach = 0.95 + rand_simple() * 0.05;
                self.editorial_excellence = 0.90 + rand_simple() * 0.10;
                self.reader_engagement = 0.80 + rand_simple() * 0.18;
            },
            PublishingFormat::DigitalPublishing => {
                self.distribution_reach = 0.95 + rand_simple() * 0.05;
                self.reader_engagement = 0.90 + rand_simple() * 0.10;
                self.content_diversity = 0.85 + rand_simple() * 0.14;
            },
            PublishingFormat::AcademicPublishing => {
                self.editorial_excellence = 0.95 + rand_simple() * 0.05;
                self.distribution_reach = 0.90 + rand_simple() * 0.10;
                self.content_diversity = 0.85 + rand_simple() * 0.14;
            },
            PublishingFormat::SelfPublishing => {
                self.content_diversity = 0.95 + rand_simple() * 0.05;
                self.distribution_reach = 0.90 + rand_simple() * 0.10;
                self.reader_engagement = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.content_diversity == 0.0 {
            self.content_diversity = (self.editorial_excellence + self.distribution_reach) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_books() {
        let mut system = PublishingSystem::new(PublishingFormat::Books);
        system.analyze_system().unwrap();
        assert!(system.editorial_excellence > 0.8);
    }
}