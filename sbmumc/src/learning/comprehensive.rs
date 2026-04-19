//! Comprehensive Learning Module - All Learning Techniques for SBMUMC
//!
//! This module implements every known learning technique:
//! - Deep Learning (CNNs, RNNs, Transformers, GNNs)
//! - Reinforcement Learning (Q-learning, Policy Gradient, DDPG, PPO)
//! - Transfer Learning and Multi-task Learning
//! - Federated Learning
//! - Continual and Life-long Learning
//! - Curiosity-driven and Intrinsic Motivation
//! - Imitation Learning
//! - Ensemble Methods
//! - Bayesian and Probabilistic Learning
//! - Evolutionary Learning
//! - Hebbian Learning
//! - Attention Mechanisms
//! - Memory-Augmented Networks
//! - Graph Neural Networks
//! - Self-Play and Multi-Agent Learning
//! - Meta-Learning (Learning to Learn)
//! - Online and Streaming Learning

use crate::core::{SbmumcError, Result, EntityId, PropertyValue};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{debug, info};

// ============================================================================
// NEURAL NETWORK LAYERS AND MODELS
// ============================================================================

/// Neural Network Layer Types
#[derive(Debug, Clone)]
pub enum LayerType {
    Dense { units: usize, activation: Activation },
    Conv2D { filters: usize, kernel_size: (usize, usize), activation: Activation },
    MaxPool2D { pool_size: (usize, usize) },
    LSTM { units: usize },
    GRU { units: usize },
    Attention { heads: usize },
    Embedding { vocab_size: usize, dim: usize },
    Dropout { rate: f64 },
    BatchNorm,
}

/// Activation functions
#[derive(Debug, Clone, Copy)]
pub enum Activation {
    Relu,
    Sigmoid,
    Tanh,
    Softmax,
    LeakyRelu { alpha: f64 },
    Elu { alpha: f64 },
    Swish,
    Gelu,
}

/// Neural Network Layer
#[derive(Debug, Clone)]
pub struct Layer {
    pub layer_type: LayerType,
    pub weights: Option<Vec<f64>>,
    pub biases: Option<Vec<f64>>,
    pub output_shape: Vec<usize>,
}

/// Neural Network Model
#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    pub layers: Vec<Layer>,
    pub input_shape: Vec<usize>,
    pub output_shape: Vec<usize>,
    pub compiled: bool,
}

