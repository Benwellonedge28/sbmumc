//! Temporal Flow Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalFlow {
    pub tf_id: String,
    pub flow_theories: Vec<FlowTheory>,
    pub direction_properties: DirectionProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowTheory {
    MovingSpotlight,
    BranchingTime,
    ImmutableFlow,
    DynamicPresent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectionProperties {
    pub temporal_direction: Direction,
    pub flow_rate: f64,
    pub subjective_continuity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    ForwardOnly,
    BackwardPossible,
    Bidirectional,
    None,
}

impl TemporalFlow {
    pub fn new() -> Self {
        Self {
            tf_id: String::from("temporal_flow_v1"),
            flow_theories: vec![],
            direction_properties: DirectionProperties {
                temporal_direction: Direction::ForwardOnly,
                flow_rate: 1.0,
                subjective_continuity: 1.0,
            },
        }
    }
}

impl Default for TemporalFlow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_flow_creation() {
        let flow = TemporalFlow::new();
        assert_eq!(flow.tf_id, "temporal_flow_v1");
    }
}
