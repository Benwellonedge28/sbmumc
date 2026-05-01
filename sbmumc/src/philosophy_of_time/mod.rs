//! Philosophy of Time Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophyOfTime {
    pub pot_id: String,
    pub theories: Vec<TimePhilosophy>,
    pub metaphysical_positions: Vec<MetaphysicalPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimePhilosophy {
    McTaggartATheory,
    McTaggartBTheory,
    B-theoreticPresentism,
    B-theoreticEternalism,
    GrowingBlockTheory,
   perdurantism,
    endurantism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphysicalPosition {
    pub position: String,
    pub key_arguments: Vec<String>,
    pub counter_arguments: Vec<String>,
}

impl PhilosophyOfTime {
    pub fn new() -> Self {
        Self {
            pot_id: String::from("philosophy_of_time_v1"),
            theories: vec![],
            metaphysical_positions: vec![],
        }
    }
}

impl Default for PhilosophyOfTime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_philosophy_of_time_creation() {
        let philosophy = PhilosophyOfTime::new();
        assert_eq!(philosophy.pot_id, "philosophy_of_time_v1");
    }
}
