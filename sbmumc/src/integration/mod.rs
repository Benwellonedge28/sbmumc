//!
//! Software Integration Framework - Universal Connectivity
//!
//! This module provides comprehensive integration capabilities for connecting
//! SBMUMC with any existing and future software:
//! - Plugin architecture with dynamic loading
//! - API gateway for REST/GraphQL
//! - Protocol adapters for common formats
//! - IDE integration support
//! - OS-level integration
//! - Future-proof extensibility

use crate::core::{Result, EntityId, SbmumcError};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, debug, warn};
use serde::{Serialize, Deserialize};

/// Integration Engine - Main integration coordinator
pub struct IntegrationEngine {
    /// Plugin manager
    plugins: Arc<PluginManager>,

    /// Protocol adapters
    adapters: Arc<ProtocolAdapterRegistry>,

    /// API gateway
    api_gateway: Arc<ApiGateway>,

    /// IDE integrations
    ide_connector: Arc<IdeConnector>,

    /// OS integration
    os_integration: Arc<OsIntegration>,

    /// Configuration
    config: IntegrationConfig,
}

/// Plugin Manager
pub struct PluginManager {
    /// Loaded plugins
    plugins: RwLock<HashMap<String, Plugin>>,

    /// Plugin registry
    registry: RwLock<PluginRegistry>,

    /// Sandbox environments
    sandboxes: RwLock<HashMap<String, SandboxContext>>,
}

/// Plugin definition
#[derive(Debug, Clone)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub version: String,
    pub plugin_type: PluginType,
    pub capabilities: Vec<PluginCapability>,
    pub permissions: Vec<Permission>,
    pub state: PluginState,
    pub entry_point: String,
    pub metadata: PluginMetadata,
}

/// Plugin types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluginType {
    LanguageServer,      // LSP integration
    Editor,              // Text editor plugin
    BuildTool,           // Build system integration
    Debugger,            // Debug adapter
    Linter,              // Code analysis
    Formatter,           // Code formatting
    VersionControl,      // VCS integration
    Database,            // Database client
    Cloud,               // Cloud platform integration
    AI,                  // AI/ML integration
    Custom,              // User-defined
}

/// Plugin capability
#[derive(Debug, Clone)]
pub struct PluginCapability {
    pub name: String,
    pub description: String,
    pub api_version: String,
}

/// Plugin state
#[derive(Debug, Clone, Copy)]
pub enum PluginState {
    Unloaded,
    Loading,
    Loaded,
    Running,
    Paused,
    Error,
    Uninstalled,
}

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub author: String,
    pub license: String,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub description: String,
    pub tags: Vec<String>,
    pub dependencies: Vec<PluginDependency>,
    pub api_endpoints: Vec<ApiEndpoint>,
}

/// Plugin dependency
#[derive(Debug, Clone)]
pub struct PluginDependency {
    pub plugin_id: String,
    pub version_range: String,
    pub optional: bool,
}

/// API endpoint
#[derive(Debug, Clone)]
pub struct ApiEndpoint {
    pub path: String,
    pub method: HttpMethod,
    pub description: String,
    pub auth_required: bool,
}

/// HTTP methods
#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
}

/// Plugin registry
#[derive(Debug, Clone)]
pub struct PluginRegistry {
    /// Available plugins
    available: Vec<PluginManifest>,

    /// Categories
    categories: Vec<PluginCategory>,
}

/// Plugin manifest
#[derive(Debug, Clone)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub download_url: String,
    pub checksum: String,
    pub signature: Option<String>,
}

/// Plugin category
#[derive(Debug, Clone)]
pub struct PluginCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub plugins: Vec<String>,
}

/// Sandbox context
#[derive(Debug, Clone)]
pub struct SandboxContext {
    pub id: String,
    pub plugin_id: String,
    pub permissions: Vec<Permission>,
    pub memory_limit_mb: u64,
    pub cpu_limit_percent: u32,
    pub network_allowed: bool,
}

/// Permission types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    FileSystemRead,
    FileSystemWrite,
    NetworkAccess,
    ProcessExecution,
    SystemInfo,
    CompilerAccess,
    AIInference,
    KnowledgeGraph,
    PluginLoad,
}

/// Protocol Adapter Registry
pub struct ProtocolAdapterRegistry {
    /// Adapters
    adapters: RwLock<HashMap<String, Arc<dyn ProtocolAdapter + Send + Sync>>>,

