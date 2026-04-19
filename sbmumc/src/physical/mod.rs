//! Physical World Interface Module
//!
//! This module implements robotics control interface, IoT device management,
//! sensor fusion engine, actuator control, and environmental interaction.
//!
//! Features:
//! - Robotics control interface
//! - IoT device management
//! - Sensor fusion engine
//! - Actuator control
//! - Environmental interaction

use crate::core::{SbmumcError, Result, EntityId, SecurityLevel};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Robot type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RobotType {
    /// Industrial robot arm
    IndustrialArm,
    /// Mobile robot
    Mobile,
    /// Humanoid robot
    Humanoid,
    /// Drone
    Drone,
    /// Medical robot
    Medical,
    /// Service robot
    Service,
    /// Educational robot
    Educational,
    /// Custom robot
    Custom,
}

/// Robot state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotState {
    /// Position
    pub position: Position3D,
    /// Orientation
    pub orientation: Orientation3D,
    /// Joint angles (for articulated robots)
    pub joint_angles: Vec<f64>,
    /// Linear velocity
    pub velocity: Velocity3D,
    /// Angular velocity
    pub angular_velocity: Orientation3D,
    /// Battery level
    pub battery_level: f64,
    /// Power consumption
    pub power_consumption: f64,
    /// Operating temperature
    pub temperature: f64,
    /// Error state
    pub error_state: Option<String>,
    /// Last update timestamp
    pub timestamp: u64,
}

impl Default for RobotState {
    fn default() -> Self {
        RobotState {
            position: Position3D::default(),
            orientation: Orientation3D::default(),
            joint_angles: Vec::new(),
            velocity: Velocity3D::default(),
            angular_velocity: Orientation3D::default(),
            battery_level: 1.0,
            power_consumption: 0.0,
            temperature: 25.0,
            error_state: None,
            timestamp: 0,
        }
    }
}

/// 3D Position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position3D {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

impl Default for Position3D {
    fn default() -> Self {
        Position3D { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Position3D {
    /// Create new position
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Position3D { x, y, z }
    }

    /// Distance to another position
    pub fn distance(&self, other: &Position3D) -> f64 {
        ((self.x - other.x).powi(2)
            + (self.y - other.y).powi(2)
            + (self.z - other.z).powi(2)).sqrt()
    }
}

/// 3D Orientation (Euler angles)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Orientation3D {
    /// Roll (X-axis rotation)
    pub roll: f64,
    /// Pitch (Y-axis rotation)
    pub pitch: f64,
    /// Yaw (Z-axis rotation)
    pub yaw: f64,
}

impl Default for Orientation3D {
    fn default() -> Self {
        Orientation3D { roll: 0.0, pitch: 0.0, yaw: 0.0 }
    }
}

impl Orientation3D {
    /// Create new orientation
    pub fn new(roll: f64, pitch: f64, yaw: f64) -> Self {
        Orientation3D { roll, pitch, yaw }
    }
}

/// 3D Velocity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Velocity3D {
    /// Linear velocity in X
    pub vx: f64,
    /// Linear velocity in Y
    pub vy: f64,
    /// Linear velocity in Z
    pub vz: f64,
}

impl Default for Velocity3D {
    fn default() -> Self {
        Velocity3D { vx: 0.0, vy: 0.0, vz: 0.0 }
    }
}

/// Robot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotConfig {
    /// Robot type
    pub robot_type: RobotType,
    /// Name
    pub name: String,
    /// Degrees of freedom
    pub degrees_of_freedom: u32,
    /// Maximum reach (for arms)
    pub max_reach: Option<f64>,
    /// Payload capacity (kg)
    pub payload_capacity: f64,
    /// Maximum speed
    pub max_speed: f64,
    /// Communication interface
    pub comm_interface: CommunicationInterface,
    /// Safety features
    pub safety_features: Vec<SafetyFeature>,
}

impl RobotConfig {
    /// Create new config
    pub fn new(name: &str, robot_type: RobotType) -> Self {
        RobotConfig {
            robot_type,
            name: name.to_string(),
            degrees_of_freedom: 6,
            max_reach: None,
            payload_capacity: 10.0,
            max_speed: 1.0,
            comm_interface: CommunicationInterface::default(),
            safety_features: Vec::new(),
        }
    }
}

