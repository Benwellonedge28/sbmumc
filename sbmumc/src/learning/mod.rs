//! Learning Module - Meta-Learning, Active Learning, and Self-Supervised Learning
//!
//! This module implements various learning mechanisms for SBMUMC including:
//! - Meta-learning: Learning how to learn
//! - Active learning: Learning by asking questions
//! - Self-supervised learning: Learning from self-generated labels
//! - Continual learning: Learning without forgetting

use crate::core::{SbmumcError, Result, EntityId, PropertyValue};
use std::collections::{HashMap, VecDeque, HashSet};
use parking_lot::RwLock;
use std::sync::Arc;
use tracing::{debug, info};

/// Meta-Learner - Learns how to learn
pub struct MetaLearner {
    /// Learning strategies
    strategies: Vec<LearningStrategy>,

    /// Current best strategy
    current_strategy: RwLock<Option<String>>,

    /// Performance history
    performance_history: RwLock<Vec<PerformanceRecord>>,

    /// Meta-knowledge
    meta_knowledge: RwLock<MetaKnowledge>,

    /// Configuration
    config: MetaLearningConfig,
}

/// Learning strategy
#[derive(Debug, Clone)]
pub struct LearningStrategy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub hyperparameters: HashMap<String, f64>,
    pub success_rate: f64,
    pub适用场景: Vec<String>,
}

/// Performance record
#[derive(Debug, Clone)]
pub struct PerformanceRecord {
    pub timestamp: crate::core::Timestamp,
    pub strategy_id: String,
    pub task_type: String,
    pub success: bool,
    pub metrics: HashMap<String, f64>,
    pub duration_ms: u64,
}

/// Meta-knowledge about learning
#[derive(Debug, Clone, Default)]
pub struct MetaKnowledge {
    pub optimal_learning_rates: HashMap<String, f64>,
    pub effective_strategies: HashMap<String, Vec<String>>,
    pub task_complexity_estimates: HashMap<String, f64>,
    pub best_hyperparameters: HashMap<String, HashMap<String, f64>>,
}

/// Meta-learning configuration
#[derive(Debug, Clone)]
pub struct MetaLearningConfig {
    pub enable_strategy_selection: bool,
    pub enable_hyperparameter_tuning: bool,
    pub exploration_rate: f64,
    pub exploitation_rate: f64,
    pub performance_threshold: f64,
}

impl Default for MetaLearningConfig {
    fn default() -> Self {
        Self {
            enable_strategy_selection: true,
            enable_hyperparameter_tuning: true,
            exploration_rate: 0.2,
            exploitation_rate: 0.8,
            performance_threshold: 0.7,
        }
    }
}

impl MetaLearner {
    /// Create a new meta-learner
    pub fn new() -> Result<Self> {
        info!("Initializing Meta-Learner");

        let mut strategies = Vec::new();

        // Add default learning strategies
        strategies.push(LearningStrategy {
            id: "gradient_descent".to_string(),
            name: "Gradient Descent".to_string(),
            description: "Standard gradient-based optimization".to_string(),
            hyperparameters: HashMap::new(),
            success_rate: 0.8,
           适用场景: vec!["optimization".to_string(), "prediction".to_string()],
        });

        strategies.push(LearningStrategy {
            id: "reinforcement".to_string(),
            name: "Reinforcement Learning".to_string(),
            description: "Learn from rewards and penalties".to_string(),
            hyperparameters: HashMap::new(),
            success_rate: 0.75,
           适用场景: vec!["control".to_string(), "decision_making".to_string()],
        });

        strategies.push(LearningStrategy {
            id: "curiosity_driven".to_string(),
            name: "Curiosity-Driven Learning".to_string(),
            description: "Learn driven by intrinsic curiosity".to_string(),
            hyperparameters: HashMap::new(),
            success_rate: 0.7,
           适用场景: vec!["exploration".to_string(), "discovery".to_string()],
        });

        Ok(Self {
            strategies,
            current_strategy: RwLock::new(None),
            performance_history: RwLock::new(Vec::new()),
            meta_knowledge: RwLock::new(MetaKnowledge::default()),
            config: MetaLearningConfig::default(),
        })
    }

