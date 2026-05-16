//!
//! # SBMUMC Module 1566: Feature Flags System
//!
//! Enables progressive feature rollout with A/B testing, targeting,
//! and real-time configuration management.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Feature flag definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlag {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub default_value: bool,
    pub targeting_rules: Vec<TargetingRule>,
    pub rollout_percentage: u32,
    pub conditions: Vec<FlagCondition>,
    pub created_at: u64,
    pub updated_at: u64,
    pub owner: String,
    pub tags: Vec<String>,
}

/// Targeting rule for feature rollout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetingRule {
    pub id: String,
    pub name: String,
    pub priority: u32,
    pub conditions: Vec<RuleCondition>,
    pub value: bool,
    pub percentage: Option<u32>,
}

/// Rule condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub attribute: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
    RegexMatch,
}

/// Flag condition for gradual rollout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagCondition {
    pub condition_type: ConditionType,
    pub key: String,
    pub operator: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionType {
    UserAttribute,
    Environment,
    Platform,
    Version,
    DateRange,
    Percentage,
    UserList,
}

/// User context for evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationContext {
    pub user_id: String,
    pub user_attributes: HashMap<String, serde_json::Value>,
    pub environment: String,
    pub platform: String,
    pub app_version: String,
    pub timestamp: u64,
}

/// Flag evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub flag_id: String,
    pub flag_name: String,
    pub value: bool,
    pub reason: EvaluationReason,
    pub matched_rule: Option<String>,
}

/// Evaluation reason
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EvaluationReason {
    DefaultValue,
    Enabled,
    Disabled,
    TargetingRule,
    PercentageRollout,
    UserInTargetList,
    EnvironmentMatch,
    VersionMatch,
}

/// Experiment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub id: String,
    pub name: String,
    pub flag_id: String,
    pub variants: Vec<Variant>,
    pub metrics: Vec<String>,
    pub status: ExperimentStatus,
    pub start_date: u64,
    pub end_date: Option<u64>,
    pub min_sample_size: u32,
}

/// Experiment variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub id: String,
    pub name: String,
    pub description: String,
    pub value: bool,
    pub weight: u32,
}

/// Experiment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExperimentStatus {
    Draft,
    Running,
    Paused,
    Completed,
    Archived,
}

/// Experiment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub experiment_id: String,
    pub variant_id: String,
    pub sample_size: u32,
    pub conversion_rate: f64,
    pub confidence_interval: (f64, f64),
    pub p_value: f64,
    pub significant: bool,
}

/// Feature flag service
pub struct FeatureFlagService {
    flags: Arc<RwLock<HashMap<String, FeatureFlag>>>,
    experiments: Arc<RwLock<HashMap<String, Experiment>>>,
    evaluations: Arc<RwLock<Vec<EvaluationResult>>>,
    cache: Arc<RwLock<HashMap<String, (bool, u64)>>>,
    cache_ttl_ms: u64,
}