/// Communication interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationInterface {
    /// Interface type
    pub interface_type: InterfaceType,
    /// Protocol
    pub protocol: String,
    /// IP address (if applicable)
    pub ip_address: Option<String>,
    /// Port
    pub port: u16,
}

impl Default for CommunicationInterface {
    fn default() -> Self {
        CommunicationInterface {
            interface_type: InterfaceType::Network,
            protocol: "TCP/IP".to_string(),
            ip_address: None,
            port: 5000,
        }
    }
}

/// Interface type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfaceType {
    /// Network (Ethernet/WiFi)
    Network,
    /// Serial
    Serial,
    /// USB
    USB,
    /// Bluetooth
    Bluetooth,
    /// CAN Bus
    CAN,
    /// EtherCAT
    EtherCAT,
    /// Modbus
    Modbus,
}

/// Safety feature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetyFeature {
    /// Collision detection
    CollisionDetection,
    /// Emergency stop
    EmergencyStop,
    /// Force limiting
    ForceLimiting,
    /// Speed limiting
    SpeedLimiting,
    /// Safe zones
    SafeZones,
    /// Human detection
    HumanDetection,
}

/// Joint control command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointCommand {
    /// Joint index
    pub joint_index: u32,
    /// Target position (radians or meters)
    pub target_position: f64,
    /// Velocity limit
    pub velocity_limit: Option<f64>,
    /// Acceleration limit
    pub acceleration_limit: Option<f64>,
    /// Execution time (seconds)
    pub execution_time: Option<f64>,
}

impl JointCommand {
    /// Create a new command
    pub fn new(joint_index: u32, target_position: f64) -> Self {
        JointCommand {
            joint_index,
            target_position,
            velocity_limit: None,
            acceleration_limit: None,
            execution_time: None,
        }
    }
}

/// Trajectory point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryPoint {
    /// Position
    pub position: Vec<f64>,
    /// Velocity
    pub velocity: Vec<f64>,
    /// Acceleration
    pub acceleration: Vec<f64>,
    /// Timestamp
    pub timestamp: f64,
}

/// Motion planning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionPlan {
    /// Trajectory
    pub trajectory: Vec<TrajectoryPoint>,
    /// Total duration
    pub duration: f64,
    /// Path length
    pub path_length: f64,
    /// Planning time
    pub planning_time_ms: f64,
    /// Success
    pub success: bool,
    /// Collision detected
    pub collision_detected: bool,
}

impl MotionPlan {
    /// Create a new plan
    pub fn new() -> Self {
        MotionPlan {
            trajectory: Vec::new(),
            duration: 0.0,
            path_length: 0.0,
            planning_time_ms: 0.0,
            success: true,
            collision_detected: false,
        }
    }
}

/// IoT device type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IoTDeviceType {
    /// Temperature sensor
    TemperatureSensor,
    /// Humidity sensor
    HumiditySensor,
    /// Motion sensor
    MotionSensor,
    /// Light sensor
    LightSensor,
    /// Pressure sensor
    PressureSensor,
    /// Smart switch
    SmartSwitch,
    /// Smart lock
    SmartLock,
    /// Smart thermostat
    SmartThermostat,
    /// Camera
    Camera,
    /// Speaker
    Speaker,
    /// Display
    Display,
    /// Actuator
    Actuator,
    /// Custom device
    Custom,
}

/// IoT device state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTDeviceState {
    /// Device identifier
    pub device_id: String,
    /// Device type
    pub device_type: IoTDeviceType,
    /// Is online
    pub online: bool,
    /// Current values
    pub values: HashMap<String, f64>,
    /// Status strings
    pub status_strings: HashMap<String, String>,
    /// Last communication
    pub last_communication: u64,
    /// Signal strength
    pub signal_strength: f64,
    /// Battery level
    pub battery_level: Option<f64>,
}

