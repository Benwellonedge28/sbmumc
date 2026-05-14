//! # SBMUMC Module 1549: Feedback Loop Monitor
//!
//! Real-time monitoring and feedback loop for continuous improvement

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackMetric {
    pub metric_name: String,
    pub value: f64,
    pub timestamp: i64,
    pub trend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackReport {
    pub report_id: String,
    pub metrics: Vec<FeedbackMetric>,
    pub overall_score: f64,
    pub recommendations: Vec<String>,
}

pub struct FeedbackLoopMonitor {
    pub monitor_id: String,
    pub metrics_history: Vec<FeedbackMetric>,
}

impl FeedbackLoopMonitor {
    pub fn new() -> Self {
        Self {
            monitor_id: crate::core::uuid_simple(),
            metrics_history: Vec::new(),
        }
    }

    pub fn record_metric(&mut self, name: &str, value: f64) {
        self.metrics_history.push(FeedbackMetric {
            metric_name: name.to_string(),
            value,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            trend: "stable".to_string(),
        });
    }

    pub fn generate_report(&self) -> FeedbackReport {
        let avg_score = if self.metrics_history.is_empty() {
            0.0
        } else {
            self.metrics_history.iter().map(|m| m.value).sum::<f64>() / self.metrics_history.len() as f64
        };

        FeedbackReport {
            report_id: crate::core::uuid_simple(),
            metrics: self.metrics_history.clone(),
            overall_score: avg_score,
            recommendations: vec![
                "Continue monitoring performance".to_string(),
                "Review metrics for anomalies".to_string(),
            ],
        }
    }

    pub fn detect_anomaly(&self) -> Option<String> {
        if self.metrics_history.len() > 10 {
            let recent = &self.metrics_history[self.metrics_history.len() - 5..];
            let avg: f64 = recent.iter().map(|m| m.value).sum::<f64>() / recent.len() as f64;
            if avg < 0.5 {
                return Some("Anomaly detected: performance degradation".to_string());
            }
        }
        None
    }
}

impl Default for FeedbackLoopMonitor {
    fn default() -> Self {
        Self::new()
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
    fn test_feedback_recording() {
        let mut monitor = FeedbackLoopMonitor::new();
        monitor.record_metric("latency", 0.95);
        monitor.record_metric("accuracy", 0.88);
        let report = monitor.generate_report();
        assert!(report.overall_score > 0.0);
    }
}