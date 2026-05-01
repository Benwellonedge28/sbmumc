//! Presentism Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Presentism {
    pub pr_id: String,
    pub present_moment: PresentMoment,
    pub temporal_extent: TemporalExtent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PresentMoment {
    Now,
    SpeciousPresent,
    ExtendedPresent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalExtent {
    pub duration_ns: f64,
    pub psychological_present: f64,
}

impl Presentism {
    pub fn new() -> Self {
        Self {
            pr_id: String::from("presentism_v1"),
            present_moment: PresentMoment::Now,
            temporal_extent: TemporalExtent {
                duration_ns: 0.0,
                psychological_present: 0.0,
            },
        }
    }
}

impl Default for Presentism {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentism_creation() {
        let presentism = Presentism::new();
        assert_eq!(presentism.pr_id, "presentism_v1");
    }
}
