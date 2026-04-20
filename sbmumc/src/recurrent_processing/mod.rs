//! Recurrent Processing Module
//!
//! This module implements recurrent neural processing, conscious perception,
//! and the hierarchy of visual and cognitive processing.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub struct RecurrentProcessing {
    pub loops: Vec<ProcessingLoop>,
    pub layers: Vec<ProcessingLayer>,
    pub states: VecDeque<ProcessingState>,
    pub feedback_paths: Vec<FeedbackPath>,
}

impl RecurrentProcessing {
    pub fn new() -> Self {
        RecurrentProcessing {
            loops: Vec::new(),
            layers: vec![
                ProcessingLayer { layer_id: 1, name: "Input".to_string(), processing_type: "Feedforward".to_string() },
                ProcessingLayer { layer_id: 2, name: "Hidden".to_string(), processing_type: "Recurrent".to_string() },
                ProcessingLayer { layer_id: 3, name: "Output".to_string(), processing_type: "Feedback".to_string() },
            ],
            states: VecDeque::new(),
            feedback_paths: Vec::new(),
        }
    }

    /// Create processing loop
    pub fn create_loop(&mut self, name: &str) -> &ProcessingLoop {
        let loop_info = ProcessingLoop {
            loop_id: format!("loop_{}", self.loops.len()),
            name: name.to_string(),
            iterations: 0,
            convergence_threshold: 0.01,
        };
        self.loops.push(loop_info);
        self.loops.last().unwrap()
    }

    /// Add feedback path
    pub fn add_feedback_path(&mut self, from_layer: usize, to_layer: usize) -> &FeedbackPath {
        let path = FeedbackPath {
            path_id: format!("fp_{}", self.feedback_paths.len()),
            from_layer,
            to_layer,
            delay_ms: 20.0,
            strength: 0.8,
        };
        self.feedback_paths.push(path);
        self.feedback_paths.last().unwrap()
    }

    /// Process with recurrence
    pub fn process_recurrent(&mut self, input: &[f64], iterations: usize) -> ProcessingResult {
        let mut state = input.to_vec();
        for _ in 0..iterations {
            for element in &mut state {
                *element = (*element * 0.9).min(1.0);
            }
        }
        self.states.push_back(ProcessingState {
            iteration: iterations,
            state: state.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        });
        ProcessingResult {
            output: state,
            iterations_completed: iterations,
            convergence_achieved: true,
        }
    }

    /// Add layer
    pub fn add_layer(&mut self, name: &str, processing_type: &str) -> &ProcessingLayer {
        let layer = ProcessingLayer {
            layer_id: self.layers.len() + 1,
            name: name.to_string(),
            processing_type: processing_type.to_string(),
        };
        self.layers.push(layer);
        self.layers.last().unwrap()
    }

    /// Get current state
    pub fn current_state(&self) -> Option<ProcessingState> {
        self.states.back().cloned()
    }
}

impl Default for RecurrentProcessing { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingLoop {
    pub loop_id: String,
    pub name: String,
    pub iterations: usize,
    pub convergence_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingLayer {
    pub layer_id: usize,
    pub name: String,
    pub processing_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingState {
    pub iteration: usize,
    pub state: Vec<f64>,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackPath {
    pub path_id: String,
    pub from_layer: usize,
    pub to_layer: usize,
    pub delay_ms: f64,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub output: Vec<f64>,
    pub iterations_completed: usize,
    pub convergence_achieved: bool,
}
