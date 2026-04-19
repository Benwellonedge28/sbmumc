//! Configuration Module for SBMUMC
//!
//! Handles loading, saving, and managing configuration for the SBMUMC system.

use super::{SbmumcError, Result, SecurityLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// SBMUMC Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbmumcConfig {
    /// System configuration
    pub system: SystemConfig,

    /// Security configuration
    pub security: SecurityConfig,

    /// Knowledge configuration
    pub knowledge: KnowledgeConfig,

    /// Learning configuration
    pub learning: LearningConfig,

    /// Language configuration
    pub language: LanguageConfig,

    /// Compiler configuration
    pub compiler: CompilerConfig,

    /// Admin configuration
    pub admin: AdminConfig,

    /// Network configuration
    pub network: NetworkConfig,

    /// Logging configuration
    pub logging: LoggingConfig,
}

impl Default for SbmumcConfig {
    fn default() -> Self {
        Self {
            system: SystemConfig::default(),
            security: SecurityConfig::default(),
            knowledge: KnowledgeConfig::default(),
            learning: LearningConfig::default(),
            language: LanguageConfig::default(),
            compiler: CompilerConfig::default(),
            admin: AdminConfig::default(),
            network: NetworkConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl SbmumcConfig {
    /// Load configuration from file
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| SbmumcError::Configuration(format!("Failed to read config file: {}", e)))?;

        toml::from_str(&content)
            .map_err(|e| SbmumcError::Configuration(format!("Failed to parse config: {}", e)))
    }

    /// Save configuration to file
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| SbmumcError::Configuration(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(path, content)
            .map_err(|e| SbmumcError::Configuration(format!("Failed to write config file: {}", e)))
    }
}

/// System configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// System name
    pub name: String,

    /// Version
    pub version: String,

    /// Owner/developer
    pub owner: String,

    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,

    /// Default timeout in milliseconds
    pub default_timeout_ms: u64,

    /// Enable distributed processing
    pub enable_distributed: bool,

    /// Enable self-improvement
    pub enable_self_improvement: bool,

    /// Maximum memory usage in MB
    pub max_memory_mb: usize,

    /// Culture settings
    pub culture: CultureConfig,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            name: "SBMUMC".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            owner: "Samuel Benwellonedge Mukandara".to_string(),
            max_concurrent_operations: 100,
            default_timeout_ms: 30000,
            enable_distributed: false,
            enable_self_improvement: true,
            max_memory_mb: 8192,
            culture: CultureConfig::default(),
        }
    }
}

/// Culture configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CultureConfig {
    /// Default region
    pub default_region: String,

    /// Primary language
    pub primary_language: String,

    /// Supported regions
    pub supported_regions: Vec<String>,

    /// Cultural adaptation enabled
    pub enabled: bool,
}

impl Default for CultureConfig {
    fn default() -> Self {
        Self {
            default_region: "global".to_string(),
            primary_language: "en".to_string(),
            supported_regions: vec![
                "global".to_string(),
                "china".to_string(),
                "usa".to_string(),
                "europe".to_string(),
                "africa".to_string(),
                "asia".to_string(),
            ],
            enabled: true,
        }
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Security level
    pub level: SecurityLevel,

    /// Enable intrusion detection
    pub enable_intrusion_detection: bool,

    /// Enable self-healing
    pub enable_self_healing: bool,

    /// Maximum failed attempts before lockout
    pub max_failed_attempts: u32,

    /// Lockout duration in seconds
    pub lockout_duration_s: u64,

    /// Require multi-factor authentication
    pub require_mfa: bool,

    /// Encryption algorithm
    pub encryption_algorithm: String,

    /// Session timeout in seconds
    pub session_timeout_s: u64,

    /// Enable audit logging
    pub enable_audit_logging: bool,

    /// Whitelist of allowed operations
    pub allowed_operations: Vec<String>,

    /// Blacklist of blocked operations
    pub blocked_operations: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            level: SecurityLevel::Secret,
            enable_intrusion_detection: true,
            enable_self_healing: true,
            max_failed_attempts: 3,
            lockout_duration_s: 300,
            require_mfa: true,
            encryption_algorithm: "AES-256-GCM".to_string(),
            session_timeout_s: 3600,
            enable_audit_logging: true,
            allowed_operations: vec![],
            blocked_operations: vec![],
        }
    }
}

