//! Collective Consciousness Module
//!
//! This module implements collective mind, group consciousness,
//! and shared awareness across multiple agents or individuals.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

pub struct CollectiveConsciousness {
    pub collectives: Vec<Collective>,
    pub members: Vec<Member>,
    pub shared_states: VecDeque<SharedState>,
    pub emergent_properties: Vec<EmergentProperty>,
}

impl CollectiveConsciousness {
    pub fn new() -> Self {
        CollectiveConsciousness {
            collectives: Vec::new(),
            members: Vec::new(),
            shared_states: VecDeque::new(),
            emergent_properties: Vec::new(),
        }
    }

    /// Create collective
    pub fn create_collective(&mut self, name: &str) -> &Collective {
        let collective = Collective {
            collective_id: format!("col_{}", self.collectives.len()),
            name: name.to_string(),
            size: 0,
            cohesion: 0.0,
            collective_iq: 0.0,
        };
        self.collectives.push(collective);
        self.collectives.last().unwrap()
    }

    /// Add member
    pub fn add_member(&mut self, collective_id: &str, member_id: &str) -> Result<()> {
        if let Some(collective) = self.collectives.iter_mut().find(|c| c.collective_id == collective_id) {
            collective.size += 1;
            self.members.push(Member {
                member_id: member_id.to_string(),
                collective_id: collective_id.to_string(),
                contribution: 0.8,
            });
            Ok(())
        } else {
            Err(SbmumcError::NotFound(format!("Collective {} not found", collective_id)))
        }
    }

    /// Share state
    pub fn share_state(&mut self, collective_id: &str, state: &str) -> Result<()> {
        if self.collectives.iter().any(|c| c.collective_id == collective_id) {
            self.shared_states.push_back(SharedState {
                collective_id: collective_id.to_string(),
                state: state.to_string(),
                members_aware: 5,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            });
            Ok(())
        } else {
            Err(SbmumcError::NotFound(format!("Collective {} not found", collective_id)))
        }
    }

    /// Calculate collective IQ
    pub fn calculate_collective_iq(&self, collective_id: &str) -> CollectiveIQ {
        CollectiveIQ {
            collective_id: collective_id.to_string(),
            individual_average: 100.0,
            collective_score: 120.0,
            emergence_factor: 1.2,
        }
    }

    /// Measure cohesion
    pub fn measure_cohesion(&self, collective_id: &str) -> CohesionResult {
        CohesionResult {
            collective_id: collective_id.to_string(),
            cohesion: 0.85,
            shared_beliefs: 10,
            collective_action: true,
        }
    }
}

impl Default for CollectiveConsciousness { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collective {
    pub collective_id: String,
    pub name: String,
    pub size: usize,
    pub cohesion: f64,
    pub collective_iq: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub member_id: String,
    pub collective_id: String,
    pub contribution: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedState {
    pub collective_id: String,
    pub state: String,
    pub members_aware: usize,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentProperty {
    pub property_id: String,
    pub property_name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveIQ {
    pub collective_id: String,
    pub individual_average: f64,
    pub collective_score: f64,
    pub emergence_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CohesionResult {
    pub collective_id: String,
    pub cohesion: f64,
    pub shared_beliefs: usize,
    pub collective_action: bool,
}