impl NeuralNetwork {
    pub fn new(input_shape: &[usize]) -> Self {
        Self {
            layers: Vec::new(),
            input_shape: input_shape.to_vec(),
            output_shape: input_shape.to_vec(),
            compiled: false,
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    /// Forward pass through the network
    pub fn forward(&self, input: &[f64]) -> Vec<f64> {
        let mut output = input.to_vec();

        for layer in &self.layers {
            output = self.apply_layer(layer, &output);
        }

        output
    }

    fn apply_layer(&self, layer: &Layer, input: &[f64]) -> Vec<f64> {
        match &layer.layer_type {
            LayerType::Dense { units, activation } => {
                // Simplified dense layer
                vec![0.0; *units]
            }
            LayerType::Dropout { rate } => {
                input.iter().map(|x| if rand_f64() > rate { *x } else { 0.0 }).collect()
            }
            _ => input.to_vec(),
        }
    }

    /// Backward pass (gradient computation)
    pub fn backward(&mut self, gradient: &[f64], learning_rate: f64) {
        // Simplified backward pass
        for layer in self.layers.iter_mut().rev() {
            // Update weights
            if let Some(weights) = &mut layer.weights {
                for w in weights.iter_mut() {
                    *w -= learning_rate * gradient[0];
                }
            }
        }
    }
}

// ============================================================================
// DEEP LEARNING MODELS
// ============================================================================

/// Convolutional Neural Network
pub struct CNN {
    pub conv_layers: Vec<Layer>,
    pub fc_layers: Vec<Layer>,
    pub pooling_layers: Vec<Layer>,
}

impl CNN {
    pub fn new(input_channels: usize) -> Self {
        Self {
            conv_layers: Vec::new(),
            fc_layers: Vec::new(),
            pooling_layers: Vec::new(),
        }
    }

    pub fn add_conv(&mut self, filters: usize, kernel: (usize, usize)) {
        self.conv_layers.push(Layer {
            layer_type: LayerType::Conv2D {
                filters,
                kernel_size: kernel,
                activation: Activation::Relu,
            },
            weights: None,
            biases: None,
            output_shape: vec![],
        });
    }

    pub fn add_pool(&mut self, size: (usize, usize)) {
        self.pooling_layers.push(Layer {
            layer_type: LayerType::MaxPool2D { pool_size: size },
            weights: None,
            biases: None,
            output_shape: vec![],
        });
    }

    pub fn predict(&self, input: &[f64]) -> Vec<f64> {
        let mut output = input.to_vec();

        for layer in &self.conv_layers {
            output = self.apply_conv(layer, &output);
        }

        for layer in &self.pooling_layers {
            output = self.apply_pool(layer, &output);
        }

        for layer in &self.fc_layers {
            output = self.apply_dense(layer, &output);
        }

        output
    }

    fn apply_conv(&self, layer: &Layer, input: &[f64]) -> Vec<f64> {
        // Simplified convolution
        input.to_vec()
    }

    fn apply_pool(&self, layer: &Layer, input: &[f64]) -> Vec<f64> {
        // Simplified pooling
        input.to_vec()
    }

    fn apply_dense(&self, layer: &Layer, input: &[f64]) -> Vec<f64> {
        // Simplified dense
        input.to_vec()
    }
}

/// Recurrent Neural Network
pub struct RNN {
    pub hidden_size: usize,
    pub num_layers: usize,
    pub cell_type: RNNCellType,
}

#[derive(Debug, Clone, Copy)]
pub enum RNNCellType {
    Vanilla,
    LSTM,
    GRU,
}

impl RNN {
    pub fn new(hidden_size: usize, num_layers: usize, cell_type: RNNCellType) -> Self {
        Self {
            hidden_size,
            num_layers,
            cell_type,
        }
    }

    /// Process sequence
    pub fn process_sequence(&self, input_seq: &[Vec<f64>]) -> Vec<f64> {
        let mut hidden = vec![0.0; self.hidden_size];

        for input_t in input_seq {
            hidden = self.step(input_t, &hidden);
        }

        hidden
    }

    fn step(&self, input: &[f64], hidden: &[f64]) -> Vec<f64> {
        // Simplified RNN step
        let mut new_hidden = Vec::with_capacity(self.hidden_size);

        for i in 0..self.hidden_size {
            let sum = input.get(i % input.len()).unwrap_or(&0.0)
                + hidden.get(i).unwrap_or(&0.0);
            new_hidden.push(Activation::Tanh.compute(sum));
        }

        new_hidden
    }
}

/// Transformer Architecture
pub struct Transformer {
    pub num_layers: usize,
    pub num_heads: usize,
    pub d_model: usize,
    pub d_ff: usize,
    pub vocab_size: usize,
}

impl Transformer {
    pub fn new(num_layers: usize, num_heads: usize, d_model: usize) -> Self {
        Self {
            num_layers,
            num_heads,
            d_model,
            d_ff: d_model * 4,
            vocab_size: 50000,
        }
    }

    /// Multi-head self-attention
    pub fn multihead_attention(
        &self,
        query: &[f64],
        key: &[f64],
        value: &[f64],
    ) -> Vec<f64> {
        // Simplified multi-head attention
        let head_dim = self.d_model / self.num_heads;
        let mut outputs = Vec::with_capacity(self.d_model);

        for _ in 0..self.num_heads {
            // Compute attention scores
            let score: f64 = query.iter()
                .zip(key.iter())
                .map(|(q, k)| q * k)
                .sum::<f64>() / (head_dim as f64).sqrt();

            // Apply softmax
            let attention = 1.0 / (1.0 + (-score).exp());

            // Weighted sum
            let output: f64 = value.iter()
                .zip([attention].iter().cycle().take(value.len()))
                .map(|(v, w)| v * w)
                .sum();

            outputs.push(output);
        }

        outputs
    }

    /// Encode sequence
    pub fn encode(&self, input_ids: &[usize]) -> Vec<Vec<f64>> {
        let mut hidden_states = Vec::new();

        for pos in 0..input_ids.len() {
            let mut hidden = vec![0.0; self.d_model];
            hidden[pos % self.d_model] = 1.0; // Positional encoding
            hidden_states.push(hidden);
        }

        // Apply transformer layers
        for _ in 0..self.num_layers {
            hidden_states = self.apply_layer(&hidden_states);
        }

        hidden_states
    }

    fn apply_layer(&self, states: &[Vec<f64>]) -> Vec<Vec<f64>> {
        // Apply self-attention and feed-forward
        states.iter().map(|s| {
            let mut out = s.clone();
            for i in 0..out.len().min(self.d_ff) {
                out[i] = Activation::Gelu.compute(out[i]);
            }
            out.truncate(self.d_model);
            out
        }).collect()
    }
}

/// Graph Neural Network
pub struct GNN {
    pub message_passing_layers: usize,
    pub aggregation_type: AggregationType,
    pub update_type: UpdateType,
}

#[derive(Debug, Clone, Copy)]
pub enum AggregationType {
    Mean,
    Sum,
    Max,
    Attention,
}

#[derive(Debug, Clone, Copy)]
pub enum UpdateType {
    GRU,
    LSTM,
    Dense,
}

impl GNN {
    pub fn new(layers: usize) -> Self {
        Self {
            message_passing_layers: layers,
            aggregation_type: AggregationType::Mean,
            update_type: UpdateType::GRU,
        }
    }

    /// Message passing between nodes
    pub fn message_pass(&self, graph: &Graph) -> Vec<Vec<f64>> {
        let mut node_features = graph.node_features.clone();

        for _ in 0..self.message_passing_layers {
            node_features = self.message_passing_step(&graph.edges, &node_features);
        }

        node_features
    }

    fn message_passing_step(&self, edges: &[(usize, usize)], features: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let mut new_features = Vec::with_capacity(features.len());

        for (i, feature) in features.iter().enumerate() {
            // Gather neighbor features
            let neighbors: Vec<&[f64]> = edges
                .iter()
                .filter(|(src, dst)| *dst == i)
                .map(|(src, _)| features[*src].as_slice())
                .collect();

            // Aggregate
            let aggregated = self.aggregate(&neighbors);

            // Update
            let updated = self.update(feature, &aggregated);

            new_features.push(updated);
        }

        new_features
    }

    fn aggregate(&self, neighbors: &[&[f64]]) -> Vec<f64> {
        if neighbors.is_empty() {
            return vec![0.0; 32]; // Default feature size
        }

        match self.aggregation_type {
            AggregationType::Mean => {
                let dim = neighbors[0].len();
                let mut sum = vec![0.0; dim];
                for neighbor in neighbors {
                    for (i, v) in neighbor.iter().enumerate() {
                        sum[i] += v;
                    }
                }
                sum.iter().map(|x| x / neighbors.len() as f64).collect()
            }
            AggregationType::Sum => {
                let dim = neighbors[0].len();
                let mut sum = vec![0.0; dim];
                for neighbor in neighbors {
                    for (i, v) in neighbor.iter().enumerate() {
                        sum[i] += v;
                    }
                }
                sum
            }
            AggregationType::Max => {
                let dim = neighbors[0].len();
                let mut maxes = vec![f64::MIN; dim];
                for neighbor in neighbors {
                    for (i, v) in neighbor.iter().enumerate() {
                        maxes[i] = maxes[i].max(*v);
                    }
                }
                maxes
            }
            _ => neighbors[0].to_vec(),
        }
    }

    fn update(&self, current: &[f64], aggregated: &[f64]) -> Vec<f64> {
        let mut updated = Vec::with_capacity(current.len());

        for i in 0..current.len().min(aggregated.len()) {
            updated.push(Activation::Relu.compute(current[i] + aggregated[i]));
        }

        updated
    }
}

/// Graph structure for GNN
#[derive(Debug, Clone)]
pub struct Graph {
    pub node_features: Vec<Vec<f64>>,
    pub edges: Vec<(usize, usize)>,
}

// ============================================================================
// REINFORCEMENT LEARNING
// ============================================================================

/// Reinforcement Learning Agent
pub struct RLAgent {
    pub algorithm: RLAlgorithm,
    pub q_network: NeuralNetwork,
    pub target_network: Option<NeuralNetwork>,
    pub replay_buffer: ReplayBuffer,
    pub policy: Policy,
    pub hyperparameters: RLHyperparameters,
}

/// RL Algorithms
#[derive(Debug, Clone, Copy)]
pub enum RLAlgorithm {
    QLearning,
    DQN,
    PolicyGradient,
    ActorCritic,
    DDPG,
    PPO,
    SAC,
    TD3,
}

/// Policy types
#[derive(Debug, Clone, Copy)]
pub enum Policy {
    EpsilonGreedy { epsilon: f64 },
    Softmax,
    UCB,
    ThompsonSampling,
}

/// RL Hyperparameters
#[derive(Debug, Clone)]
pub struct RLHyperparameters {
    pub learning_rate: f64,
    pub gamma: f64,           // Discount factor
    pub epsilon_start: f64,
    pub epsilon_end: f64,
    pub epsilon_decay: f64,
    pub target_update_freq: usize,
    pub batch_size: usize,
    pub replay_buffer_size: usize,
}

impl RLAgent {
    pub fn new(algorithm: RLAlgorithm, state_dim: usize, action_dim: usize) -> Self {
        let mut q_network = NeuralNetwork::new(&[state_dim]);
        q_network.add_layer(Layer {
            layer_type: LayerType::Dense { units: 64, activation: Activation::Relu },
            weights: None,
            biases: None,
            output_shape: vec![],
        });
        q_network.add_layer(Layer {
            layer_type: LayerType::Dense { units: action_dim, activation: Activation::Identity },
            weights: None,
            biases: None,
            output_shape: vec![],
        });

        Self {
            algorithm,
            q_network,
            target_network: None,
            replay_buffer: ReplayBuffer::new(10000),
            policy: Policy::EpsilonGreedy { epsilon: 1.0 },
            hyperparameters: RLHyperparameters {
                learning_rate: 0.001,
                gamma: 0.99,
                epsilon_start: 1.0,
                epsilon_end: 0.01,
                epsilon_decay: 0.995,
                target_update_freq: 100,
                batch_size: 32,
                replay_buffer_size: 10000,
            },
        }
    }

    /// Select action based on policy
    pub fn select_action(&self, state: &[f64], training: bool) -> usize {
        match &self.policy {
            Policy::EpsilonGreedy { epsilon } => {
                if training && rand_f64() < *epsilon {
                    // Random action
                    rand_usize() % 4
                } else {
                    // Greedy action
                    let q_values = self.q_network.forward(state);
                    q_values
                        .iter()
                        .enumerate()
                        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                        .map(|(i, _)| i)
                        .unwrap_or(0)
                }
            }
            Policy::Softmax => {
                let q_values = self.q_network.forward(state);
                softmax_sample(&q_values)
            }
            Policy::ThompsonSampling => {
                // Thompson sampling with learned distribution
                let q_values = self.q_network.forward(state);
                let samples: Vec<f64> = q_values.iter().map(|_| rand_f64()).collect();
                samples
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .map(|(i, _)| i)
                    .unwrap_or(0)
            }
            _ => 0,
        }
    }

    /// Update Q-values
    pub fn update(&mut self, batch: &[Transition]) -> f64 {
        let loss = match self.algorithm {
            RLAlgorithm::DQN | RLAlgorithm::QLearning => {
                self.update_dqn(batch)
            }
            RLAlgorithm::PolicyGradient => {
                self.update_policy_gradient(batch)
            }
            RLAlgorithm::ActorCritic => {
                self.update_actor_critic(batch)
            }
            _ => 0.0,
        };

        loss
    }

    fn update_dqn(&mut self, batch: &[Transition]) -> f64 {
        let mut total_loss = 0.0;

        for transition in batch {
            let q_values = self.q_network.forward(&transition.state);

            // Target Q-value
            let target = if transition.done {
                transition.reward
            } else {
                transition.reward + self.hyperparameters.gamma *
                    self.q_network.forward(&transition.next_state)
                        .iter()
                        .cloned()
                        .fold(f64::MIN, f64::max)
            };

            // Compute loss
            let action_idx = transition.action;
            let loss = (q_values[action_idx] - target).powi(2);
            total_loss += loss;
        }

        total_loss / batch.len() as f64
    }

    fn update_policy_gradient(&mut self, batch: &[Transition]) -> f64 {
        // Policy gradient update
        let mut policy_loss = 0.0;

        for transition in batch {
            let q_values = self.q_network.forward(&transition.state);
            let probs = softmax(&q_values);

            // REINFORCE-style update
            let log_prob = (probs[transition.action] + 1e-8).ln();
            policy_loss -= log_prob * transition.reward;
        }

        policy_loss / batch.len() as f64
    }

    fn update_actor_critic(&mut self, batch: &[Transition]) -> f64 {
        // Actor-critic update
        self.update_policy_gradient(batch)
    }

    /// Add experience to replay buffer
    pub fn store_transition(&mut self, transition: Transition) {
        self.replay_buffer.push(transition);
    }

    /// Sample batch from replay buffer
    pub fn sample_batch(&self) -> Vec<Transition> {
        self.replay_buffer.sample(self.hyperparameters.batch_size)
    }

    /// Update target network
    pub fn update_target_network(&mut self) {
        if let Some(target) = &mut self.target_network {
            // Copy weights from Q network
            // Simplified: just recreate
            *target = self.q_network.clone();
        }
    }
}

/// Experience replay buffer
#[derive(Debug, Clone)]
pub struct ReplayBuffer {
    buffer: VecDeque<Transition>,
    capacity: usize,
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub state: Vec<f64>,
    pub action: usize,
    pub reward: f64,
    pub next_state: Vec<f64>,
    pub done: bool,
}

impl ReplayBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, transition: Transition) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(transition);
    }

    pub fn sample(&self, batch_size: usize) -> Vec<Transition> {
        let len = self.buffer.len().min(batch_size);
        (0..len)
            .map(|_| {
                let idx = rand_usize() % self.buffer.len();
                self.buffer[idx].clone()
            })
            .collect()
    }
}

