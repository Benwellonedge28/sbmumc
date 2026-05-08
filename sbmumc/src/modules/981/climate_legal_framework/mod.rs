//! # SBMUMC Module 981: Climate Legal Framework
//! 
//! Legal frameworks and regulations for climate action.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegalInstrument {
    CarbonTax,
    CapAndTrade,
    EmissionStandards,
    ReportingMandate,
    LiabilityRules,
    ClimateFinanceObligation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateLaw {
    pub law_id: String,
    pub name: String,
    pub jurisdiction: String,
    pub instrument_type: LegalInstrument,
    pub effective_date: String,
    pub compliance_entities: u32,
    pub enforcement_strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateLegalSystem {
    pub system_id: String,
    pub laws: Vec<ClimateLaw>,
    pub total_regulations: u32,
    pub coverage_score: f64,
    pub enforcement_index: f64,
}

impl ClimateLaw {
    pub fn new(name: &str, jurisdiction: &str, instrument: LegalInstrument) -> Self {
        Self {
            law_id: format!("cl_{}", uuid_simple()),
            name: name.to_string(),
            jurisdiction: jurisdiction.to_string(),
            instrument_type: instrument,
            effective_date: String::new(),
            compliance_entities: 0,
            enforcement_strength: 0.0,
        }
    }

    pub fn set_parameters(&mut self, date: &str, entities: u32, enforcement: f64) {
        self.effective_date = date.to_string();
        self.compliance_entities = entities;
        self.enforcement_strength = enforcement.clamp(0.0, 1.0);
    }
}

impl ClimateLegalSystem {
    pub fn new() -> Self {
        Self {
            system_id: format!("cls_{}", uuid_simple()),
            laws: Vec::new(),
            total_regulations: 0,
            coverage_score: 0.0,
            enforcement_index: 0.0,
        }
    }

    pub fn add_law(&mut self, law: ClimateLaw) {
        self.laws.push(law);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_regulations = self.laws.len() as u32;
        self.enforcement_index = self.laws.iter()
            .map(|l| l.enforcement_strength)
            .sum::<f64>() / self.laws.len().max(1) as f64;
        self.coverage_score = (self.total_regulations as f64 / 50.0).min(1.0);
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
    fn test_climate_law() {
        let mut law = ClimateLaw::new(
            "EU Emissions Trading System",
            "European Union",
            LegalInstrument::CapAndTrade,
        );
        law.set_parameters("2025-01-01", 10000, 0.8);
        assert!(law.enforcement_strength > 0.7);
    }

    #[test]
    fn test_climate_legal_system() {
        let mut system = ClimateLegalSystem::new();
        system.add_law(ClimateLaw::new(
            "California Cap-and-Trade",
            "California",
            LegalInstrument::CapAndTrade,
        ));
        assert!(system.total_regulations > 0);
    }
}
