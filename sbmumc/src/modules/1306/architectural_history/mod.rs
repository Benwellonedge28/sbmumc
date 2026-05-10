//! # SBMUMC Module 1306: Architectural History
//!
//! Systems for studying historical architectural periods.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HistoricalPeriod {
    Ancient,
    Classical,
    Medieval,
    Renaissance,
    Baroque,
    Industrial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitecturalHistorySystem {
    pub system_id: String,
    pub historical_period: HistoricalPeriod,
    pub historical_accuracy: f64,
    pub stylistic_purity: f64,
    pub contextual_integration: f64,
    pub scholarly_value: f64,
}

impl ArchitecturalHistorySystem {
    pub fn new(historical_period: HistoricalPeriod) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            historical_period,
            historical_accuracy: 0.0,
            stylistic_purity: 0.0,
            contextual_integration: 0.0,
            scholarly_value: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.historical_period {
            HistoricalPeriod::Ancient => {
                self.stylistic_purity = 0.90 + rand_simple() * 0.10;
                self.historical_accuracy = 0.85 + rand_simple() * 0.14;
                self.scholarly_value = 0.90 + rand_simple() * 0.10;
            },
            HistoricalPeriod::Classical => {
                self.stylistic_purity = 0.95 + rand_simple() * 0.05;
                self.historical_accuracy = 0.90 + rand_simple() * 0.10;
                self.contextual_integration = 0.85 + rand_simple() * 0.14;
            },
            HistoricalPeriod::Medieval => {
                self.historical_accuracy = 0.85 + rand_simple() * 0.14;
                self.contextual_integration = 0.90 + rand_simple() * 0.10;
                self.scholarly_value = 0.85 + rand_simple() * 0.14;
            },
            HistoricalPeriod::Renaissance => {
                self.stylistic_purity = 0.90 + rand_simple() * 0.10;
                self.scholarly_value = 0.95 + rand_simple() * 0.05;
                self.historical_accuracy = 0.90 + rand_simple() * 0.10;
            },
            HistoricalPeriod::Baroque => {
                self.stylistic_purity = 0.85 + rand_simple() * 0.14;
                self.contextual_integration = 0.80 + rand_simple() * 0.18;
                self.scholarly_value = 0.85 + rand_simple() * 0.14;
            },
            HistoricalPeriod::Industrial => {
                self.historical_accuracy = 0.90 + rand_simple() * 0.10;
                self.contextual_integration = 0.85 + rand_simple() * 0.14;
                self.stylistic_purity = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.scholarly_value == 0.0 {
            self.scholarly_value = (self.historical_accuracy + self.stylistic_purity) / 2.0 * (0.6 + rand_simple() * 0.3);
        }
        Ok(())
    }
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classical() {
        let mut system = ArchitecturalHistorySystem::new(HistoricalPeriod::Classical);
        system.analyze_system().unwrap();
        assert!(system.stylistic_purity > 0.8);
    }
}
