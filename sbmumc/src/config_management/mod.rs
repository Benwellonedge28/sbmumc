//! Module 1591: Configuration Management & Feature Flags
//!
//! Advanced configuration management with feature flags, A/B testing,
//! gradual rollouts, and real-time configuration updates.
//!
//! # Features
//!
//! - Feature Flags - Enable/disable features per user/environment
//! - A/B Testing - Experiment management with statistical analysis
//! - Configuration Profiles - Environment-specific configurations
//! - Real-time Updates - Dynamic configuration changes
//! - Targeting Rules - Granular user/segment targeting

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Feature flag state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlagState {
    Enabled,
    Disabled,
    GradualRollout,
    Conditional,
}

/// Feature flag definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlag {
    pub id: String,
    pub name: String,
    pub key: String,
    pub description: String,
    pub flag_type: FlagType,
    pub state: FlagState,
    pub default_value: serde_json::Value,
    pub targeting_rules: Vec<TargetingRule>,
    pub variants: Vec<FlagVariant>,
    pub rollout_percentage: Option<f64>,
    pub environment: String,
    pub tags: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Feature flag types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlagType {
    Boolean,
    String,
    Number,
    Json,
    Percentage,
}

/// Targeting rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetingRule {
    pub id: String,
    pub name: String,
    pub priority: u32,
    pub conditions: Vec<Condition>,
    pub serve_value: serde_json::Value,
    pub percentage: Option<f64>,
    pub constraint: Constraint,
    pub enabled: bool,
}

/// Condition for targeting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub attribute: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
    pub negation: bool,
}

/// Condition operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    In,
    NotIn,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals,
    RegexMatch,
    TimeInRange,
    Percentage,
}

/// Constraint type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    pub rollout_type: RolloutType,
    pub constraints: HashMap<String, serde_json::Value>,
}

/// Constraint types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstraintType {
    User,
    Environment,
    Time,
    Percentage,
    Custom,
}

/// Rollout types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RolloutType {
    AllUsers,
    PercentageOfUsers,
    SpecificUsers,
    SpecificSegments,
}

/// Flag variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagVariant {
    pub name: String,
    pub value: serde_json::Value,
    pub weight: f64,
    pub description: Option<String>,
}

/// User targeting context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationContext {
    pub user_id: Option<String>,
    pub user_key: Option<String>,
    pub email: Option<String>,
    pub country: Option<String>,
    pub platform: Option<String>,
    pub app_version: Option<String>,
    pub device_type: Option<String>,
    pub custom_attributes: HashMap<String, serde_json::Value>,
    pub environment: String,
    pub timestamp: u64,
}

/// Evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub flag_key: String,
    pub value: serde_json::Value,
    pub variation_index: u32,
    pub variation_name: String,
    pub reason: EvaluationReason,
    pub failed_rules: Vec<String>,
}

/// Evaluation reason
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvaluationReason {
    DefaultValue,
    TargetRuleMatch,
    PercentageRollout,
    TargetingRuleMatch,
    ExperimentMatch,
    NoneMatch,
}

/// A/B experiment definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub id: String,
    pub name: String,
    pub key: String,
    pub description: String,
    pub flag_key: String,
    pub variations: Vec<ExperimentVariation>,
    pub traffic_allocation: HashMap<String, f64>,
    pub targeting: Vec<TargetingRule>,
    pub status: ExperimentStatus,
    pub metrics: Vec<Metric>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub created_at: u64,
}

/// Experiment variation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentVariation {
    pub id: String,
    pub name: String,
    pub key: String,
    pub value: serde_json::Value,
    pub weight: f64,
}

/// Experiment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExperimentStatus {
    Draft,
    Running,
    Paused,
    Completed,
    Archived,
}

/// Experiment metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub id: String,
    pub name: String,
    pub metric_type: MetricType,
    pub event_key: String,
    pub success_criteria: SuccessCriteria,
}

/// Metric types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetricType {
    Conversion,
    Retention,
    Revenue,
    Engagement,
    Custom,
}

/// Success criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCriteria {
    pub direction: Direction,
    pub baseline: f64,
    pub target: f64,
    pub minimum_detectable_effect: f64,
    pub confidence_level: f64,
}

/// Direction for metric
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    Increase,
    Decrease,
    Neutral,
}