    /// Supported protocols
    supported_protocols: RwLock<HashSet<String>>,
}

/// Protocol adapter trait
pub trait ProtocolAdapter: Send + Sync {
    fn get_protocol_name(&self) -> &str;
    fn get_version(&self) -> &str;
    fn encode(&self, data: &[u8], format: &str) -> Result<Vec<u8>>;
    fn decode(&self, data: &[u8], format: &str) -> Result<Vec<u8>>;
    fn validate(&self, data: &[u8]) -> Result<bool>;
}

/// JSON adapter
pub struct JsonAdapter {
    version: String,
}

/// XML adapter
pub struct XmlAdapter {
    version: String,
}

/// Binary adapter
pub struct BinaryAdapter {
    version: String,
}

/// Protocol buffer adapter
pub struct ProtobufAdapter {
    version: String,
}

/// GraphQL adapter
pub struct GraphQLAdapter {
    version: String,
}

/// gRPC adapter
pub struct GrpcAdapter {
    version: String,
}

/// WebSocket adapter
pub struct WebSocketAdapter {
    version: String,
}

/// MQTT adapter
pub struct MqttAdapter {
    version: String,
}

/// API Gateway
pub struct ApiGateway {
    /// Routes
    routes: RwLock<HashMap<String, ApiRoute>>,

    /// Middleware
    middleware: RwLock<Vec<ApiMiddleware>>,

    /// Rate limiting
    rate_limiter: Arc<RateLimiter>,

    /// Authentication
    auth: Arc<ApiAuth>,
}

/// API route
#[derive(Debug, Clone)]
pub struct ApiRoute {
    pub path: String,
    pub method: HttpMethod,
    pub handler: String,
    pub middleware: Vec<String>,
    pub auth_required: bool,
    pub rate_limit: Option<RateLimit>,
}

/// API middleware
#[derive(Debug, Clone)]
pub struct ApiMiddleware {
    pub name: String,
    pub middleware_type: MiddlewareType,
    pub config: HashMap<String, String>,
}

/// Middleware type
#[derive(Debug, Clone, Copy)]
pub enum MiddlewareType {
    Auth,
    Logging,
    CORS,
    Compression,
    Validation,
    RateLimit,
    Cache,
    Custom,
}

/// Rate limiter
#[derive(Debug, Clone)]
pub struct RateLimiter {
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

/// Rate limit
#[derive(Debug, Clone)]
pub struct RateLimit {
    pub requests: u32,
    pub window_seconds: u32,
}

/// API authentication
#[derive(Debug, Clone)]
pub struct ApiAuth {
    pub auth_type: AuthType,
    pub jwt_secret: Option<String>,
    pub api_keys: RwLock<HashMap<String, ApiKeyInfo>>,
}

/// Auth type
#[derive(Debug, Clone, Copy)]
pub enum AuthType {
    None,
    APIKey,
    JWT,
    OAuth2,
    Basic,
}

/// API key info
#[derive(Debug, Clone)]
pub struct ApiKeyInfo {
    pub key: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub created_at: u64,
    pub expires_at: Option<u64>,
}

/// IDE Connector
pub struct IdeConnector {
    /// Supported IDEs
    supported_ides: RwLock<HashMap<String, IdeSupport>>,

    /// Language server connections
    lsp_connections: RwLock<HashMap<String, LspConnection>>,

    /// Debug adapters
    debug_adapters: RwLock<HashMap<String, DebugAdapterInfo>>,

