//!
//! # SBMUMC Module 1563: Observability and Metrics Collection
//!
//! Comprehensive metrics collection with Prometheus integration,
//! custom dashboards, alerting rules, and performance analytics.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Metric type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
    SummaryWithQuantiles,
}

/// Metric value with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    pub value: f64,
    pub timestamp: u64,
    pub labels: HashMap<String, String>,
}

/// Metric definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDefinition {
    pub name: String,
    pub metric_type: MetricType,
    pub description: String,
    pub unit: Option<String>,
    pub labels: Vec<String>,
    pub buckets: Option<Vec<f64>>,
    pub quantiles: Option<Vec<f64>>,
}

/// Collected metric sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSample {
    pub name: String,
    pub value: f64,
    pub timestamp: u64,
    pub labels: HashMap<String, String>,
}

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub id: String,
    pub name: String,
    pub panels: Vec<Panel>,
    pub variables: Vec<DashboardVariable>,
    pub refresh_interval_secs: u32,
    pub time_range: TimeRange,
}

/// Dashboard panel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    pub id: String,
    pub title: String,
    pub visualization: VisualizationType,
    pub query: String,
    pub position: PanelPosition,
    pub options: PanelOptions,
}

/// Panel visualization types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VisualizationType {
    Graph,
    Gauge,
    Stat,
    Table,
    Heatmap,
    PieChart,
    BarChart,
    Text,
}

/// Panel layout position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelPosition {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Panel display options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelOptions {
    pub show_legend: bool,
    pub show_grid: bool,
    pub thresholds: Option<Vec<Threshold>>,
    pub color_scheme: Option<String>,
}

/// Threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threshold {
    pub value: f64,
    pub color: String,
    pub label: Option<String>,
}

/// Dashboard variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardVariable {
    pub name: String,
    pub var_type: VariableType,
    pub query: Option<String>,
    pub options: Vec<String>,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VariableType {
    Query,
    Interval,
    Custom,
}

/// Time range for dashboards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub from: String,
    pub to: String,
}

/// Alert rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: String,
    pub name: String,
    pub condition: AlertCondition,
    pub duration: Duration,
    pub severity: AlertSeverity,
    pub annotations: HashMap<String, String>,
    pub labels: HashMap<String, String>,
    pub actions: Vec<AlertAction>,
    pub enabled: bool,
}

/// Alert condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCondition {
    pub metric: String,
    pub operator: ComparisonOperator,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Alert action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertAction {
    pub action_type: ActionType,
    pub target: String,
    pub config: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionType {
    Email,
    Webhook,
    Slack,
    PagerDuty,
    Custom,
}

/// Alert instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub rule_id: String,
    pub status: AlertStatus,
    pub started_at: u64,
    pub ended_at: Option<u64>,
    pub values: HashMap<String, f64>,
    pub annotations: HashMap<String, String>,
}

/// Alert status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertStatus {
    Pending,
    Firing,
    Resolved,
}

/// Metrics collector
pub struct MetricsCollector {
    registry: Arc<RwLock<HashMap<String, MetricDefinition>>>,
    storage: Arc<RwLock<HashMap<String, Vec<MetricSample>>>>,
    aggregation_window: Duration,
}

impl MetricsCollector {
    pub fn new(aggregation_window_secs: u64) -> Self {
        Self {
            registry: Arc::new(RwLock::new(HashMap::new())),
            storage: Arc::new(RwLock::new(HashMap::new())),
            aggregation_window: Duration::from_secs(aggregation_window_secs),
        }
    }

    /// Register a new metric
    pub fn register_metric(&self, definition: MetricDefinition) -> Result<(), MetricsError> {
        let mut registry = self.registry.write().unwrap();

        if registry.contains_key(&definition.name) {
            return Err(MetricsError::MetricAlreadyExists);
        }

        registry.insert(definition.name.clone(), definition);
        Ok(())
    }