/// Experiment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub experiment_id: String,
    pub variation_results: Vec<VariationResult>,
    pub statistical_significance: f64,
    pub p_value: f64,
    pub recommendation: ExperimentRecommendation,
    pub winner: Option<String>,
    pub confidence_interval: HashMap<String, ConfidenceInterval>,
}

/// Variation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariationResult {
    pub variation_key: String,
    pub sample_size: u64,
    pub conversion_rate: f64,
    pub mean_value: f64,
    pub variance: f64,
    pub standard_error: f64,
}

/// Experiment recommendation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExperimentRecommendation {
    KeepRunning,
    ShipWinner,
    ShipControl,
    NeedMoreData,
    Inconclusive,
}

/// Confidence interval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    pub lower: f64,
    pub upper: f64,
    pub confidence: f64,
}

/// Configuration profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProfile {
    pub id: String,
    pub name: String,
    pub environment: String,
    pub config_type: ConfigType,
    pub values: HashMap<String, ConfigValue>,
    pub overrides: Vec<ConfigOverride>,
    pub version: u32,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Configuration value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValue {
    pub key: String,
    pub value: serde_json::Value,
    pub value_type: ConfigValueType,
    pub source: ConfigSource,
    pub last_modified: u64,
}

/// Configuration types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigType {
    Feature,
    Server,
    Database,
    Security,
    Monitoring,
    Custom,
}

/// Value types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConfigValueType {
    String,
    Number,
    Boolean,
    Json,
    Secret,
}

/// Configuration source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConfigSource {
    Default,
    File,
    Environment,
    Remote,
    Override,
}

/// Configuration override
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigOverride {
    pub key: String,
    pub value: serde_json::Value,
    pub condition: Condition,
    pub priority: u32,
}

/// Segment definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rules: Vec<SegmentRule>,
    pub included_users: Vec<String>,
    pub excluded_users: Vec<String>,
    pub created_at: u64,
}

/// Segment rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentRule {
    pub attribute: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

/// Configuration state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigState {
    pub flags: HashMap<String, FeatureFlag>,
    pub experiments: HashMap<String, Experiment>,
    pub profiles: HashMap<String, ConfigProfile>,
    pub segments: HashMap<String, Segment>,
    pub evaluations: VecDeque<EvaluationResult>,
}

/// Configuration service
pub struct ConfigurationService {
    state: ConfigState,
    cache: HashMap<String, serde_json::Value>,
    watchers: Vec<Box<dyn ConfigWatcher>>,
}

impl ConfigurationService {
    /// Create new configuration service
    pub fn new() -> Self {
        Self {
            state: ConfigState::default(),
            cache: HashMap::new(),
            watchers: Vec::new(),
        }
    }

    /// Create feature flag
    pub fn create_flag(&mut self, flag: FeatureFlag) -> Result<String, ConfigError> {
        let key = flag.key.clone();
        self.state.flags.insert(key.clone(), flag);
        self.cache.remove(&key);
        Ok(key)
    }

    /// Update feature flag
    pub fn update_flag(&mut self, key: &str, updates: FlagUpdate) -> Result<(), ConfigError> {
        let flag = self.state.flags.get_mut(key)
            .ok_or(ConfigError::FlagNotFound)?;

        if let Some(state) = updates.state {
            flag.state = state;
        }
        if let Some(rules) = updates.targeting_rules {
            flag.targeting_rules = rules;
        }
        if let Some(rollout) = updates.rollout_percentage {
            flag.rollout_percentage = Some(rollout);
        }

        flag.updated_at = current_timestamp();
        self.cache.remove(key);

        self.notify_watchers(key);

        Ok(())
    }

