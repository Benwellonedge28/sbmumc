//! Working Consciousness Module
//!
//! This module implements working consciousness, active thought maintenance,
//! and the cognitive workspace of conscious processing.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub struct WorkingConsciousness {
    pub contents: VecDeque<WorkingContent>,
    pub maintenance_slots: Vec<MaintenanceSlot>,
    pub active_processes: Vec<ActiveProcess>,
    pub workspace_capacity: usize,
}

impl WorkingConsciousness {
    pub fn new() -> Self {
        WorkingConsciousness {
            contents: VecDeque::new(),
            maintenance_slots: (0..4).map(|i| MaintenanceSlot {
                slot_id: i,
                content: None,
                refresh_count: 0,
            }).collect(),
            active_processes: Vec::new(),
            workspace_capacity: 4,
        }
    }

    /// Hold in consciousness
    pub fn hold(&mut self, content: &str, priority: usize) -> Result<()> {
        if self.contents.len() < self.workspace_capacity {
            self.contents.push_back(WorkingContent {
                content: content.to_string(),
                priority,
                freshness: 1.0,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            });
            Ok(())
        } else {
            Err(SbmumcError::ResourceExhausted("Workspace at capacity".to_string()))
        }
    }

    /// Maintain slot
    pub fn maintain_slot(&mut self, slot_id: usize, content: &str) -> Result<()> {
        if slot_id < self.maintenance_slots.len() {
            self.maintenance_slots[slot_id].content = Some(content.to_string());
            self.maintenance_slots[slot_id].refresh_count += 1;
            Ok(())
        } else {
            Err(SbmumcError::NotFound(format!("Slot {} not found", slot_id)))
        }
    }

    /// Refresh content
    pub fn refresh(&mut self, content_id: usize) -> Result<()> {
        if content_id < self.contents.len() {
            self.contents[content_id].freshness = 1.0;
            Ok(())
        } else {
            Err(SbmumcError::NotFound(format!("Content {} not found", content_id)))
        }
    }

    /// Add active process
    pub fn add_process(&mut self, name: &str) -> &ActiveProcess {
        let process = ActiveProcess {
            process_id: format!("proc_{}", self.active_processes.len()),
            name: name.to_string(),
            active: true,
            resource_usage: 0.2,
        };
        self.active_processes.push(process);
        self.active_processes.last().unwrap()
    }

    /// Retrieve by priority
    pub fn retrieve_by_priority(&self, min_priority: usize) -> Vec<String> {
        self.contents.iter()
            .filter(|c| c.priority >= min_priority)
            .map(|c| c.content.clone())
            .collect()
    }
}

impl Default for WorkingConsciousness { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingContent {
    pub content: String,
    pub priority: usize,
    pub freshness: f64,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceSlot {
    pub slot_id: usize,
    pub content: Option<String>,
    pub refresh_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveProcess {
    pub process_id: String,
    pub name: String,
    pub active: bool,
    pub resource_usage: f64,
}
