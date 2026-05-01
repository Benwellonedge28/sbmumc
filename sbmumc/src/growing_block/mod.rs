//! Growing Block Universe Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowingBlock {
    pub gb_id: String,
    pub past_grows: bool,
    pub block_state: BlockState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockState {
    pub past_completed: Vec<TemporalEvent>,
    pub present_active: Vec<TemporalEvent>,
    pub future_open: Vec<TemporalEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalEvent {
    pub event_id: String,
    pub time_coordinate: f64,
    pub causal_relations: Vec<String>,
}

impl GrowingBlock {
    pub fn new() -> Self {
        Self {
            gb_id: String::from("growing_block_v1"),
            past_grows: true,
            block_state: BlockState {
                past_completed: vec![],
                present_active: vec![],
                future_open: vec![],
            },
        }
    }
}

impl Default for GrowingBlock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growing_block_creation() {
        let block = GrowingBlock::new();
        assert!(block.past_grows);
    }
}