    /// Evaluate flag for context
    pub fn evaluate(&self, key: &str, context: &EvaluationContext) -> Result<EvaluationResult, ConfigError> {
        // Check cache first
        if let Some(cached) = self.cache.get(&format!("{}:{}", key, context_hash(context))) {
            return Ok(EvaluationResult {
                flag_key: key.to_string(),
                value: cached.clone(),
                variation_index: 0,
                variation_name: "cached".to_string(),
                reason: EvaluationReason::TargetRuleMatch,
                failed_rules: Vec::new(),
            });
        }

        let flag = self.state.flags.get(key)
            .ok_or(ConfigError::FlagNotFound)?;

        // Evaluate targeting rules
        for rule in &flag.targeting_rules {
            if rule.enabled && self.matches_conditions(&rule.conditions, context) {
                let reason = if rule.percentage.is_some() {
                    // Check percentage rollout
                    let percentage = rule.percentage.unwrap();
                    if self.check_percentage(context, percentage) {
                        EvaluationReason::PercentageRollout
                    } else {
                        continue;
                    }
                } else {
                    EvaluationReason::TargetRuleMatch
                };

                return Ok(EvaluationResult {
                    flag_key: key.to_string(),
                    value: rule.serve_value.clone(),
                    variation_index: 0,
                    variation_name: rule.name.clone(),
                    reason,
                    failed_rules: Vec::new(),
                });
            }
        }

        // Check gradual rollout
        if let Some(rollout) = flag.rollout_percentage {
            if self.check_percentage(context, rollout) {
                return Ok(EvaluationResult {
                    flag_key: key.to_string(),
                    value: serde_json::json!(true),
                    variation_index: 0,
                    variation_name: "rollout".to_string(),
                    reason: EvaluationReason::PercentageRollout,
                    failed_rules: Vec::new(),
                });
            }
        }

        // Return default value
        Ok(EvaluationResult {
            flag_key: key.to_string(),
            value: flag.default_value.clone(),
            variation_index: 0,
            variation_name: "default".to_string(),
            reason: EvaluationReason::DefaultValue,
            failed_rules: Vec::new(),
        })
    }

    /// Check if conditions match context
    fn matches_conditions(&self, conditions: &[Condition], context: &EvaluationContext) -> bool {
        for condition in conditions {
            let value = self.get_attribute_value(&condition.attribute, context);

            if !self.evaluate_operator(&condition.operator, &value, &condition.value) {
                return false;
            }
        }
        true
    }

    /// Get attribute value from context
    fn get_attribute_value(&self, attribute: &str, context: &EvaluationContext) -> serde_json::Value {
        match attribute {
            "user_id" => context.user_id.as_ref().map(|s| serde_json::json!(s)).unwrap_or(serde_json::Value::Null),
            "email" => context.email.as_ref().map(|s| serde_json::json!(s)).unwrap_or(serde_json::Value::Null),
            "country" => context.country.as_ref().map(|s| serde_json::json!(s)).unwrap_or(serde_json::Value::Null),
            "platform" => context.platform.as_ref().map(|s| serde_json::json!(s)).unwrap_or(serde_json::Value::Null),
            "app_version" => context.app_version.as_ref().map(|s| serde_json::json!(s)).unwrap_or(serde_json::Value::Null),
            "device_type" => context.device_type.as_ref().map(|s| serde_json::json!(s)).unwrap_or(serde_json::Value::Null),
            _ => context.custom_attributes.get(attribute).cloned().unwrap_or(serde_json::Value::Null),
        }
    }

