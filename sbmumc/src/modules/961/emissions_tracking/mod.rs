//! # SBMUMC Module 961: Emissions Tracking
//! 
//! Systems for tracking and monitoring greenhouse gas emissions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmissionsSource {
    StationaryCombustion,
    MobileCombustion,
    IndustrialProcesses,
    FugitiveEmissions,
    Waste,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmissionsRecord {
    pub record_id: String,
    pub source: EmissionsSource,
    pub entity_name: String,
    pub period: String,
    pub co2_tons: f64,
    pub ch4_tons: f64,
    pub n2o_tons: f64,
    pub verification_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmissionsInventory {
    pub inventory_id: String,
    pub records: Vec<EmissionsRecord>,
    pub total_co2: f64,
    pub total_co2e: f64,
    pub reduction_target: f64,
    pub progress_percentage: f64,
}

impl EmissionsRecord {
    pub fn new(source: EmissionsSource, entity: &str, period: &str) -> Self {
        Self {
            record_id: format!("er_{}", uuid_simple()),
            source,
            entity_name: entity.to_string(),
            period: period.to_string(),
            co2_tons: 0.0,
            ch4_tons: 0.0,
            n2o_tons: 0.0,
            verification_status: "unverified".to_string(),
        }
    }

    pub fn set_emissions(&mut self, co2: f64, ch4: f64, n2o: f64) {
        self.co2_tons = co2;
        self.ch4_tons = ch4;
        self.n2o_tons = n2o;
    }

    pub fn total_co2e(&self) -> f64 {
        self.co2_tons + (self.ch4_tons * 28.0) + (self.n2o_tons * 265.0)
    }
}

impl EmissionsInventory {
    pub fn new() -> Self {
        Self {
            inventory_id: format!("ei_{}", uuid_simple()),
            records: Vec::new(),
            total_co2: 0.0,
            total_co2e: 0.0,
            reduction_target: 0.0,
            progress_percentage: 0.0,
        }
    }

    pub fn add_record(&mut self, record: EmissionsRecord) {
        self.records.push(record);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_co2 = self.records.iter()
            .map(|r| r.co2_tons)
            .sum();
        self.total_co2e = self.records.iter()
            .map(|r| r.total_co2e())
            .sum();
    }

    pub fn compute_progress(&mut self, baseline: f64, target: f64) {
        self.reduction_target = target;
        if baseline > 0.0 {
            self.progress_percentage = ((baseline - self.total_co2e) / baseline * 100.0).max(0.0);
        }
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{:x}{:x}", duration.as_secs(), duration.subsec_nanos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emissions_record() {
        let mut record = EmissionsRecord::new(
            EmissionsSource::StationaryCombustion,
            "Steel Plant A",
            "2025-Q1",
        );
        record.set_emissions(50000.0, 100.0, 50.0);
        assert!(record.total_co2e() > record.co2_tons);
    }

    #[test]
    fn test_emissions_inventory() {
        let mut inventory = EmissionsInventory::new();
        inventory.add_record(EmissionsRecord::new(
            EmissionsSource::IndustrialProcesses,
            "Cement Factory",
            "2025",
        ));
        assert!(inventory.total_co2 >= 0.0);
    }
}