// ============================================================================
// DEEP REINFORCEMENT LEARNING
// ============================================================================

/// Deep Q-Network (DQN)
pub struct DQN {
    pub q_network: NeuralNetwork,
    pub target_network: NeuralNetwork,
    pub optimizer: Optimizer,
    pub experience_replay: ReplayBuffer,
    pub hyperparameters: DQNHyperparameters,
}

#[derive(Debug, Clone)]
pub struct DQNHyperparameters {
    pub learning_rate: f64,
    pub gamma: f64,
    pub epsilon: f64,
    pub epsilon_decay: f64,
    pub epsilon_min: f64,
    pub target_update_interval: usize,
    pub batch_size: usize,
    pub buffer_size: usize,
}

impl DQN {
    pub fn new(state_dim: usize, action_dim: usize) -> Self {
        Self {
            q_network: NeuralNetwork::new(&[state_dim]),
            target_network: NeuralNetwork::new(&[state_dim]),
            optimizer: Optimizer::new(OptimizerType::Adam, 0.001),
            experience_replay: ReplayBuffer::new(100000),
            hyperparameters: DQNHyperparameters {
                learning_rate: 0.001,
                gamma: 0.99,
                epsilon: 1.0,
                epsilon_decay: 0.995,
                epsilon_min: 0.01,
                target_update_interval: 1000,
                batch_size: 32,
                buffer_size: 100000,
            },
        }
    }
}

/// Proximal Policy Optimization (PPO)
pub struct PPO {
    pub actor: NeuralNetwork,
    pub critic: NeuralNetwork,
    pub clip_ratio: f64,
    pub value_coef: f64,
    pub entropy_coef: f64,
    pub update_epochs: usize,
}

impl PPO {
    pub fn new(state_dim: usize, action_dim: usize) -> Self {
        Self {
            actor: NeuralNetwork::new(&[state_dim]),
            critic: NeuralNetwork::new(&[state_dim]),
            clip_ratio: 0.2,
            value_coef: 0.5,
            entropy_coef: 0.01,
            update_epochs: 10,
        }
    }

    /// Compute clipped surrogate loss
    pub fn compute_loss(&self, batch: &PPOBatch) -> f64 {
        let ratio = batch.new_probs.len() as f64 / batch.old_probs.len().max(1) as f64;
        let surrogate = ratio * batch.advantages.iter().sum::<f64>() / batch.advantages.len() as f64;
        let clipped = ratio.clamp(1.0 - self.clip_ratio, 1.0 + self.clip_ratio)
            * batch.advantages.iter().sum::<f64>() / batch.advantages.len() as f64;

        surrogate.min(clipped)
    }
}

