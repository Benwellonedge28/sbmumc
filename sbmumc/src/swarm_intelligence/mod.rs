//! # SBMUMC Module 1605: Swarm Intelligence
//!
//! Collective intelligence systems and swarm algorithms.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmConfig {
    pub agent_count: usize,
    pub algorithm: SwarmAlgorithm,
    pub neighborhood_radius: f64,
    pub convergence_threshold: f64,
    pub max_iterations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwarmAlgorithm {
    ParticleSwarm,
    AntColony,
    BeeColony,
    CuckooSearch,
    Firefly,
    Bat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub agent_id: String,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub personal_best: Vec<f64>,
    pub personal_best_score: f64,
    pub role: AgentRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentRole {
    Scout,
    Worker,
    Leader,
    Explorer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmState {
    pub state_id: String,
    pub iteration: usize,
    pub global_best: Vec<f64>,
    pub global_best_score: f64,
    pub avg_fitness: f64,
    pub convergence_rate: f64,
}

pub struct SwarmOptimizer {
    config: SwarmConfig,
    agents: Vec<Agent>,
    history: Vec<SwarmState>,
    objective_function: fn(&[f64]) -> f64,
}

impl SwarmOptimizer {
    pub fn new(config: SwarmConfig, objective: fn(&[f64]) -> f64) -> Self {
        Self {
            config,
            agents: Vec::new(),
            history: Vec::new(),
            objective_function: objective,
        }
    }

    pub fn initialize(&mut self, dimensions: usize) -> Result<()> {
        for i in 0..self.config.agent_count {
            let position: Vec<f64> = (0..dimensions)
                .map(|_| rand::random::<f64>() * 100.0 - 50.0)
                .collect();

            let velocity: Vec<f64> = (0..dimensions)
                .map(|_| rand::random::<f64>() * 2.0 - 1.0)
                .collect();

            let agent = Agent {
                agent_id: format!("agent_{}", i),
                position: position.clone(),
                velocity,
                personal_best: position,
                personal_best_score: f64::MAX,
                role: if i == 0 { AgentRole::Leader } else { AgentRole::Worker },
            };

            self.agents.push(agent);
        }

        Ok(())
    }

    pub fn optimize(&mut self) -> Result<SwarmState> {
        let mut global_best = vec![0.0; self.agents.first().map(|a| a.position.len()).unwrap_or(0)];
        let mut global_best_score = f64::MAX;

        for iteration in 0..self.config.max_iterations {
            for agent in &mut self.agents {
                let score = (self.objective_function)(&agent.position);

                if score < agent.personal_best_score {
                    agent.personal_best = agent.position.clone();
                    agent.personal_best_score = score;
                }

                if score < global_best_score {
                    global_best = agent.position.clone();
                    global_best_score = score;
                }

                self.update_velocity(agent, &global_best);
                self.update_position(agent);
            }

            let avg_fitness = self.agents.iter()
                .map(|a| (self.objective_function)(&a.position))
                .sum::<f64>() / self.agents.len() as f64;

            let state = SwarmState {
                state_id: uuid::Uuid::new_v4().to_string(),
                iteration,
                global_best: global_best.clone(),
                global_best_score,
                avg_fitness,
                convergence_rate: 1.0 / (1.0 + iteration as f64),
            };

            self.history.push(state);

            if global_best_score < self.config.convergence_threshold {
                break;
            }
        }

        Ok(self.history.last().unwrap().clone())
    }

    fn update_velocity(&self, agent: &mut Agent, global_best: &[f64]) {
        let inertia = 0.7;
        let cognitive = 1.5;
        let social = 1.5;

        for i in 0..agent.velocity.len() {
            let r1 = rand::random::<f64>();
            let r2 = rand::random::<f64>();

            agent.velocity[i] = inertia * agent.velocity[i] +
                cognitive * r1 * (agent.personal_best[i] - agent.position[i]) +
                social * r2 * (global_best[i] - agent.position[i]);
        }
    }

    fn update_position(&self, agent: &mut Agent) {
        for i in 0..agent.position.len() {
            agent.position[i] += agent.velocity[i];

            agent.position[i] = agent.position[i].max(-100.0).min(100.0);
        }
    }

    pub fn get_history(&self) -> &[SwarmState] {
        &self.history
    }

    pub fn get_best_solution(&self) -> Option<&Vec<f64>> {
        self.history.last().map(|s| &s.global_best)
    }
}

fn rosenbrock(x: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..x.len() - 1 {
        sum += 100.0 * (x[i + 1] - x[i].powi(2)).powi(2) + (x[i] - 1.0).powi(2);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swarm_optimizer() {
        let config = SwarmConfig {
            agent_count: 30,
            algorithm: SwarmAlgorithm::ParticleSwarm,
            neighborhood_radius: 10.0,
            convergence_threshold: 0.001,
            max_iterations: 100,
        };

        let mut swarm = SwarmOptimizer::new(config, rosenbrock);
        swarm.initialize(10).unwrap();

        let result = swarm.optimize();
        assert!(result.is_ok());

        let best = swarm.get_best_solution();
        assert!(best.is_some());
    }
}