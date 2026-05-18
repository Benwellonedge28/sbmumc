//! # SBMUMC Module 1595: Federated Learning
//!
//! Privacy-preserving distributed machine learning across multiple clients.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedConfig {
    pub num_clients: usize,
    pub min_clients_per_round: usize,
    pub rounds: usize,
    pub fraction_fit: f64,
    pub fraction_eval: f64,
    pub local_epochs: usize,
    pub batch_size: usize,
    pub learning_rate: f64,
    pub server_learning_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub client_id: String,
    pub data_size: usize,
    pub available: bool,
    pub trustworthiness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientState {
    pub client_id: String,
    pub model_weights: HashMap<String, Vec<f32>>,
    pub num_samples: usize,
    pub accuracy: f64,
    pub training_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalModel {
    pub round: usize,
    pub weights: HashMap<String, Vec<f32>>,
    pub accuracy: f64,
    pub loss: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationResult {
    pub aggregated_weights: HashMap<String, Vec<f32>>,
    pub client_weights: Vec<(String, f32)>,
    pub num_clients: usize,
    pub update_variance: f64,
}

pub struct FederatedLearningServer {
    config: FederatedConfig,
    global_model: Option<GlobalModel>,
    clients: HashMap<String, ClientConfig>,
    history: Vec<GlobalModel>,
    privacy_budget: f64,
}

impl FederatedLearningServer {
    pub fn new(config: FederatedConfig) -> Self {
        Self {
            config,
            global_model: None,
            clients: HashMap::new(),
            history: Vec::new(),
            privacy_budget: 1.0,
        }
    }

    pub fn register_client(&mut self, client: ClientConfig) -> Result<()> {
        self.clients.insert(client.client_id.clone(), client);
        Ok(())
    }

    pub fn initialize_model(&mut self, initial_weights: HashMap<String, Vec<f32>>) -> Result<()> {
        self.global_model = Some(GlobalModel {
            round: 0,
            weights: initial_weights,
            accuracy: 0.0,
            loss: f64::MAX,
            timestamp: chrono::Utc::now().timestamp(),
        });
        Ok(())
    }

    pub fn run_round(&mut self, client_updates: Vec<ClientState>) -> Result<AggregationResult> {
        let required_clients = (self.config.fraction_fit * self.config.num_clients as f64) as usize;
        required_clients.max(1);

        if client_updates.len() < self.config.min_clients_per_round {
            return Err(SbmumcError::Internal(
                format!("Not enough clients: {} < {}", client_updates.len(), self.config.min_clients_per_round)
            ));
        }

        let result = self.aggregate_updates(client_updates)?;

        if let Some(ref mut model) = self.global_model {
            model.round += 1;
            model.weights = result.aggregated_weights.clone();
            model.timestamp = chrono::Utc::now().timestamp();
            self.history.push(model.clone());
        }

        Ok(result)
    }

    fn aggregate_updates(&self, updates: Vec<ClientState>) -> Result<AggregationResult> {
        if updates.is_empty() {
            return Err(SbmumcError::Internal("No updates to aggregate".into()));
        }

        let mut aggregated: HashMap<String, Vec<f32>> = HashMap::new();
        let mut total_weight = 0.0;
        let mut client_weights: Vec<(String, f32)> = Vec::new();

        for update in &updates {
            let weight = update.num_samples as f32;
            total_weight += weight;
            client_weights.push((update.client_id.clone(), weight));
        }

        for update in &updates {
            let weight = update.num_samples as f32 / total_weight;

            for (key, values) in &update.model_weights {
                let entry = aggregated.entry(key.clone()).or_insert_with(Vec::new);

                if entry.is_empty() {
                    *entry = values.iter().map(|v| v * weight).collect();
                } else {
                    for (i, v) in values.iter().enumerate() {
                        if i < entry.len() {
                            entry[i] += v * weight;
                        }
                    }
                }
            }
        }

        let mut variance_sum = 0.0;
        let num_layers = aggregated.len();
        for (_, values) in &aggregated {
            if !values.is_empty() {
                let mean = values.iter().sum::<f32>() / values.len() as f32;
                variance_sum += values.iter().map(|v| (v - mean).powi(2) as f64).sum::<f64>() / values.len() as f64;
            }
        }
        let update_variance = if num_layers > 0 { variance_sum / num_layers as f64 } else { 0.0 };

        Ok(AggregationResult {
            aggregated_weights: aggregated,
            client_weights,
            num_clients: updates.len(),
            update_variance,
        })
    }

    pub fn get_global_model(&self) -> Option<&GlobalModel> {
        self.global_model.as_ref()
    }

    pub fn get_client(&self, client_id: &str) -> Option<&ClientConfig> {
        self.clients.get(client_id)
    }

    pub fn get_history(&self) -> &[GlobalModel] {
        &self.history
    }

    pub fn apply_differential_privacy(&mut self, epsilon: f64, delta: f64) -> Result<()> {
        if let Some(ref mut model) = self.global_model {
            for values in model.weights.values_mut() {
                let sensitivity = 1.0 / epsilon;
                for v in values.iter_mut() {
                    let noise = Self::laplace_sample(sensitivity);
                    *v = (*v as f64 + noise) as f32;
                }
            }
            self.privacy_budget -= epsilon;
        }
        Ok(())
    }

    fn laplace_sample(scale: f64) -> f64 {
        let u: f64 = rand::random();
        let v: f64 = rand::random();
        let uniform = u - 0.5;
        -scale * uniform.signum() * (1.0 - (2.0 * v).abs()).ln()
    }

    pub fn evaluate_model(&self, test_data: &[Vec<f32>], test_labels: &[f32]) -> Result<(f64, f64)> {
        let model = self.global_model.as_ref()
            .ok_or_else(|| SbmumcError::Internal("No model to evaluate".into()))?;

        let mut correct = 0;
        let mut total_loss = 0.0;

        for (i, data) in test_data.iter().enumerate() {
            let prediction = self.predict(&model.weights, data)?;
            let label = test_labels.get(i).unwrap_or(&0.0);

            if (prediction - *label).abs() < 0.5 {
                correct += 1;
            }
            total_loss += (prediction - *label).powi(2) as f64;
        }

        let accuracy = correct as f64 / test_data.len() as f64;
        let loss = total_loss / test_data.len() as f64;

        Ok((accuracy, loss))
    }

    fn predict(&self, weights: &HashMap<String, Vec<f32>>, input: &[f32]) -> Result<f32> {
        if let Some(first) = weights.values().next() {
            let mut result = 0.0_f32;
            for (i, w) in first.iter().enumerate() {
                if i < input.len() {
                    result += input[i] * w;
                }
            }
            Ok(result / first.len() as f32)
        } else {
            Ok(0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_federated_learning() {
        let config = FederatedConfig {
            num_clients: 10,
            min_clients_per_round: 3,
            rounds: 5,
            fraction_fit: 0.5,
            fraction_eval: 0.3,
            local_epochs: 2,
            batch_size: 32,
            learning_rate: 0.01,
            server_learning_rate: 1.0,
        };

        let mut server = FederatedLearningServer::new(config);

        let mut weights = HashMap::new();
        weights.insert("layer1".to_string(), vec![0.1, 0.2, 0.3]);

        server.initialize_model(weights).unwrap();

        let client_updates = vec![
            ClientState {
                client_id: "client1".to_string(),
                model_weights: {
                    let mut m = HashMap::new();
                    m.insert("layer1".to_string(), vec![0.15, 0.25, 0.35]);
                    m
                },
                num_samples: 100,
                accuracy: 0.85,
                training_time_ms: 5000,
            },
            ClientState {
                client_id: "client2".to_string(),
                model_weights: {
                    let mut m = HashMap::new();
                    m.insert("layer1".to_string(), vec![0.12, 0.22, 0.32]);
                    m
                },
                num_samples: 150,
                accuracy: 0.87,
                training_time_ms: 6000,
            },
        ];

        let result = server.run_round(client_updates);
        assert!(result.is_ok());
    }
}