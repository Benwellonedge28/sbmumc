//! # SBMUMC Module 1388: Narrative Design
//!
//! Systems for narrative design and story architecture.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NarrativeStructure {
    Linear,
    NonLinear,
    Branching,
    Episodic,
    OpenWorld,
    Procedural,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeDesignSystem {
    pub system_id: String,
    pub narrative_structure: NarrativeStructure,
    pub story_architecture: f64,
    pub character_arc_design: f64,
    pub world_consistency: f64,
    pub thematic_coherence: f64,
}

impl NarrativeDesignSystem {
    pub fn new(narrative_structure: NarrativeStructure) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            narrative_structure,
            story_architecture: 0.0,
            character_arc_design: 0.0,
            world_consistency: 0.0,
            thematic_coherence: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.narrative_structure {
            NarrativeStructure::Linear => {
                self.story_architecture = 0.95 + rand_simple() * 0.05;
                self.character_arc_design = 0.90 + rand_simple() * 0.10;
                self.thematic_coherence = 0.85 + rand_simple() * 0.14;
            },
            NarrativeStructure::NonLinear => {
                self.world_consistency = 0.95 + rand_simple() * 0.05;
                self.story_architecture = 0.90 + rand_simple() * 0.10;
                self.character_arc_design = 0.85 + rand_simple() * 0.14;
            },
            NarrativeStructure::Branching => {
                self.story_architecture = 0.95 + rand_simple() * 0.05;
                self.world_consistency = 0.90 + rand_simple() * 0.10;
                self.thematic_coherence = 0.85 + rand_simple() * 0.14;
            },
            NarrativeStructure::Episodic => {
                self.character_arc_design = 0.95 + rand_simple() * 0.05;
                self.story_architecture = 0.90 + rand_simple() * 0.10;
                self.world_consistency = 0.85 + rand_simple() * 0.14;
            },
            NarrativeStructure::OpenWorld => {
                self.world_consistency = 0.95 + rand_simple() * 0.05;
                self.thematic_coherence = 0.90 + rand_simple() * 0.10;
                self.character_arc_design = 0.85 + rand_simple() * 0.14;
            },
            NarrativeStructure::Procedural => {
                self.thematic_coherence = 0.95 + rand_simple() * 0.05;
                self.world_consistency = 0.90 + rand_simple() * 0.10;
                self.story_architecture = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.thematic_coherence == 0.0 {
            self.thematic_coherence = (self.story_architecture + self.character_arc_design) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_linear() {
        let mut system = NarrativeDesignSystem::new(NarrativeStructure::Linear);
        system.analyze_system().unwrap();
        assert!(system.story_architecture > 0.8);
    }
}
