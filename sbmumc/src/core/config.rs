//! # SBMUMC Configuration System
//!
//! Centralized configuration management for online/offline operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbmumcConfig {
    pub system: SystemConfig,
    pub omnidev: OmniDevConfig,
    pub network: NetworkConfig,
    pub security: SecurityConfig,
    pub storage: StorageConfig,
    pub plugins: PluginConfig,
    pub logging: LoggingConfig,
    pub features: FeatureFlags,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub name: String,
    pub version: String,
    pub mode: OperationMode,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub log_level: String,
    pub max_memory_mb: usize,
    pub threads: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationMode {
    Online,
    Offline,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmniDevConfig {
    pub enabled: bool,
    pub semantic_graph_path: PathBuf,
    pub max_context_nodes: usize,
    pub latency_target_ms: u64,
    pub enable_formal_verification: bool,
    pub enable_evas_filter: bool,
    pub enable_audit_trail: bool,
    pub incremental_indexing: bool,
    pub parallel_workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub mode: NetworkMode,
    pub api_endpoint: Option<String>,
    pub ws_endpoint: Option<String>,
    pub timeout_seconds: u64,
    pub retry_attempts: usize,
    pub cache_enabled: bool,
    pub cache_ttl_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMode {
    Online,
    Offline,
    Cached,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub encryption_enabled: bool,
    pub audit_trail_immutable: bool,
    pub human_override_required: bool,
    pub max_failed_attempts: usize,
    pub session_timeout_minutes: u64,
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub backend: StorageBackend,
    pub path: PathBuf,
    pub max_size_mb: usize,
    pub auto_cleanup: bool,
    pub compression: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackend {
    Memory,
    File,
    Hybrid,
    Distributed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    pub plugin_dir: PathBuf,
    pub sandbox_enabled: bool,
    pub allowed_signatures: Vec<String>,
    pub auto_load: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file_path: Option<PathBuf>,
    pub max_size_mb: usize,
    pub rotation: String,
    pub structured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub enable_quantum: bool,
    pub enable_biological: bool,
    pub enable_exoplanetary: bool,
    pub enable_consciousness_transfer: bool,
    pub enable_temporal: bool,
}

impl SbmumcConfig {
    pub fn default() -> Self {
        Self {
            system: SystemConfig {
                name: "SBMUMC".to_string(),
                version: "1.0.0".to_string(),
                mode: OperationMode::Hybrid,
                data_dir: PathBuf::from("/var/sbmumc/data"),
                cache_dir: PathBuf::from("/var/sbmumc/cache"),
                log_level: "info".to_string(),
                max_memory_mb: 4096,
                threads: 16,
            },
            omnidev: OmniDevConfig {
                enabled: true,
                semantic_graph_path: PathBuf::from("/var/sbmumc/graph"),
                max_context_nodes: 1000,
                latency_target_ms: 100,
                enable_formal_verification: true,
                enable_evas_filter: true,
                enable_audit_trail: true,
                incremental_indexing: true,
                parallel_workers: 16,
            },
            network: NetworkConfig {
                mode: NetworkMode::Hybrid,
                api_endpoint: Some("https://api.sbmumc.local".to_string()),
                ws_endpoint: Some("wss://ws.sbmumc.local".to_string()),
                timeout_seconds: 30,
                retry_attempts: 3,
                cache_enabled: true,
                cache_ttl_seconds: 3600,
            },
            security: SecurityConfig {
                encryption_enabled: true,
                audit_trail_immutable: true,
                human_override_required: false,
                max_failed_attempts: 5,
                session_timeout_minutes: 60,
                allowed_origins: vec!["*".to_string()],
            },
            storage: StorageConfig {
                backend: StorageBackend::Hybrid,
                path: PathBuf::from("/var/sbmumc/storage"),
                max_size_mb: 10240,
                auto_cleanup: true,
                compression: true,
            },
            plugins: PluginConfig {
                enabled: true,
                plugin_dir: PathBuf::from("/var/sbmumc/plugins"),
                sandbox_enabled: true,
                allowed_signatures: vec![],
                auto_load: true,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file_path: Some(PathBuf::from("/var/sbmumc/logs/sbmumc.log")),
                max_size_mb: 100,
                rotation: "daily".to_string(),
                structured: true,
            },
            features: FeatureFlags {
                enable_quantum: false,
                enable_biological: false,
                enable_exoplanetary: false,
                enable_consciousness_transfer: false,
                enable_temporal: false,
            },
        }
    }

    pub fn offline() -> Self {
        let mut config = Self::default();
        config.system.mode = OperationMode::Offline;
        config.network.mode = NetworkMode::Offline;
        config.network.api_endpoint = None;
        config.network.ws_endpoint = None;
        config
    }

    pub fn online() -> Self {
        let mut config = Self::default();
        config.system.mode = OperationMode::Online;
        config.network.mode = NetworkMode::Online;
        config
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)
            .map_err(|e| ConfigError::IoError(e.to_string()))?;

        serde_yaml::from_str(&content)
            .map_err(|e| ConfigError::ParseError(e.to_string()))
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), ConfigError> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| ConfigError::SerializeError(e.to_string()))?;

        fs::write(path, content)
            .map_err(|e| ConfigError::IoError(e.to_string()))
    }

    pub fn to_env_vars(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        vars.insert("SBMUMC_MODE".to_string(), format!("{:?}", self.system.mode));
        vars.insert("SBMUMC_DATA_DIR".to_string(), self.system.data_dir.to_string_lossy().to_string());
        vars.insert("SBMUMC_LOG_LEVEL".to_string(), self.system.log_level.clone());
        vars.insert("SBMUMC_OMNIVDEV_ENABLED".to_string(), self.omnidev.enabled.to_string());
        vars.insert("SBMUMC_LATENCY_TARGET_MS".to_string(), self.omnidev.latency_target_ms.to_string());
        vars.insert("SBMUMC_NETWORK_MODE".to_string(), format!("{:?}", self.network.mode));
        vars.insert("SBMUMC_ENCRYPTION".to_string(), self.security.encryption_enabled.to_string());
        vars
    }

    pub fn from_env_vars() -> Self {
        let mut config = Self::default();

        if let Ok(mode) = std::env::var("SBMUMC_MODE") {
            config.system.mode = match mode.as_str() {
                "Online" => OperationMode::Online,
                "Offline" => OperationMode::Offline,
                _ => OperationMode::Hybrid,
            };
        }

        if let Ok(data_dir) = std::env::var("SBMUMC_DATA_DIR") {
            config.system.data_dir = PathBuf::from(data_dir);
        }

        if let Ok(log_level) = std::env::var("SBMUMC_LOG_LEVEL") {
            config.system.log_level = log_level;
        }

        if let Ok(enabled) = std::env::var("SBMUMC_OMNIVDEV_ENABLED") {
            config.omnidev.enabled = enabled == "true";
        }

        config
    }

    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if self.omnidev.latency_target_ms == 0 {
            errors.push(ValidationError {
                field: "omnidev.latency_target_ms".to_string(),
                message: "Latency target must be greater than 0".to_string(),
            });
        }

        if self.system.max_memory_mb < 256 {
            errors.push(ValidationError {
                field: "system.max_memory_mb".to_string(),
                message: "Minimum memory allocation is 256MB".to_string(),
            });
        }

        if self.network.timeout_seconds == 0 {
            errors.push(ValidationError {
                field: "network.timeout_seconds".to_string(),
                message: "Network timeout must be greater than 0".to_string(),
            });
        }

        errors
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigError {
    pub kind: ConfigErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigErrorKind {
    IoError,
    ParseError,
    SerializeError,
    ValidationError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = SbmumcConfig::default();
        assert_eq!(config.system.name, "SBMUMC");
        assert!(config.omnidev.enabled);
    }

    #[test]
    fn test_offline_config() {
        let config = SbmumcConfig::offline();
        assert!(matches!(config.system.mode, OperationMode::Offline));
        assert!(matches!(config.network.mode, NetworkMode::Offline));
        assert!(config.network.api_endpoint.is_none());
    }

    #[test]
    fn test_config_validation() {
        let config = SbmumcConfig::default();
        let errors = config.validate();
        assert!(errors.is_empty());
    }

    #[test]
    fn test_env_vars() {
        let config = SbmumcConfig::default();
        let vars = config.to_env_vars();
        assert!(vars.contains_key("SBMUMC_MODE"));
    }
}