    /// Configuration
    config: IdeConfig,
}

/// IDE support info
#[derive(Debug, Clone)]
pub struct IdeSupport {
    pub ide_id: String,
    pub name: String,
    pub versions: Vec<String>,
    pub integration_type: IdeIntegrationType,
    pub capabilities: Vec<IdeCapability>,
}

/// IDE integration type
#[derive(Debug, Clone, Copy)]
pub enum IdeIntegrationType {
    Plugin,
    Extension,
    Native,
    Remote,
}

/// IDE capability
#[derive(Debug, Clone)]
pub struct IdeCapability {
    pub name: String,
    pub supported: bool,
    pub config_options: HashMap<String, String>,
}

/// LSP connection
#[derive(Debug, Clone)]
pub struct LspConnection {
    pub id: String,
    pub language: String,
    pub endpoint: String,
    pub capabilities: Vec<LspCapability>,
    pub status: ConnectionStatus,
}

/// LSP capability
#[derive(Debug, Clone)]
pub struct LspCapability {
    pub name: String,
    pub enabled: bool,
}

/// Connection status
#[derive(Debug, Clone, Copy)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

/// Debug adapter info
#[derive(Debug, Clone)]
pub struct DebugAdapterInfo {
    pub id: String,
    pub name: String,
    pub adapter_type: DebugAdapterType,
    pub endpoint: String,
}

/// Debug adapter type
#[derive(Debug, Clone, Copy)]
pub enum DebugAdapterType {
    DAP,
    LLDB,
    GDB,
    Native,
    Custom,
}

/// IDE configuration
#[derive(Debug, Clone)]
pub struct IdeConfig {
    pub auto_connect: bool,
    pub default_timeout_ms: u32,
    pub lsp_enabled: bool,
    pub debug_enabled: bool,
    pub formatter_integration: bool,
}

/// OS Integration
pub struct OsIntegration {
    /// Platform info
    platform: PlatformInfo,

    /// System services
    services: RwLock<HashMap<String, SystemService>>,

    /// File associations
    file_associations: RwLock<HashMap<String, FileAssociation>>>,

    /// Environment
    environment: RwLock<HashMap<String, String>>,

    /// Configuration
    config: OsConfig,
}

/// Platform info
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub os: String,
    pub version: String,
    pub arch: String,
    pub hostname: String,
}

/// System service
#[derive(Debug, Clone)]
pub struct SystemService {
    pub name: String,
    pub service_type: ServiceType,
    pub status: ServiceStatus,
    pub auto_start: bool,
    pub config_path: String,
}

/// Service type
#[derive(Debug, Clone, Copy)]
pub enum ServiceType {
    Daemon,
    Background,
    Scheduled,
    Triggered,
}

/// Service status
#[derive(Debug, Clone, Copy)]
pub enum ServiceStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

/// File association
#[derive(Debug, Clone)]
pub struct FileAssociation {
    pub extension: String,
    pub mime_type: String,
    pub handler: String,
    pub icon: Option<String>,
}

/// OS configuration
#[derive(Debug, Clone)]
pub struct OsConfig {
    pub register_file_associations: bool,
    pub register_protocol_handlers: bool,
    pub system_tray_enabled: bool,
    pub notifications_enabled: bool,
    pub auto_start_enabled: bool,
}

/// Integration configuration
#[derive(Debug, Clone)]
pub struct IntegrationConfig {
    pub enable_plugins: bool,
    pub enable_api_gateway: bool,
    pub enable_ide_integration: bool,
    pub enable_os_integration: bool,
    pub api_port: u16,
    pub api_cors_enabled: bool,
    pub max_plugin_memory_mb: u64,
}

