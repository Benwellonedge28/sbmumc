//! # SBMUMC Module 1601: Edge Computing
//!
//! Distributed computing at the network edge for low-latency processing.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeConfig {
    pub num_nodes: usize,
    pub latency_threshold_ms: u64,
    pub bandwidth_mbps: u64,
    pub compute_capacity: ComputeCapacity,
    pub storage_capacity_gb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeCapacity {
    pub cpu_cores: usize,
    pub cpu_ghz: f64,
    pub ram_gb: usize,
    pub gpu_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeNode {
    pub node_id: String,
    pub location: NodeLocation,
    pub capacity: ComputeCapacity,
    pub status: NodeStatus,
    pub active_services: Vec<String>,
    pub connected_devices: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeLocation {
    pub region: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeStatus {
    Online,
    Offline,
    Degraded,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeTask {
    pub task_id: String,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub data_size_mb: f64,
    pub compute_requirements: ComputeRequirements,
    pub deadline_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Inference,
    DataProcessing,
    Storage,
    Communication,
    Computation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Critical,
    High,
    Normal,
    Low,
    Background,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeRequirements {
    pub cpu_cycles: u64,
    pub memory_mb: usize,
    pub gpu_required: bool,
    pub estimated_time_ms: u64,
}

pub struct EdgeComputing {
    config: EdgeConfig,
    nodes: HashMap<String, EdgeNode>,
    tasks: HashMap<String, EdgeTask>,
    routing_table: HashMap<String, Vec<String>>,
}

impl EdgeComputing {
    pub fn new(config: EdgeConfig) -> Self {
        Self {
            config,
            nodes: HashMap::new(),
            tasks: HashMap::new(),
            routing_table: HashMap::new(),
        }
    }

    pub fn register_node(&mut self, node: EdgeNode) -> Result<()> {
        self.nodes.insert(node.node_id.clone(), node);
        Ok(())
    }

    pub fn submit_task(&mut self, task: EdgeTask) -> Result<String> {
        let target_node = self.find_optimal_node(&task)?;
        self.tasks.insert(task.task_id.clone(), task);

        if let Some(routes) = self.routing_table.get_mut(&target_node) {
            routes.push(task.task_id.clone());
        } else {
            self.routing_table.insert(target_node, vec![task.task_id.clone()]);
        }

        Ok(target_node)
    }

    fn find_optimal_node(&self, task: &EdgeTask) -> Result<String> {
        let mut candidates: Vec<&EdgeNode> = self.nodes.values()
            .filter(|n| n.status == NodeStatus::Online)
            .collect();

        if candidates.is_empty() {
            return Err(SbmumcError::Internal("No available nodes".into()));
        }

        candidates.sort_by(|a, b| {
            let a_score = self.score_node(a, task);
            let b_score = self.score_node(b, task);
            b_score.partial_cmp(&a_score).unwrap()
        });

        candidates.first()
            .map(|n| n.node_id.clone())
            .ok_or_else(|| SbmumcError::Internal("No suitable node found".into()))
    }

    fn score_node(&self, node: &EdgeNode, task: &EdgeTask) -> f64 {
        let capacity_score = node.capacity.cpu_cores as f64 / 64.0;
        let latency_factor = if task.deadline_ms > 0 {
            1.0 - (task.estimated_time_ms as f64 / task.deadline_ms as f64).min(1.0)
        } else {
            0.5
        };

        capacity_score * 0.6 + latency_factor * 0.4
    }

    pub fn execute_task(&mut self, task_id: &str) -> Result<TaskResult> {
        let task = self.tasks.get(task_id)
            .ok_or_else(|| SbmumcError::Internal("Task not found".into()))?;

        let result = TaskResult {
            task_id: task_id.to_string(),
            executed_at: chrono::Utc::now().timestamp(),
            execution_time_ms: rand::random::<u64>() % 1000,
            success: true,
            output: format!("result_for_task_{}", task_id),
            metrics: ExecutionMetrics {
                cpu_used: rand::random::<u64>() % 100,
                memory_used_mb: rand::random::<usize>() % 10000,
                energy_consumed_joules: rand::random::<f64>() * 100.0,
            },
        };

        Ok(result)
    }

    pub fn get_node_status(&self, node_id: &str) -> Option<&EdgeNode> {
        self.nodes.get(node_id)
    }

    pub fn get_all_nodes(&self) -> Vec<&EdgeNode> {
        self.nodes.values().collect()
    }

    pub fn get_pending_tasks(&self) -> Vec<&EdgeTask> {
        self.tasks.values().collect()
    }

    pub fn optimize_routing(&mut self) -> Result<()> {
        let mut routing = HashMap::new();

        for (node_id, _) in &self.nodes {
            let neighbors: Vec<String> = self.nodes.keys()
                .filter(|k| *k != node_id)
                .take(3)
                .cloned()
                .collect();
            routing.insert(node_id.clone(), neighbors);
        }

        self.routing_table = routing;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: String,
    pub executed_at: i64,
    pub execution_time_ms: u64,
    pub success: bool,
    pub output: String,
    pub metrics: ExecutionMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub cpu_used: u64,
    pub memory_used_mb: usize,
    pub energy_consumed_joules: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_computing() {
        let config = EdgeConfig {
            num_nodes: 10,
            latency_threshold_ms: 50,
            bandwidth_mbps: 1000,
            compute_capacity: ComputeCapacity {
                cpu_cores: 16,
                cpu_ghz: 3.5,
                ram_gb: 32,
                gpu_available: true,
            },
            storage_capacity_gb: 500,
        };

        let mut edge = EdgeComputing::new(config);

        let node = EdgeNode {
            node_id: "node_1".to_string(),
            location: NodeLocation {
                region: "us-west".to_string(),
                city: "San Francisco".to_string(),
                latitude: 37.7749,
                longitude: -122.4194,
            },
            capacity: ComputeCapacity {
                cpu_cores: 32,
                cpu_ghz: 4.0,
                ram_gb: 64,
                gpu_available: true,
            },
            status: NodeStatus::Online,
            active_services: vec!["inference".to_string()],
            connected_devices: 100,
        };

        edge.register_node(node).unwrap();
        assert_eq!(edge.get_all_nodes().len(), 1);
    }
}