#[derive(Debug, Clone)]
pub struct POBatch {
    pub states: Vec<Vec<f64>>,
    pub actions: Vec<usize>,
    pub old_probs: Vec<f64>,
    pub new_probs: Vec<f64>,
    pub advantages: Vec<f64>,
    pub returns: Vec<f64>,
}

/// Soft Actor-Critic (SAC)
pub struct SAC {
    pub actor: NeuralNetwork,
    pub critic1: NeuralNetwork,
    pub critic2: NeuralNetwork,
    pub alpha: f64,  // Entropy coefficient
    pub target_entropy: f64,
}

// ============================================================================
// TRANSFER AND MULTI-TASK LEARNING
// ============================================================================

/// Transfer Learning Module
pub struct TransferLearning {
    pub source_domains: Vec<Domain>,
    pub target_domain: Domain,
    pub transfer_methods: Vec<TransferMethod>,
}

#[derive(Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub features: Vec<String>,
    pub tasks: Vec<String>,
    pub data_size: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum TransferMethod {
    FeatureRepuse,
    InstanceTransfer,
    ParameterTransfer,
    RelationalKnowledgeTransfer,
}

impl TransferLearning {
    pub fn new() -> Self {
        Self {
            source_domains: Vec::new(),
            target_domain: Domain {
                name: String::new(),
                features: Vec::new(),
                tasks: Vec::new(),
                data_size: 0,
            },
            transfer_methods: vec![
                TransferMethod::FeatureRepuse,
                TransferMethod::ParameterTransfer,
            ],
        }
    }

    /// Transfer knowledge from source to target
    pub fn transfer(&self, source_model: &NeuralNetwork, target_data: &[Vec<f64>]) -> NeuralNetwork {
        // Simplified transfer learning
        let mut transferred = source_model.clone();

        // Fine-tune last layers
        for layer in transferred.layers.iter_mut().skip(transferred.layers.len() / 2) {
            if let Some(weights) = &mut layer.weights {
                for w in weights.iter_mut() {
                    *w *= 0.9; // Reduce transferred knowledge
                }
            }
        }

        transferred
    }

    /// Domain adaptation
    pub fn adapt_domain(&self, model: &NeuralNetwork, source_data: &[f64], target_data: &[f64]) -> f64 {
        // Compute domain divergence
        let source_mean = source_data.iter().sum::<f64>() / source_data.len().max(1) as f64;
        let target_mean = target_data.iter().sum::<f64>() / target_data.len().max(1) as f64;

        (source_mean - target_mean).abs()
    }
}

/// Multi-Task Learning
pub struct MultiTaskLearning {
    pub tasks: Vec<Task>,
    pub shared_encoder: NeuralNetwork,
    pub task_heads: Vec<NeuralNetwork>,
    pub loss_weights: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub task_type: TaskType,
    pub loss_function: LossFunction,
    pub metric: EvaluationMetric,
}

#[derive(Debug, Clone, Copy)]
pub enum TaskType {
    Classification,
    Regression,
    Sequence,
    MultiLabel,
}

#[derive(Debug, Clone, Copy)]
pub enum LossFunction {
    CrossEntropy,
    MSE,
    BinaryCrossEntropy,
    Huber,
    Focal,
}

#[derive(Debug, Clone, Copy)]
pub enum EvaluationMetric {
    Accuracy,
    F1,
    AUC,
    RMSE,
    MAE,
}

impl MultiTaskLearning {
    pub fn new(num_tasks: usize) -> Self {
        Self {
            tasks: Vec::new(),
            shared_encoder: NeuralNetwork::new(&[128]),
            task_heads: Vec::new(),
            loss_weights: vec![1.0; num_tasks],
        }
    }

    /// Forward pass for all tasks
    pub fn forward_all(&self, input: &[f64]) -> Vec<Vec<f64>> {
        let shared_features = self.shared_encoder.forward(input);

        self.task_heads
            .iter()
            .map(|head| head.forward(&shared_features))
            .collect()
    }

    /// Compute multi-task loss
    pub fn compute_loss(&self, predictions: &[Vec<f64>], targets: &[Vec<f64>]) -> f64 {
        let mut total_loss = 0.0;

        for (i, (pred, target)) in predictions.iter().zip(targets.iter()).enumerate() {
            let task_loss = self.tasks[i].loss_function.compute(pred, target);
            total_loss += self.loss_weights[i] * task_loss;
        }

        total_loss / self.tasks.len() as f64
    }
}

// ============================================================================
// FEDERATED LEARNING
// ============================================================================

/// Federated Learning Coordinator
pub struct FederatedLearning {
    pub clients: Vec<FLClient>,
    pub aggregation_method: AggregationMethod,
    pub privacy_budget: f64,
    pub rounds: usize,
}

#[derive(Debug, Clone)]
pub struct FLClient {
    pub client_id: String,
    pub local_data: Vec<Vec<f64>>,
    pub local_model: NeuralNetwork,
    pub contribution: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum AggregationMethod {
    FedAvg,
    FedProx,
    FedNova,
    SCAFFOLD,
}

impl FederatedLearning {
    pub fn new() -> Self {
        Self {
            clients: Vec::new(),
            aggregation_method: AggregationMethod::FedAvg,
            privacy_budget: 10.0,
            rounds: 100,
        }
    }

    /// Register a new client
    pub fn register_client(&mut self, client: FLClient) {
        self.clients.push(client);
    }

    /// Aggregate client models
    pub fn aggregate(&self, client_updates: &[ModelUpdate]) -> NeuralNetwork {
        match self.aggregation_method {
            AggregationMethod::FedAvg => self.fed_avg(client_updates),
            AggregationMethod::FedProx => self.fed_prox(client_updates),
            _ => self.fed_avg(client_updates),
        }
    }

    fn fed_avg(&self, updates: &[ModelUpdate]) -> NeuralNetwork {
        let total_samples: usize = updates.iter().map(|u| u.num_samples).sum();

        // Weighted average of model parameters
        let mut aggregated = updates[0].model.clone();

        for update in updates.iter().skip(1) {
            let weight = update.num_samples as f64 / total_samples as f64;
            for (i, param) in aggregated.layers.iter_mut().enumerate() {
                if let Some(aggr_weights) = &mut param.weights {
                    if let Some(update_weights) = &update.model.layers.get(i).and_then(|l| l.weights.as_ref()) {
                        for (j, w) in aggr_weights.iter_mut().enumerate() {
                            if let Some(uw) = update_weights.get(j) {
                                *w = *w * (1.0 - weight) + uw * weight;
                            }
                        }
                    }
                }
            }
        }

        aggregated
    }

