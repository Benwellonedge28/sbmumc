//! Attention Schema Module
//!
//! This module implements attention modeling, focus control,
//! and the cognitive architecture of selective awareness.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct AttentionSchema {
    pub schemas: Vec<AttentionSchemaModel>,
    pub focus_states: Vec<FocusState>,
    pub allocations: Vec<AttentionAllocation>,
}

impl AttentionSchema {
    pub fn new() -> Self {
        AttentionSchema {
            schemas: Vec::new(),
            focus_states: Vec::new(),
            allocations: Vec::new(),
        }
    }

    /// Create attention schema
    pub fn create_schema(&mut self, name: &str) -> &AttentionSchemaModel {
        let schema = AttentionSchemaModel {
            schema_id: format!("as_{}", self.schemas.len()),
            name: name.to_string(),
            attention_capacity: 7,
            current_focus: "Default".to_string(),
        };
        self.schemas.push(schema);
        self.schemas.last().unwrap()
    }

    /// Allocate attention
    pub fn allocate(&mut self, schema_id: &str, target: &str, weight: f64) -> &AttentionAllocation {
        let allocation = AttentionAllocation {
            schema_id: schema_id.to_string(),
            target: target.to_string(),
            weight,
            priority: (weight * 10.0) as usize,
        };
        self.allocations.push(allocation);
        self.allocations.last().unwrap()
    }

    /// Focus shift
    pub fn shift_focus(&mut self, from: &str, to: &str) -> FocusShiftResult {
        FocusShiftResult {
            from_target: from.to_string(),
            to_target: to.to_string(),
            transition_time_ms: 200.0,
            attention_loss: 0.05,
        }
    }

    /// Set focus state
    pub fn set_focus_state(&mut self, name: &str, mode: &str) -> &FocusState {
        let state = FocusState {
            state_id: format!("fs_{}", self.focus_states.len()),
            name: name.to_string(),
            mode: mode.to_string(),
            alertness: 0.9,
        };
        self.focus_states.push(state);
        self.focus_states.last().unwrap()
    }

    /// Calculate attention map
    pub fn attention_map(&self) -> HashMap<String, f64> {
        let mut map = HashMap::new();
        map.insert("Visual".to_string(), 0.4);
        map.insert("Auditory".to_string(), 0.3);
        map.insert("Somatic".to_string(), 0.2);
        map.insert("Cognitive".to_string(), 0.1);
        map
    }
}

impl Default for AttentionSchema { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionSchemaModel {
    pub schema_id: String,
    pub name: String,
    pub attention_capacity: usize,
    pub current_focus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusState {
    pub state_id: String,
    pub name: String,
    pub mode: String,
    pub alertness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionAllocation {
    pub schema_id: String,
    pub target: String,
    pub weight: f64,
    pub priority: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusShiftResult {
    pub from_target: String,
    pub to_target: String,
    pub transition_time_ms: f64,
    pub attention_loss: f64,
}
