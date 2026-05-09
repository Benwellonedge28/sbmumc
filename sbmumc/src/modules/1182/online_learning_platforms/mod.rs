//! # SBMUMC Module 1182: Online Learning Platforms
//!
//! Digital platforms for delivering educational content online.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OnlinePlatformType {
    MOOC,
    LMS,
    VideoBased,
    Interactive,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineLearningPlatformSystem {
    pub system_id: String,
    pub platform_type: OnlinePlatformType,
    pub user_experience: f64,
    pub content_quality: f64,
    pub engagement_mechanisms: f64,
    pub completion_support: f64,
}

impl OnlineLearningPlatformSystem {
    pub fn new(platform_type: OnlinePlatformType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            platform_type,
            user_experience: 0.0,
            content_quality: 0.0,
            engagement_mechanisms: 0.0,
            completion_support: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.platform_type {
            OnlinePlatformType::MOOC => {
                self.user_experience = 0.70 + rand_simple() * 0.25;
                self.content_quality = 0.85 + rand_simple() * 0.14;
                self.engagement_mechanisms = 0.55 + rand_simple() * 0.40;
                self.completion_support = 0.50 + rand_simple() * 0.40;
            },
            OnlinePlatformType::LMS => {
                self.user_experience = 0.75 + rand_simple() * 0.22;
                self.content_quality = 0.70 + rand_simple() * 0.25;
                self.completion_support = 0.80 + rand_simple() * 0.18;
            },
            OnlinePlatformType::VideoBased => {
                self.user_experience = 0.80 + rand_simple() * 0.18;
                self.content_quality = 0.75 + rand_simple() * 0.22;
                self.engagement_mechanisms = 0.60 + rand_simple() * 0.35;
            },
            OnlinePlatformType::Interactive => {
                self.user_experience = 0.75 + rand_simple() * 0.22;
                self.engagement_mechanisms = 0.85 + rand_simple() * 0.14;
                self.completion_support = 0.70 + rand_simple() * 0.25;
            },
            OnlinePlatformType::Hybrid => {
                self.user_experience = 0.70 + rand_simple() * 0.25;
                self.content_quality = 0.75 + rand_simple() * 0.22;
                self.engagement_mechanisms = 0.70 + rand_simple() * 0.25;
                self.completion_support = 0.65 + rand_simple() * 0.30;
            },
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
    fn test_interactive_platform() {
        let mut system = OnlineLearningPlatformSystem::new(OnlinePlatformType::Interactive);
        system.analyze_system().unwrap();
        assert!(system.engagement_mechanisms > 0.7);
    }
}