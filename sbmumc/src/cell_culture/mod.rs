//! Cell Culture Module (700)
//!
//! In vitro cell growth, tissue culture, and cell line maintenance systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CultureType {
    2D,
    3D,
    Organoid,
    Spheroid,
    Bioreactor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellLine {
    pub line_id: String,
    pub cell_type: String,
    pub origin: String,
    pub passage_number: u32,
    pub viability_percent: f64,
    pub confluence_percent: f64,
    pub doubling_time_hours: f64,
    pub contamination_free: bool,
}

impl CellLine {
    pub fn new(line_id: String, cell_type: String) -> Self {
        Self {
            line_id,
            cell_type,
            origin: "Unknown".into(),
            passage_number: 0,
            viability_percent: 100.0,
            confluence_percent: 0.0,
            doubling_time_hours: 24.0,
            contamination_free: true,
        }
    }

    pub fn split(&mut self, ratio: f64) {
        self.passage_number += 1;
        self.confluence_percent = 100.0 / ratio;
    }

    pub fn is_healthy(&self) -> bool {
        self.viability_percent > 80.0 && self.contamination_free
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cell_line() {
        let line = CellLine::new("HEK293".into(), "Human Embryonic Kidney".into());
        assert_eq!(line.line_id, "HEK293");
    }
}
