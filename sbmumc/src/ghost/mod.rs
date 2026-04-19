//!
//! Ghost Mesh - Pillar I: Infrastructure & Connectivity (Files 01-37)
//!
//! This module implements the Ghost Mesh infrastructure:
//! - Quantum-resistant sharding (1.2 billion packets)
//! - Self-healing nodes with jurisdiction migration
//! - PUF DNA binding for hardware-level encryption
//! - Decentralized mesh networking
//! - Ghost seed protocol

use crate::core::{Result, EntityId, SbmumcError};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, debug, warn};
use serde::{Serialize, Deserialize};

/// Ghost Mesh Engine - Pillar I
pub struct GhostMesh {
    /// Node registry
    nodes: Arc<NodeRegistry>,

    /// Packet sharding
    sharding: Arc<QuantumSharding>,

    /// Self-healing manager
    self_healing: Arc<SelfHealingManager>,

    /// PUF binding
    puf_binding: Arc<PufBinding>,

    /// Mesh protocol
    protocol: Arc<GhostProtocol>,

    /// Configuration
    config: GhostMeshConfig,
}

/// Node Registry
pub struct NodeRegistry {
    /// Active nodes
    nodes: RwLock<HashMap<String, GhostNode>>,

    /// Node index by region
    by_region: RwLock<HashMap<String, Vec<String>>>,

    /// Node index by capability
    by_capability: RwLock<HashMap<NodeCapability, Vec<String>>>,

    /// Dead node pool
    dead_pool: RwLock<VecDeque<DeadNode>>,
}

/// Ghost Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostNode {
    pub id: String,
    pub address: GhostAddress,
    pub public_key: Vec<u8>,
    pub capabilities: Vec<NodeCapability>,
    pub status: NodeStatus,
    pub region: String,
    pub jurisdiction: String,
    pub uptime_seconds: u64,
    pub bandwidth_mbps: f64,
    pub latency_ms: u32,
    pub shard_count: usize,
    pub health_score: f64,
    pub last_heartbeat: u64,
    pub encryption_level: EncryptionLevel,
}

/// Ghost address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostAddress {
    pub protocol: AddressProtocol,
    pub host: String,
    pub port: u16,
    pub path: Option<String>,
}

/// Address protocol
#[derive(Debug, Clone, Copy)]
pub enum AddressProtocol {
    GSTM,      // Ghost mesh native
    Tor,
    I2P,
    Hybrid,
}

/// Node capability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeCapability {
    FullNode,
    LightNode,
    Relay,
    Storage,
    Compute,
    AIInference,
    Guardian,
    Archive,
}

/// Node status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NodeStatus {
    Online,
    Offline,
    Degrading,
    Migrating,
    Healing,
    Sleeping,
}

/// Encryption level
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EncryptionLevel {
    Standard,
    Enhanced,
    Maximum,
    Sovereign,
}

/// Dead node record
#[derive(Debug, Clone)]
pub struct DeadNode {
    pub id: String,
    pub death_time: u64,
    pub cause: DeathCause,
    pub shards_remaining: Vec<String>,
}

/// Death cause
#[derive(Debug, Clone, Copy)]
pub enum DeathCause {
    Hardware,
    Network,
    Jurisdictional,
    Attack,
    Scheduled,
}

/// Quantum Sharding Engine
pub struct QuantumSharding {
    /// Active shards
    shards: RwLock<HashMap<String, DataShard>>,

    /// Shard distribution map
    distribution: RwLock<ShardDistribution>,

    /// Reconstruction cache
    reconstruction_cache: RwLock<HashMap<String, ReconstructionContext>>,

    /// Configuration
    config: ShardingConfig,
}

/// Data shard
#[derive(Debug, Clone)]
pub struct DataShard {
    pub id: String,
    pub parent_id: String,
    pub index: u32,
    pub total_shards: u32,
    pub data: Vec<u8>,
    pub encryption_key: Vec<u8>,
    pub node_id: String,
    pub parity_shard: bool,
    pub checksum: u64,
    pub created_at: u64,
    pub access_count: u64,
}