impl IoTDeviceState {
    /// Create new device state
    pub fn new(device_id: &str, device_type: IoTDeviceType) -> Self {
        IoTDeviceState {
            device_id: device_id.to_string(),
            device_type,
            online: true,
            values: HashMap::new(),
            status_strings: HashMap::new(),
            last_communication: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            signal_strength: 0.8,
            battery_level: None,
        }
    }
}

/// Sensor reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    /// Sensor ID
    pub sensor_id: String,
    /// Sensor type
    pub sensor_type: SensorType,
    /// Value
    pub value: f64,
    /// Unit
    pub unit: String,
    /// Timestamp
    pub timestamp: u64,
    /// Quality (0-1)
    pub quality: f64,
    /// Confidence
    pub confidence: f64,
}

/// Sensor type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorType {
    /// Temperature
    Temperature,
    /// Humidity
    Humidity,
    /// Pressure
    Pressure,
    /// Light
    Light,
    /// Sound
    Sound,
    /// Accelerometer
    Accelerometer,
    /// Gyroscope
    Gyroscope,
    /// Magnetometer
    Magnetometer,
    /// Proximity
    Proximity,
    /// Range
    Range,
    /// Force
    Force,
    /// Torque
    Torque,
    /// Current
    Current,
    /// Voltage
    Voltage,
    /// GPS
    GPS,
    /// IMU
    IMU,
    /// Camera
    Camera,
    /// LIDAR
    LIDAR,
    /// Radar
    Radar,
    /// Custom
    Custom,
}

impl SensorType {
    /// Get default unit
    pub fn default_unit(&self) -> &'static str {
        match self {
            SensorType::Temperature => "°C",
            SensorType::Humidity => "%",
            SensorType::Pressure => "Pa",
            SensorType::Light => "lux",
            SensorType::Sound => "dB",
            SensorType::Accelerometer => "m/s²",
            SensorType::Gyroscope => "rad/s",
            SensorType::Magnetometer => "μT",
            SensorType::Proximity => "cm",
            SensorType::Range => "m",
            SensorType::Force => "N",
            SensorType::Torque => "Nm",
            SensorType::Current => "A",
            SensorType::Voltage => "V",
            SensorType::GPS => "degrees",
            SensorType::IMU => "combined",
            SensorType::Camera => "pixels",
            SensorType::LIDAR => "m",
            SensorType::Radar => "m",
            SensorType::Custom => "",
        }
    }
}

/// Actuator command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActuatorCommand {
    /// Actuator ID
    pub actuator_id: String,
    /// Command type
    pub command_type: ActuatorCommandType,
    /// Value
    pub value: f64,
    /// Duration (if applicable)
    pub duration_ms: Option<u32>,
}

impl ActuatorCommand {
    /// Create a new command
    pub fn new(actuator_id: &str, command_type: ActuatorCommandType, value: f64) -> Self {
        ActuatorCommand {
            actuator_id: actuator_id.to_string(),
            command_type,
            value,
            duration_ms: None,
        }
    }
}

/// Actuator command type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActuatorCommandType {
    /// Position control
    Position,
    /// Velocity control
    Velocity,
    /// Force/torque control
    Force,
    /// PWM control
    PWM,
    /// On/Off
    OnOff,
    /// Set state
    SetState,
}

/// Environment perception
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentPerception {
    /// Detected objects
    pub objects: Vec<DetectedObject>,
    /// Occupancy grid
    pub occupancy_grid: Option<OccupancyGrid>,
    /// Point cloud
    pub point_cloud: Vec<Point3D>,
    /// Semantic segmentation
    pub semantic_map: HashMap<String, Vec<String>>,
    /// Safety zones
    pub safety_zones: Vec<SafetyZone>,
    /// Timestamp
    pub timestamp: u64,
}

/// Detected object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedObject {
    /// Object ID
    pub object_id: String,
    /// Class
    pub class_name: String,
    /// Position
    pub position: Position3D,
    /// Size
    pub size: (f64, f64, f64),
    /// Confidence
    pub confidence: f64,
    /// Tracking ID
    pub tracking_id: Option<u32>,
}

