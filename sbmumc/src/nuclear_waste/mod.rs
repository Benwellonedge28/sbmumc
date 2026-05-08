//! Nuclear Waste Module (749)
//!
//! Radioactive waste management, storage, and disposal solutions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WasteCategory {
    LowLevel,
    IntermediateLevel,
    HighLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearWaste {
    pub waste_id: String,
    pub waste_category: WasteCategory,
    pub activity_curies: f64,
    pub half_life_years: f64,
    pub volume_m3: f64,
    pub storage_method: String,
    pub disposal_path: String,
    pub heat_generation_kw_m3: f64,
}

impl NuclearWaste {
    pub fn new(waste_id: String, category: WasteCategory) -> Self {
        Self {
            waste_id,
            waste_category: category,
            activity_curies: 0.0,
            half_life_years: 0.0,
            volume_m3: 0.0,
            storage_method: "Dry Cask".into(),
            disposal_path: "Deep Geological".into(),
            heat_generation_kw_m3: 0.0,
        }
    }

    pub fn decay_time_years(&self) -> f64 {
        self.half_life_years * 10.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nuclear_waste() {
        let waste = NuclearWaste::new("NW-001".into(), WasteCategory::HighLevel);
        assert!(matches!(waste.waste_category, WasteCategory::HighLevel));
    }
}