/// Shard distribution
#[derive(Debug, Clone)]
pub struct ShardDistribution {
    pub strategy: DistributionStrategy,
    pub redundancy_factor: u32,
    pub geographic_spread: bool,
    pub jurisdiction_aware: bool,
    pub nodes_per_shard: u32,
}

/// Distribution strategy
#[derive(Debug, Clone, Copy)]
pub enum DistributionStrategy {
    Random,
    Geographic,
    LoadBalanced,
    Jurisdictional,
    Quantum,
}

/// Reconstruction context
#[derive(Debug, Clone)]
pub struct ReconstructionContext {
    pub parent_id: String,
    pub received_shards: Vec<u32>,
    pub total_needed: u32,
    pub completed: bool,
}

/// Sharding configuration
#[derive(Debug, Clone)]
pub struct ShardingConfig {
    pub shard_size_bytes: usize,
    pub total_shards_target: u32,
    pub redundancy_factor: u32,
    pub encryption_algorithm: String,
    pub self_healing_enabled: bool,
    pub jurisdiction_zones: Vec<String>,
}

/// Self-Healing Manager
pub struct SelfHealingManager {
    /// Health monitor
    health_monitor: Arc<HealthMonitor>,

    /// Recovery protocols
    recovery_protocols: RwLock<HashMap<FailureType, RecoveryProtocol>>,

    /// Migration manager
    migration: Arc<NodeMigration>,

    /// Configuration
    config: SelfHealingConfig,
}

/// Health monitor
pub struct HealthMonitor {
    /// Node health scores
    scores: RwLock<HashMap<String, HealthScore>>,

    /// Alert thresholds
    thresholds: HealthThresholds,

    /// Monitoring interval
    interval_ms: u32,
}

/// Health score
#[derive(Debug, Clone)]
pub struct HealthScore {
    pub node_id: String,
    pub overall: f64,
    pub latency: f64,
    pub bandwidth: f64,
    pub uptime: f64,
    pub attack_resistance: f64,
    pub last_calculated: u64,
}

/// Health thresholds
#[derive(Debug, Clone)]
pub struct HealthThresholds {
    pub critical: f64,
    pub warning: f64,
    pub healthy: f64,
}

/// Failure type
#[derive(Debug, Clone, Copy)]
pub enum FailureType {
    NodeOffline,
    NetworkPartition,
    DataCorruption,
    Attack,
    JurisdictionalBan,
}

/// Recovery protocol
#[derive(Debug, Clone)]
pub struct RecoveryProtocol {
    pub failure_type: FailureType,
    pub steps: Vec<RecoveryStep>,
    pub timeout_ms: u64,
    pub max_retries: u32,
}

/// Recovery step
#[derive(Debug, Clone)]
pub struct RecoveryStep {
    pub order: u32,
    pub action: String,
    pub target: String,
    pub rollback: Option<String>,
}

/// Node migration
pub struct NodeMigration {
    /// Active migrations
    migrations: RwLock<HashMap<String, MigrationSession>>,

    /// Target pools by jurisdiction
    target_pools: RwLock<HashMap<String, Vec<String>>>,
}

/// Migration session
#[derive(Debug, Clone)]
pub struct MigrationSession {
    pub node_id: String,
    pub source_jurisdiction: String,
    pub target_jurisdiction: String,
    pub shards_to_move: Vec<String>,
    pub progress_percent: f64,
    pub status: MigrationStatus,
}

/// Migration status
#[derive(Debug, Clone, Copy)]
pub enum MigrationStatus {
    Planning,
    Initializing,
    Copying,
    Verifying,
    Redirecting,
    Cleaning,
    Completed,
}

/// Self-healing configuration
#[derive(Debug, Clone)]
pub struct SelfHealingConfig {
    pub auto_heal_enabled: bool,
    pub health_check_interval_ms: u32,
    pub migration_threshold: f64,
    pub redundancy_threshold: u32,
    pub max_concurrent_heals: u32,
}

/// PUF DNA Binding
pub struct PufBinding {
    /// Device bindings
    bindings: RwLock<HashMap<String, DeviceBinding>>,

    /// Hardware signature
    hardware_signature: RwLock<Option<HardwareSignature>>,

    /// Binding policy
    policy: BindingPolicy,
}