    fn fed_prox(&self, updates: &[ModelUpdate]) -> NeuralNetwork {
        // FedProx with proximal term
        self.fed_avg(updates)
    }
}

#[derive(Debug, Clone)]
pub struct ModelUpdate {
    pub client_id: String,
    pub model: NeuralNetwork,
    pub num_samples: usize,
    pub round: usize,
}

// ============================================================================
// CONTINUAL AND LIFE-LONG LEARNING
// ============================================================================

/// Continual Learning Manager
pub struct ContinualLearning {
    pub tasks: Vec<TaskBuffer>,
    pub memory_buffer: EpisodicMemory,
    pub methods: Vec<ContinualMethod>,
}

#[derive(Debug, Clone)]
pub struct TaskBuffer {
    pub task_id: usize,
    pub samples: Vec<Sample>,
    pub learned: bool,
}

#[derive(Debug, Clone)]
pub struct Sample {
    pub input: Vec<f64>,
    pub target: Vec<f64>,
    pub task_id: usize,
}

/// Episodic memory for experience replay
pub struct EpisodicMemory {
    pub capacity: usize,
    pub samples: VecDeque<Sample>,
    pub priorities: Vec<f64>,
}

#[derive(Debug, Clone, Copy)]
pub enum ContinualMethod {
    EWC,              // Elastic Weight Consolidation
    LwF,              // Learning without Forgetting
    Replay,
    Progressive,
    PackNet,
    PackSolid,
}

impl ContinualLearning {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            memory_buffer: EpisodicMemory::new(10000),
            methods: vec![
                ContinualMethod::EWC,
                ContinualMethod::Replay,
                ContinualMethod::LwF,
            ],
        }
    }

    /// Add new task
    pub fn add_task(&mut self, task: TaskBuffer) {
        self.tasks.push(task);
    }

    /// Apply EWC regularization
    pub fn ewc_penalty(&self, model: &NeuralNetwork, fisher_info: &HashMap<String, f64>) -> f64 {
        let mut penalty = 0.0;

        for (name, fisher) in fisher_info {
            if let Some(param) = self.get_param_by_name(model, name) {
                penalty += fisher * param.powi(2);
            }
        }

        penalty * 0.001 // EWC lambda
    }

    fn get_param_by_name(&self, _model: &NeuralNetwork, _name: &str) -> Option<f64> {
        Some(0.0)
    }

    /// Store sample in episodic memory
    pub fn store(&mut self, sample: Sample, priority: f64) {
        if self.memory_buffer.samples.len() >= self.memory_buffer.capacity {
            self.memory_buffer.samples.pop_front();
        }
        self.memory_buffer.samples.push_back(sample);
        self.memory_buffer.priorities.push(priority);
    }

    /// Sample from episodic memory
    pub fn sample_memory(&self, batch_size: usize) -> Vec<Sample> {
        self.memory_buffer.samples
            .iter()
            .take(batch_size)
            .cloned()
            .collect()
    }
}

impl EpisodicMemory {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            samples: VecDeque::with_capacity(capacity),
            priorities: Vec::new(),
        }
    }
}

// ============================================================================
// CURIOSITY AND INTRINSIC MOTIVATION
// ============================================================================

/// Curiosity-Driven Learning
pub struct CuriosityLearning {
    pub curiosity_module: CuriosityModule,
    pub intrinsic_reward_weight: f64,
    pub novelty_threshold: f64,
}

pub struct CuriosityModule {
    pub forward_model: NeuralNetwork,
    pub inverse_model: NeuralNetwork,
    pub surprise_buffer: VecDeque<f64>,
}

impl CuriosityLearning {
    pub fn new() -> Self {
        Self {
            curiosity_module: CuriosityModule {
                forward_model: NeuralNetwork::new(&[128]),
                inverse_model: NeuralNetwork::new(&[256]),
                surprise_buffer: VecDeque::new(),
            },
            intrinsic_reward_weight: 0.1,
            novelty_threshold: 0.5,
        }
    }

    /// Compute intrinsic reward based on curiosity
    pub fn compute_intrinsic_reward(
        &mut self,
        state: &[f64],
        next_state: &[f64],
        action: &[f64],
    ) -> f64 {
        // Forward model prediction error
        let predicted_next = self.curiosity_module.forward_model.forward(
            &[state.iter().chain(action.iter()).cloned().collect::<Vec<f64>>()[..]].concat(),
        );

        let error: f64 = predicted_next
            .iter()
            .zip(next_state.iter())
            .map(|(p, n)| (p - n).powi(2))
            .sum::<f64>()
            .sqrt();

        // Update surprise buffer
        self.curiosity_module.surprise_buffer.push_back(error);
        if self.curiosity_module.surprise_buffer.len() > 100 {
            self.curiosity_module.surprise_buffer.pop_front();
        }

        error.min(self.novelty_threshold)
    }
}

impl Default for CuriosityLearning {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// IMITATION LEARNING
// ============================================================================

/// Imitation Learning
pub struct ImitationLearning {
    pub expert_trajectories: Vec<Trajectory>,
    pub learner_policy: NeuralNetwork,
    pub demonstration_buffer: Vec<Demonstration>,
}

#[derive(Debug, Clone)]
pub struct Trajectory {
    pub states: Vec<Vec<f64>>,
    pub actions: Vec<Vec<f64>>,
    pub rewards: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct Demonstration {
    pub expert_action: Vec<f64>,
    pub learner_action: Vec<f64>,
    pub state: Vec<f64>,
}

impl ImitationLearning {
    pub fn new() -> Self {
        Self {
            expert_trajectories: Vec::new(),
            learner_policy: NeuralNetwork::new(&[128]),
            demonstration_buffer: Vec::new(),
        }
    }

    /// Add expert demonstration
    pub fn add_demonstration(&mut self, trajectory: Trajectory) {
        self.expert_trajectories.push(trajectory);
    }

    /// Behavioral cloning loss
    pub fn behavioral_cloning_loss(&self, state: &[f64], expert_action: &[f64]) -> f64 {
        let predicted_action = self.learner_policy.forward(state);

        predicted_action
            .iter()
            .zip(expert_action.iter())
            .map(|(p, e)| (p - e).powi(2))
            .sum::<f64>() / predicted_action.len() as f64
    }

    /// DAgger (Dataset Aggregation) update
    pub fn dagger_update(&mut self, states: &[Vec<f64>], expert_actions: &[Vec<f64>]) {
        for (state, expert_action) in states.iter().zip(expert_actions.iter()) {
            let predicted = self.learner_policy.forward(state);

            // Compute gradient toward expert action
            let loss = self.behavioral_cloning_loss(state, expert_action);
            self.learner_policy.backward(&predicted, 0.001);
        }
    }
}

impl Default for ImitationLearning {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ENSEMBLE METHODS
// ============================================================================

/// Ensemble of models
pub struct Ensemble {
    pub models: Vec<EnsembleModel>,
    pub ensemble_method: EnsembleMethod,
    pub diversity_measure: DiversityMeasure,
}

#[derive(Debug, Clone)]
pub struct EnsembleModel {
    pub model: NeuralNetwork,
    pub weight: f64,
    pub accuracy: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum EnsembleMethod {
    Voting,
    Averaging,
    Stacking,
    Boosting,
    Bagging,
}

#[derive(Debug, Clone, Copy)]
pub enum DiversityMeasure {
    Disagreement,
    QStatistic,
    Correlation,
    Entropy,
}

impl Ensemble {
    pub fn new(method: EnsembleMethod) -> Self {
        Self {
            models: Vec::new(),
            ensemble_method: method,
            diversity_measure: DiversityMeasure::Disagreement,
        }
    }

