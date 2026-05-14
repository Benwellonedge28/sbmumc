//! # SBMUMC Plugin Architecture
//!
//! Dynamic plugin loading with sandboxed execution

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManager {
    pub plugins: HashMap<String, PluginInfo>,
    pub plugin_dir: PathBuf,
    pub sandbox_enabled: bool,
    pub auto_load: bool,
    pub loaded_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub dependencies: Vec<String>,
    pub permissions: Vec<Permission>,
    pub enabled: bool,
    pub loaded_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub name: PermissionName,
    pub level: PermissionLevel,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionName {
    FileSystem,
    Network,
    Process,
    Memory,
    GraphAccess,
    CodeExecution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionLevel {
    None,
    ReadOnly,
    ReadWrite,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub manifest_version: String,
    pub plugin: PluginMetadata,
    pub permissions: Vec<PermissionSpec>,
    pub entry_point: String,
    pub hooks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub repository: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSpec {
    pub name: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContext {
    pub plugin_id: String,
    pub data_dir: PathBuf,
    pub config: HashMap<String, String>,
    pub capabilities: Vec<String>,
}

impl PluginManager {
    pub fn new(plugin_dir: PathBuf) -> Self {
        Self {
            plugins: HashMap::new(),
            plugin_dir,
            sandbox_enabled: true,
            auto_load: true,
            loaded_count: 0,
        }
    }

    pub fn load_plugin(&mut self, manifest: PluginManifest) -> Result<(), PluginError> {
        // Validate permissions
        if self.sandbox_enabled {
            self.validate_permissions(&manifest.permissions)?;
        }

        let plugin_id = manifest.plugin.id.clone();
        let plugin_info = PluginInfo {
            id: manifest.plugin.id,
            name: manifest.plugin.name,
            version: manifest.plugin.version,
            description: manifest.plugin.description,
            author: manifest.plugin.author,
            dependencies: vec![],
            permissions: self.create_permissions(&manifest.permissions),
            enabled: true,
            loaded_at: Some(chrono::Utc::now().timestamp()),
        };

        self.plugins.insert(plugin_id.clone(), plugin_info);
        self.loaded_count += 1;

        Ok(())
    }

    fn validate_permissions(&self, permissions: &[PermissionSpec]) -> Result<(), PluginError> {
        for perm in permissions {
            if perm.required {
                match perm.name.as_str() {
                    "filesystem" | "network" | "code_execution" => {
                        // In sandbox mode, require explicit approval
                        return Err(PluginError::PermissionDenied(perm.name.clone()));
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn create_permissions(&self, specs: &[PermissionSpec]) -> Vec<Permission> {
        specs.iter().map(|spec| {
            Permission {
                name: match spec.name.as_str() {
                    "filesystem" => PermissionName::FileSystem,
                    "network" => PermissionName::Network,
                    "process" => PermissionName::Process,
                    "memory" => PermissionName::Memory,
                    "graph_access" => PermissionName::GraphAccess,
                    "code_execution" => PermissionName::CodeExecution,
                    _ => PermissionName::FileSystem,
                },
                level: PermissionLevel::ReadOnly,
                description: spec.description.clone(),
            }
        }).collect()
    }

    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        if let Some(mut info) = self.plugins.remove(plugin_id) {
            info.enabled = false;
            self.loaded_count = self.loaded_count.saturating_sub(1);
            Ok(())
        } else {
            Err(PluginError::NotFound(plugin_id.to_string()))
        }
    }

    pub fn enable_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.enabled = true;
            Ok(())
        } else {
            Err(PluginError::NotFound(plugin_id.to_string()))
        }
    }

    pub fn disable_plugin(&mut self, plugin_id: &str) -> Result<(), PluginError> {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.enabled = false;
            Ok(())
        } else {
            Err(PluginError::NotFound(plugin_id.to_string()))
        }
    }

    pub fn list_plugins(&self) -> Vec<&PluginInfo> {
        self.plugins.values().collect()
    }

    pub fn get_plugin(&self, plugin_id: &str) -> Option<&PluginInfo> {
        self.plugins.get(plugin_id)
    }

    pub fn create_context(&self, plugin_id: &str) -> Result<PluginContext, PluginError> {
        if let Some(plugin) = self.plugins.get(plugin_id) {
            Ok(PluginContext {
                plugin_id: plugin.id.clone(),
                data_dir: self.plugin_dir.join(&plugin.id),
                config: HashMap::new(),
                capabilities: plugin.permissions.iter()
                    .map(|p| format!("{:?}", p.name))
                    .collect(),
            })
        } else {
            Err(PluginError::NotFound(plugin_id.to_string()))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginError {
    NotFound(String),
    PermissionDenied(String),
    LoadFailed(String),
    ValidationError(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::NotFound(id) => write!(f, "Plugin not found: {}", id),
            PluginError::PermissionDenied(name) => write!(f, "Permission denied: {}", name),
            PluginError::LoadFailed(msg) => write!(f, "Plugin load failed: {}", msg),
            PluginError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for PluginError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new(PathBuf::from("/plugins"));
        assert_eq!(manager.loaded_count, 0);
    }

    #[test]
    fn test_plugin_loading() {
        let mut manager = PluginManager::new(PathBuf::from("/plugins"));
        let manifest = PluginManifest {
            manifest_version: "1.0".to_string(),
            plugin: PluginMetadata {
                id: "test-plugin".to_string(),
                name: "Test Plugin".to_string(),
                version: "1.0.0".to_string(),
                description: "A test plugin".to_string(),
                author: "Test Author".to_string(),
                license: "MIT".to_string(),
                repository: None,
            },
            permissions: vec![],
            entry_point: "main".to_string(),
            hooks: vec![],
        };

        let result = manager.load_plugin(manifest);
        assert!(result.is_ok());
        assert_eq!(manager.loaded_count, 1);
    }
}