    /// Select the best learning strategy for a task
    pub fn select_strategy(&self, task_type: &str) -> Result<LearningStrategy> {
        debug!("Selecting strategy for task: {}", task_type);

        // Filter applicable strategies
        let applicable: Vec<&LearningStrategy> = self
            .strategies
            .iter()
            .filter(|s| s.适用场景.iter().any(|t| t == task_type))
            .collect();

        if applicable.is_empty() {
            // Fallback to first strategy
            return self.strategies.first().cloned().ok_or_else(|| {
                SbmumcError::Learning("No learning strategies available".to_string())
            });
        }

        // Use exploration vs exploitation
        let use_exploration = rand_simple() < self.config.exploration_rate;

        if use_exploration {
            // Random selection for exploration
            let idx = (rand_simple() * applicable.len() as f64) as usize;
            return Ok(applicable[idx].clone());
        }

        // Select best by success rate
        let best = applicable
            .iter()
            .max_by(|a, b| {
                a.success_rate
                    .partial_cmp(&b.success_rate)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
            .ok_or_else(|| SbmumcError::Learning("No applicable strategy".to_string()))?;

        *self.current_strategy.write() = Some(best.id.clone());
        Ok(best.clone())
    }

    /// Update strategy performance
    pub fn update_performance(&self, record: PerformanceRecord) {
        debug!("Updating performance for strategy: {}", record.strategy_id);

        // Add to history
        {
            let mut history = self.performance_history.write();
            history.push(record.clone());

            // Keep only last 1000 records
            if history.len() > 1000 {
                history.remove(0);
            }
        }

        // Update strategy success rate
        if let Some(strategy) = self.strategies.iter_mut().find(|s| s.id == record.strategy_id) {
            let history = self.performance_history.read();
            let relevant: Vec<_> = history
                .iter()
                .filter(|r| r.strategy_id == record.strategy_id)
                .collect();

            if !relevant.is_empty() {
                let success_count = relevant.iter().filter(|r| r.success).count();
                strategy.success_rate = success_count as f64 / relevant.len() as f64;
            }
        }
    }

    /// Learn from experience and improve
    pub fn learn_from_experience(&self, experience: &Experience) -> Result<LearningUpdate> {
        debug!("Learning from experience: {:?}", experience.experience_type);

        let mut update = LearningUpdate::default();

        match &experience.experience_type {
            ExperienceType::Success(data) => {
                update.hyperparameter_adjustments = self.optimize_hyperparameters(data)?;
                update.new_patterns = self.extract_patterns(data)?;
                update.confidence_boost = 0.1;
            }
            ExperienceType::Failure(data) => {
                update.hyperparameter_adjustments = self.adjust_for_failure(data)?;
                update.avoid_patterns = self.identify_failure_patterns(data)?;
                update.confidence_boost = -0.05;
            }
            ExperienceType::Curiosity(data) => {
                update.exploration_targets = data.targets.clone();
                update.curiosity_score = data.score;
            }
            ExperienceType::Feedback(feedback) => {
                update.adjustments = self.process_feedback(feedback)?;
            }
        }

        Ok(update)
    }

    /// Optimize hyperparameters based on successful experience
    fn optimize_hyperparameters(&self, data: &SuccessData) -> Result<HashMap<String, f64>> {
        let mut adjustments = HashMap::new();

        // Simple optimization logic
        if let Some(learning_rate) = data.metrics.get("learning_rate") {
            // Increase learning rate if successful
            adjustments.insert("learning_rate".to_string(), learning_rate * 1.1);
        }

        Ok(adjustments)
    }

    /// Adjust hyperparameters after failure
    fn adjust_for_failure(&self, data: &FailureData) -> Result<HashMap<String, f64>> {
        let mut adjustments = HashMap::new();

        if let Some(learning_rate) = data.metrics.get("learning_rate") {
            // Decrease learning rate if failed
            adjustments.insert("learning_rate".to_string(), learning_rate * 0.9);
        }

        Ok(adjustments)
    }

    /// Extract patterns from successful experience
    fn extract_patterns(&self, data: &SuccessData) -> Result<Vec<Pattern>> {
        // Simple pattern extraction
        let mut patterns = Vec::new();

        for (key, value) in &data.context {
            patterns.push(Pattern {
                pattern_type: "feature".to_string(),
                features: vec![key.clone()],
                outcome: value.clone(),
                confidence: 0.8,
            });
        }

        Ok(patterns)
    }

    /// Identify patterns that led to failure
    fn identify_failure_patterns(&self, data: &FailureData) -> Result<Vec<String>> {
        let mut patterns = Vec::new();

        for key in data.context.keys() {
            patterns.push(key.clone());
        }

        Ok(patterns)
    }

    /// Process feedback from humans or other sources
    fn process_feedback(&self, feedback: &Feedback) -> Result<HashMap<String, f64>> {
        let mut adjustments = HashMap::new();

        // Convert feedback to adjustments
        for (key, value) in &feedback.values {
            adjustments.insert(key.clone(), *value);
        }

        Ok(adjustments)
    }

    /// Get learning statistics
    pub fn get_stats(&self) -> LearningStats {
        let history = self.performance_history.read();
        let total = history.len();
        let successful = history.iter().filter(|r| r.success).count();

        LearningStats {
            total_experiences: total,
            success_rate: if total > 0 { successful as f64 / total as f64 } else { 0.0 },
            strategies_count: self.strategies.len(),
            current_strategy: self.current_strategy.read().clone(),
        }
    }
}

impl Default for MetaLearner {
    fn default() -> Self {
        Self::new().expect("Failed to create MetaLearner")
    }
}

/// Experience representation
#[derive(Debug, Clone)]
pub struct Experience {
    pub id: EntityId,
    pub experience_type: ExperienceType,
    pub timestamp: crate::core::Timestamp,
    pub context: HashMap<String, PropertyValue>,
}

impl Experience {
    pub fn new(experience_type: ExperienceType) -> Self {
        Self {
            id: EntityId::new(),
            experience_type,
            timestamp: crate::core::Timestamp::now(),
            context: HashMap::new(),
        }
    }
}

/// Experience types
#[derive(Debug, Clone)]
pub enum ExperienceType {
    Success(SuccessData),
    Failure(FailureData),
    Curiosity(CuriosityData),
    Feedback(Feedback),
}

/// Data from successful experience
#[derive(Debug, Clone)]
pub struct SuccessData {
    pub metrics: HashMap<String, f64>,
    pub context: HashMap<String, PropertyValue>,
}

/// Data from failed experience
#[derive(Debug, Clone)]
pub struct FailureData {
    pub metrics: HashMap<String, f64>,
    pub context: HashMap<String, PropertyValue>,
    pub error: Option<String>,
}

/// Curiosity-driven learning data
#[derive(Debug, Clone)]
pub struct CuriosityData {
    pub targets: Vec<String>,
    pub score: f64,
}

/// Feedback from external source
#[derive(Debug, Clone)]
pub struct Feedback {
    pub source: String,
    pub feedback_type: FeedbackType,
    pub values: HashMap<String, f64>,
}

/// Feedback types
#[derive(Debug, Clone)]
pub enum FeedbackType {
    Positive,
    Negative,
    Corrective,
    Comparative,
}

/// Learning update from experience
#[derive(Debug, Clone, Default)]
pub struct LearningUpdate {
    pub hyperparameter_adjustments: HashMap<String, f64>,
    pub new_patterns: Vec<Pattern>,
    pub avoid_patterns: Vec<String>,
    pub exploration_targets: Vec<String>,
    pub adjustments: HashMap<String, f64>,
    pub confidence_boost: f64,
    pub curiosity_score: f64,
}

/// Learned pattern
#[derive(Debug, Clone)]
pub struct Pattern {
    pub pattern_type: String,
    pub features: Vec<String>,
    pub outcome: PropertyValue,
    pub confidence: f64,
}

/// Learning statistics
#[derive(Debug, Clone)]
pub struct LearningStats {
    pub total_experiences: usize,
    pub success_rate: f64,
    pub strategies_count: usize,
    pub current_strategy: Option<String>,
}

// ============================================================================
// Self-Supervised Learning
// ============================================================================

/// Self-supervised learning module
pub struct SelfSupervisedLearner {
    /// Pretext tasks
    pretext_tasks: Vec<PretextTask>,

    /// Learned representations
    representations: RwLock<HashMap<String, Vec<f64>>>,

    /// Configuration
    config: SelfSupervisedConfig,
}

/// Pretext task for self-supervised learning
#[derive(Debug, Clone)]
pub struct PretextTask {
    pub id: String,
    pub name: String,
    pub description: String,
    pub task_fn: Box<dyn Fn(&[u8]) -> Vec<f64> + Send + Sync>,
}

impl SelfSupervisedLearner {
    /// Create a new self-supervised learner
    pub fn new() -> Result<Self> {
        info!("Initializing Self-Supervised Learner");

        Ok(Self {
            pretext_tasks: Vec::new(),
            representations: RwLock::new(HashMap::new()),
            config: SelfSupervisedConfig::default(),
        })
    }

    /// Learn from unlabeled data
    pub fn learn(&self, data: &[u8]) -> Result<Representation> {
        debug!("Self-supervised learning on {} bytes", data.len());

        // Apply pretext tasks
        let mut features = Vec::new();
        for task in &self.pretext_tasks {
            let task_features = (task.task_fn)(data);
            features.extend(task_features);
        }

        // Store representation
        let id = format!("rep_{}", data.len());
        {
            let mut reps = self.representations.write();
            reps.insert(id.clone(), features.clone());
        }

        Ok(Representation {
            id,
            features,
            dimension: features.len(),
        })
    }

    /// Contrastive learning between examples
    pub fn contrastive_learn(&self, anchor: &[u8], positive: &[u8], negative: &[u8]) -> Result<f64> {
        let anchor_rep = self.learn(anchor)?;
        let positive_rep = self.learn(positive)?;
        let negative_rep = self.learn(negative)?;

        let pos_sim = cosine_similarity(&anchor_rep.features, &positive_rep.features);
        let neg_sim = cosine_similarity(&anchor_rep.features, &negative_rep.features);

        // Loss: maximize positive similarity, minimize negative similarity
        let loss = (neg_sim - pos_sim).max(0.0);
        Ok(loss)
    }

    /// Generate pseudo-labels for data
    pub fn generate_pseudo_labels(&self, data: &[u8], num_labels: usize) -> Result<Vec<PseudoLabel>> {
        let representation = self.learn(data)?;

        // Simple clustering for pseudo-labels
        let mut labels = Vec::new();
        let cluster_size = representation.features.len() / num_labels;

        for i in 0..num_labels {
            labels.push(PseudoLabel {
                label: i,
                confidence: 0.7 + (rand_simple() * 0.3),
                features: representation.features
                    [i * cluster_size..(i + 1) * cluster_size]
                    .to_vec(),
            });
        }

        Ok(labels)
    }
}

impl Default for SelfSupervisedLearner {
    fn default() -> Self {
        Self::new().expect("Failed to create SelfSupervisedLearner")
    }
}

/// Self-supervised configuration
#[derive(Debug, Clone)]
pub struct SelfSupervisedConfig {
    pub enable_contrastive: bool,
    pub enable_rotation: bool,
    pub enable_jigsaw: bool,
    pub feature_dimensions: usize,
}

impl Default for SelfSupervisedConfig {
    fn default() -> Self {
        Self {
            enable_contrastive: true,
            enable_rotation: true,
            enable_jigsaw: false,
            feature_dimensions: 128,
        }
    }
}

/// Learned representation
#[derive(Debug, Clone)]
pub struct Representation {
    pub id: String,
    pub features: Vec<f64>,
    pub dimension: usize,
}

/// Pseudo-label for semi-supervised learning
#[derive(Debug, Clone)]
pub struct PseudoLabel {
    pub label: usize,
    pub confidence: f64,
    pub features: Vec<f64>,
}

// ============================================================================
// Active Learning
// ============================================================================

/// Active learning module
pub struct ActiveLearner {
    /// Query strategies
    strategies: Vec<QueryStrategy>,

    /// Labeled data pool
    labeled_pool: RwLock<Vec<LabeledData>>,

    /// Unlabeled data pool
    unlabeled_pool: RwLock<Vec<UnlabeledData>>,

    /// Configuration
    config: ActiveLearningConfig,
}

/// Query strategy for active learning
#[derive(Debug, Clone)]
pub enum QueryStrategy {
    UncertaintySampling,
    QueryByCommittee,
    ExpectedModelChange,
    ExpectedErrorReduction,
    DiversitySampling,
}

/// Labeled data
#[derive(Debug, Clone)]
pub struct LabeledData {
    pub id: EntityId,
    pub features: Vec<f64>,
    pub label: String,
    pub confidence: f64,
}

/// Unlabeled data
#[derive(Debug, Clone)]
pub struct UnlabeledData {
    pub id: EntityId,
    pub features: Vec<f64>,
    pub acquisition_score: f64,
}

impl ActiveLearner {
    /// Create a new active learner
    pub fn new() -> Result<Self> {
        info!("Initializing Active Learner");

        Ok(Self {
            strategies: vec![
                QueryStrategy::UncertaintySampling,
                QueryStrategy::DiversitySampling,
            ],
            labeled_pool: RwLock::new(Vec::new()),
            unlabeled_pool: RwLock::new(Vec::new()),
            config: ActiveLearningConfig::default(),
        })
    }

    /// Select the most informative sample to label
    pub fn select_sample(&self) -> Result<Option<UnlabeledData>> {
        let pool = self.unlabeled_pool.read();

        if pool.is_empty() {
            return Ok(None);
        }

        // Use uncertainty sampling
        let best = pool
            .iter()
            .max_by(|a, b| {
                a.acquisition_score
                    .partial_cmp(&b.acquisition_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned();

        Ok(best)
    }

    /// Add a labeled example
    pub fn add_labeled(&self, data: LabeledData) {
        let mut pool = self.labeled_pool.write();
        pool.push(data);
    }

    /// Add unlabeled examples
    pub fn add_unlabeled(&self, data: Vec<UnlabeledData>) {
        let mut pool = self.unlabeled_pool.write();
        pool.extend(data);
    }

    /// Update acquisition scores
    pub fn update_scores(&self) {
        let mut pool = self.unlabeled_pool.write();

        for data in pool.iter_mut() {
            // Update based on uncertainty
            data.acquisition_score = self.calculate_uncertainty(&data.features);
        }
    }

    /// Calculate uncertainty score
    fn calculate_uncertainty(&self, features: &[f64]) -> f64 {
        // Simple uncertainty calculation
        // In a full implementation, this would use the model
        features.iter().map(|f| f.abs()).sum::<f64>() / features.len() as f64
    }

    /// Get the number of labeled examples
    pub fn labeled_count(&self) -> usize {
        self.labeled_pool.read().len()
    }

    /// Get the number of unlabeled examples
    pub fn unlabeled_count(&self) -> usize {
        self.unlabeled_pool.read().len()
    }
}

impl Default for ActiveLearner {
    fn default() -> Self {
        Self::new().expect("Failed to create ActiveLearner")
    }
}

/// Active learning configuration
#[derive(Debug, Clone)]
pub struct ActiveLearningConfig {
    pub batch_size: usize,
    pub query_budget: usize,
    pub stopping_criterion: StoppingCriterion,
}

/// Stopping criterion for active learning
#[derive(Debug, Clone)]
pub enum StoppingCriterion {
    BudgetDepleted,
    PerformanceThreshold(f64),
    LabeledRatio(f64),
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Simple random number generator
fn rand_simple() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64) / (u32::MAX as f64)
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

// ============================================================================
// COMPREHENSIVE LEARNING MODULE
// ============================================================================

pub mod comprehensive;
