//! Grid Integration Module (762)
//!
//! Renewable interconnection and power system integration.

use serde::{Deserialize, Serialize};

pub struct GridIntegration {
    pub study_id: String,
    pub renewable_capacity_mw: f64,
    pub stability_margin_percent: f64,
}

impl GridIntegration {
    pub fn new(id: String) -> Self { Self { study_id: id, renewable_capacity_mw: 0.0, stability_margin_percent: 50.0 } }
}

#[cfg(test)]
mod tests { use super::*; #[test] fn test_gi() { let g = GridIntegration::new("GI-1".into()); assert_eq!(g.study_id, "GI-1"); } }
