//! # SBMUMC Module 1597: Differential Privacy
//!
//! Mathematical framework for privacy-preserving data analysis.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DPConfig {
    pub epsilon: f64,
    pub delta: f64,
    pub sensitivity: f64,
    pub mechanism: DPMechanism,
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DPMechanism {
    Laplace,
    Gaussian,
    Exponential,
    ReportNoisyMax,
    SampleAndAggregate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyBudget {
    pub epsilon_used: f64,
    pub delta_used: f64,
    pub remaining_epsilon: f64,
    pub remaining_delta: f64,
    pub compositions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DPQuery {
    pub query_id: String,
    pub query_type: QueryType,
    pub sensitivity: f64,
    pub data_range: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryType {
    Count,
    Sum,
    Mean,
    Variance,
    Histogram { bins: usize },
    Percentile { p: f64 },
}

pub struct DifferentialPrivacy {
    config: DPConfig,
    budget: PrivacyBudget,
    history: Vec<DPQuery>,
}

impl DifferentialPrivacy {
    pub fn new(config: DPConfig) -> Self {
        Self {
            budget: PrivacyBudget {
                epsilon_used: 0.0,
                delta_used: 0.0,
                remaining_epsilon: config.epsilon,
                remaining_delta: config.delta,
                compositions: 0,
            },
            config,
            history: Vec::new(),
        }
    }

    pub fn add_noise_to_value(&self, value: f64) -> f64 {
        match self.config.mechanism {
            DPMechanism::Laplace => self.add_laplace_noise(value),
            DPMechanism::Gaussian => self.add_gaussian_noise(value),
            DPMechanism::Exponential => self.add_exponential_noise(value),
            _ => self.add_laplace_noise(value),
        }
    }

    fn add_laplace_noise(&self, value: f64) -> f64 {
        let scale = self.config.sensitivity / self.config.epsilon;
        let uniform1: f64 = rand::random();
        let uniform2: f64 = rand::random();
        let u = uniform1 - 0.5;
        let noise = -scale * u.signum() * (1.0 - (2.0 * uniform2).abs()).ln();
        value + noise
    }

    fn add_gaussian_noise(&self, value: f64) -> f64 {
        let sigma = self.config.sensitivity * (2.0 * (1.0 + self.config.epsilon).ln() / self.config.epsilon).sqrt();
        let u1: f64 = rand::random();
        let u2: f64 = rand::random();
        let gaussian = ( -2.0 * u1.min(1.0 - u1).max(0.0001).ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        value + sigma * gaussian
    }

    fn add_exponential_noise(&self, value: f64) -> f64 {
        let rate = self.config.epsilon / self.config.sensitivity;
        let u: f64 = rand::random();
        let noise = -(1.0 / rate) * (1.0 - u).ln();
        value + noise
    }

    pub fn compute_with_dp(&mut self, query: &DPQuery, data: &[f64]) -> Result<DPResult> {
        if self.budget.remaining_epsilon < self.config.epsilon / 10.0 {
            return Err(SbmumcError::Internal("Privacy budget exhausted".into()));
        }

        let result = self.compute_query(query, data)?;
        let noisy_result = self.add_noise_to_value(result.value);

        self.budget.epsilon_used += self.config.epsilon / data.len().max(1) as f64;
        self.budget.remaining_epsilon -= self.config.epsilon / data.len().max(1) as f64;
        self.budget.compositions += 1;

        self.history.push(query.clone());

        Ok(DPResult {
            query_id: query.query_id.clone(),
            noisy_value: noisy_result,
            true_value: result.value,
            privacy_cost: self.config.epsilon,
            confidence: self.compute_confidence(),
        })
    }

    fn compute_query(&self, query: &DPQuery, data: &[f64]) -> Result<QueryResult> {
        if data.is_empty() {
            return Err(SbmumcError::Internal("Empty data".into()));
        }

        let value = match query.query_type {
            QueryType::Count => data.len() as f64,
            QueryType::Sum => data.iter().sum(),
            QueryType::Mean => data.iter().sum::<f64>() / data.len() as f64,
            QueryType::Variance => {
                let mean = data.iter().sum::<f64>() / data.len() as f64;
                data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64
            }
            QueryType::Histogram { bins } => {
                let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
                let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let range = max - min;
                let bin_width = range / bins as f64;
                (bins as f64 * bin_width).round()
            }
            QueryType::Percentile { p } => {
                let mut sorted = data.to_vec();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let idx = (p * sorted.len() as f64).floor() as usize;
                sorted.get(idx).copied().unwrap_or(0.0)
            }
        };

        Ok(QueryResult { value })
    }

    fn compute_confidence(&self) -> f64 {
        let base_confidence = 0.95;
        let budget_factor = (self.budget.remaining_epsilon / self.config.epsilon).max(0.0);
        base_confidence * budget_factor.min(1.0)
    }

    pub fn get_privacy_budget(&self) -> &PrivacyBudget {
        &self.budget
    }

    pub fn reset_budget(&mut self) {
        self.budget = PrivacyBudget {
            epsilon_used: 0.0,
            delta_used: 0.0,
            remaining_epsilon: self.config.epsilon,
            remaining_delta: self.config.delta,
            compositions: 0,
        };
    }

    pub fn compose_queries(&mut self, queries: usize) -> Result<()> {
        let composed_epsilon = self.config.epsilon * (queries as f64).sqrt();

        if composed_epsilon > self.budget.remaining_epsilon {
            return Err(SbmumcError::Internal(
                format!("Composed epsilon {} exceeds remaining {}", composed_epsilon, self.budget.remaining_epsilon)
            ));
        }

        self.budget.compositions += queries;
        self.budget.epsilon_used += composed_epsilon;
        self.budget.remaining_epsilon -= composed_epsilon;

        Ok(())
    }

    pub fn get_query_history(&self) -> &[DPQuery] {
        &self.history
    }

    pub fn compute_sensitivity_for_query(&self, query_type: &QueryType, max_change: f64) -> f64 {
        match query_type {
            QueryType::Count => 1.0 * max_change,
            QueryType::Sum => max_change,
            QueryType::Mean => max_change / 100.0,
            QueryType::Variance => max_change.powi(2),
            QueryType::Histogram { .. } => 1.0,
            QueryType::Percentile { .. } => max_change,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DPResult {
    pub query_id: String,
    pub noisy_value: f64,
    pub true_value: f64,
    pub privacy_cost: f64,
    pub confidence: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_differential_privacy() {
        let config = DPConfig {
            epsilon: 1.0,
            delta: 1e-5,
            sensitivity: 1.0,
            mechanism: DPMechanism::Laplace,
            batch_size: 100,
        };

        let mut dp = DifferentialPrivacy::new(config);

        let query = DPQuery {
            query_id: "test_query".to_string(),
            query_type: QueryType::Mean,
            sensitivity: 1.0,
            data_range: (0.0, 100.0),
        };

        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

        let result = dp.compute_with_dp(&query, &data);
        assert!(result.is_ok());

        let budget = dp.get_privacy_budget();
        assert!(budget.remaining_epsilon < config.epsilon);
    }

    #[test]
    fn test_gaussian_mechanism() {
        let config = DPConfig {
            epsilon: 0.5,
            delta: 1e-6,
            sensitivity: 1.0,
            mechanism: DPMechanism::Gaussian,
            batch_size: 100,
        };

        let dp = DifferentialPrivacy::new(config);
        let noisy = dp.add_noise_to_value(10.0);

        assert!((noisy - 10.0).abs() < 10.0);
    }
}