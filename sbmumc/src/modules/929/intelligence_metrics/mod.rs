//! # SBMUMC Module 929: Intelligence Metrics
//! 
//! Measurement frameworks and metrics for assessing intelligence.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Cognitive,
    General,
    Domain,
    Emotional,
    Social,
    Creative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceScore {
    pub metric_type: MetricType,
    pub raw_score: f64,
    pub normalized_score: f64,
    pub confidence_interval: (f64, f64),
    pub assessment_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuite {
    pub suite_id: String,
    pub name: String,
    pub test_tasks: Vec<String>,
    pub human_baseline: f64,
    pub agi_baseline: f64,
    pub coverage: Vec<MetricType>,
}

impl IntelligenceScore {
    pub fn new(metric_type: MetricType, raw_score: f64) -> Self {
        let normalized = match metric_type {
            MetricType::Cognitive => (raw_score / 100.0).min(1.0),
            MetricType::General => (raw_score / 100.0).min(1.0),
            MetricType::Domain => (raw_score / 100.0).min(1.0),
            MetricType::Emotional => (raw_score / 100.0).min(1.0),
            MetricType::Social => (raw_score / 100.0).min(1.0),
            MetricType::Creative => (raw_score / 100.0).min(1.0),
        };
        Self {
            metric_type,
            raw_score,
            normalized_score: normalized,
            confidence_interval: (normalized * 0.9, normalized * 1.1),
            assessment_method: "standardized".to_string(),
        }
    }

    pub fn compute_composite(&self, other: &IntelligenceScore) -> f64 {
        (self.normalized_score + other.normalized_score) / 2.0
    }
}

impl BenchmarkSuite {
    pub fn new(name: &str) -> Self {
        Self {
            suite_id: format!("benchmark_{}", uuid_simple()),
            name: name.to_string(),
            test_tasks: Vec::new(),
            human_baseline: 0.0,
            agi_baseline: 0.0,
            coverage: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: &str) {
        self.test_tasks.push(task.to_string());
    }

    pub fn compute_coverage(&self) -> f64 {
        self.test_tasks.len() as f64 / 100.0
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
    fn test_intelligence_score() {
        let score = IntelligenceScore::new(MetricType::General, 85.0);
        assert!(score.normalized_score > 0.8);
    }

    #[test]
    fn test_benchmark_suite() {
        let mut suite = BenchmarkSuite::new("GLUE-AI");
        suite.add_task("Natural Language Inference");
        suite.add_task("Question Answering");
        assert!(suite.test_tasks.len() == 2);
    }
}
