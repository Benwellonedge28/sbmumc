//! Differential Privacy Core Module (529)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifferentialPrivacyCore {
    pub dpc_id: String,
    pub privacy_budget: f64,
    pub epsilon: f64,
    pub delta: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyMechanism {
    pub mechanism_type: MechanismType,
    pub sensitivity: f64,
    pub noise_distribution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MechanismType {
    Laplace,
    Gaussian,
    Exponential,
    RandomizedResponse,
    ReportNoisyMax,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub result: f64,
    pub noise_added: f64,
    pub privacy_spent: f64,
}

impl DifferentialPrivacyCore {
    pub fn new() -> Self {
        Self {
            dpc_id: String::from("differential_privacy_core_v1"),
            privacy_budget: 1.0,
            epsilon: 0.1,
            delta: 1e-5,
        }
    }

    pub fn add_noise(&self, value: f64, mechanism: &PrivacyMechanism) -> QueryResult {
        let noise = match mechanism.mechanism_type {
            MechanismType::Laplace => 0.1,
            MechanismType::Gaussian => 0.05,
            _ => 0.2,
        };
        QueryResult {
            result: value + noise,
            noise_added: noise,
            privacy_spent: self.epsilon * mechanism.sensitivity,
        }
    }

    pub fn compose_queries(&self, queries: &[QueryResult]) -> f64 {
        queries.iter().map(|q| q.privacy_spent).sum()
    }
}

impl Default for DifferentialPrivacyCore {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_differential_privacy() {
        let dp = DifferentialPrivacyCore::new();
        let mechanism = PrivacyMechanism {
            mechanism_type: MechanismType::Laplace,
            sensitivity: 1.0,
            noise_distribution: String::from("laplace"),
        };
        let result = dp.add_noise(100.0, &mechanism);
        assert!(result.noise_added > 0.0);
    }
}