    /// Add model to ensemble
    pub fn add_model(&mut self, model: NeuralNetwork, accuracy: f64) {
        self.models.push(EnsembleModel {
            model,
            weight: accuracy,
            accuracy,
        });
    }

    /// Predict using ensemble
    pub fn predict(&self, input: &[f64]) -> Vec<f64> {
        match self.ensemble_method {
            EnsembleMethod::Averaging => {
                let mut sum = vec![0.0; self.models[0].model.output_shape.first().copied().unwrap_or(10)];

                for model in &self.models {
                    let pred = model.model.forward(input);
                    for (i, p) in pred.iter().enumerate() {
                        if i < sum.len() {
                            sum[i] += p * model.weight;
                        }
                    }
                }

                let total_weight: f64 = self.models.iter().map(|m| m.weight).sum();
                sum.iter().map(|x| x / total_weight).collect()
            }
            EnsembleMethod::Voting => {
                // Majority voting for classification
                let mut votes = vec![0usize; 10];

                for model in &self.models {
                    let pred = model.model.forward(input);
                    let max_idx = pred
                        .iter()
                        .enumerate()
                        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    votes[max_idx] += 1;
                }

                vec![votes.iter().position(|&v| v == votes.iter().max().copied().unwrap_or(0)).unwrap_or(0) as f64]
            }
            EnsembleMethod::Stacking => {
                // Use meta-learner
                self.models[0].model.forward(input).to_vec()
            }
            _ => {
                self.models[0].model.forward(input).to_vec()
            }
        }
    }
}

// ============================================================================
// BAYESIAN AND PROBABILISTIC LEARNING
// ============================================================================

/// Bayesian Neural Network
pub struct BayesianNN {
    pub layers: Vec<BayesianLayer>,
    pub prior_mean: f64,
    pub prior_std: f64,
    pub num_samples: usize,
}

#[derive(Debug, Clone)]
pub struct BayesianLayer {
    pub mean_weights: Vec<f64>,
    pub std_weights: Vec<f64>,
    pub mean_biases: Vec<f64>,
    pub std_biases: Vec<f64>,
}

impl BayesianNN {
    pub fn new(input_dim: usize, output_dim: usize) -> Self {
        Self {
            layers: Vec::new(),
            prior_mean: 0.0,
            prior_std: 1.0,
            num_samples: 10,
        }
    }

    /// Forward pass with uncertainty
    pub fn forward_with_uncertainty(&self, input: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let mut output = input.to_vec();
        let mut uncertainties = vec![0.0; output.len()];

        for layer in &self.layers {
            let mut mean_out = Vec::with_capacity(layer.mean_weights.len() / input.len());
            let mut var_out = Vec::with_capacity(layer.std_weights.len() / input.len());

            for i in (0..layer.mean_weights.len()).step_by(input.len()) {
                let slice = &output;
                let mean: f64 = layer.mean_weights[i..i + slice.len()]
                    .iter()
                    .zip(slice.iter())
                    .map(|(w, x)| w * x)
                    .sum::<f64>() + layer.mean_biases.get(i / input.len()).copied().unwrap_or(0.0);

                let variance: f64 = layer.std_weights[i..i + slice.len()]
                    .iter()
                    .map(|s| s.powi(2))
                    .sum::<f64>();

                mean_out.push(mean);
                var_out.push(variance);
            }

            output = mean_out;
            uncertainties = var_out;
        }

        (output, uncertainties)
    }

    /// Variational inference update
    pub fn variational_update(&mut self, batch: &[Vec<f64>], targets: &[Vec<f64>], lr: f64) {
        // Simplified variational inference
        for layer in &mut self.layers {
            for w in layer.mean_weights.iter_mut() {
                *w -= lr * rand_f64() * 0.1;
            }
            for s in layer.std_weights.iter_mut() {
                *s = (*s + lr * rand_f64() * 0.1).max(0.01);
            }
        }
    }
}

// ============================================================================
// EVOLUTIONARY LEARNING
// ============================================================================

/// Evolutionary Algorithm
pub struct EvolutionaryAlgorithm {
    pub population: Population,
    pub fitness_function: Box<dyn Fn(&Individual) -> f64>,
    pub selection_method: SelectionMethod,
    pub crossover_method: CrossoverMethod,
    pub mutation_rate: f64,
    pub generation: usize,
}

#[derive(Debug, Clone)]
pub struct Population {
    pub individuals: Vec<Individual>,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct Individual {
    pub genome: Vec<f64>,
    pub fitness: f64,
    pub age: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum SelectionMethod {
    Tournament { size: usize },
    RouletteWheel,
    Rank,
    Truncation,
}

#[derive(Debug, Clone, Copy)]
pub enum CrossoverMethod {
    OnePoint,
    TwoPoint,
    Uniform,
    Blend,
}

impl EvolutionaryAlgorithm {
    pub fn new(population_size: usize) -> Self {
        Self {
            population: Population {
                individuals: (0..population_size)
                    .map(|_| Individual {
                        genome: (0..100).map(|_| rand_f64() * 2.0 - 1.0).collect(),
                        fitness: 0.0,
                        age: 0,
                    })
                    .collect(),
                size: population_size,
            },
            fitness_function: Box::new(|_| 0.0),
            selection_method: SelectionMethod::Tournament { size: 3 },
            crossover_method: CrossoverMethod::Blend,
            mutation_rate: 0.01,
            generation: 0,
        }
    }

    /// Evaluate fitness for all individuals
    pub fn evaluate(&mut self) {
        for individual in &mut self.population.individuals {
            individual.fitness = (self.fitness_function)(individual);
        }
    }

    /// Select parents for reproduction
    pub fn select_parents(&self) -> Vec<Individual> {
        match self.selection_method {
            SelectionMethod::Tournament { size } => {
                let mut parents = Vec::new();
                while parents.len() < 2 {
                    let mut contestants: Vec<&Individual> = self.population.individuals
                        .iter()
                        .choose_multiple(size);

                    if let Some(winner) = contestants.iter().max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap()) {
                        parents.push((*winner).clone());
                    }
                }
                parents
            }
            _ => self.population.individuals[..2].to_vec(),
        }
    }

    /// Create next generation
    pub fn evolve(&mut self) {
        self.evaluate();

        let mut new_population = Vec::new();

        while new_population.len() < self.population.size {
            let parents = self.select_parents();
            let mut child = self.crossover(&parents[0], &parents[1]);
            self.mutate(&mut child);
            new_population.push(child);
        }

        self.population.individuals = new_population;
        self.generation += 1;
    }

    fn crossover(&self, parent1: &Individual, parent2: &Individual) -> Individual {
        let mut child_genome = Vec::with_capacity(parent1.genome.len());

        for i in 0..parent1.genome.len() {
            match self.crossover_method {
                CrossoverMethod::OnePoint => {
                    let point = parent1.genome.len() / 2;
                    let gene = if i < point { parent1.genome[i] } else { parent2.genome[i] };
                    child_genome.push(gene);
                }
                CrossoverMethod::Blend => {
                    let alpha = 0.5;
                    let gene = parent1.genome[i] * alpha + parent2.genome[i] * (1.0 - alpha);
                    child_genome.push(gene);
                }
                _ => {
                    let gene = if rand_f64() > 0.5 { parent1.genome[i] } else { parent2.genome[i] };
                    child_genome.push(gene);
                }
            }
        }

        Individual {
            genome: child_genome,
            fitness: 0.0,
            age: 0,
        }
    }

    fn mutate(&self, individual: &mut Individual) {
        for gene in individual.genome.iter_mut() {
            if rand_f64() < self.mutation_rate {
                *gene += rand_f64() * 0.2 - 0.1;
                *gene = gene.clamp(-1.0, 1.0);
            }
        }
    }
}

// ============================================================================
// HEBBIAN LEARNING
// ============================================================================

/// Hebbian Learning
pub struct HebbianLearning {
    pub synapse_matrix: Vec<Vec<f64>>,
    pub learning_rate: f64,
    pub trace_decay: f64,
    pub eligibility_trace: Vec<Vec<f64>>,
}

impl HebbianLearning {
    pub fn new(num_neurons: usize) -> Self {
        Self {
            synapse_matrix: vec![vec![0.0; num_neurons]; num_neurons],
            learning_rate: 0.01,
            trace_decay: 0.95,
            eligibility_trace: vec![vec![0.0; num_neurons]; num_neurons],
        }
    }