/// Knowledge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeConfig {
    /// Maximum concepts in memory
    pub max_concepts: usize,

    /// Enable concept pruning
    pub enable_pruning: bool,

    /// Pruning threshold (confidence below this is pruned)
    pub pruning_threshold: f64,

    /// Enable knowledge persistence
    pub enable_persistence: bool,

    /// Persistence directory
    pub persistence_dir: String,

    /// Enable knowledge sharing
    pub enable_sharing: bool,

    /// Maximum relationships per concept
    pub max_relationships: usize,

    /// Knowledge update interval in seconds
    pub update_interval_s: u64,

    /// Enable cross-domain knowledge
    pub enable_cross_domain: bool,
}

impl Default for KnowledgeConfig {
    fn default() -> Self {
        Self {
            max_concepts: 1_000_000,
            enable_pruning: true,
            pruning_threshold: 0.1,
            enable_persistence: true,
            persistence_dir: "data/knowledge".to_string(),
            enable_sharing: false,
            max_relationships: 1000,
            update_interval_s: 60,
            enable_cross_domain: true,
        }
    }
}

/// Learning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    /// Enable meta-learning
    pub enable_meta_learning: bool,

    /// Enable active learning
    pub enable_active_learning: bool,

    /// Enable self-supervised learning
    pub enable_self_supervised: bool,

    /// Learning rate
    pub learning_rate: f64,

    /// Batch size
    pub batch_size: usize,

    /// Maximum training iterations
    pub max_iterations: usize,

    /// Enable transfer learning
    pub enable_transfer_learning: bool,

    /// Curiosity coefficient
    pub curiosity_coefficient: f64,

    /// Exploration rate
    pub exploration_rate: f64,

    /// Memory buffer size
    pub memory_buffer_size: usize,

    /// Enable continual learning
    pub enable_continual_learning: bool,

    /// Learning from human feedback
    pub enable_human_feedback: bool,
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            enable_meta_learning: true,
            enable_active_learning: true,
            enable_self_supervised: true,
            learning_rate: 0.001,
            batch_size: 32,
            max_iterations: 10000,
            enable_transfer_learning: true,
            curiosity_coefficient: 0.1,
            exploration_rate: 0.1,
            memory_buffer_size: 10000,
            enable_continual_learning: true,
            enable_human_feedback: true,
        }
    }
}

/// Language configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    /// Primary language
    pub primary_language: String,

    /// Supported languages
    pub supported_languages: Vec<String>,

    /// Enable multilingual support
    pub enable_multilingual: bool,

    /// Enable translation
    pub enable_translation: bool,

    /// NLP model type
    pub nlp_model_type: String,

    /// Maximum context length
    pub max_context_length: usize,

    /// Enable emotion detection
    pub enable_emotion_detection: bool,

    /// Enable sarcasm detection
    pub enable_sarcasm_detection: bool,

    /// Enable intent recognition
    pub enable_intent_recognition: bool,

    /// Enable entity extraction
    pub enable_entity_extraction: bool,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            primary_language: "en".to_string(),
            supported_languages: vec![
                "en".to_string(),
                "zh".to_string(),
                "es".to_string(),
                "fr".to_string(),
                "de".to_string(),
                "ja".to_string(),
                "ko".to_string(),
                "ar".to_string(),
                "hi".to_string(),
                "pt".to_string(),
            ],
            enable_multilingual: true,
            enable_translation: true,
            nlp_model_type: "transformer".to_string(),
            max_context_length: 4096,
            enable_emotion_detection: true,
            enable_sarcasm_detection: true,
            enable_intent_recognition: true,
            enable_entity_extraction: true,
        }
    }
}