    /// Record a metric value
    pub fn record(&self, name: &str, value: f64, labels: HashMap<String, String>) -> Result<(), MetricsError> {
        let registry = self.registry.read().unwrap();

        if !registry.contains_key(name) {
            return Err(MetricsError::MetricNotFound);
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let sample = MetricSample {
            name: name.to_string(),
            value,
            timestamp,
            labels,
        };

        let mut storage = self.storage.write().unwrap();
        storage.entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(sample);

        Ok(())
    }

    /// Increment counter
    pub fn increment_counter(&self, name: &str, labels: HashMap<String, String>) -> Result<(), MetricsError> {
        self.record(name, 1.0, labels)
    }

    /// Set gauge value
    pub fn set_gauge(&self, name: &str, value: f64, labels: HashMap<String, String>) -> Result<(), MetricsError> {
        self.record(name, value, labels)
    }

    /// Observe histogram value
    pub fn observe_histogram(&self, name: &str, value: f64, labels: HashMap<String, String>) -> Result<(), MetricsError> {
        self.record(name, value, labels)
    }

    /// Get current metric value
    pub fn get_current(&self, name: &str, labels: Option<HashMap<String, String>>) -> Result<Vec<MetricSample>, MetricsError> {
        let storage = self.storage.read().unwrap();

        if let Some(samples) = storage.get(name) {
            match labels {
                Some(l) => Ok(samples.iter()
                    .filter(|s| s.labels.iter().all(|(k, v)| l.get(k) == Some(v))
                    .cloned()
                    .collect()),
                None => Ok(samples.clone()),
            }
        } else {
            Err(MetricsError::MetricNotFound)
        }
    }

    /// Get aggregated statistics
    pub fn get_statistics(&self, name: &str, duration: Duration) -> Result<MetricStatistics, MetricsError> {
        let storage = self.storage.read().unwrap();

        if let Some(samples) = storage.get(name) {
            let cutoff = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64 - duration.as_millis() as u64;

            let values: Vec<f64> = samples
                .iter()
                .filter(|s| s.timestamp >= cutoff)
                .map(|s| s.value)
                .collect();

            if values.is_empty() {
                return Err(MetricsError::NoData);
            }

            let sum: f64 = values.iter().sum();
            let count = values.len() as f64;
            let mean = sum / count;

            let mut sorted = values.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let min = *sorted.first().unwrap();
            let max = *sorted.last().unwrap();
            let median = sorted[sorted.len() / 2];

            Ok(MetricStatistics {
                count,
                sum,
                mean,
                min,
                max,
                median,
                std_dev: Self::calculate_std_dev(&values, mean),
                p50: median,
                p90: sorted[(sorted.len() as f64 * 0.9) as usize],
                p95: sorted[(sorted.len() as f64 * 0.95) as usize],
                p99: sorted[(sorted.len() as f64 * 0.99) as usize],
            })
        } else {
            Err(MetricsError::MetricNotFound)
        }
    }

    fn calculate_std_dev(values: &[f64], mean: f64) -> f64 {
        let variance: f64 = values.iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        variance.sqrt()
    }

    /// Export metrics in Prometheus format
    pub fn export_prometheus(&self) -> String {
        let registry = self.registry.read().unwrap();
        let storage = self.storage.read().unwrap();

        let mut output = String::new();

        for (name, def) in registry.iter() {
            output.push_str(&format!("# HELP {} {}\n", name, def.description));
            output.push_str(&format!("# TYPE {} {}\n", name, match def.metric_type {
                MetricType::Counter => "counter",
                MetricType::Gauge => "gauge",
                MetricType::Histogram => "histogram",
                MetricType::Summary => "summary",
                MetricType::SummaryWithQuantiles => "summary",
            }));

            if let Some(samples) = storage.get(name) {
                for sample in samples {
                    let labels: String = if sample.labels.is_empty() {
                        String::new()
                    } else {
                        format!("{{{}}}",
                            sample.labels.iter()
                                .map(|(k, v)| format!("{}=\"{}\"", k, v))
                                .collect::<Vec<_>>()
                                .join(",")
                        )
                    };
                    output.push_str(&format!("{} {}{}\n", name, sample.value, labels));
                }
            }
        }

        output
    }

    /// Clean up old data
    pub fn cleanup(&self, retention: Duration) -> usize {
        let cutoff = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 - retention.as_millis() as u64;

        let mut storage = self.storage.write().unwrap();
        let mut removed = 0;

        for samples in storage.values_mut() {
            let original_len = samples.len();
            samples.retain(|s| s.timestamp >= cutoff);
            removed += original_len - samples.len();
        }

        removed
    }
}

/// Metric statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStatistics {
    pub count: f64,
    pub sum: f64,
    pub mean: f64,
    pub min: f64,
    pub max: f64,
    pub median: f64,
    pub std_dev: f64,
    pub p50: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
}

/// Metrics error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricsError {
    MetricNotFound,
    MetricAlreadyExists,
    NoData,
    InvalidValue,
}

impl std::fmt::Display for MetricsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricsError::MetricNotFound => write!(f, "Metric not found"),
            MetricsError::MetricAlreadyExists => write!(f, "Metric already exists"),
            MetricsError::NoData => write!(f, "No data available"),
            MetricsError::InvalidValue => write!(f, "Invalid metric value"),
        }
    }
}

