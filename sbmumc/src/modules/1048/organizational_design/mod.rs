//! # SBMUMC Module 1048: Organizational Design
//!
//! Principles and frameworks for designing effective organizations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrgStructureType {
    Functional,
    Divisional,
    Matrix,
    Network,
    Flat,
    Holacracy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationDesign {
    pub design_id: String,
    pub organization_name: String,
    pub structure_type: OrgStructureType,
    pub complexity_score: u8,
    pub agility_score: f64,
    pub coordination_efficiency: f64,
    pub innovation_capacity: f64,
}

impl OrganizationDesign {
    pub fn new(name: String, structure: OrgStructureType) -> Self {
        Self {
            design_id: crate::core::uuid_simple(),
            organization_name: name,
            structure_type: structure,
            complexity_score: 0,
            agility_score: 0.0,
            coordination_efficiency: 0.0,
            innovation_capacity: 0.0,
        }
    }

    pub fn evaluate_design(&mut self) -> Result<()> {
        match self.structure_type {
            OrgStructureType::Functional => {
                self.complexity_score = 4;
                self.agility_score = 0.5 + rand_simple() * 0.2;
                self.coordination_efficiency = 0.8 + rand_simple() * 0.15;
                self.innovation_capacity = 0.5 + rand_simple() * 0.2;
            },
            OrgStructureType::Matrix => {
                self.complexity_score = 8;
                self.agility_score = 0.6 + rand_simple() * 0.2;
                self.coordination_efficiency = 0.65 + rand_simple() * 0.2;
                self.innovation_capacity = 0.7 + rand_simple() * 0.2;
            },
            OrgStructureType::Flat => {
                self.complexity_score = 2;
                self.agility_score = 0.85 + rand_simple() * 0.15;
                self.coordination_efficiency = 0.55 + rand_simple() * 0.25;
                self.innovation_capacity = 0.8 + rand_simple() * 0.15;
            },
            OrgStructureType::Holacracy => {
                self.complexity_score = 6;
                self.agility_score = 0.9 + rand_simple() * 0.1;
                self.coordination_efficiency = 0.5 + rand_simple() * 0.3;
                self.innovation_capacity = 0.85 + rand_simple() * 0.15;
            },
            _ => {
                self.complexity_score = 5;
                self.agility_score = 0.6 + rand_simple() * 0.2;
                self.coordination_efficiency = 0.7 + rand_simple() * 0.2;
                self.innovation_capacity = 0.6 + rand_simple() * 0.2;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamStructure {
    pub team_id: String,
    pub team_name: String,
    pub member_count: usize,
    pub span_of_control: u8,
    pub collaboration_score: f64,
    pub performance_index: f64,
}

impl TeamStructure {
    pub fn new(name: String, members: usize) -> Self {
        Self {
            team_id: crate::core::uuid_simple(),
            team_name: name,
            member_count: members,
            span_of_control: 0,
            collaboration_score: 0.0,
            performance_index: 0.0,
        }
    }

    pub fn assess_team(&mut self) -> Result<()> {
        self.span_of_control = (10.0 / (1.0 + self.member_count as f64 / 10.0)) as u8;
        self.span_of_control = self.span_of_control.max(1).min(8);

        self.collaboration_score = 0.6 + rand_simple() * 0.4;
        self.performance_index = self.collaboration_score * (1.0 - self.span_of_control as f64 / 20.0);
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

pub fn optimize_org_structure(org_id: &str, size: usize) -> Result<OrgStructureType> {
    if size < 50 {
        Ok(OrgStructureType::Flat)
    } else if size < 200 {
        Ok(OrgStructureType::Functional)
    } else {
        Ok(OrgStructureType::Divisional)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_organization() {
        let mut design = OrganizationDesign::new(
            "Tech_Corp".to_string(),
            OrgStructureType::Matrix,
        );
        design.evaluate_design().unwrap();
        assert!(design.innovation_capacity > 0.5);
    }

    #[test]
    fn test_team_assessment() {
        let mut team = TeamStructure::new("Engineering_Squad".to_string(), 8);
        team.assess_team().unwrap();
        assert!(team.performance_index > 0.0);
    }
}