/// Compiler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
    /// Enable meta-compilation
    pub enable_meta_compilation: bool,

    /// Enable grammar compilation
    pub enable_grammar_compilation: bool,

    /// Enable language compilation
    pub enable_language_compilation: bool,

    /// Enable framework compilation
    pub enable_framework_compilation: bool,

    /// Enable OS compilation
    pub enable_os_compilation: bool,

    /// Supported target architectures
    pub supported_targets: Vec<String>,

    /// Default optimization level
    pub optimization_level: usize,

    /// Enable parallel compilation
    pub enable_parallel: bool,

    /// Maximum parallel jobs
    pub max_parallel_jobs: usize,

    /// Enable IDE integration
    pub enable_ide_integration: bool,

    /// Output directory
    pub output_dir: String,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            enable_meta_compilation: true,
            enable_grammar_compilation: true,
            enable_language_compilation: true,
            enable_framework_compilation: true,
            enable_os_compilation: true,
            supported_targets: vec![
                "universal".to_string(),
                "x86_64".to_string(),
                "aarch64".to_string(),
                "wasm".to_string(),
                "llvm".to_string(),
            ],
            optimization_level: 2,
            enable_parallel: true,
            max_parallel_jobs: 8,
            enable_ide_integration: true,
            output_dir: "output".to_string(),
        }
    }
}

/// Admin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfig {
    /// Enable admin interface
    pub enable_admin_interface: bool,

    /// Admin port
    pub admin_port: u16,

    /// Admin host
    pub admin_host: String,

    /// Require authentication
    pub require_auth: bool,

    /// Allowed admin users
    pub allowed_users: Vec<String>,

    /// Enable document compilation
    pub enable_document_compilation: bool,

    /// Supported document formats
    pub supported_formats: Vec<String>,

    /// Enable file watching
    pub enable_file_watching: bool,

    /// Watched directories
    pub watched_dirs: Vec<String>,
}

impl Default for AdminConfig {
    fn default() -> Self {
        Self {
            enable_admin_interface: true,
            admin_port: 8080,
            admin_host: "127.0.0.1".to_string(),
            require_auth: true,
            allowed_users: vec!["admin".to_string()],
            enable_document_compilation: true,
            supported_formats: vec![
                "txt".to_string(),
                "md".to_string(),
                "pdf".to_string(),
                "docx".to_string(),
                "html".to_string(),
                "json".to_string(),
                "xml".to_string(),
                "yaml".to_string(),
            ],
            enable_file_watching: true,
            watched_dirs: vec!["admin/drops".to_string()],
        }
    }
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Enable network access
    pub enable_network: bool,

    /// Maximum connections
    pub max_connections: usize,

    /// Connection timeout in seconds
    pub connection_timeout_s: u64,

    /// Enable API access
    pub enable_api: bool,

    /// API prefix
    pub api_prefix: String,

    /// Enable WebSocket
    pub enable_websocket: bool,

    /// WebSocket port
    pub websocket_port: u16,

    /// Allowed domains
    pub allowed_domains: Vec<String>,

    /// Blocked domains
    pub blocked_domains: Vec<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            enable_network: true,
            max_connections: 100,
            connection_timeout_s: 30,
            enable_api: true,
            api_prefix: "/api/v1".to_string(),
            enable_websocket: true,
            websocket_port: 8081,
            allowed_domains: vec![],
            blocked_domains: vec![],
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,

    /// Log directory
    pub log_dir: String,

    /// Enable file logging
    pub enable_file_logging: bool,

    /// Enable console logging
    pub enable_console_logging: bool,

    /// Maximum log file size in MB
    pub max_file_size_mb: usize,

    /// Maximum number of log files
    pub max_log_files: usize,

    /// Include timestamp
    pub include_timestamp: bool,

    /// Include source location
    pub include_source_location: bool,

    /// Custom fields
    pub custom_fields: HashMap<String, String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            log_dir: "logs".to_string(),
            enable_file_logging: true,
            enable_console_logging: true,
            max_file_size_mb: 100,
            max_log_files: 10,
            include_timestamp: true,
            include_source_location: true,
            custom_fields: HashMap::new(),
        }
    }
}