impl DetectedObject {
    /// Create a new detected object
    pub fn new(object_id: &str, class: &str, position: Position3D) -> Self {
        DetectedObject {
            object_id: object_id.to_string(),
            class_name: class.to_string(),
            position,
            size: (0.1, 0.1, 0.1),
            confidence: 0.8,
            tracking_id: None,
        }
    }
}

/// Point in 3D space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point3D {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
    /// Intensity (optional)
    pub intensity: Option<f64>,
}

impl Point3D {
    /// Create a new point
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z, intensity: None }
    }
}

/// Occupancy grid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccupancyGrid {
    /// Grid width (cells)
    pub width: u32,
    /// Grid height (cells)
    pub height: u32,
    /// Cell size (meters)
    pub cell_size: f64,
    /// Grid origin
    pub origin: Position3D,
    /// Grid data (row-major)
    pub data: Vec<f64>,
}

impl OccupancyGrid {
    /// Create a new grid
    pub fn new(width: u32, height: u32, cell_size: f64, origin: Position3D) -> Self {
        OccupancyGrid {
            width,
            height,
            cell_size,
            origin,
            data: vec![0.5; (width * height) as usize], // 0.5 = unknown
        }
    }

    /// Get cell value
    pub fn get_cell(&self, x: u32, y: u32) -> Option<f64> {
        if x < self.width && y < self.height {
            Some(self.data[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    /// Set cell value
    pub fn set_cell(&mut self, x: u32, y: u32, value: f64) {
        if x < self.width && y < self.height {
            self.data[(y * self.width + x) as usize] = value.clamp(0.0, 1.0);
        }
    }
}

/// Safety zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyZone {
    /// Zone ID
    pub zone_id: String,
    /// Zone type
    pub zone_type: SafetyZoneType,
    /// Center position
    pub center: Position3D,
    /// Radius or dimensions
    pub dimensions: ZoneDimensions,
    /// Priority
    pub priority: u32,
}

impl SafetyZone {
    /// Create a new zone
    pub fn new(zone_id: &str, zone_type: SafetyZoneType, center: Position3D) -> Self {
        SafetyZone {
            zone_id: zone_id.to_string(),
            zone_type,
            center,
            dimensions: ZoneDimensions::Sphere { radius: 1.0 },
            priority: 0,
        }
    }
}

/// Zone dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZoneDimensions {
    /// Spherical zone
    Sphere { radius: f64 },
    /// Cuboid zone
    Cuboid { width: f64, depth: f64, height: f64 },
    /// Cylindrical zone
    Cylinder { radius: f64, height: f64 },
}

/// Safety zone type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetyZoneType {
    /// Restricted zone
    Restricted,
    /// Slow zone
    Slow,
    /// Work zone
    Work,
    /// Human presence zone
    HumanPresence,
    /// Emergency zone
    Emergency,
}

/// Sensor fusion result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorFusionResult {
    /// Fused position estimate
    pub position: Position3D,
    /// Fused orientation estimate
    pub orientation: Orientation3D,
    /// Velocity estimate
    pub velocity: Velocity3D,
    /// Uncertainty
    pub uncertainty: f64,
    /// Data sources used
    pub sources: Vec<String>,
    /// Confidence
    pub confidence: f64,
}

impl Default for SensorFusionResult {
    fn default() -> Self {
        SensorFusionResult {
            position: Position3D::default(),
            orientation: Orientation3D::default(),
            velocity: Velocity3D::default(),
            uncertainty: 0.1,
            sources: Vec::new(),
            confidence: 0.5,
        }
    }
}

/// Physical interface system
pub struct PhysicalInterface {
    /// Connected robots
    pub robots: HashMap<String, RobotState>,
    /// Robot configurations
    pub robot_configs: HashMap<String, RobotConfig>,
    /// IoT devices
    pub iot_devices: HashMap<String, IoTDeviceState>,
    /// Sensors
    pub sensors: HashMap<String, SensorReading>,
    /// Environment perception
    pub environment: Option<EnvironmentPerception>,
    /// Safety system
    pub safety_system: SafetySystem,
    /// Sensor fusion
    pub sensor_fusion: SensorFusion,
}