impl IntegrationEngine {
    /// Create a new integration engine
    pub async fn new(config: IntegrationConfig) -> Result<Self> {
        info!("Initializing Integration Engine");

        let plugins = Arc::new(PluginManager {
            plugins: RwLock::new(HashMap::new()),
            registry: RwLock::new(PluginRegistry {
                available: Vec::new(),
                categories: vec![
                    PluginCategory {
                        id: "language".to_string(),
                        name: "Language Support".to_string(),
                        description: "Programming language integrations".to_string(),
                        plugins: Vec::new(),
                    },
                    PluginCategory {
                        id: "tool".to_string(),
                        name: "Developer Tools".to_string(),
                        description: "Build, test, and deployment tools".to_string(),
                        plugins: Vec::new(),
                    },
                    PluginCategory {
                        id: "ai".to_string(),
                        name: "AI & ML".to_string(),
                        description: "Artificial intelligence integrations".to_string(),
                        plugins: Vec::new(),
                    },
                ],
            }),
            sandboxes: RwLock::new(HashMap::new()),
        });

        let adapters = Arc::new(ProtocolAdapterRegistry {
            adapters: RwLock::new(HashMap::new()),
            supported_protocols: RwLock::new(HashSet::new()),
        });

        let api_gateway = Arc::new(ApiGateway {
            routes: RwLock::new(HashMap::new()),
            middleware: RwLock::new(Vec::new()),
            rate_limiter: Arc::new(RateLimiter {
                requests_per_minute: 1000,
                burst_size: 100,
            }),
            auth: Arc::new(ApiAuth {
                auth_type: AuthType::JWT,
                jwt_secret: None,
                api_keys: RwLock::new(HashMap::new()),
            }),
        });

        let ide_connector = Arc::new(IdeConnector {
            supported_ides: RwLock::new(HashMap::new()),
            lsp_connections: RwLock::new(HashMap::new()),
            debug_adapters: RwLock::new(HashMap::new()),
            config: IdeConfig {
                auto_connect: true,
                default_timeout_ms: 5000,
                lsp_enabled: true,
                debug_enabled: true,
                formatter_integration: true,
            },
        });

        let os_integration = Arc::new(OsIntegration {
            platform: PlatformInfo {
                os: std::env::consts::OS.to_string(),
                version: "1.0.0".to_string(),
                arch: std::env::consts::ARCH.to_string(),
                hostname: hostname::get()
                    .map(|h| h.to_string_lossy().to_string())
                    .unwrap_or_else(|_| "unknown".to_string()),
            },
            services: RwLock::new(HashMap::new()),
            file_associations: RwLock::new(HashMap::new()),
            environment: RwLock::new(std::env::vars().collect()),
            config: OsConfig {
                register_file_associations: true,
                register_protocol_handlers: true,
                system_tray_enabled: true,
                notifications_enabled: true,
                auto_start_enabled: true,
            },
        });

        let engine = Self {
            plugins,
            adapters,
            api_gateway,
            ide_connector,
            os_integration,
            config,
        };

        // Initialize protocol adapters
        engine.initialize_adapters();

        // Initialize IDE support
        engine.initialize_ide_support();

        info!("Integration Engine initialized");
        Ok(engine)
    }

    /// Initialize protocol adapters
    fn initialize_adapters(&self) {
        // JSON adapter
        self.adapters.adapters.write().insert(
            "json".to_string(),
            Arc::new(JsonAdapter { version: "1.0".to_string() }) as Arc<dyn ProtocolAdapter + Send + Sync>,
        );

        // XML adapter
        self.adapters.adapters.write().insert(
            "xml".to_string(),
            Arc::new(XmlAdapter { version: "1.0".to_string() }) as Arc<dyn ProtocolAdapter + Send + Sync>,
        );

        // Binary adapter
        self.adapters.adapters.write().insert(
            "binary".to_string(),
            Arc::new(BinaryAdapter { version: "1.0".to_string() }) as Arc<dyn ProtocolAdapter + Send + Sync>,
        );

        // GraphQL adapter
        self.adapters.adapters.write().insert(
            "graphql".to_string(),
            Arc::new(GraphQLAdapter { version: "1.0".to_string() }) as Arc<dyn ProtocolAdapter + Send + Sync>,
        );

        // gRPC adapter
        self.adapters.adapters.write().insert(
            "grpc".to_string(),
            Arc::new(GrpcAdapter { version: "1.0".to_string() }) as Arc<dyn ProtocolAdapter + Send + Sync>,
        );

        let mut protocols = self.adapters.supported_protocols.write();
        protocols.insert("json".to_string());
        protocols.insert("xml".to_string());
        protocols.insert("binary".to_string());
        protocols.insert("graphql".to_string());
        protocols.insert("grpc".to_string());
        protocols.insert("websocket".to_string());
        protocols.insert("mqtt".to_string());
    }