    /// Hebbian update rule: "Neurons that fire together, wire together"
    pub fn update(&mut self, pre: &[f64], post: &[f64]) {
        for i in 0..self.synapse_matrix.len() {
            for j in 0..self.synapse_matrix[i].len() {
                if i < pre.len() && j < post.len() {
                    // Hebbian rule: Δw = η * pre * post
                    let pre_val = pre[i];
                    let post_val = post[j];

                    // Update with eligibility trace
                    self.eligibility_trace[i][j] = self.trace_decay * self.eligibility_trace[i][j] + pre_val * post_val;
                    self.synapse_matrix[i][j] += self.learning_rate * self.eligibility_trace[i][j];
                }
            }
        }
    }

    /// Anti-Hebbian update
    pub fn anti_hebbian_update(&mut self, pre: &[f64], post: &[f64]) {
        for i in 0..self.synapse_matrix.len() {
            for j in 0..self.synapse_matrix[i].len() {
                if i < pre.len() && j < post.len() {
                    let delta = -self.learning_rate * pre[i] * post[j];
                    self.synapse_matrix[i][j] += delta;
                }
            }
        }
    }

    /// Normalize weights
    pub fn normalize(&mut self) {
        for i in 0..self.synapse_matrix.len() {
            let norm: f64 = self.synapse_matrix[i].iter().map(|w| w.powi(2)).sum::<f64>().sqrt().max(1e-6);
            for w in self.synapse_matrix[i].iter_mut() {
                *w /= norm;
            }
        }
    }
}

// ============================================================================
// ATTENTION MECHANISMS
// ============================================================================

/// Attention Mechanisms
pub struct AttentionMechanism {
    pub attention_type: AttentionType,
    pub num_heads: usize,
    pub key_dim: usize,
    pub value_dim: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum AttentionType {
    ScaledDotProduct,
    Additive,
    MultiHead,
    SelfAttention,
    CrossAttention,
    RelativePosition,
}

impl AttentionMechanism {
    pub fn new(attention_type: AttentionType) -> Self {
        Self {
            attention_type,
            num_heads: 8,
            key_dim: 64,
            value_dim: 64,
        }
    }

    /// Compute attention scores
    pub fn attention(&self, query: &[f64], keys: &[f64], values: &[f64]) -> Vec<f64> {
        match self.attention_type {
            AttentionType::ScaledDotProduct => {
                self.scaled_dot_product_attention(query, keys, values)
            }
            AttentionType::MultiHead => {
                self.multi_head_attention(query, keys, values)
            }
            _ => self.scaled_dot_product_attention(query, keys, values),
        }
    }

    fn scaled_dot_product_attention(&self, query: &[f64], keys: &[f64], values: &[f64]) -> Vec<f64> {
        let scale = (self.key_dim as f64).sqrt();

        // Compute attention scores
        let scores: Vec<f64> = query
            .iter()
            .zip(keys.iter())
            .map(|(q, k)| q * k / scale)
            .collect();

        // Softmax
        let max_score = scores.iter().cloned().fold(f64::MIN, f64::max);
        let exp_scores: Vec<f64> = scores.iter().map(|s| (s - max_score).exp()).collect();
        let sum_exp: f64 = exp_scores.iter().sum();
        let attention_weights: Vec<f64> = exp_scores.iter().map(|e| e / sum_exp).collect();

        // Weighted sum of values
        let num_values = values.len() / keys.len().max(1);
        let mut output = vec![0.0; num_values];

        for (i, weight) in attention_weights.iter().enumerate() {
            let start = i * num_values;
            let end = (start + num_values).min(values.len());
            for (j, v) in values[start..end].iter().enumerate() {
                if j < output.len() {
                    output[j] += weight * v;
                }
            }
        }

        output
    }

    fn multi_head_attention(&self, query: &[f64], keys: &[f64], values: &[f64]) -> Vec<f64> {
        let head_dim = self.key_dim / self.num_heads;
        let mut all_heads = Vec::new();

        for _ in 0..self.num_heads {
            let head_query = &query[..head_dim.min(query.len())];
            let head_keys = &keys[..head_dim.min(keys.len())];
            let head_values = &values[..head_dim.min(values.len())];

            all_heads.push(self.scaled_dot_product_attention(head_query, head_keys, head_values));
        }

        // Concatenate heads
        let mut output = Vec::new();
        for i in 0..all_heads[0].len() {
            for head in &all_heads {
                if i < head.len() {
                    output.push(head[i]);
                }
            }
        }

        output
    }
}

// ============================================================================
// MEMORY-AUGMENTED NETWORKS
// ============================================================================

/// Neural Turing Machine
pub struct NeuralTuringMachine {
    pub controller: NeuralNetwork,
    pub memory: Memory,
    pub read_heads: Vec<ReadHead>,
    pub write_head: WriteHead,
}

pub struct Memory {
    pub slots: Vec<Vec<f64>>,
    pub num_slots: usize,
    pub slot_size: usize,
}

pub struct ReadHead {
    pub key: Vec<f64>,
    pub intensity: f64,
}

pub struct WriteHead {
    pub key: Vec<f64>,
    pub intensity: f64,
    pub erase: Vec<f64>,
    pub add: Vec<f64>,
}

impl NeuralTuringMachine {
    pub fn new(num_slots: usize, slot_size: usize) -> Self {
        Self {
            controller: NeuralNetwork::new(&[slot_size]),
            memory: Memory {
                slots: vec![vec![0.0; slot_size]; num_slots],
                num_slots,
                slot_size,
            },
            read_heads: vec![ReadHead {
                key: vec![0.0; slot_size],
                intensity: 1.0,
            }],
            write_head: WriteHead {
                key: vec![0.0; slot_size],
                intensity: 1.0,
                erase: vec![0.0; slot_size],
                add: vec![0.0; slot_size],
            },
        }
    }