impl std::error::Error for MetricsError {}

/// Alert manager
pub struct AlertManager {
    rules: Arc<RwLock<HashMap<String, AlertRule>>>,
    active_alerts: Arc<RwLock<HashMap<String, Alert>>>,
    collector: Arc<MetricsCollector>,
    notification_handlers: Arc<RwLock<HashMap<ActionType, Vec<fn(Alert)>>>>,
}

impl AlertManager {
    pub fn new(collector: Arc<MetricsCollector>) -> Self {
        Self {
            rules: Arc::new(RwLock::new(HashMap::new())),
            active_alerts: Arc::new(RwLock::new(HashMap::new())),
            collector,
            notification_handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create alert rule
    pub fn create_rule(&self, rule: AlertRule) -> String {
        let rule_id = Uuid::new_v4().to_string();
        let mut rules = self.rules.write().unwrap();
        rules.insert(rule_id.clone(), rule);
        rule_id
    }

    /// Evaluate alert rules
    pub fn evaluate(&self) -> Vec<Alert> {
        let rules = self.rules.read().unwrap();
        let mut triggered = Vec::new();

        for (rule_id, rule) in rules.iter() {
            if let Ok(stats) = self.collector.get_statistics(&rule.condition.metric, rule.duration) {
                let should_alert = match rule.condition.operator {
                    ComparisonOperator::GreaterThan => stats.mean > rule.condition.value,
                    ComparisonOperator::LessThan => stats.mean < rule.condition.value,
                    ComparisonOperator::Equal => (stats.mean - rule.condition.value).abs() < f64::EPSILON,
                    ComparisonOperator::NotEqual => (stats.mean - rule.condition.value).abs() > f64::EPSILON,
                    ComparisonOperator::GreaterThanOrEqual => stats.mean >= rule.condition.value,
                    ComparisonOperator::LessThanOrEqual => stats.mean <= rule.condition.value,
                };

                if should_alert {
                    let alert = self.create_alert(rule_id, &rule, stats);
                    triggered.push(alert);
                }
            }
        }

        triggered
    }

    fn create_alert(&self, rule_id: &str, rule: &AlertRule, stats: MetricStatistics) -> Alert {
        let alert_id = Uuid::new_v4().to_string();
        let mut values = HashMap::new();
        values.insert("mean".to_string(), stats.mean);
        values.insert("max".to_string(), stats.max);

        Alert {
            id: alert_id,
            rule_id: rule_id.to_string(),
            status: AlertStatus::Pending,
            started_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            ended_at: None,
            values,
            annotations: rule.annotations.clone(),
        }
    }

    /// Fire alert
    pub fn fire_alert(&self, alert: Alert) {
        let mut alerts = self.active_alerts.write().unwrap();
        alerts.insert(alert.id.clone(), alert.clone());

        // Trigger notifications
        let rules = self.rules.read().unwrap();
        if let Some(rule) = rules.get(&alert.rule_id) {
            for action in &rule.actions {
                self.send_notification(&alert, action);
            }
        }
    }

    fn send_notification(&self, alert: &Alert, action: &AlertAction) {
        // In real implementation, send notification via configured handler
        println!("Alert notification: {:?} -> {:?}", action.action_type, action.target);
    }
}

/// Dashboard service
pub struct DashboardService {
    dashboards: Arc<RwLock<HashMap<String, Dashboard>>>,
    collector: Arc<MetricsCollector>,
}

impl DashboardService {
    pub fn new(collector: Arc<MetricsCollector>) -> Self {
        Self {
            dashboards: Arc::new(RwLock::new(HashMap::new())),
            collector,
        }
    }

    /// Create dashboard
    pub fn create_dashboard(&self, dashboard: Dashboard) -> String {
        let mut dashboards = self.dashboards.write().unwrap();
        dashboards.insert(dashboard.id.clone(), dashboard.clone());
        dashboard.id
    }

    /// Get dashboard
    pub fn get_dashboard(&self, id: &str) -> Option<Dashboard> {
        let dashboards = self.dashboards.read().unwrap();
        dashboards.get(id).cloned()
    }

    /// List dashboards
    pub fn list_dashboards(&self) -> Vec<Dashboard> {
        let dashboards = self.dashboards.read().unwrap();
        dashboards.values().cloned().collect()
    }
}

// Re-export types
pub use MetricDefinition;
pub use MetricSample;
pub use MetricStatistics;
pub use Dashboard;
pub use Panel;
pub use AlertRule;
pub use Alert;
pub use MetricsCollector;
pub use AlertManager;
pub use DashboardService;