impl PhysicalInterface {
    /// Create a new physical interface
    pub fn new() -> Self {
        PhysicalInterface {
            robots: HashMap::new(),
            robot_configs: HashMap::new(),
            iot_devices: HashMap::new(),
            sensors: HashMap::new(),
            environment: None,
            safety_system: SafetySystem::new(),
            sensor_fusion: SensorFusion::new(),
        }
    }

    /// Add a robot
    pub fn add_robot(&mut self, robot_id: &str, config: RobotConfig) {
        self.robots.insert(robot_id.to_string(), RobotState::default());
        self.robot_configs.insert(robot_id.to_string(), config);
    }

    /// Add an IoT device
    pub fn add_iot_device(&mut self, device: IoTDeviceState) {
        self.iot_devices.insert(device.device_id.clone(), device);
    }

    /// Update sensor reading
    pub fn update_sensor(&mut self, reading: SensorReading) {
        self.sensors.insert(reading.sensor_id.clone(), reading);
    }

    /// Command robot joints
    pub fn command_joints(&mut self, robot_id: &str, commands: Vec<JointCommand>) -> Result<()> {
        if !self.robots.contains_key(robot_id) {
            return Err(SbmumcError::NotFound(format!("Robot {} not found", robot_id)));
        }

        // In real implementation, would send commands to robot
        Ok(())
    }

    /// Plan motion for robot
    pub fn plan_motion(&self, robot_id: &str, target: &Position3D) -> Result<MotionPlan> {
        if !self.robots.contains_key(robot_id) {
            return Err(SbmumcError::NotFound(format!("Robot {} not found", robot_id)));
        }

        // Simplified motion planning
        let mut plan = MotionPlan::new();
        plan.success = true;

        Ok(plan)
    }

    /// Execute motion plan
    pub fn execute_motion(&mut self, robot_id: &str, plan: &MotionPlan) -> Result<()> {
        if !self.robots.contains_key(robot_id) {
            return Err(SbmumcError::NotFound(format!("Robot {} not found", robot_id)));
        }

        if plan.collision_detected {
            return Err(SbmumcError::InvalidState("Collision detected in plan".to_string()));
        }

        // In real implementation, would execute trajectory
        Ok(())
    }

    /// Command IoT device
    pub fn command_iot_device(&mut self, device_id: &str, command: &str, value: f64) -> Result<()> {
        if let Some(device) = self.iot_devices.get_mut(device_id) {
            device.values.insert(command.to_string(), value);
            Ok(())
        } else {
            Err(SbmumcError::NotFound(format!("Device {} not found", device_id)))
        }
    }

    /// Command actuator
    pub fn command_actuator(&mut self, actuator_id: &str, command: ActuatorCommand) -> Result<()> {
        // In real implementation, would send command to actuator
        Ok(())
    }

    /// Get robot state
    pub fn get_robot_state(&self, robot_id: &str) -> Result<&RobotState> {
        self.robots.get(robot_id)
            .ok_or_else(|| SbmumcError::NotFound(format!("Robot {} not found", robot_id)))
    }

    /// Get device state
    pub fn get_device_state(&self, device_id: &str) -> Result<&IoTDeviceState> {
        self.iot_devices.get(device_id)
            .ok_or_else(|| SbmumcError::NotFound(format!("Device {} not found", device_id)))
    }

    /// Check safety
    pub fn check_safety(&self) -> SafetyCheckResult {
        self.safety_system.check_all(self)
    }

    /// Fuse sensor data
    pub fn fuse_sensors(&mut self, sensor_ids: &[String]) -> SensorFusionResult {
        let mut readings = Vec::new();
        for id in sensor_ids {
            if let Some(reading) = self.sensors.get(id) {
                readings.push(reading);
            }
        }

        self.sensor_fusion.fuse(&readings)
    }

    /// Update environment perception
    pub fn update_environment(&mut self, perception: EnvironmentPerception) {
        self.environment = Some(perception);
    }

    /// Get all online devices
    pub fn get_online_devices(&self) -> Vec<&IoTDeviceState> {
        self.iot_devices.values()
            .filter(|d| d.online)
            .collect()
    }