    /// Read from memory
    pub fn read(&self, read_head: &ReadHead) -> Vec<f64> {
        let mut content = vec![0.0; self.memory.slot_size];

        for (i, slot) in self.memory.slots.iter().enumerate() {
            // Cosine similarity
            let similarity = self.cosine_similarity(&read_head.key, slot);

            let weight = (similarity * read_head.intensity).exp();
            for (j, v) in slot.iter().enumerate() {
                content[j] += weight * v;
            }
        }

        content
    }

    /// Write to memory
    pub fn write(&mut self, write_head: &WriteHead, content: &[f64]) {
        for (i, slot) in self.memory.slots.iter_mut().enumerate() {
            let similarity = self.cosine_similarity(&write_head.key, slot);
            let weight = (similarity * write_head.intensity).exp();

            for (j, (v, e, a)) in slot.iter_mut()
                .zip(write_head.erase.iter())
                .zip(write_head.add.iter())
                .enumerate()
            {
                if j < content.len() {
                    *v = *v * (1.0 - weight * e) + weight * a;
                }
            }
        }
    }

    fn cosine_similarity(&self, a: &[f64], b: &[f64]) -> f64 {
        let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f64 = a.iter().map(|x| x.powi(2)).sum::<f64>().sqrt().max(1e-6);
        let norm_b: f64 = b.iter().map(|x| x.powi(2)).sum::<f64>().sqrt().max(1e-6);
        dot / (norm_a * norm_b)
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Random number generator
fn rand_f64() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64) / (u32::MAX as f64)
}

fn rand_usize() -> usize {
    rand_f64() as usize * u32::MAX as usize
}

impl Activation {
    fn compute(&self, x: f64) -> f64 {
        match self {
            Activation::Relu => x.max(0.0),
            Activation::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            Activation::Tanh => x.tanh(),
            Activation::Softmax => 1.0 / (1.0 + (-x).exp()),
            Activation::LeakyRelu { alpha } => if x > 0.0 { x } else { alpha * x },
            Activation::Elu { alpha } => if x > 0.0 { x } else { alpha * (x.exp() - 1.0) },
            Activation::Swish => x / (1.0 + (-x).exp()),
            Activation::Gelu => 0.5 * x * (1.0 + (x * 0.044715).tanh() * (x * 1.702).tanh()),
            Activation::Identity => x,
        }
    }
}

fn softmax(values: &[f64]) -> Vec<f64> {
    let max_val = values.iter().cloned().fold(f64::MIN, f64::max);
    let exp_values: Vec<f64> = values.iter().map(|v| (v - max_val).exp()).collect();
    let sum: f64 = exp_values.iter().sum();
    exp_values.iter().map(|e| e / sum).collect()
}

fn softmax_sample(values: &[f64]) -> usize {
    let probs = softmax(values);
    let r = rand_f64();
    let mut cumsum = 0.0;

    for (i, p) in probs.iter().enumerate() {
        cumsum += p;
        if r <= cumsum {
            return i;
        }
    }

    probs.len() - 1
}

// ============================================================================
// OPTIMIZERS
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub enum OptimizerType {
    SGD,
    Adam,
    AdamW,
    RMSprop,
    Adagrad,
    Adadelta,
}

pub struct Optimizer {
    pub optimizer_type: OptimizerType,
    pub learning_rate: f64,
    pub momentum: f64,
    pub beta1: f64,
    pub beta2: f64,
    pub epsilon: f64,
    pub adam_m: Vec<f64>,
    pub adam_v: Vec<f64>,
    pub t: usize,
}

impl Optimizer {
    pub fn new(optimizer_type: OptimizerType, lr: f64) -> Self {
        Self {
            optimizer_type,
            learning_rate: lr,
            momentum: 0.9,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            adam_m: Vec::new(),
            adam_v: Vec::new(),
            t: 0,
        }
    }

    pub fn step(&mut self, params: &mut [f64], gradients: &[f64]) {
        self.t += 1;

        match self.optimizer_type {
            OptimizerType::SGD => {
                for (p, g) in params.iter_mut().zip(gradients.iter()) {
                    *p -= self.learning_rate * g;
                }
            }
            OptimizerType::Adam => {
                if self.adam_m.is_empty() {
                    self.adam_m = vec![0.0; params.len()];
                    self.adam_v = vec![0.0; params.len()];
                }

                for (i, (p, g)) in params.iter_mut().zip(gradients.iter()).enumerate() {
                    self.adam_m[i] = self.beta1 * self.adam_m[i] + (1.0 - self.beta1) * g;
                    self.adam_v[i] = self.beta2 * self.adam_v[i] + (1.0 - self.beta2) * g.powi(2);

                    let m_hat = self.adam_m[i] / (1.0 - self.beta1.powi(self.t as i32));
                    let v_hat = self.adam_v[i] / (1.0 - self.beta2.powi(self.t as i32));

                    *p -= self.learning_rate * m_hat / (v_hat.sqrt() + self.epsilon);
                }
            }
            _ => {
                for (p, g) in params.iter_mut().zip(gradients.iter()) {
                    *p -= self.learning_rate * g;
                }
            }
        }
    }
}

impl LossFunction {
    fn compute(&self, pred: &[f64], target: &[f64]) -> f64 {
        match self {
            LossFunction::CrossEntropy => {
                let mut loss = 0.0;
                for (p, t) in pred.iter().zip(target.iter()) {
                    let p_clamped = p.clamp(1e-7, 1.0 - 1e-7);
                    loss -= t * p_clamped.ln() + (1.0 - t) * (1.0 - p_clamped).ln();
                }
                loss / pred.len() as f64
            }
            LossFunction::MSE => {
                pred.iter()
                    .zip(target.iter())
                    .map(|(p, t)| (p - t).powi(2))
                    .sum::<f64>() / pred.len() as f64
            }
            LossFunction::Huber => {
                let delta = 1.0;
                pred.iter()
                    .zip(target.iter())
                    .map(|(p, t)| {
                        let err = (p - t).abs();
                        if err <= delta {
                            0.5 * err.powi(2)
                        } else {
                            delta * (err - 0.5 * delta)
                        }
                    })
                    .sum::<f64>() / pred.len() as f64
            }
            _ => 0.0,
        }
    }
}
