//! # SBMUMC Module 1609: Ambient Intelligence
//!
//! Context-aware ubiquitous computing systems.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbientConfig {
    pub sensing_density: f64,
    pub context_types: Vec<ContextType>,
    pub prediction_horizon_min: u64,
    pub personalization_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextType {
    Location,
    Activity,
    Mood,
    Social,
    Environmental,
    Temporal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextState {
    pub state_id: String,
    pub timestamp: i64,
    pub location: LocationContext,
    pub activity: ActivityContext,
    pub social: SocialContext,
    pub environmental: EnvironmentalContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationContext {
    pub location_type: LocationType,
    pub confidence: f64,
    pub zone: String,
    pub nearby_devices: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationType {
    Home,
    Office,
    Public,
    Vehicle,
    Outdoor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityContext {
    pub activity_type: ActivityType,
    pub confidence: f64,
    pub duration_min: u64,
    pub interruptions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    Working,
    Resting,
    Eating,
    Exercising,
    Commuting,
    Socializing,
    Sleeping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialContext {
    pub nearby_people: Vec<String>,
    pub conversation_active: bool,
    pub noise_level: NoiseLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NoiseLevel {
    Silent,
    Quiet,
    Moderate,
    Loud,
    VeryLoud,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalContext {
    pub temperature_c: f64,
    pub humidity_pct: f64,
    pub light_lux: f64,
    pub air_quality: AirQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AirQuality {
    Excellent,
    Good,
    Moderate,
    Poor,
    Hazardous,
}

pub struct AmbientIntelligence {
    config: AmbientConfig,
    sensor_network: SensorNetwork,
    context_history: Vec<ContextState>,
    predictions: HashMap<String, Prediction>,
    adaptors: Vec<Adaptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorNetwork {
    pub network_id: String,
    pub sensors: Vec<Sensor>,
    pub mesh_topology: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sensor {
    pub sensor_id: String,
    pub sensor_type: SensorType,
    pub location: (f64, f64),
    pub battery_pct: f64,
    pub data_rate_hz: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensorType {
    Motion,
    Temperature,
    Light,
    Sound,
    Proximity,
    Accelerometer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub prediction_id: String,
    pub prediction_type: PredictionType,
    pub confidence: f64,
    pub predicted_value: String,
    pub horizon_min: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PredictionType {
    NextActivity,
    Location,
    Social,
    Health,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adaptor {
    pub adaptor_id: String,
    pub adaptor_type: AdaptorType,
    pub target_action: String,
    pub trigger_conditions: Vec<TriggerCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdaptorType {
    Lighting,
    Climate,
    Entertainment,
    Security,
    Energy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerCondition {
    pub context_type: ContextType,
    pub operator: ConditionOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    GreaterThan,
    LessThan,
    Contains,
    Before,
    After,
}

impl AmbientIntelligence {
    pub fn new(config: AmbientConfig) -> Self {
        Self {
            config,
            sensor_network: SensorNetwork {
                network_id: uuid::Uuid::new_v4().to_string(),
                sensors: vec![],
                mesh_topology: true,
            },
            context_history: Vec::new(),
            predictions: HashMap::new(),
            adaptors: vec![],
        }
    }

    pub fn add_sensor(&mut self, sensor: Sensor) -> Result<()> {
        self.sensor_network.sensors.push(sensor);
        Ok(())
    }

    pub fn sense_environment(&self) -> Result<EnvironmentalContext> {
        Ok(EnvironmentalContext {
            temperature_c: 22.0 + rand::random::<f64>() * 5.0,
            humidity_pct: 40.0 + rand::random::<f64>() * 30.0,
            light_lux: 300.0 + rand::random::<f64>() * 500.0,
            air_quality: AirQuality::Good,
        })
    }

    pub fn infer_context(&self) -> Result<ContextState> {
        let environmental = self.sense_environment()?;

        let state = ContextState {
            state_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            location: LocationContext {
                location_type: LocationType::Office,
                confidence: 0.85,
                zone: "zone_a".to_string(),
                nearby_devices: vec![],
            },
            activity: ActivityContext {
                activity_type: ActivityType::Working,
                confidence: 0.75,
                duration_min: 45,
                interruptions: 2,
            },
            social: SocialContext {
                nearby_people: vec![],
                conversation_active: false,
                noise_level: NoiseLevel::Quiet,
            },
            environmental,
        };

        Ok(state)
    }

    pub fn update_context(&mut self, context: ContextState) -> Result<()> {
        self.context_history.push(context);

        if self.context_history.len() > 1000 {
            self.context_history.remove(0);
        }

        Ok(())
    }

    pub fn predict_next_activity(&self, user_id: &str) -> Result<Prediction> {
        let last_activities: Vec<_> = self.context_history.iter()
            .rev()
            .take(10)
            .collect();

        let predicted = if last_activities.len() >= 3 {
            let work_count = last_activities.iter()
                .filter(|c| c.activity.activity_type == ActivityType::Working)
                .count();

            if work_count > 5 {
                ActivityType::Resting
            } else {
                ActivityType::Working
            }
        } else {
            ActivityType::Working
        };

        Ok(Prediction {
            prediction_id: uuid::Uuid::new_v4().to_string(),
            prediction_type: PredictionType::NextActivity,
            confidence: 0.7,
            predicted_value: format!("{:?}", predicted),
            horizon_min: 15,
        })
    }

    pub fn create_adaptor(&mut self, adaptor: Adaptor) -> Result<()> {
        self.adaptors.push(adaptor);
        Ok(())
    }

    pub fn execute_adaptors(&mut self, context: &ContextState) -> Result<Vec<AdaptorAction>> {
        let mut actions = Vec::new();

        for adaptor in &self.adaptors {
            if self.evaluate_triggers(&adaptor.trigger_conditions, context)? {
                let action = AdaptorAction {
                    action_id: uuid::Uuid::new_v4().to_string(),
                    adaptor_id: adaptor.adaptor_id.clone(),
                    action_type: adaptor.target_action.clone(),
                    executed_at: chrono::Utc::now().timestamp(),
                };
                actions.push(action);
            }
        }

        Ok(actions)
    }

    fn evaluate_triggers(&self, conditions: &[TriggerCondition], context: &ContextState) -> Result<bool> {
        for condition in conditions {
            match condition.context_type {
                ContextType::Location => {
                    let matches = condition.value == format!("{:?}", context.location.location_type);
                    if !matches { return Ok(false); }
                }
                ContextType::Activity => {
                    let matches = condition.value == format!("{:?}", context.activity.activity_type);
                    if !matches { return Ok(false); }
                }
                ContextType::Environmental => {
                    let threshold = condition.value.parse::<f64>().unwrap_or(20.0);
                    if context.environmental.temperature_c > threshold {
                        if condition.operator != ConditionOperator::GreaterThan {
                            return Ok(false);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(true)
    }

    pub fn learn_preferences(&mut self, user_id: &str) -> Result<PreferenceModel> {
        let recent_contexts: Vec<_> = self.context_history.iter()
            .filter(|c| c.timestamp > chrono::Utc::now().timestamp() - 86400)
            .collect();

        let temp_preference = recent_contexts.iter()
            .map(|c| c.environmental.temperature_c)
            .sum::<f64>() / recent_contexts.len().max(1) as f64;

        Ok(PreferenceModel {
            model_id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            temperature_pref_c: temp_preference,
            light_level_preferred: 400.0,
            noise_tolerance: NoiseLevel::Quiet,
        })
    }

    pub fn get_context_history(&self) -> &[ContextState] {
        &self.context_history
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptorAction {
    pub action_id: String,
    pub adaptor_id: String,
    pub action_type: String,
    pub executed_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferenceModel {
    pub model_id: String,
    pub user_id: String,
    pub temperature_pref_c: f64,
    pub light_level_preferred: f64,
    pub noise_tolerance: NoiseLevel,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ambient_intelligence() {
        let config = AmbientConfig {
            sensing_density: 0.8,
            context_types: vec![ContextType::Location, ContextType::Activity],
            prediction_horizon_min: 30,
            personalization_level: 0.7,
        };

        let mut ambient = AmbientIntelligence::new(config);

        for i in 0..5 {
            let sensor = Sensor {
                sensor_id: format!("sensor_{}", i),
                sensor_type: SensorType::Motion,
                location: (rand::random::<f64>() * 100.0, rand::random::<f64>() * 100.0),
                battery_pct: 80.0,
                data_rate_hz: 10.0,
            };
            ambient.add_sensor(sensor).unwrap();
        }

        let context = ambient.infer_context().unwrap();
        ambient.update_context(context).unwrap();

        let adaptor = Adaptor {
            adaptor_id: "adaptor_1".to_string(),
            adaptor_type: AdaptorType::Lighting,
            target_action: "adjust_lights".to_string(),
            trigger_conditions: vec![
                TriggerCondition {
                    context_type: ContextType::Location,
                    operator: ConditionOperator::Equals,
                    value: "Office".to_string(),
                }
            ],
        };

        ambient.create_adaptor(adaptor).unwrap();

        let predictions = ambient.predict_next_activity("user_1").unwrap();
        assert!(predictions.confidence > 0.0);
    }
}