    /// Emergency stop
    pub fn emergency_stop(&mut self) -> Result<()> {
        // Stop all robots
        for robot in self.robots.values_mut() {
            robot.velocity = Velocity3D::default();
            robot.angular_velocity = Orientation3D::default();
        }

        // Stop all actuators
        // In real implementation, would send stop commands

        self.safety_system.last_emergency = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());

        Ok(())
    }
}

impl Default for PhysicalInterface {
    fn default() -> Self {
        Self::new()
    }
}

/// Safety system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetySystem {
    /// Is enabled
    pub enabled: bool,
    /// Active safety zones
    pub active_zones: Vec<SafetyZone>,
    /// Last emergency stop
    pub last_emergency: Option<u64>,
    /// Safety violations
    pub violations: Vec<SafetyViolation>,
}

impl SafetySystem {
    /// Create new safety system
    pub fn new() -> Self {
        SafetySystem {
            enabled: true,
            active_zones: Vec::new(),
            last_emergency: None,
            violations: Vec::new(),
        }
    }

    /// Check all safety conditions
    pub fn check_all(&self, interface: &PhysicalInterface) -> SafetyCheckResult {
        let mut safe = true;
        let mut violations = Vec::new();

        // Check robot states
        for (id, state) in &interface.robots {
            if state.temperature > 80.0 {
                safe = false;
                violations.push(SafetyViolation {
                    violation_type: ViolationType::Overheating,
                    source_id: id.clone(),
                    description: "Temperature exceeds safe limit".to_string(),
                    severity: Severity::High,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                });
            }

            if state.battery_level < 0.1 {
                violations.push(SafetyViolation {
                    violation_type: ViolationType::LowBattery,
                    source_id: id.clone(),
                    description: "Battery critically low".to_string(),
                    severity: Severity::Medium,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                });
            }
        }

        SafetyCheckResult {
            safe,
            violations,
            recommendations: if safe { vec![] } else { vec!["Consider emergency stop".to_string()] },
        }
    }
}

impl Default for SafetySystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Safety check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyCheckResult {
    /// Is system safe
    pub safe: bool,
    /// Violations found
    pub violations: Vec<SafetyViolation>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Safety violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyViolation {
    /// Violation type
    pub violation_type: ViolationType,
    /// Source ID
    pub source_id: String,
    /// Description
    pub description: String,
    /// Severity
    pub severity: Severity,
    /// Timestamp
    pub timestamp: u64,
}

/// Violation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationType {
    /// Collision detected
    Collision,
    /// Zone violation
    ZoneViolation,
    /// Overheating
    Overheating,
    /// Speed violation
    SpeedViolation,
    /// Force violation
    ForceViolation,
    /// Low battery
    LowBattery,
    /// Communication failure
    CommunicationFailure,
    /// Unknown
    Unknown,
}

/// Severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Sensor fusion engine
pub struct SensorFusion {
    /// Fusion algorithm
    pub algorithm: FusionAlgorithm,
    /// Kalman filters
    pub kalman_filters: HashMap<String, KalmanFilter>,
}

impl SensorFusion {
    /// Create new sensor fusion
    pub fn new() -> Self {
        SensorFusion {
            algorithm: FusionAlgorithm::Kalman,
            kalman_filters: HashMap::new(),
        }
    }

    /// Fuse sensor readings
    pub fn fuse(&mut self, readings: &[&SensorReading]) -> SensorFusionResult {
        let mut result = SensorFusionResult::default();

        for reading in readings {
            result.sources.push(reading.sensor_id.clone());
            result.confidence = (result.confidence + reading.confidence) / 2.0;
        }

        result.confidence = result.sources.len() as f64 * 0.2; // Adjust based on sources
        result.uncertainty = 1.0 / result.sources.len() as f64;

        result
    }
}

impl Default for SensorFusion {
    fn default() -> Self {
        Self::new()
    }
}

/// Fusion algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FusionAlgorithm {
    /// Kalman filter
    Kalman,
    /// Extended Kalman filter
    ExtendedKalman,
    /// Unscented Kalman filter
    UnscentedKalman,
    /// Particle filter
    Particle,
    /// Weighted average
    WeightedAverage,
    /// Bayesian
    Bayesian,
}