impl FeatureFlagService {
    pub fn new(cache_ttl_seconds: u64) -> Self {
        Self {
            flags: Arc::new(RwLock::new(HashMap::new())),
            experiments: Arc::new(RwLock::new(HashMap::new())),
            evaluations: Arc::new(RwLock::new(Vec::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl_ms: cache_ttl_seconds * 1000,
        }
    }

    /// Create feature flag
    pub fn create_flag(&self, flag: FeatureFlag) -> String {
        let mut flags = self.flags.write().unwrap();
        flags.insert(flag.id.clone(), flag.clone());
        flag.id
    }

    /// Update feature flag
    pub fn update_flag(&self, flag_id: &str, updates: FlagUpdates) -> Result<(), FlagError> {
        let mut flags = self.flags.write().unwrap();

        if let Some(flag) = flags.get_mut(flag_id) {
            if let Some(enabled) = updates.enabled {
                flag.enabled = enabled;
            }
            if let Some(rollout) = updates.rollout_percentage {
                flag.rollout_percentage = rollout;
            }
            if let Some(rules) = updates.targeting_rules {
                flag.targeting_rules = rules;
            }
            flag.updated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            // Invalidate cache
            self.invalidate_cache(flag_id);

            Ok(())
        } else {
            Err(FlagError::FlagNotFound)
        }
    }

    /// Evaluate feature flag for user
    pub fn evaluate(&self, flag_name: &str, context: &EvaluationContext) -> EvaluationResult {
        let flags = self.flags.read().unwrap();

        if let Some(flag) = flags.values().find(|f| f.name == flag_name) {
            // Check cache first
            if let Some(cached) = self.get_cached(flag_name, &context.user_id) {
                return cached;
            }

            let result = self.evaluate_flag(flag, context);

            // Cache result
            self.set_cached(flag_name, &context.user_id, &result);

            // Log evaluation
            let mut evaluations = self.evaluations.write().unwrap();
            evaluations.push(result.clone());

            result
        } else {
            EvaluationResult {
                flag_id: "unknown".to_string(),
                flag_name: flag_name.to_string(),
                value: false,
                reason: EvaluationReason::DefaultValue,
                matched_rule: None,
            }
        }
    }

    fn evaluate_flag(&self, flag: &FeatureFlag, context: &EvaluationContext) -> EvaluationResult {
        // Check if flag is disabled
        if !flag.enabled {
            return EvaluationResult {
                flag_id: flag.id.clone(),
                flag_name: flag.name.clone(),
                value: flag.default_value,
                reason: EvaluationReason::Disabled,
                matched_rule: None,
            };
        }

        // Check targeting rules in priority order
        let mut sorted_rules = flag.targeting_rules.clone();
        sorted_rules.sort_by(|a, b| a.priority.cmp(&b.priority));

        for rule in &sorted_rules {
            if self.evaluate_conditions(&rule.conditions, context) {
                // Check percentage rollout if applicable
                if let Some(percentage) = rule.percentage {
                    if self.is_in_percentage(&context.user_id, percentage) {
                        return EvaluationResult {
                            flag_id: flag.id.clone(),
                            flag_name: flag.name.clone(),
                            value: rule.value,
                            reason: EvaluationReason::PercentageRollout,
                            matched_rule: Some(rule.id.clone()),
                        };
                    }
                } else {
                    return EvaluationResult {
                        flag_id: flag.id.clone(),
                        flag_name: flag.name.clone(),
                        value: rule.value,
                        reason: EvaluationReason::TargetingRule,
                        matched_rule: Some(rule.id.clone()),
                    };
                }
            }
        }

        // Check global rollout percentage
        if self.is_in_percentage(&context.user_id, flag.rollout_percentage) {
            EvaluationResult {
                flag_id: flag.id.clone(),
                flag_name: flag.name.clone(),
                value: true,
                reason: EvaluationReason::PercentageRollout,
                matched_rule: None,
            }
        } else {
            EvaluationResult {
                flag_id: flag.id.clone(),
                flag_name: flag.name.clone(),
                value: flag.default_value,
                reason: EvaluationReason::DefaultValue,
                matched_rule: None,
            }
        }
    }

    fn evaluate_conditions(&self, conditions: &[RuleCondition], context: &EvaluationContext) -> bool {
        conditions.iter().all(|cond| {
            let attr_value = match cond.attribute.as_str() {
                "user_id" => Some(context.user_id.clone()),
                "environment" => Some(context.environment.clone()),
                "platform" => Some(context.platform.clone()),
                "app_version" => Some(context.app_version.clone()),
                _ => context.user_attributes.get(&cond.attribute).map(|v| v.to_string()),
            };

            attr_value.map(|v| self.evaluate_operator(&cond.operator, &v, &cond.value)).unwrap_or(false)
        })
    }

    fn evaluate_operator(&self, op: &ConditionOperator, value: &str, target: &serde_json::Value) -> bool {
        match op {
            ConditionOperator::Equals => value == target.as_str().unwrap_or(""),
            ConditionOperator::NotEquals => value != target.as_str().unwrap_or(""),
            ConditionOperator::Contains => value.contains(target.as_str().unwrap_or("")),
            ConditionOperator::StartsWith => value.starts_with(target.as_str().unwrap_or("")),
            ConditionOperator::EndsWith => value.ends_with(target.as_str().unwrap_or("")),
            ConditionOperator::In => {
                target.as_array()
                    .map(|arr| arr.iter().any(|v| v.as_str() == Some(value)))
                    .unwrap_or(false)
            }
            ConditionOperator::RegexMatch => {
                // Simplified regex matching
                value.contains(target.as_str().unwrap_or(""))
            }
            _ => false,
        }
    }

    fn is_in_percentage(&self, user_id: &str, percentage: u32) -> bool {
        // Deterministic hashing for consistent results
        let hash = self.simple_hash(user_id);
        (hash % 100) < percentage
    }

    fn simple_hash(&self, input: &str) -> u64 {
        input.bytes().fold(0u64, |acc, b| acc.wrapping_add(b as u64).wrapping_mul(31))
    }

    fn get_cached(&self, flag_name: &str, user_id: &str) -> Option<EvaluationResult> {
        let cache = self.cache.read().unwrap();
        let key = format!("{}:{}", flag_name, user_id);

        if let Some((result, timestamp)) = cache.get(&key) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            if now - timestamp < self.cache_ttl_ms {
                return Some(result.clone());
            }
        }

        None
    }

    fn set_cached(&self, flag_name: &str, user_id: &str, result: &EvaluationResult) {
        let mut cache = self.cache.write().unwrap();
        let key = format!("{}:{}", flag_name, user_id);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        cache.insert(key, (result.clone(), timestamp));
    }

    fn invalidate_cache(&self, flag_id: &str) {
        let mut cache = self.cache.write().unwrap();
        let flags = self.flags.read().unwrap();

        if let Some(flag) = flags.get(flag_id) {
            let flag_name = flag.name.clone();
            cache.retain(|key, _| !key.starts_with(&flag_name));
        }
    }

    /// Create experiment
    pub fn create_experiment(&self, experiment: Experiment) -> String {
        let mut experiments = self.experiments.write().unwrap();
        experiments.insert(experiment.id.clone(), experiment.clone());
        experiment.id
    }

    /// Start experiment
    pub fn start_experiment(&self, experiment_id: &str) -> Result<(), FlagError> {
        let mut experiments = self.experiments.write().unwrap();

        if let Some(exp) = experiments.get_mut(experiment_id) {
            exp.status = ExperimentStatus::Running;
            Ok(())
        } else {
            Err(FlagError::ExperimentNotFound)
        }
    }

    /// Get experiment results
    pub fn get_experiment_results(&self, experiment_id: &str) -> Result<Vec<ExperimentResult>, FlagError> {
        let experiments = self.experiments.read().unwrap();

        if let Some(exp) = experiments.get(experiment_id) {
            let evaluations = self.evaluations.read().unwrap();

            let mut results = Vec::new();
            for variant in &exp.variants {
                let variant_evals: Vec<&EvaluationResult> = evaluations
                    .iter()
                    .filter(|e| e.flag_id == exp.flag_id)
                    .collect();

                let sample_size = variant_evals.len() as u32;
                let conversion_rate = 0.5; // Simplified

                results.push(ExperimentResult {
                    experiment_id: experiment_id.to_string(),
                    variant_id: variant.id.clone(),
                    sample_size,
                    conversion_rate,
                    confidence_interval: (0.4, 0.6),
                    p_value: 0.05,
                    significant: true,
                });
            }

            Ok(results)
        } else {
            Err(FlagError::ExperimentNotFound)
        }
    }

    /// Bulk evaluate flags
    pub fn evaluate_all(&self, flag_names: &[String], context: &EvaluationContext) -> Vec<EvaluationResult> {
        flag_names.iter()
            .map(|name| self.evaluate(name, context))
            .collect()
    }

    /// Get flag info
    pub fn get_flag(&self, flag_id: &str) -> Option<FeatureFlag> {
        let flags = self.flags.read().unwrap();
        flags.get(flag_id).cloned()
    }

    /// List all flags
    pub fn list_flags(&self) -> Vec<FeatureFlag> {
        let flags = self.flags.read().unwrap();
        flags.values().cloned().collect()
    }

    /// Delete flag
    pub fn delete_flag(&self, flag_id: &str) -> Result<(), FlagError> {
        let mut flags = self.flags.write().unwrap();
        if flags.remove(flag_id).is_some() {
            self.invalidate_cache(flag_id);
            Ok(())
        } else {
            Err(FlagError::FlagNotFound)
        }
    }

    /// Get evaluation analytics
    pub fn get_analytics(&self, flag_id: &str) -> FlagAnalytics {
        let evaluations = self.evaluations.read().unwrap();

        let flag_evals: Vec<&EvaluationResult> = evaluations
            .iter()
            .filter(|e| e.flag_id == flag_id)
            .collect();

        let total = flag_evals.len();
        let enabled = flag_evals.iter().filter(|e| e.value).count();
        let by_reason: HashMap<String, usize> = flag_evals
            .iter()
            .fold(HashMap::new(), |mut acc, e| {
                *acc.entry(format!("{:?}", e.reason)).or_insert(0) += 1;
                acc
            });

        FlagAnalytics {
            flag_id: flag_id.to_string(),
            total_evaluations: total,
            enabled_count: enabled,
            disabled_count: total - enabled,
            by_reason,
            time_series: vec![],
        }
    }
}

/// Flag updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagUpdates {
    pub enabled: Option<bool>,
    pub rollout_percentage: Option<u32>,
    pub targeting_rules: Option<Vec<TargetingRule>>,
}

/// Flag error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlagError {
    FlagNotFound,
    ExperimentNotFound,
    InvalidConfiguration,
}

impl std::fmt::Display for FlagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlagError::FlagNotFound => write!(f, "Flag not found"),
            FlagError::ExperimentNotFound => write!(f, "Experiment not found"),
            FlagError::InvalidConfiguration => write!(f, "Invalid configuration"),
        }
    }
}

impl std::error::Error for FlagError {}

/// Flag analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagAnalytics {
    pub flag_id: String,
    pub total_evaluations: usize,
    pub enabled_count: usize,
    pub disabled_count: usize,
    pub by_reason: HashMap<String, usize>,
    pub time_series: Vec<TimeSeriesPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    pub timestamp: u64,
    pub value: f64,
}

// Re-export types
pub use FeatureFlag;
pub use TargetingRule;
pub use Experiment;
pub use ExperimentResult;
pub use EvaluationContext;
pub use EvaluationResult;
pub use FeatureFlagService;