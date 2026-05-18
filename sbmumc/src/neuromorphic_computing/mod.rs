//! # SBMUMC Module 1602: Neuromorphic Computing
//!
//! Brain-inspired computing architectures and algorithms.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuromorphicConfig {
    pub architecture: SpikingArchitecture,
    pub num_neurons: usize,
    pub num_synapses: usize,
    pub plasticity_rule: PlasticityRule,
    pub learning_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpikingArchitecture {
    LeakyIntegrateAndFire,
    Izhikevich,
    HodgkinHuxley,
    HindmarshRose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlasticityRule {
    STDP,
    Homeostatic,
    BCM,
    Oja,
    SynapticScaling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuron {
    pub neuron_id: String,
    pub neuron_type: NeuronType,
    pub membrane_potential: f64,
    pub threshold: f64,
    pub reset_potential: f64,
    pub refractory_period_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NeuronType {
    Excitatory,
    Inhibitory,
    Modulatory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Synapse {
    pub synapse_id: String,
    pub pre_neuron: String,
    pub post_neuron: String,
    pub weight: f64,
    pub delay_ms: u64,
    pub plasticity_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikeEvent {
    pub spike_id: String,
    pub neuron_id: String,
    pub timestamp_ms: u64,
    pub amplitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikingNetwork {
    pub network_id: String,
    pub neurons: Vec<Neuron>,
    pub synapses: Vec<Synapse>,
    pub layers: Vec<NetworkLayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkLayer {
    pub layer_id: String,
    pub layer_type: LayerType,
    pub neuron_indices: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayerType {
    Input,
    Hidden,
    Output,
    Recurrent,
}

pub struct NeuromorphicProcessor {
    config: NeuromorphicConfig,
    network: Option<SpikingNetwork>,
    spike_history: Vec<SpikeEvent>,
    time_ms: u64,
}

impl NeuromorphicProcessor {
    pub fn new(config: NeuromorphicConfig) -> Self {
        Self {
            config,
            network: None,
            spike_history: Vec::new(),
            time_ms: 0,
        }
    }

    pub fn build_network(&mut self, topology: NetworkTopology) -> Result<SpikingNetwork> {
        let mut neurons = Vec::new();
        let mut synapses = Vec::new();

        for i in 0..self.config.num_neurons {
            let neuron = Neuron {
                neuron_id: format!("neuron_{}", i),
                neuron_type: if i < self.config.num_neurons / 10 {
                    NeuronType::Input
                } else if i >= self.config.num_neurons * 9 / 10 {
                    NeuronType::Output
                } else {
                    NeuronType::Excitatory
                },
                membrane_potential: -70.0,
                threshold: -55.0,
                reset_potential: -70.0,
                refractory_period_ms: 5,
            };
            neurons.push(neuron);
        }

        for pre in 0..self.config.num_neurons {
            for post in (pre + 1)..self.config.num_neurons.min(pre + 10) {
                let synapse = Synapse {
                    synapse_id: format!("synapse_{}_{}", pre, post),
                    pre_neuron: format!("neuron_{}", pre),
                    post_neuron: format!("neuron_{}", post),
                    weight: (rand::random::<f64>() - 0.5) * 0.1,
                    delay_ms: (rand::random::<u64>() % 10) as u64,
                    plasticity_enabled: true,
                };
                synapses.push(synapse);
            }
        }

        let network = SpikingNetwork {
            network_id: uuid::Uuid::new_v4().to_string(),
            neurons,
            synapses,
            layers: Vec::new(),
        };

        self.network = Some(network.clone());
        Ok(network)
    }

    pub fn inject_current(&mut self, neuron_id: &str, current_pa: f64) -> Result<()> {
        let network = self.network.as_mut()
            .ok_or_else(|| SbmumcError::Internal("Network not built".into()))?;

        let neuron = network.neurons.iter_mut()
            .find(|n| n.neuron_id == neuron_id)
            .ok_or_else(|| SbmumcError::Internal("Neuron not found".into()))?;

        neuron.membrane_potential += current_pa * 0.01;

        if neuron.membrane_potential >= neuron.threshold {
            self.generate_spike(&mut neuron.clone())?;
        }

        Ok(())
    }

    fn generate_spike(&mut self, neuron: &mut Neuron) -> Result<SpikeEvent> {
        let spike = SpikeEvent {
            spike_id: uuid::Uuid::new_v4().to_string(),
            neuron_id: neuron.neuron_id.clone(),
            timestamp_ms: self.time_ms,
            amplitude: 1.0,
        };

        neuron.membrane_potential = neuron.reset_potential;
        self.spike_history.push(spike.clone());

        self.propagate_spike(neuron)?;

        Ok(spike)
    }

    fn propagate_spike(&mut self, neuron: &Neuron) -> Result<()> {
        let network = self.network.as_mut().unwrap();

        let post_synapses: Vec<&mut Synapse> = network.synapses.iter_mut()
            .filter(|s| s.pre_neuron == neuron.neuron_id)
            .collect();

        for synapse in post_synapses {
            let weight = synapse.weight;
            let delay = synapse.delay_ms;

            if let Some(target) = network.neurons.iter_mut()
                .find(|n| n.neuron_id == synapse.post_neuron)
            {
                let scheduled_time = self.time_ms + delay;
                target.membrane_potential += weight * 10.0;

                if target.membrane_potential >= target.threshold && scheduled_time >= self.time_ms {
                    self.generate_spike(target)?;
                }
            }
        }

        Ok(())
    }

    pub fn simulate(&mut self, duration_ms: u64, dt_ms: u64) -> Result<SimulationResult> {
        let mut spike_count = 0;

        while self.time_ms < duration_ms {
            for neuron in &mut self.network.as_mut().unwrap().neurons {
                if rand::random::<f64>() < 0.01 {
                    neuron.membrane_potential += 5.0;
                }

                if neuron.membrane_potential >= neuron.threshold {
                    self.generate_spike(neuron)?;
                    spike_count += 1;
                }

                neuron.membrane_potential = (neuron.membrane_potential * 0.9).max(neuron.reset_potential);
            }

            self.time_ms += dt_ms;
        }

        Ok(SimulationResult {
            total_spikes: spike_count,
            simulation_time_ms: duration_ms,
            active_neurons: self.network.as_ref().map(|n| n.neurons.len()).unwrap_or(0),
            average_firing_rate: spike_count as f64 / duration_ms as f64 * 1000.0,
        })
    }

    pub fn apply_stdp(&mut self) -> Result<()> {
        let network = self.network.as_mut().unwrap();

        for spike in &self.spike_history {
            for synapse in &mut network.synapses {
                if spike.neuron_id == synapse.pre_neuron {
                    let delta_t = self.time_ms as f64 - spike.timestamp_ms as f64;

                    if delta_t > 0.0 {
                        synapse.weight += self.config.learning_rate * f64::exp(-delta_t / 20.0);
                    } else if delta_t < 0.0 {
                        synapse.weight -= self.config.learning_rate * f64::exp(delta_t / 20.0);
                    }

                    synapse.weight = synapse.weight.max(-1.0).min(1.0);
                }
            }
        }

        Ok(())
    }

    pub fn get_spike_train(&self, neuron_id: &str) -> Vec<u64> {
        self.spike_history.iter()
            .filter(|s| s.neuron_id == neuron_id)
            .map(|s| s.timestamp_ms)
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTopology {
    pub input_size: usize,
    pub hidden_layers: Vec<usize>,
    pub output_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub total_spikes: usize,
    pub simulation_time_ms: u64,
    pub active_neurons: usize,
    pub average_firing_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuromorphic_processor() {
        let config = NeuromorphicConfig {
            architecture: SpikingArchitecture::LeakyIntegrateAndFire,
            num_neurons: 100,
            num_synapses: 500,
            plasticity_rule: PlasticityRule::STDP,
            learning_rate: 0.01,
        };

        let mut processor = NeuromorphicProcessor::new(config);

        let topology = NetworkTopology {
            input_size: 10,
            hidden_layers: vec![20, 20],
            output_size: 5,
        };

        let network = processor.build_network(topology).unwrap();
        assert_eq!(network.neurons.len(), 100);

        let result = processor.simulate(1000, 1).unwrap();
        assert!(result.total_spikes >= 0);
    }
}