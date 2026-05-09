//! # SBMUMC Module 1187: Peer Learning
//!
//! Collaborative learning through peer interaction and mutual support.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeerLearningStructure {
    StudyGroups,
    PeerTutoring,
    CollaborativeProjects,
    DiscussionForums,
    LearningCircles,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerLearningFramework {
    pub framework_id: String,
    pub learning_structure: PeerLearningStructure,
    pub collaborative_engagement: f64,
    pub knowledge_sharing: f64,
    pub social_presence: f64,
    pub learning_outcomes: f64,
}

impl PeerLearningFramework {
    pub fn new(learning_structure: PeerLearningStructure) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            learning_structure,
            collaborative_engagement: 0.0,
            knowledge_sharing: 0.0,
            social_presence: 0.0,
            learning_outcomes: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.learning_structure {
            PeerLearningStructure::StudyGroups => {
                self.collaborative_engagement = 0.85 + rand_simple() * 0.14;
                self.knowledge_sharing = 0.75 + rand_simple() * 0.22;
            },
            PeerLearningStructure::PeerTutoring => {
                self.knowledge_sharing = 0.90 + rand_simple() * 0.10;
                self.learning_outcomes = 0.80 + rand_simple() * 0.18;
            },
            PeerLearningStructure::CollaborativeProjects => {
                self.collaborative_engagement = 0.80 + rand_simple() * 0.18;
                self.social_presence = 0.75 + rand_simple() * 0.22;
                self.learning_outcomes = 0.75 + rand_simple() * 0.22;
            },
            PeerLearningStructure::DiscussionForums => {
                self.social_presence = 0.85 + rand_simple() * 0.14;
                self.knowledge_sharing = 0.70 + rand_simple() * 0.25;
            },
            PeerLearningStructure::LearningCircles => {
                self.collaborative_engagement = 0.90 + rand_simple() * 0.10;
                self.social_presence = 0.85 + rand_simple() * 0.14;
                self.knowledge_sharing = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.learning_outcomes == 0.0 {
            self.learning_outcomes = (self.collaborative_engagement + self.knowledge_sharing) / 2.0 * (0.7 + rand_simple() * 0.3);
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
    fn test_learning_circles() {
        let mut framework = PeerLearningFramework::new(PeerLearningStructure::LearningCircles);
        framework.analyze_framework().unwrap();
        assert!(framework.collaborative_engagement > 0.7);
    }
}