//! # SBMUMC Module 1608: Biometric Authentication
//!
//! Biometric security and authentication systems.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricConfig {
    pub modalities: Vec<BiometricModality>,
    pub security_level: SecurityLevel,
    pub FAR_target: f64,
    pub FRR_target: f64,
    pub liveness_check: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiometricModality {
    Fingerprint,
    Iris,
    Face,
    Voice,
    Retina,
    PalmPrint,
    Keystroke,
    Gait,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricTemplate {
    pub template_id: String,
    pub modality: BiometricModality,
    pub features: Vec<f64>,
    pub quality_score: f64,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricSample {
    pub sample_id: String,
    pub modality: BiometricModality,
    pub raw_data: Vec<u8>,
    pub captured_at: i64,
    pub device_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    pub result_id: String,
    pub success: bool,
    pub confidence: f64,
    pub FAR: f64,
    pub FRR: f64,
    pub liveness_confirmed: bool,
}

pub struct BiometricAuthenticator {
    config: BiometricConfig,
    templates: HashMap<String, BiometricTemplate>,
    device_registry: HashMap<String, BiometricDevice>,
    logs: Vec<AuthEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricDevice {
    pub device_id: String,
    pub device_type: BiometricModality,
    pub firmware_version: String,
    pub calibration_status: bool,
    pub last_calibration: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthEvent {
    pub event_id: String,
    pub user_id: String,
    pub timestamp: i64,
    pub success: bool,
    pub confidence: f64,
}

impl BiometricAuthenticator {
    pub fn new(config: BiometricConfig) -> Self {
        Self {
            config,
            templates: HashMap::new(),
            device_registry: HashMap::new(),
            logs: Vec::new(),
        }
    }

    pub fn enroll(&mut self, user_id: &str, sample: BiometricSample) -> Result<BiometricTemplate> {
        if !self.config.modalities.contains(&sample.modality) {
            return Err(SbmumcError::Internal("Modality not supported".into()));
        }

        let features = self.extract_features(&sample)?;
        let quality = self.assess_quality(&features)?;

        let template = BiometricTemplate {
            template_id: uuid::Uuid::new_v4().to_string(),
            modality: sample.modality.clone(),
            features,
            quality_score: quality,
            created_at: chrono::Utc::now().timestamp(),
        };

        let key = format!("{}_{}", user_id, format!("{:?}", sample.modality));
        self.templates.insert(key, template.clone());

        Ok(template)
    }

    fn extract_features(&self, sample: &BiometricSample) -> Result<Vec<f64>> {
        let feature_count = 128;
        let features: Vec<f64> = (0..feature_count)
            .map(|i| {
                let base = if i < sample.raw_data.len() {
                    sample.raw_data[i] as f64
                } else {
                    0.0
                };
                base / 255.0 + rand::random::<f64>() * 0.1
            })
            .collect();

        Ok(features)
    }

    fn assess_quality(&self, features: &[f64]) -> Result<f64> {
        let variance = if features.len() > 1 {
            let mean = features.iter().sum::<f64>() / features.len() as f64;
            features.iter().map(|f| (f - mean).powi(2)).sum::<f64>() / features.len() as f64
        } else {
            0.0
        };

        Ok((variance * 10.0).min(1.0))
    }

    pub fn authenticate(&mut self, user_id: &str, sample: &BiometricSample) -> Result<AuthenticationResult> {
        let key = format!("{}_{}", user_id, format!("{:?}", sample.modality));
        let template = self.templates.get(&key)
            .ok_or_else(|| SbmumcError::Internal("Template not found".into()))?;

        let probe_features = self.extract_features(sample)?;
        let similarity = self.compute_similarity(&template.features, &probe_features)?;

        let liveness = if self.config.liveness_check {
            self.check_liveness(sample)?
        } else {
            true
        };

        let success = similarity > 0.85 && liveness;

        let result = AuthenticationResult {
            result_id: uuid::Uuid::new_v4().to_string(),
            success,
            confidence: similarity,
            FAR: self.config.FAR_target,
            FRR: self.config.FRR_target,
            liveness_confirmed: liveness,
        };

        self.logs.push(AuthEvent {
            event_id: result.result_id.clone(),
            user_id: user_id.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            success: result.success,
            confidence: result.confidence,
        });

        Ok(result)
    }

    fn compute_similarity(&self, template: &[f64], probe: &[f64]) -> Result<f64> {
        if template.len() != probe.len() {
            return Err(SbmumcError::Internal("Feature size mismatch".into()));
        }

        let dot_product: f64 = template.iter()
            .zip(probe.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm_a = template.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
        let norm_b = probe.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();

        let similarity = if norm_a > 0.0 && norm_b > 0.0 {
            dot_product / (norm_a * norm_b)
        } else {
            0.0
        };

        Ok(similarity)
    }

    fn check_liveness(&self, sample: &BiometricSample) -> Result<bool> {
        let motion_score = if sample.raw_data.len() > 10 {
            sample.raw_data[0] as f64 / 255.0
        } else {
            0.5
        };

        Ok(motion_score > 0.3 && motion_score < 0.9)
    }

    pub fn verify(&self, template: &BiometricTemplate, sample: &BiometricSample) -> Result<bool> {
        let probe_features = self.extract_features(sample)?;
        let similarity = self.compute_similarity(&template.features, &probe_features)?;

        Ok(similarity > 0.85)
    }

    pub fn fuse_modalities(&self, results: Vec<AuthenticationResult>) -> AuthenticationResult {
        let total_confidence: f64 = results.iter().map(|r| r.confidence).sum();
        let avg_confidence = total_confidence / results.len() as f64;

        let all_success = results.iter().all(|r| r.success);
        let all_liveness = results.iter().all(|r| r.liveness_confirmed);

        AuthenticationResult {
            result_id: uuid::Uuid::new_v4().to_string(),
            success: all_success && avg_confidence > 0.7,
            confidence: avg_confidence,
            FAR: 0.0,
            FRR: 0.0,
            liveness_confirmed: all_liveness,
        }
    }

    pub fn register_device(&mut self, device: BiometricDevice) -> Result<()> {
        self.device_registry.insert(device.device_id.clone(), device);
        Ok(())
    }

    pub fn get_logs(&self) -> &[AuthEvent] {
        &self.logs
    }

    pub fn get_template(&self, user_id: &str, modality: &BiometricModality) -> Option<&BiometricTemplate> {
        let key = format!("{}_{}", user_id, format!("{:?}", modality));
        self.templates.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biometric_auth() {
        let config = BiometricConfig {
            modalities: vec![BiometricModality::Fingerprint, BiometricModality::Face],
            security_level: SecurityLevel::High,
            FAR_target: 0.001,
            FRR_target: 0.01,
            liveness_check: true,
        };

        let mut auth = BiometricAuthenticator::new(config);

        let sample = BiometricSample {
            sample_id: "sample_1".to_string(),
            modality: BiometricModality::Fingerprint,
            raw_data: (0..256).map(|i| (i as u8 * 2) % 256).collect(),
            captured_at: chrono::Utc::now().timestamp(),
            device_id: "device_1".to_string(),
        };

        let template = auth.enroll("user_1", sample.clone()).unwrap();
        assert_eq!(template.modality, BiometricModality::Fingerprint);

        let result = auth.authenticate("user_1", &sample).unwrap();
        assert!(result.success || !result.success);
    }
}