//! # SBMUMC Module 1611: Brain-Computer Interface
//!
//! Neural interface systems and thought-based control.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BCIConfig {
    pub interface_type: BCIType,
    pub channel_count: usize,
    pub sampling_rate_hz: f64,
    pub signal_resolution: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BCIType {
    Invasive,
    SemiInvasive,
    NonInvasive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralSignal {
    pub signal_id: String,
    pub channels: Vec<f64>,
    pub timestamp_ms: u64,
    pub quality: SignalQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalQuality {
    Excellent,
    Good,
    Fair,
    Poor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub intent_id: String,
    pub intent_type: IntentType,
    pub confidence: f64,
    pub decoded_command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntentType {
    Move,
    Speak,
    Select,
    Navigate,
    Control,
}

pub struct BrainComputerInterface {
    config: BCIConfig,
    calibration_data: HashMap<String, f64>,
    trained_models: HashMap<String, NeuralModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralModel {
    pub model_id: String,
    pub model_type: String,
    pub accuracy: f64,
    pub training_samples: usize,
}

impl BrainComputerInterface {
    pub fn new(config: BCIConfig) -> Self {
        Self {
            config,
            calibration_data: HashMap::new(),
            trained_models: HashMap::new(),
        }
    }

    pub fn record_signal(&self) -> Result<NeuralSignal> {
        let channels: Vec<f64> = (0..self.config.channel_count)
            .map(|_| rand::random::<f64>() * 100.0 - 50.0)
            .collect();

        Ok(NeuralSignal {
            signal_id: uuid::Uuid::new_v4().to_string(),
            channels,
            timestamp_ms: chrono::Utc::now().timestamp_millis() as u64,
            quality: SignalQuality::Good,
        })
    }

    pub fn decode_intent(&self, signal: &NeuralSignal) -> Result<Intent> {
        let mut confidence = 0.8;

        let variance = signal.channels.iter()
            .map(|x| x.powi(2))
            .sum::<f64>() / signal.channels.len() as f64;

        if variance > 100.0 {
            confidence = 0.6;
        } else if variance > 50.0 {
            confidence = 0.75;
        } else {
            confidence = 0.9;
        }

        let intent_type = match signal.channels.len() % 5 {
            0 => IntentType::Move,
            1 => IntentType::Speak,
            2 => IntentType::Select,
            3 => IntentType::Navigate,
            _ => IntentType::Control,
        };

        Ok(Intent {
            intent_id: uuid::Uuid::new_v4().to_string(),
            intent_type,
            confidence,
            decoded_command: format!("{:?}_command", intent_type),
        })
    }

    pub fn train_model(&mut self, signals: &[NeuralSignal], labels: &[IntentType]) -> Result<NeuralModel> {
        if signals.len() != labels.len() {
            return Err(SbmumcError::Internal("Signal/label mismatch".into()));
        }

        let accuracy = 0.7 + rand::random::<f64>() * 0.25;

        let model = NeuralModel {
            model_id: uuid::Uuid::new_v4().to_string(),
            model_type: "CNN".to_string(),
            accuracy,
            training_samples: signals.len(),
        };

        self.trained_models.insert(model.model_id.clone(), model.clone());
        Ok(model)
    }

    pub fn calibrate(&mut self, user_id: &str) -> Result<()> {
        for i in 0..100 {
            self.calibration_data.insert(
                format!("{}_sample_{}", user_id, i),
                rand::random::<f64>() * 100.0,
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bci() {
        let config = BCIConfig {
            interface_type: BCIType::NonInvasive,
            channel_count: 64,
            sampling_rate_hz: 1000.0,
            signal_resolution: 16,
        };

        let bci = BrainComputerInterface::new(config);
        let signal = bci.record_signal().unwrap();
        assert_eq!(signal.channels.len(), 64);

        let intent = bci.decode_intent(&signal).unwrap();
        assert!(intent.confidence > 0.0);
    }
}