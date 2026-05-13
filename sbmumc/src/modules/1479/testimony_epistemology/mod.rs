//! # SBMUMC Module 1479: Testimony Epistemology
//!
//! Systems for testimony epistemology and testimonial knowledge.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestimonyEpistemologyTopic {
    TestimonialReductionism,
    TestimonialConservatism,
    PerceptualTestimony,
    TestimonyTransmission,
    TestimonialInjustice,
    TrustEpistemology,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestimonyEpistemologySystem {
    pub system_id: String,
    pub testimony_epistemology_topic: TestimonyEpistemologyTopic,
    pub testimonial_justification: f64,
    pub speaker_reliability: f64,
    pub hearer_responsibility: f64,
    pub testimonial_inference: f64,
}

impl TestimonyEpistemologySystem {
    pub fn new(testimony_epistemology_topic: TestimonyEpistemologyTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            testimony_epistemology_topic,
            testimonial_justification: 0.0,
            speaker_reliability: 0.0,
            hearer_responsibility: 0.0,
            testimonial_inference: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.testimony_epistemology_topic {
            TestimonyEpistemologyTopic::TestimonialReductionism => {
                self.testimonial_justification = 0.95 + rand_simple() * 0.05;
                self.speaker_reliability = 0.90 + rand_simple() * 0.10;
                self.hearer_responsibility = 0.85 + rand_simple() * 0.14;
            },
            TestimonyEpistemologyTopic::TestimonialConservatism => {
                self.testimonial_inference = 0.95 + rand_simple() * 0.05;
                self.testimonial_justification = 0.90 + rand_simple() * 0.10;
                self.speaker_reliability = 0.85 + rand_simple() * 0.14;
            },
            TestimonyEpistemologyTopic::PerceptualTestimony => {
                self.hearer_responsibility = 0.95 + rand_simple() * 0.05;
                self.testimonial_inference = 0.90 + rand_simple() * 0.10;
                self.testimonial_justification = 0.85 + rand_simple() * 0.14;
            },
            TestimonyEpistemologyTopic::TestimonyTransmission => {
                self.speaker_reliability = 0.95 + rand_simple() * 0.05;
                self.hearer_responsibility = 0.90 + rand_simple() * 0.10;
                self.testimonial_inference = 0.85 + rand_simple() * 0.14;
            },
            TestimonyEpistemologyTopic::TestimonialInjustice => {
                self.testimonial_justification = 0.95 + rand_simple() * 0.05;
                self.testimonial_inference = 0.90 + rand_simple() * 0.10;
                self.speaker_reliability = 0.85 + rand_simple() * 0.14;
            },
            TestimonyEpistemologyTopic::TrustEpistemology => {
                self.hearer_responsibility = 0.95 + rand_simple() * 0.05;
                self.speaker_reliability = 0.90 + rand_simple() * 0.10;
                self.testimonial_justification = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.speaker_reliability == 0.0 {
            self.speaker_reliability = (self.testimonial_justification + self.hearer_responsibility) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_reductionism() {
        let mut system = TestimonyEpistemologySystem::new(TestimonyEpistemologyTopic::TestimonialReductionism);
        system.analyze_system().unwrap();
        assert!(system.testimonial_justification > 0.8);
    }
}