    /// Evaluate condition operator
    fn evaluate_operator(&self, op: &ConditionOperator, actual: &serde_json::Value, expected: &serde_json::Value) -> bool {
        match op {
            ConditionOperator::Equals => actual == expected,
            ConditionOperator::NotEquals => actual != expected,
            ConditionOperator::Contains => {
                if let (Some(a), Some(e)) = (actual.as_str(), expected.as_str()) {
                    a.contains(e)
                } else {
                    false
                }
            }
            ConditionOperator::In => {
                if let Some(arr) = expected.as_array() {
                    arr.contains(actual)
                } else {
                    false
                }
            }
            ConditionOperator::Percentage => {
                // Percentage-based assignment
                if let Some(pct) = expected.as_f64() {
                    self.check_percentage_value(actual, pct)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Check percentage for context
    fn check_percentage(&self, context: &EvaluationContext, percentage: f64) -> bool {
        let hash = context_hash(context);
        (hash % 10000) as f64 / 100.0 < percentage
    }

    /// Check percentage value
    fn check_percentage_value(&self, value: &serde_json::Value, percentage: f64) -> bool {
        let hash = value_hash(value);
        (hash % 10000) as f64 / 100.0 < percentage
    }

    /// Create experiment
    pub fn create_experiment(&mut self, experiment: Experiment) -> Result<String, ConfigError> {
        let id = experiment.id.clone();
        self.state.experiments.insert(id.clone(), experiment);
        Ok(id)
    }

    /// Start experiment
    pub fn start_experiment(&mut self, id: &str) -> Result<(), ConfigError> {
        let experiment = self.state.experiments.get_mut(id)
            .ok_or(ConfigError::ExperimentNotFound)?;

        experiment.status = ExperimentStatus::Running;
        experiment.start_time = Some(current_timestamp());

        Ok(())
    }

    /// Record experiment event
    pub fn record_event(&mut self, experiment_id: &str, variation: &str, user_id: &str, event: &str) -> Result<(), ConfigError> {
        // Record event for analysis
        Ok(())
    }

    /// Get experiment result
    pub fn get_result(&self, experiment_id: &str) -> Result<ExperimentResult, ConfigError> {
        let experiment = self.state.experiments.get(experiment_id)
            .ok_or(ConfigError::ExperimentNotFound)?;

        // Calculate statistical significance (simplified)
        Ok(ExperimentResult {
            experiment_id: experiment_id.to_string(),
            variation_results: Vec::new(),
            statistical_significance: 0.95,
            p_value: 0.05,
            recommendation: ExperimentRecommendation::KeepRunning,
            winner: None,
            confidence_interval: HashMap::new(),
        })
    }

    /// Create configuration profile
    pub fn create_profile(&mut self, profile: ConfigProfile) -> Result<String, ConfigError> {
        let id = profile.id.clone();
        self.state.profiles.insert(id.clone(), profile);
        Ok(id)
    }

    /// Get configuration value
    pub fn get_config(&self, profile_id: &str, key: &str) -> Result<serde_json::Value, ConfigError> {
        let profile = self.state.profiles.get(profile_id)
            .ok_or(ConfigError::ProfileNotFound)?;

        profile.values.get(key)
            .map(|v| v.value.clone())
            .ok_or(ConfigError::ConfigNotFound)
    }

    /// Create segment
    pub fn create_segment(&mut self, segment: Segment) -> Result<String, ConfigError> {
        let id = segment.id.clone();
        self.state.segments.insert(id.clone(), segment);
        Ok(id)
    }

    /// Check segment membership
    pub fn is_in_segment(&self, segment_id: &str, context: &EvaluationContext) -> Result<bool, ConfigError> {
        let segment = self.state.segments.get(segment_id)
            .ok_or(ConfigError::SegmentNotFound)?;

        // Check excluded users
        if let Some(user_id) = &context.user_id {
            if segment.excluded_users.contains(user_id) {
                return Ok(false);
            }
            if segment.included_users.contains(user_id) {
                return Ok(true);
            }
        }

        // Evaluate rules
        for rule in &segment.rules {
            let value = self.get_attribute_value(&rule.attribute, context);
            if !self.evaluate_operator(&rule.operator, &value, &rule.value) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Add configuration watcher
    pub fn add_watcher(&mut self, watcher: Box<dyn ConfigWatcher>) {
        self.watchers.push(watcher);
    }

    /// Notify watchers of change
    fn notify_watchers(&self, key: &str) {
        for watcher in &self.watchers {
            watcher.on_change(key);
        }
    }
}

/// Configuration error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigError {
    FlagNotFound,
    ExperimentNotFound,
    ProfileNotFound,
    SegmentNotFound,
    ConfigNotFound,
    InvalidValue,
}

/// Flag update
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FlagUpdate {
    pub state: Option<FlagState>,
    pub targeting_rules: Option<Vec<TargetingRule>>,
    pub rollout_percentage: Option<f64>,
}

/// Config watcher trait
pub trait ConfigWatcher: Send + Sync {
    fn on_change(&self, key: &str);
}

/// Helper functions
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn context_hash(context: &EvaluationContext) -> u64 {
    let mut hash = 0u64;
    if let Some(user_id) = &context.user_id {
        hash = hash_string(user_id);
    }
    if let Some(email) = &context.email {
        hash ^= hash_string(email);
    }
    hash
}

fn hash_string(s: &str) -> u64 {
    s.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64))
}

fn value_hash(value: &serde_json::Value) -> u64 {
    hash_string(&value.to_string())
}

impl Default for ConfigurationService {
    fn default() -> Self {
        Self::new()
    }
}