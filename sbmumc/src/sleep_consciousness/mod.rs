//! Sleep Consciousness Module
//!
//! This module implements sleep states, dreaming, and the spectrum
//! of consciousness from wakefulness to deep sleep.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub struct SleepConsciousness {
    pub sleep_stages: Vec<SleepStage>,
    pub dreams: Vec<Dream>,
    pub current_stage: String,
    pub cycles: Vec<SleepCycle>,
}

impl SleepConsciousness {
    pub fn new() -> Self {
        SleepConsciousness {
            sleep_stages: vec![
                SleepStage { stage: "Wake".to_string(), consciousness_level: 1.0, frequency_range_hz: "15-30".to_string() },
                SleepStage { stage: "N1".to_string(), consciousness_level: 0.7, frequency_range_hz: "4-8".to_string() },
                SleepStage { stage: "N2".to_string(), consciousness_level: 0.4, frequency_range_hz: "12-15".to_string() },
                SleepStage { stage: "N3".to_string(), consciousness_level: 0.1, frequency_range_hz: "0.5-2".to_string() },
                SleepStage { stage: "REM".to_string(), consciousness_level: 0.8, frequency_range_hz: "4-8".to_string() },
            ],
            dreams: Vec::new(),
            current_stage: "Wake".to_string(),
            cycles: Vec::new(),
        }
    }

    /// Enter sleep stage
    pub fn enter_stage(&mut self, stage: &str) -> &SleepStage {
        self.current_stage = stage.to_string();
        self.sleep_stages.iter().find(|s| s.stage == stage).unwrap()
    }

    /// Record dream
    pub fn record_dream(&mut self, narrative: &str, vividness: f64) -> &Dream {
        let dream = Dream {
            dream_id: format!("dream_{}", self.dreams.len()),
            narrative: narrative.to_string(),
            vividness,
            emotional_tone: "Neutral".to_string(),
            lucidity: 0.0,
        };
        self.dreams.push(dream);
        self.dreams.last().unwrap()
    }

    /// Start sleep cycle
    pub fn start_cycle(&mut self) -> &SleepCycle {
        let cycle = SleepCycle {
            cycle_id: format!("cycle_{}", self.cycles.len()),
            stages: vec!["N1".to_string(), "N2".to_string(), "N3".to_string(), "N2".to_string(), "REM".to_string()],
            duration_minutes: 90.0,
            completed: false,
        };
        self.cycles.push(cycle);
        self.cycles.last().unwrap()
    }

    /// Become lucid
    pub fn become_lucid(&mut self, dream_id: usize) -> Result<()> {
        if dream_id < self.dreams.len() {
            self.dreams[dream_id].lucidity = 1.0;
            Ok(())
        } else {
            Err(SbmumcError::NotFound(format!("Dream {} not found", dream_id)))
        }
    }

    /// Assess consciousness level
    pub fn assess_level(&self, stage: &str) -> ConsciousnessLevel {
        let stage_info = self.sleep_stages.iter().find(|s| s.stage == stage);
        ConsciousnessLevel {
            stage: stage.to_string(),
            consciousness: stage_info.map(|s| s.consciousness_level).unwrap_or(0.0),
            reportable: stage != "N3",
        }
    }
}

impl Default for SleepConsciousness { fn default() -> Self { Self::new() } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepStage {
    pub stage: String,
    pub consciousness_level: f64,
    pub frequency_range_hz: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dream {
    pub dream_id: String,
    pub narrative: String,
    pub vividness: f64,
    pub emotional_tone: String,
    pub lucidity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepCycle {
    pub cycle_id: String,
    pub stages: Vec<String>,
    pub duration_minutes: f64,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessLevel {
    pub stage: String,
    pub consciousness: f64,
    pub reportable: bool,
}
