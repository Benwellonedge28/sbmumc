//! # SBMUMC Module 1606: Autonomous Vehicles
//!
//! Self-driving systems and autonomous navigation.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleConfig {
    pub vehicle_type: VehicleType,
    pub sensor_suite: Vec<SensorType>,
    pub autonomy_level: AutonomyLevel,
    pub max_speed_kmh: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VehicleType {
    Car,
    Truck,
    Bus,
    Drone,
    Boat,
    Robot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensorType {
    Camera,
    Lidar,
    Radar,
    Ultrasonic,
    IMU,
    GPS,
    V2X,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutonomyLevel {
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleState {
    pub vehicle_id: String,
    pub position: Position,
    pub velocity: Velocity,
    pub orientation: Orientation,
    pub sensors: HashMap<String, SensorReading>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Velocity {
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub speed_kmh: f64,
    pub heading: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Orientation {
    pub pitch: f64,
    pub roll: f64,
    pub yaw: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub sensor_id: String,
    pub sensor_type: SensorType,
    pub timestamp_ms: u64,
    pub data: Vec<f64>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionResult {
    pub obstacles: Vec<Obstacle>,
    pub lane_markings: Vec<LaneMarking>,
    pub traffic_signs: Vec<TrafficSign>,
    pub other_vehicles: Vec<Vehicle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub obstacle_id: String,
    pub position: Position,
    pub size: Vec<f64>,
    pub obstacle_type: ObstacleType,
    pub velocity: Velocity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObstacleType {
    Pedestrian,
    Cyclist,
    Vehicle,
    Static,
    Animal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaneMarking {
    pub marking_id: String,
    pub points: Vec<(f64, f64)>,
    pub marking_type: MarkingType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarkingType {
    Solid,
    Dashed,
    Double,
    Edge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficSign {
    pub sign_id: String,
    pub sign_type: TrafficSignType,
    pub position: Position,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrafficSignType {
    SpeedLimit,
    Stop,
    Yield,
    TrafficLight,
    NoEntry,
    Warning,
}

pub struct AutonomousVehicle {
    config: VehicleConfig,
    state: Option<VehicleState>,
    map: Option<HDMap>,
    planner: Option<RoutePlanner>,
    controller: VehicleController,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HDMap {
    pub map_id: String,
    pub lanes: Vec<Lane>,
    pub intersections: Vec<Intersection>,
    pub landmarks: Vec<Landmark>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lane {
    pub lane_id: String,
    pub center_line: Vec<(f64, f64)>,
    pub width_m: f64,
    pub allowed_direction: Direction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    Forward,
    Backward,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intersection {
    pub intersection_id: String,
    pub position: (f64, f64),
    pub connections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Landmark {
    pub landmark_id: String,
    pub landmark_type: String,
    pub position: (f64, f64),
}

pub struct RoutePlanner {
    pub route: Vec<(f64, f64)>,
    pub estimated_time_s: u64,
    pub distance_m: f64,
}

pub struct VehicleController {
    pub throttle: f64,
    pub brake: f64,
    pub steering: f64,
}

impl AutonomousVehicle {
    pub fn new(config: VehicleConfig) -> Self {
        Self {
            config,
            state: None,
            map: None,
            planner: None,
            controller: VehicleController {
                throttle: 0.0,
                brake: 0.0,
                steering: 0.0,
            },
        }
    }

    pub fn initialize(&mut self, start_position: Position) -> Result<()> {
        self.state = Some(VehicleState {
            vehicle_id: uuid::Uuid::new_v4().to_string(),
            position: start_position.clone(),
            velocity: Velocity {
                vx: 0.0,
                vy: 0.0,
                vz: 0.0,
                speed_kmh: 0.0,
                heading: 0.0,
            },
            orientation: Orientation {
                pitch: 0.0,
                roll: 0.0,
                yaw: 0.0,
            },
            sensors: HashMap::new(),
        });

        self.map = Some(HDMap {
            map_id: uuid::Uuid::new_v4().to_string(),
            lanes: vec![],
            intersections: vec![],
            landmarks: vec![],
        });

        Ok(())
    }

    pub fn perceive(&self) -> Result<PerceptionResult> {
        let state = self.state.as_ref()
            .ok_or_else(|| SbmumcError::Internal("Vehicle not initialized".into()))?;

        let obstacles = vec![
            Obstacle {
                obstacle_id: uuid::Uuid::new_v4().to_string(),
                position: Position {
                    x: state.position.x + 10.0,
                    y: state.position.y,
                    z: 0.0,
                    latitude: state.position.latitude,
                    longitude: state.position.longitude,
                },
                size: vec![2.0, 1.5, 1.5],
                obstacle_type: ObstacleType::Vehicle,
                velocity: Velocity {
                    vx: 0.0,
                    vy: -5.0,
                    vz: 0.0,
                    speed_kmh: 5.0,
                    heading: 0.0,
                },
            },
        ];

        Ok(PerceptionResult {
            obstacles,
            lane_markings: vec![],
            traffic_signs: vec![],
            other_vehicles: vec![],
        })
    }

    pub fn plan(&mut self, destination: Position) -> Result<RoutePlanner> {
        let state = self.state.as_ref()
            .ok_or_else(|| SbmumcError::Internal("Vehicle not initialized".into()))?;

        let route = vec![
            (state.position.x, state.position.y),
            (destination.x, state.position.y),
            (destination.x, destination.y),
        ];

        let planner = RoutePlanner {
            route,
            estimated_time_s: 300,
            distance_m: ((destination.x - state.position.x).powi(2) +
                       (destination.y - state.position.y).powi(2)).sqrt(),
        };

        self.planner = Some(planner.clone());
        Ok(planner)
    }

    pub fn control(&mut self, perception: &PerceptionResult) -> Result<VehicleController> {
        let mut controller = VehicleController {
            throttle: 0.5,
            brake: 0.0,
            steering: 0.0,
        };

        if !perception.obstacles.is_empty() {
            let closest = &perception.obstacles[0];
            if closest.position.x < 20.0 {
                controller.throttle = 0.0;
                controller.brake = 0.8;
            }
        }

        self.controller = controller.clone();
        Ok(controller)
    }

    pub fn update(&mut self, dt_ms: u64) -> Result<()> {
        let state = self.state.as_mut()
            .ok_or_else(|| SbmumcError::Internal("Vehicle not initialized".into()))?;

        state.velocity.vx += self.controller.throttle * 0.1;
        state.velocity.vy += self.controller.steering * 0.05;

        state.position.x += state.velocity.vx * dt_ms as f64 / 1000.0;
        state.position.y += state.velocity.vy * dt_ms as f64 / 1000.0;

        state.velocity.speed_kmh = (state.velocity.vx.powi(2) + state.velocity.vy.powi(2)).sqrt() * 3.6;

        Ok(())
    }

    pub fn get_state(&self) -> Option<&VehicleState> {
        self.state.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autonomous_vehicle() {
        let config = VehicleConfig {
            vehicle_type: VehicleType::Car,
            sensor_suite: vec![SensorType::Camera, SensorType::Lidar, SensorType::Radar],
            autonomy_level: AutonomyLevel::Level4,
            max_speed_kmh: 120.0,
        };

        let mut vehicle = AutonomousVehicle::new(config);

        let start = Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            latitude: 37.7749,
            longitude: -122.4194,
        };

        vehicle.initialize(start).unwrap();

        let perception = vehicle.perceive().unwrap();
        assert!(!perception.obstacles.is_empty());

        let dest = Position {
            x: 100.0,
            y: 100.0,
            z: 0.0,
            latitude: 37.7750,
            longitude: -122.4193,
        };

        let planner = vehicle.plan(dest).unwrap();
        assert!(!planner.route.is_empty());
    }
}