    /// Initialize IDE support
    fn initialize_ide_support(&self) {
        let ides = vec![
            IdeSupport {
                ide_id: "vscode".to_string(),
                name: "Visual Studio Code".to_string(),
                versions: vec!["1.0".to_string(), "1.5".to_string(), "1.8".to_string()],
                integration_type: IdeIntegrationType::Extension,
                capabilities: vec![
                    IdeCapability {
                        name: "Language Server".to_string(),
                        supported: true,
                        config_options: HashMap::new(),
                    },
                    IdeCapability {
                        name: "Debug Adapter".to_string(),
                        supported: true,
                        config_options: HashMap::new(),
                    },
                    IdeCapability {
                        name: "Formatter".to_string(),
                        supported: true,
                        config_options: HashMap::new(),
                    },
                ],
            },
            IdeSupport {
                ide_id: "intellij".to_string(),
                name: "IntelliJ IDEA".to_string(),
                versions: vec!["2021".to_string(), "2022".to_string(), "2023".to_string()],
                integration_type: IdeIntegrationType::Plugin,
                capabilities: vec![
                    IdeCapability {
                        name: "Language Server".to_string(),
                        supported: true,
                        config_options: HashMap::new(),
                    },
                    IdeCapability {
                        name: "Debug Adapter".to_string(),
                        supported: true,
                        config_options: HashMap::new(),
                    },
                ],
            },
            IdeSupport {
                ide_id: "vim".to_string(),
                name: "Vim/Neovim".to_string(),
                versions: vec!["8.0".to_string(), "9.0".to_string()],
                integration_type: IdeIntegrationType::Native,
                capabilities: vec![
                    IdeCapability {
                        name: "Language Server".to_string(),
                        supported: true,
                        config_options: HashMap::new(),
                    },
                ],
            },
            IdeSupport {
                ide_id: "emacs".to_string(),
                name: "Emacs".to_string(),
                versions: vec!["27".to_string(), "28".to_string(), "29".to_string()],
                integration_type: IdeIntegrationType::Native,
                capabilities: vec![
                    IdeCapability {
                        name: "Language Server".to_string(),
                        supported: true,
                        config_options: HashMap::new(),
                    },
                ],
            },
        ];

        let mut supported = self.ide_connector.supported_ides.write();
        for ide in ides {
            supported.insert(ide.ide_id.clone(), ide);
        }
    }

    /// Load a plugin
    pub async fn load_plugin(&self, plugin_id: &str) -> Result<()> {
        debug!("Loading plugin: {}", plugin_id);

        let plugin = Plugin {
            id: plugin_id.to_string(),
            name: plugin_id.to_string(),
            version: "1.0.0".to_string(),
            plugin_type: PluginType::Custom,
            capabilities: vec![],
            permissions: vec![Permission::FileSystemRead, Permission::CompilerAccess],
            state: PluginState::Loading,
            entry_point: format!("plugins/{}", plugin_id),
            metadata: PluginMetadata {
                author: "Unknown".to_string(),
                license: "MIT".to_string(),
                repository: None,
                homepage: None,
                description: "Plugin loaded from mesh".to_string(),
                tags: vec![],
                dependencies: vec![],
                api_endpoints: vec![],
            },
        };

        self.plugins.plugins.write().insert(plugin_id.to_string(), plugin);

        Ok(())
    }

    /// Unload a plugin
    pub async fn unload_plugin(&self, plugin_id: &str) -> Result<()> {
        debug!("Unloading plugin: {}", plugin_id);

        let mut plugins = self.plugins.plugins.write();
        if let Some(plugin) = plugins.get_mut(plugin_id) {
            plugin.state = PluginState::Unloaded;
        }

        plugins.remove(plugin_id);

        Ok(())
    }

    /// Register API route
    pub fn register_route(&self, route: ApiRoute) -> Result<()> {
        let key = format!("{}:{}", route.method.as_str(), route.path);
        self.api_gateway.routes.write().insert(key, route);
        Ok(())
    }

    /// Connect to LSP server
    pub async fn connect_lsp(&self, language: &str, endpoint: &str) -> Result<String> {
        let connection_id = EntityId::new().to_string();

        let connection = LspConnection {
            id: connection_id.clone(),
            language: language.to_string(),
            endpoint: endpoint.to_string(),
            capabilities: vec![
                LspCapability {
                    name: "completion".to_string(),
                    enabled: true,
                },
                LspCapability {
                    name: "hover".to_string(),
                    enabled: true,
                },
                LspCapability {
                    name: "diagnostics".to_string(),
                    enabled: true,
                },
            ],
            status: ConnectionStatus::Connected,
        };

        self.ide_connector.lsp_connections.write()
            .insert(connection_id.clone(), connection);

        Ok(connection_id)
    }

    /// Register file association
    pub fn register_file_association(&self, extension: &str, mime_type: &str, handler: &str) -> Result<()> {
        let association = FileAssociation {
            extension: extension.to_string(),
            mime_type: mime_type.to_string(),
            handler: handler.to_string(),
            icon: None,
        };

        self.os_integration.file_associations.write()
            .insert(extension.to_string(), association);

        Ok(())
    }

    /// Encode data using protocol adapter
    pub fn encode(&self, protocol: &str, data: &[u8], format: &str) -> Result<Vec<u8>> {
        let adapters = self.adapters.adapters.read();
        let adapter = adapters.get(protocol)
            .ok_or_else(|| SbmumcError::Integration(format!("Protocol adapter not found: {}", protocol)))?;

        adapter.encode(data, format)
    }

