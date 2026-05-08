//! # SBMUMC Module 984: Climate Education
//! 
//! Educational frameworks for climate literacy and awareness.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EducationLevel {
    Primary,
    Secondary,
    Tertiary,
    Professional,
    Public,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateEducationProgram {
    pub program_id: String,
    pub level: EducationLevel,
    pub name: String,
    pub participants: u32,
    pub curriculum_hours: u32,
    pub knowledge_gain_score: f64,
    pub behavior_change_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateEducationSystem {
    pub system_id: String,
    pub programs: Vec<ClimateEducationProgram>,
    pub total_participants: u32,
    pub average_knowledge_gain: f64,
    pub behavior_impact_score: f64,
}

impl ClimateEducationProgram {
    pub fn new(level: EducationLevel, name: &str) -> Self {
        Self {
            program_id: format!("cep_{}", uuid_simple()),
            level,
            name: name.to_string(),
            participants: 0,
            curriculum_hours: 0,
            knowledge_gain_score: 0.0,
            behavior_change_score: 0.0,
        }
    }

    pub fn configure(&mut self, participants: u32, hours: u32, knowledge: f64, behavior: f64) {
        self.participants = participants;
        self.curriculum_hours = hours;
        self.knowledge_gain_score = knowledge.clamp(0.0, 1.0);
        self.behavior_change_score = behavior.clamp(0.0, 1.0);
    }
}

impl ClimateEducationSystem {
    pub fn new() -> Self {
        Self {
            system_id: format!("ces_{}", uuid_simple()),
            programs: Vec::new(),
            total_participants: 0,
            average_knowledge_gain: 0.0,
            behavior_impact_score: 0.0,
        }
    }

    pub fn add_program(&mut self, program: ClimateEducationProgram) {
        self.programs.push(program);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_participants = self.programs.iter()
            .map(|p| p.participants)
            .sum();
        self.average_knowledge_gain = self.programs.iter()
            .map(|p| p.knowledge_gain_score)
            .sum::<f64>() / self.programs.len().max(1) as f64;
        self.behavior_impact_score = self.programs.iter()
            .map(|p| p.behavior_change_score)
            .sum::<f64>() / self.programs.len().max(1) as f64;
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{:x}{:x}", duration.as_secs(), duration.subsec_nanos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climate_education_program() {
        let mut program = ClimateEducationProgram::new(
            EducationLevel::Secondary,
            "Climate Science Basics",
        );
        program.configure(1000, 40, 0.8, 0.5);
        assert!(program.knowledge_gain_score > 0.7);
    }

    #[test]
    fn test_climate_education_system() {
        let mut system = ClimateEducationSystem::new();
        system.add_program(ClimateEducationProgram::new(
            EducationLevel::Tertiary,
            "Sustainability Engineering",
        ));
        assert!(system.total_participants >= 0);
    }
}
