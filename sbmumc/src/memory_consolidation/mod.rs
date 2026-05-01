//! Memory Consolidation Module (509)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConsolidation {
    pub mc_id: String,
    pub consolidation_method: ConsolidationMethod,
    pub short_term_capacity_items: usize,
    pub long_term_storage_tb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsolidationMethod {
    HippocampalReplay,
    SystemsConsolidation,
    SynapticConsolidation,
    ActiveSystem,
    OfflineReplay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryTrace {
    pub trace_id: String,
    pub memory_type: MemoryType,
    pub strength: f64,
    pub emotional_valence: f64,
    pub timestamp_ns: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType {
    Episodic,
    Semantic,
    Procedural,
    Working,
    Prospective,
}

impl MemoryConsolidation {
    pub fn new() -> Self {
        Self {
            mc_id: String::from("memory_consolidation_v1"),
            consolidation_method: ConsolidationMethod::SystemsConsolidation,
            short_term_capacity_items: 7,
            long_term_storage_tb: 1024,
        }
    }

    pub fn consolidate(&mut self, trace: MemoryTrace) {
        let priority = self.calculate_priority(&trace);
        if priority > 0.7 {
            trace.strength = 1.0;
        }
    }

    fn calculate_priority(&self, trace: &MemoryTrace) -> f64 {
        (trace.emotional_valence + trace.strength) / 2.0
    }
}

impl Default for MemoryConsolidation {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_memory_consolidation() {
        let mut mc = MemoryConsolidation::new();
        let trace = MemoryTrace {
            trace_id: String::from("trace_1"),
            memory_type: MemoryType::Episodic,
            strength: 0.8,
            emotional_valence: 0.6,
            timestamp_ns: 0,
        };
        mc.consolidate(trace);
    }
}
