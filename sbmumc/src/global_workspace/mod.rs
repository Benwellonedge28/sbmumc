//! Global Workspace Module
//!
//! This module implements global workspace theory, information broadcast,
//! and consciousness as a shared workspace.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub struct GlobalWorkspace {
    pub contents: VecDeque<WorkspaceContent>,
    pub processes: Vec<Process>,
    pub broadcasts: Vec<Broadcast>,
    pub workspace_size: usize,
}

impl GlobalWorkspace {
    pub fn new() -> Self {
        GlobalWorkspace {
            contents: VecDeque::new(),
            processes: Vec::new(),
            broadcasts: Vec::new(),
            workspace_size: 7,
        }
    }

    /// Add to workspace
    pub fn add_content(&mut self, content: &str, source: &str) -> usize {
        let id = self.contents.len();
        self.contents.push_back(WorkspaceContent {
            content_id: id,
            content: content.to_string(),
            source: source.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            access_count: 0,
        });
        id
    }

    /// Broadcast
    pub fn broadcast(&mut self, content_id: usize) -> Result<Broadcast> {
        if content_id < self.contents.len() {
            let content = &self.contents[content_id];
            let broadcast = Broadcast {
                content: content.content.clone(),
                recipients: vec!["Perception".to_string(), "Memory".to_string(), "Action".to_string()],
                duration_ms: 100.0,
            };
            self.broadcasts.push(broadcast.clone());
            self.contents[content_id].access_count += 1;
            Ok(broadcast)
        } else {
            Err(SbmumcError::NotFound(format!("Content {} not found", content_id)))
        }
    }

    /// Compete for access
    pub fn compete(&mut self) -> usize {
        let winner_idx = if !self.contents.is_empty() {
            self.contents.len() - 1
        } else {
            0
        };
        self.add_content("Winner", "Competition")
    }

    /// Register process
    pub fn register_process(&mut self, name: &str, is_conscious: bool) -> &Process {
        let process = Process {
            process_id: format!("proc_{}", self.processes.len()),
            name: name.to_string(),
            is_conscious,
            access_level: if is_conscious { 3 } else { 1 },
        };
        self.processes.push(process);
        self.processes.last().unwrap()
    }

    /// Query workspace
    pub fn query(&self, filter: &str) -> Vec<String> {
        self.contents.iter()
            .filter(|c| c.content.contains(filter))
            .map(|c| c.content.clone())
            .collect()
    }
}

impl Default for GlobalWorkspace { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceContent {
    pub content_id: usize,
    pub content: String,
    pub source: String,
    pub timestamp: f64,
    pub access_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    pub process_id: String,
    pub name: String,
    pub is_conscious: bool,
    pub access_level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Broadcast {
    pub content: String,
    pub recipients: Vec<String>,
    pub duration_ms: f64,
}