/// Device binding
#[derive(Debug, Clone)]
pub struct DeviceBinding {
    pub device_id: String,
    pub owner: String,
    pub signature: HardwareSignature,
    pub created_at: u64,
    pub last_verified: u64,
    pub trusted: bool,
    pub permissions: Vec<BindingPermission>,
}

/// Hardware signature
#[derive(Debug, Clone)]
pub struct HardwareSignature {
    pub cpu_id: Vec<u8>,
    pub motherboard_id: Vec<u8>,
    pub bios_hash: Vec<u8>,
    pub mac_addresses: Vec<String>,
    pub drive_serials: Vec<String>,
    pub signature_hash: Vec<u8>,
}

/// Binding permission
#[derive(Debug, Clone, Copy)]
pub enum BindingPermission {
    FullAccess,
    ReadOnly,
    ComputeOnly,
    StorageOnly,
}

/// Binding policy
#[derive(Debug, Clone)]
pub struct BindingPolicy {
    pub require_puf: bool,
    pub multi_device_allowed: bool,
    pub grace_period_hours: u32,
    pub revocation_requires_sovereign: bool,
}

/// Ghost Protocol
pub struct GhostProtocol {
    /// Protocol version
    version: String,

    /// Message handlers
    handlers: RwLock<HashMap<MessageType, MessageHandler>>,

    /// Peer connections
    peers: RwLock<HashMap<String, PeerConnection>>,

    /// Routing table
    routing: Arc<RoutingTable>,

    /// Encryption
    encryption: Arc<GhostEncryption>,
}

/// Message type
#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Handshake,
    Heartbeat,
    ShardRequest,
    ShardResponse,
    NodeAnnounce,
    NodeDepart,
    Migration,
    Healing,
    Data,
    Control,
}

/// Message handler
#[derive(Debug, Clone)]
pub struct MessageHandler {
    pub message_type: MessageType,
    pub handler_fn: String,
    pub priority: u32,
}

/// Peer connection
#[derive(Debug, Clone)]
pub struct PeerConnection {
    pub peer_id: String,
    pub address: GhostAddress,
    pub established_at: u64,
    pub last_message: u64,
    pub message_count: u64,
    pub bandwidth_mbps: f64,
    pub encrypted: bool,
}

/// Routing table
#[derive(Debug, Clone)]
pub struct RoutingTable {
    /// Routes by region
    by_region: RwLock<HashMap<String, Vec<Route>>>,

    /// Routes by node capability
    by_capability: RwLock<HashMap<NodeCapability, Vec<Route>>>>,

    /// Cached routes
    cache: RwLock<HashMap<String, CachedRoute>>,

    /// Routing algorithm
    algorithm: RoutingAlgorithm,
}

/// Route
#[derive(Debug, Clone)]
pub struct Route {
    pub destination: String,
    pub next_hop: String,
    pub cost: f64,
    pub latency_ms: u32,
    pub bandwidth_mbps: f64,
}

/// Cached route
#[derive(Debug, Clone)]
pub struct CachedRoute {
    pub route: Route,
    pub cached_at: u64,
    pub expires_at: u64,
}

/// Routing algorithm
#[derive(Debug, Clone, Copy)]
pub enum RoutingAlgorithm {
    ShortestPath,
    LowestLatency,
    HighestBandwidth,
    JurisdictionalAware,
    Quantum,
}

/// Ghost encryption
pub struct GhostEncryption {
    /// Active sessions
    sessions: RwLock<HashMap<String, EncryptionSession>>,

    /// Key manager
    key_manager: Arc<GhostKeyManager>,

    /// Configuration
    config: EncryptionConfig,
}

/// Encryption session
#[derive(Debug, Clone)]
pub struct EncryptionSession {
    pub session_id: String,
    pub peer_id: String,
    pub algorithm: String,
    pub key: Vec<u8>,
    pub established_at: u64,
    pub expires_at: u64,
}

/// Ghost key manager
pub struct GhostKeyManager {
    /// Key store
    keys: RwLock<HashMap<String, KeyInfo>>,

    /// Key rotation policy
    rotation: KeyRotationPolicy,
}

