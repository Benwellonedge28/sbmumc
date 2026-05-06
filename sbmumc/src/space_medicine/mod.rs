//! Space Medicine Module (660)
//!
//! Medical systems, health monitoring, and treatment protocols for space environments.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpaceMedicalCondition {
    SpaceAdaptationSyndrome,
    BoneDensityLoss,
    MuscleAtrophy,
    RadiationExposure,
    VisionImpairment,
    FluidShift,
    CardiacDeconditioning,
    PsychologicalStress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceMedicalSystem {
    pub system_name: String,
    pub diagnostic_capability: Vec<String>,
    pub treatment_capability: Vec<String>,
    pub surgical_capability: bool,
    pub telemedicine: bool,
    pub ai_diagnosis: bool,
    pub drug_inventory: u32,
    pub emergency_shelter: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewHealthProfile {
    pub crew_member_id: String,
    pub age: u32,
    pub fitness_level: f64,
    pub radiation_exposure: f64,       // mSv
    pub bone_density_change: f64,     // percent
    pub muscle_mass_change: f64,      // percent
    pub cardiovascular_fitness: f64,
    pub psychological_score: f64,
    pub mission_risk_assessment: String,
}

impl SpaceMedicalSystem {
    pub fn new(system_name: String) -> Self {
        Self {
            system_name,
            diagnostic_capability: vec!["Vital Signs".into(), "Blood Analysis".into()],
            treatment_capability: vec!["Medication".into(), "Basic Surgery".into()],
            surgical_capability: false,
            telemedicine: true,
            ai_diagnosis: true,
            drug_inventory: 50,
            emergency_shelter: false,
        }
    }

    pub fn assess_crew_member(&self, profile: &CrewHealthProfile) -> String {
        if profile.radiation_exposure > 1000.0 {
            return "High Risk - Radiation Monitor".into();
        }
        if profile.bone_density_change < -5.0 {
            return "Monitor - Bone Density".into();
        }
        "Healthy".into()
    }
}

impl CrewHealthProfile {
    pub fn new(crew_member_id: String, age: u32) -> Self {
        Self {
            crew_member_id,
            age,
            fitness_level: 75.0,
            radiation_exposure: 0.0,
            bone_density_change: 0.0,
            muscle_mass_change: 0.0,
            cardiovascular_fitness: 80.0,
            psychological_score: 80.0,
            mission_risk_assessment: "Standard".into(),
        }
    }

    pub fn health_score(&self) -> f64 {
        (self.fitness_level + self.cardiovascular_fitness + self.psychological_score) / 3.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_medical_system() {
        let system = SpaceMedicalSystem::new("ISS Medical Bay".into());
        assert!(system.ai_diagnosis);
    }
}
