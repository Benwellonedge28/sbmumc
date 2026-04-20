//! Attention Consciousness Module
//!
//! This module implements attention as consciousness, spotlight of awareness,
//! and attentional consciousness models.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct AttentionConsciousness {
    pub spotlights: Vec<AttentionSpotlight>,
    pub filters: Vec<AttentionalFilter>,
    pub allocations: Vec<AttentionAllocation>,
    pub capacity_limits: Vec<CapacityLimit>,
}

impl AttentionConsciousness {
    pub fn new() -> Self {
        AttentionConsciousness {
            spotlights: Vec::new(),
            filters: vec![
                AttentionalFilter { filter_type: "Inhibition".to_string(), strength: 0.7 },
                AttentionalFilter { filter_type: "Enhancement".to_string(), strength: 0.8 },
            ],
            allocations: Vec::new(),
            capacity_limits: vec![
                CapacityLimit { limit_type: "Working Memory".to_string(), capacity: 7 },
                CapacityLimit { limit_type: "Selective Attention".to_string(), capacity: 1 },
            ],
        }
    }

    /// Create spotlight
    pub fn create_spotlight(&mut self, location: &[f64], radius: f64) -> &AttentionSpotlight {
        let spotlight = AttentionSpotlight {
            spotlight_id: format!("spot_{}", self.spotlights.len()),
            focus_point: location.to_vec(),
            radius,
            brightness: 1.0,
        };
        self.spotlights.push(spotlight);
        self.spotlights.last().unwrap()
    }

    /// Allocate attention
    pub fn allocate(&mut self, target: &str, weight: f64) -> &AttentionAllocation {
        let allocation = AttentionAllocation {
            allocation_id: format!("alloc_{}", self.allocations.len()),
            target: target.to_string(),
            weight,
            priority: (weight * 10.0) as usize,
        };
        self.allocations.push(allocation);
        self.allocations.last().unwrap()
    }

    /// Shift spotlight
    pub fn shift_spotlight(&mut self, spotlight_id: &str, new_location: &[f64]) -> Result<()> {
        if let Some(spotlight) = self.spotlights.iter_mut().find(|s| s.spotlight_id == spotlight_id) {
            spotlight.focus_point = new_location.to_vec();
            Ok(())
        } else {
            Err(SbmumcError::NotFound(format!("Spotlight {} not found", spotlight_id)))
        }
    }

    /// Add filter
    pub fn add_filter(&mut self, filter_type: &str, strength: f64) -> &AttentionalFilter {
        let filter = AttentionalFilter {
            filter_type: filter_type.to_string(),
            strength,
        };
        self.filters.push(filter);
        self.filters.last().unwrap()
    }

    /// Check capacity
    pub fn check_capacity(&self, demand: usize) -> CapacityResult {
        CapacityResult {
            demand,
            capacity: 7,
            overloaded: demand > 7,
            load_percentage: (demand as f64 / 7.0) * 100.0,
        }
    }
}

impl Default for AttentionConsciousness { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionSpotlight {
    pub spotlight_id: String,
    pub focus_point: Vec<f64>,
    pub radius: f64,
    pub brightness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionalFilter {
    pub filter_type: String,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionAllocation {
    pub allocation_id: String,
    pub target: String,
    pub weight: f64,
    pub priority: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityLimit {
    pub limit_type: String,
    pub capacity: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityResult {
    pub demand: usize,
    pub capacity: usize,
    pub overloaded: bool,
    pub load_percentage: f64,
}