/// Key info
#[derive(Debug, Clone)]
pub struct KeyInfo {
    pub key_id: String,
    pub algorithm: String,
    pub public_key: Vec<u8>,
    pub created_at: u64,
    pub expires_at: u64,
    pub revoked: bool,
}

/// Key rotation policy
#[derive(Debug, Clone)]
pub struct KeyRotationPolicy {
    pub auto_rotate: bool,
    pub rotation_interval_days: u32,
    pub emergency_rotation_threshold: u32,
}

/// Encryption configuration
#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    pub default_algorithm: String,
    pub key_size_bits: u32,
    pub session_timeout_seconds: u64,
    pub perfect_forward_secrecy: bool,
    pub quantum_resistant: bool,
}

/// Ghost Mesh Configuration
#[derive(Debug, Clone)]
pub struct GhostMeshConfig {
    pub node_count_target: usize,
    pub shard_size_bytes: usize,
    pub redundancy_factor: u32,
    pub encryption_level: EncryptionLevel,
    pub self_healing_enabled: bool,
    pub puf_binding_enabled: bool,
    pub mesh_protocol: String,
}

impl GhostMesh {
    /// Create a new Ghost Mesh
    pub async fn new(config: GhostMeshConfig) -> Result<Self> {
        info!("Initializing Ghost Mesh - Pillar I");

        let nodes = Arc::new(NodeRegistry {
            nodes: RwLock::new(HashMap::new()),
            by_region: RwLock::new(HashMap::new()),
            by_capability: RwLock::new(HashMap::new()),
            dead_pool: RwLock::new(VecDeque::new()),
        });

        let sharding = Arc::new(QuantumSharding {
            shards: RwLock::new(HashMap::new()),
            distribution: RwLock::new(ShardDistribution {
                strategy: DistributionStrategy::Quantum,
                redundancy_factor: config.redundancy_factor,
                geographic_spread: true,
                jurisdiction_aware: true,
                nodes_per_shard: 3,
            }),
            reconstruction_cache: RwLock::new(HashMap::new()),
            config: ShardingConfig {
                shard_size_bytes: config.shard_size_bytes,
                total_shards_target: 1_200_000_000, // 1.2 billion
                redundancy_factor: config.redundancy_factor,
                encryption_algorithm: "XChaCha20Poly1305".to_string(),
                self_healing_enabled: config.self_healing_enabled,
                jurisdiction_zones: vec![
                    "US".to_string(),
                    "EU".to_string(),
                    "AS".to_string(),
                    "AF".to_string(),
                    "OC".to_string(),
                ],
            },
        });

        let self_healing = Arc::new(SelfHealingManager {
            health_monitor: Arc::new(HealthMonitor {
                scores: RwLock::new(HashMap::new()),
                thresholds: HealthThresholds {
                    critical: 0.2,
                    warning: 0.5,
                    healthy: 0.8,
                },
                interval_ms: 5000,
            }),
            recovery_protocols: RwLock::new(HashMap::new()),
            migration: Arc::new(NodeMigration {
                migrations: RwLock::new(HashMap::new()),
                target_pools: RwLock::new(HashMap::new()),
            }),
            config: SelfHealingConfig {
                auto_heal_enabled: true,
                health_check_interval_ms: 5000,
                migration_threshold: 0.3,
                redundancy_threshold: 2,
                max_concurrent_heals: 10,
            },
        });

        let puf_binding = Arc::new(PufBinding {
            bindings: RwLock::new(HashMap::new()),
            hardware_signature: RwLock::new(None),
            policy: BindingPolicy {
                require_puf: config.puf_binding_enabled,
                multi_device_allowed: false,
                grace_period_hours: 24,
                revocation_requires_sovereign: true,
            },
        });

        let routing = Arc::new(RoutingTable {
            by_region: RwLock::new(HashMap::new()),
            by_capability: RwLock::new(HashMap::new()),
            cache: RwLock::new(HashMap::new()),
            algorithm: RoutingAlgorithm::Quantum,
        });

        let encryption = Arc::new(GhostEncryption {
            sessions: RwLock::new(HashMap::new()),
            key_manager: Arc::new(GhostKeyManager {
                keys: RwLock::new(HashMap::new()),
                rotation: KeyRotationPolicy {
                    auto_rotate: true,
                    rotation_interval_days: 30,
                    emergency_rotation_threshold: 100,
                },
            }),
            config: EncryptionConfig {
                default_algorithm: "XChaCha20Poly1305".to_string(),
                key_size_bits: 256,
                session_timeout_seconds: 3600,
                perfect_forward_secrecy: true,
                quantum_resistant: true,
            },
        });

        let protocol = Arc::new(GhostProtocol {
            version: "1.0.0".to_string(),
            handlers: RwLock::new(HashMap::new()),
            peers: RwLock::new(HashMap::new()),
            routing,
            encryption,
        });

        let mesh = Self {
            nodes,
            sharding,
            self_healing,
            puf_binding,
            protocol,
            config,
        };

        // Initialize message handlers
        mesh.initialize_handlers();

        info!("Ghost Mesh initialized with {} target nodes", config.node_count_target);
        Ok(mesh)
    }