/// Kalman filter state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KalmanFilter {
    /// State estimate
    pub state: Vec<f64>,
    /// Error covariance
    pub error_covariance: Vec<Vec<f64>>,
    /// Process noise
    pub process_noise: f64,
    /// Measurement noise
    pub measurement_noise: f64,
}

impl KalmanFilter {
    /// Create new filter
    pub fn new(state_dim: usize) -> Self {
        KalmanFilter {
            state: vec![0.0; state_dim],
            error_covariance: vec![vec![1.0; state_dim]; state_dim],
            process_noise: 0.1,
            measurement_noise: 0.5,
        }
    }

    /// Predict step
    pub fn predict(&mut self) {
        // Simplified prediction
        for val in &mut self.state {
            *val += self.process_noise;
        }
    }

    /// Update step
    pub fn update(&mut self, measurement: &[f64]) {
        // Simplified update
        let k = self.process_noise / (self.process_noise + self.measurement_noise);
        for (i, m) in measurement.iter().enumerate() {
            if i < self.state.len() {
                self.state[i] += k * (m - self.state[i]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position3d() {
        let p1 = Position3D::new(0.0, 0.0, 0.0);
        let p2 = Position3D::new(3.0, 4.0, 0.0);

        assert_eq!(p1.distance(&p2), 5.0);
    }

    #[test]
    fn test_robot_state() {
        let state = RobotState::default();
        assert_eq!(state.battery_level, 1.0);
        assert!(state.error_state.is_none());
    }

    #[test]
    fn test_iot_device() {
        let device = IoTDeviceState::new("sensor_1", IoTDeviceType::TemperatureSensor);
        assert_eq!(device.device_type, IoTDeviceType::TemperatureSensor);
        assert!(device.online);
    }

    #[test]
    fn test_occupancy_grid() {
        let mut grid = OccupancyGrid::new(10, 10, 0.1, Position3D::default());
        grid.set_cell(5, 5, 0.0);

        assert_eq!(grid.get_cell(5, 5), Some(0.0));
        assert_eq!(grid.get_cell(9, 9), Some(0.5)); // Unknown
    }

    #[test]
    fn test_sensor_reading() {
        let reading = SensorReading {
            sensor_id: "temp_1".to_string(),
            sensor_type: SensorType::Temperature,
            value: 25.0,
            unit: SensorType::Temperature.default_unit().to_string(),
            timestamp: 0,
            quality: 1.0,
            confidence: 0.9,
        };

        assert_eq!(reading.unit, "°C");
    }

    #[test]
    fn test_physical_interface() {
        let mut interface = PhysicalInterface::new();

        let config = RobotConfig::new("robot1", RobotType::IndustrialArm);
        interface.add_robot("robot1", config);

        assert!(interface.robots.contains_key("robot1"));
    }

    #[test]
    fn test_safety_system() {
        let system = SafetySystem::new();
        assert!(system.enabled);
    }

    #[test]
    fn test_sensor_fusion() {
        let mut fusion = SensorFusion::new();
        let reading1 = SensorReading {
            sensor_id: "s1".to_string(),
            sensor_type: SensorType::Range,
            value: 1.0,
            unit: "m".to_string(),
            timestamp: 0,
            quality: 0.9,
            confidence: 0.8,
        };

        let result = fusion.fuse(&[&reading1]);
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_kalman_filter() {
        let mut filter = KalmanFilter::new(3);
        filter.predict();
        filter.update(&[1.0, 2.0, 3.0]);

        assert_eq!(filter.state.len(), 3);
    }

    #[test]
    fn test_joint_command() {
        let cmd = JointCommand::new(0, 1.57);
        assert_eq!(cmd.joint_index, 0);
        assert_eq!(cmd.target_position, 1.57);
    }

    #[test]
    fn test_actuator_command() {
        let cmd = ActuatorCommand::new("act1", ActuatorCommandType::Position, 90.0);
        assert_eq!(cmd.command_type, ActuatorCommandType::Position);
    }

    #[test]
    fn test_detected_object() {
        let obj = DetectedObject::new("obj1", "person", Position3D::default());
        assert_eq!(obj.class_name, "person");
    }
}