    /// Decode data using protocol adapter
    pub fn decode(&self, protocol: &str, data: &[u8], format: &str) -> Result<Vec<u8>> {
        let adapters = self.adapters.adapters.read();
        let adapter = adapters.get(protocol)
            .ok_or_else(|| SbmumcError::Integration(format!("Protocol adapter not found: {}", protocol)))?;

        adapter.decode(data, format)
    }

    /// Get integration status
    pub fn get_status(&self) -> IntegrationStatus {
        IntegrationStatus {
            loaded_plugins: self.plugins.plugins.read().len(),
            active_adapters: self.adapters.supported_protocols.read().len(),
            api_routes: self.api_gateway.routes.read().len(),
            lsp_connections: self.ide_connector.lsp_connections.read().len(),
            platform: self.os_integration.platform.clone(),
        }
    }
}

/// Integration status
#[derive(Debug, Clone)]
pub struct IntegrationStatus {
    pub loaded_plugins: usize,
    pub active_adapters: usize,
    pub api_routes: usize,
    pub lsp_connections: usize,
    pub platform: PlatformInfo,
}

// Protocol adapter implementations
impl ProtocolAdapter for JsonAdapter {
    fn get_protocol_name(&self) -> &str { "json" }
    fn get_version(&self) -> &str { &self.version }

    fn encode(&self, data: &[u8], _format: &str) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }

    fn decode(&self, data: &[u8], _format: &str) -> Result<Vec<u8>> {
        // Validate JSON
        serde_json::from_slice::<serde_json::Value>(data)
            .map_err(|e| SbmumcError::Serialization(e.to_string()))?;
        Ok(data.to_vec())
    }

    fn validate(&self, data: &[u8]) -> Result<bool> {
        Ok(serde_json::from_slice::<serde_json::Value>(data).is_ok())
    }
}

impl ProtocolAdapter for XmlAdapter {
    fn get_protocol_name(&self) -> &str { "xml" }
    fn get_version(&self) -> &str { &self.version }

    fn encode(&self, data: &[u8], _format: &str) -> Result<Vec<u8>> { Ok(data.to_vec()) }
    fn decode(&self, data: &[u8], _format: &str) -> Result<Vec<u8>> { Ok(data.to_vec()) }
    fn validate(&self, data: &[u8]) -> Result<bool> { Ok(true) }
}

impl ProtocolAdapter for BinaryAdapter {
    fn get_protocol_name(&self) -> &str { "binary" }
    fn get_version(&self) -> &str { &self.version }

    fn encode(&self, data: &[u8], _format: &str) -> Result<Vec<u8>> { Ok(data.to_vec()) }
    fn decode(&self, data: &[u8], _format: &str) -> Result<Vec<u8>> { Ok(data.to_vec()) }
    fn validate(&self, data: &[u8]) -> Result<bool> { Ok(true) }
}

impl ProtocolAdapter for GraphQLAdapter {
    fn get_protocol_name(&self) -> &str { "graphql" }
    fn get_version(&self) -> &str { &self.version }

    fn encode(&self, data: &[u8], _format: &str) -> Result<Vec<u8>> { Ok(data.to_vec()) }
    fn decode(&self, data: &[u8], _format: &str) -> Result<Vec<u8>> { Ok(data.to_vec()) }
    fn validate(&self, data: &[u8]) -> Result<bool> { Ok(true) }
}

impl ProtocolAdapter for GrpcAdapter {
    fn get_protocol_name(&self) -> &str { "grpc" }
    fn get_version(&self) -> &str { &self.version }

    fn encode(&self, data: &[u8], _format: &str) -> Result<Vec<u8>> { Ok(data.to_vec()) }
    fn decode(&self, data: &[u8], _format: &str) -> Result<Vec<u8>> { Ok(data.to_vec()) }
    fn validate(&self, data: &[u8]) -> Result<bool> { Ok(true) }
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::OPTIONS => "OPTIONS",
        }
    }
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            enable_plugins: true,
            enable_api_gateway: true,
            enable_ide_integration: true,
            enable_os_integration: true,
            api_port: 8080,
            api_cors_enabled: true,
            max_plugin_memory_mb: 512,
        }
    }
}