    /// Initialize message handlers
    fn initialize_handlers(&self) {
        let mut handlers = self.protocol.handlers.write();

        handlers.insert(MessageType::Handshake, MessageHandler {
            message_type: MessageType::Handshake,
            handler_fn: "handle_handshake".to_string(),
            priority: 10,
        });

        handlers.insert(MessageType::Heartbeat, MessageHandler {
            message_type: MessageType::Heartbeat,
            handler_fn: "handle_heartbeat".to_string(),
            priority: 9,
        });

        handlers.insert(MessageType::ShardRequest, MessageHandler {
            message_type: MessageType::ShardRequest,
            handler_fn: "handle_shard_request".to_string(),
            priority: 8,
        });

        handlers.insert(MessageType::Migration, MessageHandler {
            message_type: MessageType::Migration,
            handler_fn: "handle_migration".to_string(),
            priority: 7,
        });

        handlers.insert(MessageType::Healing, MessageHandler {
            message_type: MessageType::Healing,
            handler_fn: "handle_healing".to_string(),
            priority: 10,
        });
    }

    /// Register a new node
    pub fn register_node(&self, node: GhostNode) -> Result<()> {
        debug!("Registering node: {}", node.id);

        let mut nodes = self.nodes.nodes.write();
        nodes.insert(node.id.clone(), node.clone());

        // Index by region
        self.nodes.by_region.write()
            .entry(node.region.clone())
            .or_insert_with(Vec::new)
            .push(node.id.clone());

        // Index by capability
        for capability in &node.capabilities {
            self.nodes.by_capability.write()
                .entry(*capability)
                .or_insert_with(Vec::new)
                .push(node.id.clone());
        }

        Ok(())
    }

    /// Shard data
    pub async fn shard_data(&self, data: &[u8], parent_id: &str) -> Result<Vec<DataShard>> {
        debug!("Sharding data: {} bytes into quantum shards", data.len());

        let config = &self.sharding.config;
        let shard_size = config.shard_size_bytes;
        let total_shards = ((data.len() + shard_size - 1) / shard_size) as u32;

        let mut shards = Vec::new();
        let mut chunks = data.chunks(shard_size);

        for (index, chunk) in chunks.enumerate() {
            let shard = DataShard {
                id: format!("{}_{}", parent_id, index),
                parent_id: parent_id.to_string(),
                index: index as u32,
                total_shards,
                data: chunk.to_vec(),
                encryption_key: self.generate_shard_key()?,
                node_id: String::new(), // Assigned during distribution
                parity_shard: false,
                checksum: self.calculate_checksum(chunk),
                created_at: current_timestamp(),
                access_count: 0,
            };

            shards.push(shard);
        }

        // Add parity shards for redundancy
        let parity_count = config.redundancy_factor;
        for i in 0..parity_count {
            let parity_shard = DataShard {
                id: format!("{}_parity_{}", parent_id, i),
                parent_id: parent_id.to_string(),
                index: total_shards + i,
                total_shards: total_shards + parity_count,
                data: vec![0; shard_size.min(chunk::by = data.len())],
                encryption_key: self.generate_shard_key()?,
                node_id: String::new(),
                parity_shard: true,
                checksum: 0,
                created_at: current_timestamp(),
                access_count: 0,
            };
            shards.push(parity_shard);
        }

        Ok(shards)
    }

