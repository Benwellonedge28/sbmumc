//! # SBMUMC Module 944: Carbon Capture
//! 
//! Technologies and frameworks for carbon dioxide capture.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaptureMethod {
    DirectAirCapture,
    PointSourceCapture,
   OceanBased,
    Mineralization,
    Biological,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureFacility {
    pub facility_id: String,
    pub method: CaptureMethod,
    pub capacity_tons_per_day: f64,
    pub energy_requirement: f64,
    pub cost_per_ton: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturePortfolio {
    pub portfolio_id: String,
    pub facilities: Vec<CaptureFacility>,
    pub total_capacity: f64,
    pub average_cost: f64,
}

impl CaptureFacility {
    pub fn new(method: CaptureMethod) -> Self {
        Self {
            facility_id: format!("cf_{}", uuid_simple()),
            method,
            capacity_tons_per_day: 0.0,
            energy_requirement: 0.0,
            cost_per_ton: 0.0,
        }
    }

    pub fn configure(&mut self, capacity: f64, energy: f64, cost: f64) {
        self.capacity_tons_per_day = capacity;
        self.energy_requirement = energy;
        self.cost_per_ton = cost;
    }

    pub fn efficiency(&self) -> f64 {
        if self.energy_requirement == 0.0 { 0.0 }
        else { (self.capacity_tons_per_day / self.energy_requirement).min(1.0) }
    }
}

impl CapturePortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("portfolio_{}", uuid_simple()),
            facilities: Vec::new(),
            total_capacity: 0.0,
            average_cost: 0.0,
        }
    }

    pub fn add_facility(&mut self, facility: CaptureFacility) {
        self.facilities.push(facility);
        self.compute_totals();
    }

    pub fn compute_totals(&mut self) {
        self.total_capacity = self.facilities.iter()
            .map(|f| f.capacity_tons_per_day)
            .sum();
        if !self.facilities.is_empty() {
            self.average_cost = self.facilities.iter()
                .map(|f| f.cost_per_ton)
                .sum::<f64>() / self.facilities.len() as f64;
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
    fn test_capture_facility() {
        let mut facility = CaptureFacility::new(CaptureMethod::DirectAirCapture);
        facility.configure(1000.0, 1500.0, 400.0);
        assert!(facility.capacity_tons_per_day > 0.0);
    }

    #[test]
    fn test_capture_portfolio() {
        let mut portfolio = CapturePortfolio::new();
        portfolio.add_facility(CaptureFacility::new(CaptureMethod::OceanBased));
        assert!(portfolio.total_capacity >= 0.0);
    }
}