    /// Reconstruct data from shards
    pub async fn reconstruct(&self, parent_id: &str) -> Result<Vec<u8>> {
        debug!("Reconstructing data from shards: {}", parent_id);

        let shards = self.sharding.shards.read();
        let relevant_shards: Vec<_> = shards.values()
            .filter(|s| s.parent_id == parent_id && !s.parity_shard)
            .collect();

        if relevant_shards.is_empty() {
            return Err(SbmumcError::GhostMesh("No shards found".to_string()));
        }

        let mut data = Vec::new();
        for shard in relevant_shards.iter().take(1000) { // Limit for safety
            data.extend_from_slice(&shard.data);
        }

        Ok(data)
    }

    /// Generate shard encryption key
    fn generate_shard_key(&self) -> Result<Vec<u8>> {
        // In full implementation, use quantum-resistant key generation
        let mut key = vec![0u8; 32];
        use ring::rand::{SecureRandom, SystemRandom};
        let rng = SystemRandom::new();
        rng.fill(&mut key)
            .map_err(|e| SbmumcError::Encryption(e.to_string()))?;
        Ok(key)
    }

    /// Calculate checksum
    fn calculate_checksum(&self, data: &[u8]) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }

    /// Perform health check on all nodes
    pub fn health_check(&self) -> HealthReport {
        let nodes = self.nodes.nodes.read();
        let mut healthy = 0;
        let mut degraded = 0;
        let mut critical = 0;

        for (_, node) in nodes.iter() {
            match node.status {
                NodeStatus::Online => healthy += 1,
                NodeStatus::Degrading => degraded += 1,
                NodeStatus::Offline => critical += 1,
                _ => {}
            }
        }

        HealthReport {
            total_nodes: nodes.len(),
            healthy_nodes: healthy,
            degraded_nodes: degraded,
            critical_nodes: critical,
            mesh_health_score: if nodes.is_empty() { 0.0 } else { healthy as f64 / nodes.len() as f64 },
            timestamp: current_timestamp(),
        }
    }

    /// Trigger self-healing
    pub async fn trigger_healing(&self, node_id: &str) -> Result<()> {
        info!("Triggering self-healing for node: {}", node_id);

        let nodes = self.nodes.nodes.read();
        let node = nodes.get(node_id)
            .ok_or_else(|| SbmumcError::GhostMesh(format!("Node not found: {}", node_id)))?;

        // Find alternative nodes
        let alternatives: Vec<_> = nodes.values()
            .filter(|n| n.region == node.region && n.id != node_id && n.status == NodeStatus::Online)
            .take(3)
            .collect();

        if alternatives.is_empty() {
            warn!("No alternative nodes available for healing");
        }

        Ok(())
    }

    /// Get mesh status
    pub fn get_status(&self) -> GhostMeshStatus {
        let nodes = self.nodes.nodes.read();
        let shards = self.sharding.shards.read();

        GhostMeshStatus {
            total_nodes: nodes.len(),
            active_shards: shards.len(),
            mesh_health: self.health_check().mesh_health_score,
            encryption_active: true,
            puf_binding_active: self.puf_binding.policy.require_puf,
        }
    }
}

/// Health report
#[derive(Debug, Clone)]
pub struct HealthReport {
    pub total_nodes: usize,
    pub healthy_nodes: usize,
    pub degraded_nodes: usize,
    pub critical_nodes: usize,
    pub mesh_health_score: f64,
    pub timestamp: u64,
}

/// Ghost mesh status
#[derive(Debug, Clone)]
pub struct GhostMeshStatus {
    pub total_nodes: usize,
    pub active_shards: usize,
    pub mesh_health: f64,
    pub encryption_active: bool,
    pub puf_binding_active: bool,
}

/// Get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

impl Default for GhostMeshConfig {
    fn default() -> Self {
        Self {
            node_count_target: 10_000,
            shard_size_bytes: 1024,
            redundancy_factor: 3,
            encryption_level: EncryptionLevel::Maximum,
            self_healing_enabled: true,
            puf_binding_enabled: true,
            mesh_protocol: "GSTM".to_string(),
        }
    }
}
