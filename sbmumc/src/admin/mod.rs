//! Admin Module - Administration Interface for SBMUMC
//!
//! This module provides the administration interface for managing and modifying
//! SBMUMC through document compilation and direct configuration.

use crate::core::{SbmumcError, Result, EntityId, SecurityLevel, PropertyValue};
use crate::knowledge::KnowledgeGraph;
use crate::security::SecurityLayer;
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

/// Admin Interface - Main administration system
pub struct AdminInterface {
    /// Knowledge graph reference
    knowledge: Arc<KnowledgeGraph>,

    /// Security layer reference
    security: Arc<SecurityLayer>,

    /// Document compiler
    document_compiler: DocumentCompiler,

    /// Configuration manager
    config_manager: ConfigManager,

    /// Command history
    command_history: RwLock<Vec<AdminCommand>>,

    /// Active sessions
    active_sessions: RwLock<HashMap<String, AdminSession>>,

    /// File watcher
    file_watcher: Option<FileWatcher>,
}

/// Document compiler - compiles documents into SBMUMC modifications
pub struct DocumentCompiler {
    /// Supported formats
    supported_formats: HashSet<String>,

    /// Compiled patterns
    patterns: Vec<CompilationPattern>,

    /// Maximum file size
    max_file_size: usize,
}

/// Compilation pattern
#[derive(Debug, Clone)]
pub struct CompilationPattern {
    pub pattern: String,
    pub pattern_type: PatternType,
    pub action: CompilationAction,
}

/// Pattern types
#[derive(Debug, Clone, Copy)]
pub enum PatternType {
    Command,
    Configuration,
    Knowledge,
    Instruction,
}

/// Compilation action
#[derive(Debug, Clone)]
pub enum CompilationAction {
    AddConcept { name: String, description: String },
    UpdateProperty { key: String, value: PropertyValue },
    SetConfiguration { path: String, value: String },
    ExecuteCommand { command: String },
    AddKnowledge { domain: String, content: String },
}

/// Configuration manager
pub struct ConfigManager {
    /// Current configuration
    config: RwLock<HashMap<String, PropertyValue>>,

    /// Pending changes
    pending_changes: RwLock<Vec<ConfigChange>>,
}

/// Config change record
#[derive(Debug, Clone)]
pub struct ConfigChange {
    pub path: String,
    pub old_value: Option<PropertyValue>,
    pub new_value: PropertyValue,
    pub timestamp: crate::core::Timestamp,
    pub approved: bool,
}

/// Admin command record
#[derive(Debug, Clone)]
pub struct AdminCommand {
    pub id: EntityId,
    pub command: String,
    pub timestamp: crate::core::Timestamp,
    pub executed_by: String,
    pub result: CommandResult,
}

/// Command execution result
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub errors: Vec<String>,
}

/// Admin session
#[derive(Debug, Clone)]
pub struct AdminSession {
    pub id: String,
    pub user_id: String,
    pub created_at: crate::core::Timestamp,
    pub last_activity: crate::core::Timestamp,
    pub permissions: Vec<AdminPermission>,
    pub security_level: SecurityLevel,
}

/// Admin permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdminPermission {
    Read,
    Write,
    Execute,
    Configure,
    ManageUsers,
    ViewLogs,
    ModifySecurity,
    CompileDocuments,
}

/// File watcher for admin drops
pub struct FileWatcher {
    /// Watched paths
    watched_paths: HashSet<PathBuf>,

    /// Event channel
    event_tx: mpsc::Sender<WatchEvent>,

    /// Running state
    running: RwLock<bool>,
}

/// Watch event
#[derive(Debug, Clone)]
pub struct WatchEvent {
    pub path: PathBuf,
    pub event_type: WatchEventType,
    pub timestamp: crate::core::Timestamp,
}

/// Watch event types
#[derive(Debug, Clone, Copy)]
pub enum WatchEventType {
    Created,
    Modified,
    Deleted,
}

impl AdminInterface {
    /// Create a new admin interface
    pub async fn new(
        knowledge: Arc<KnowledgeGraph>,
        security: Arc<SecurityLayer>,
    ) -> Result<Self> {
        info!("Initializing Admin Interface");

        let document_compiler = DocumentCompiler::new()?;
        let config_manager = ConfigManager::new()?;

        // Initialize file watcher
        let file_watcher = None;

        Ok(Self {
            knowledge,
            security,
            document_compiler,
            config_manager,
            command_history: RwLock::new(Vec::new()),
            active_sessions: RwLock::new(HashMap::new()),
            file_watcher,
        })
    }

    /// Start the admin interface
    pub async fn start(&self) -> Result<()> {
        info!("Starting Admin Interface");

        // Start file watcher if configured
        self.start_file_watcher().await?;

        Ok(())
    }

    /// Stop the admin interface
    pub async fn stop(&self) -> Result<()> {
        info!("Stopping Admin Interface");

        // Stop file watcher
        if let Some(watcher) = &self.file_watcher {
            watcher.stop().await?;
        }

        Ok(())
    }

    /// Start file watcher
    async fn start_file_watcher(&self) -> Result<()> {
        let (tx, _rx) = mpsc::channel(100);

        let watcher = FileWatcher {
            watched_paths: HashSet::new(),
            event_tx: tx,
            running: RwLock::new(true),
        };

        // In a full implementation, this would use notify crate
        // to watch directories for new files

        Ok(())
    }

    /// Process an admin command
    pub fn execute_command(&self, command: &str, user_id: &str) -> Result<CommandResult> {
        debug!("Executing admin command: {}", command);

        // Record command
        let cmd_record = AdminCommand {
            id: EntityId::new(),
            command: command.to_string(),
            timestamp: crate::core::Timestamp::now(),
            executed_by: user_id.to_string(),
            result: CommandResult::default(),
        };

        // Parse command
        let result = self.parse_and_execute(command);

        // Update record
        let mut record = cmd_record;
        record.result = result.clone();

        // Add to history
        let mut history = self.command_history.write();
        history.push(record);

        Ok(result)
    }

    /// Parse and execute command
    fn parse_and_execute(&self, command: &str) -> CommandResult {
        let parts: Vec<&str> = command.split_whitespace().collect();

        if parts.is_empty() {
            return CommandResult {
                success: false,
                output: String::new(),
                errors: vec!["Empty command".to_string()],
            };
        }

        match parts[0] {
            "status" => self.cmd_status(),
            "stats" => self.cmd_stats(),
            "knowledge" => self.cmd_knowledge(&parts[1..]),
            "config" => self.cmd_config(&parts[1..]),
            "compile" => self.cmd_compile(&parts[1..]),
            "help" => self.cmd_help(),
            _ => CommandResult {
                success: false,
                output: String::new(),
                errors: vec![format!("Unknown command: {}", parts[0])],
            },
        }
    }

    /// Get status
    fn cmd_status(&self) -> CommandResult {
        let status = self.security.get_status();
        CommandResult {
            success: true,
            output: format!("Security Status: {:?}", status),
            errors: Vec::new(),
        }
    }

    /// Get statistics
    fn cmd_stats(&self) -> CommandResult {
        let concept_count = self.knowledge.concept_count();
        CommandResult {
            success: true,
            output: format!("Knowledge concepts: {}", concept_count),
            errors: Vec::new(),
        }
    }

    /// Knowledge command
    fn cmd_knowledge(&self, args: &[&str]) -> CommandResult {
        if args.is_empty() {
            return CommandResult {
                success: false,
                output: String::new(),
                errors: vec!["Usage: knowledge <add|list|search>".to_string()],
            };
        }

        match args[0] {
            "list" => {
                let count = self.knowledge.concept_count();
                CommandResult {
                    success: true,
                    output: format!("Total concepts: {}", count),
                    errors: Vec::new(),
                }
            }
            "add" => {
                if args.len() < 3 {
                    return CommandResult {
                        success: false,
                        output: String::new(),
                        errors: vec!["Usage: knowledge add <name> <description>".to_string()],
                    };
                }

                let concept = crate::knowledge::Concept {
                    id: EntityId::new(),
                    name: args[1].to_string(),
                    description: args[2..].join(" "),
                    concept_type: "admin_added".to_string(),
                    properties: HashMap::new(),
                    metadata: crate::core::Metadata::default(),
                };

                if let Err(e) = self.knowledge.add_concept(concept) {
                    CommandResult {
                        success: false,
                        output: String::new(),
                        errors: vec![format!("Failed to add concept: {}", e)],
                    }
                } else {
                    CommandResult {
                        success: true,
                        output: format!("Added concept: {}", args[1]),
                        errors: Vec::new(),
                    }
                }
            }
            _ => CommandResult {
                success: false,
                output: String::new(),
                errors: vec![format!("Unknown knowledge command: {}", args[0])],
            },
        }
    }

    /// Config command
    fn cmd_config(&self, args: &[&str]) -> CommandResult {
        if args.is_empty() {
            return CommandResult {
                success: false,
                output: String::new(),
                errors: vec!["Usage: config <get|set|list>".to_string()],
            };
        }

        match args[0] {
            "list" => {
                let configs = self.config_manager.list_all();
                CommandResult {
                    success: true,
                    output: format!("Configs: {:?}", configs),
                    errors: Vec::new(),
                }
            }
            "get" => {
                if args.len() < 2 {
                    return CommandResult {
                        success: false,
                        output: String::new(),
                        errors: vec!["Usage: config get <path>".to_string()],
                    };
                }

                match self.config_manager.get(args[1]) {
                    Some(value) => CommandResult {
                        success: true,
                        output: format!("{} = {:?}", args[1], value),
                        errors: Vec::new(),
                    },
                    None => CommandResult {
                        success: false,
                        output: String::new(),
                        errors: vec![format!("Config not found: {}", args[1])],
                    },
                }
            }
            "set" => {
                if args.len() < 3 {
                    return CommandResult {
                        success: false,
                        output: String::new(),
                        errors: vec!["Usage: config set <path> <value>".to_string()],
                    };
                }

                // Parse value (simplified)
                let value = PropertyValue::String(args[2..].join(" "));
                self.config_manager.set(args[1], value);

                CommandResult {
                    success: true,
                    output: format!("Set {} = {}", args[1], args[2]),
                    errors: Vec::new(),
                }
            }
            _ => CommandResult {
                success: false,
                output: String::new(),
                errors: vec![format!("Unknown config command: {}", args[0])],
            },
        }
    }

    /// Compile command
    fn cmd_compile(&self, args: &[&str]) -> CommandResult {
        if args.is_empty() {
            return CommandResult {
                success: false,
                output: String::new(),
                errors: vec!["Usage: compile <file>".to_string()],
            };
        }

        let path = args[0];
        match self.document_compiler.compile_file(path) {
            Ok(result) => CommandResult {
                success: true,
                output: format!("Compiled: {} - {} changes", path, result.changes_count),
                errors: Vec::new(),
            },
            Err(e) => CommandResult {
                success: false,
                output: String::new(),
                errors: vec![format!("Compilation failed: {}", e)],
            },
        }
    }

    /// Help command
    fn cmd_help(&self) -> CommandResult {
        let help_text = r#"
Available commands:
  status                 - Show system status
  stats                  - Show statistics
  knowledge add <name> <desc>  - Add a concept
  knowledge list         - List concepts
  config list            - List configurations
  config get <path>      - Get configuration value
  config set <path> <val> - Set configuration value
  compile <file>         - Compile document
  help                   - Show this help
"#;

        CommandResult {
            success: true,
            output: help_text.to_string(),
            errors: Vec::new(),
        }
    }

    /// Compile a document file
    pub fn compile_document(&self, path: &str) -> Result<CompilationResult> {
        debug!("Compiling document: {}", path);

        self.document_compiler.compile_file(path)
    }

    /// Compile document content directly
    pub fn compile_content(&self, content: &str, format: &str) -> Result<CompilationResult> {
        debug!("Compiling content: {} chars, format: {}", content.len(), format);

        self.document_compiler.compile_content(content, format)
    }

    /// Get command history
    pub fn get_history(&self, limit: usize) -> Vec<AdminCommand> {
        let history = self.command_history.read();
        history.iter().rev().take(limit).cloned().collect()
    }

    /// Create admin session
    pub fn create_session(&self, user_id: &str, security_level: SecurityLevel) -> String {
        let session_id = uuid::Uuid::new_v4().to_string();

        let session = AdminSession {
            id: session_id.clone(),
            user_id: user_id.to_string(),
            created_at: crate::core::Timestamp::now(),
            last_activity: crate::core::Timestamp::now(),
            permissions: vec![AdminPermission::Read],
            security_level,
        };

        self.active_sessions.write().insert(session_id.clone(), session);
        session_id
    }

    /// End admin session
    pub fn end_session(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.active_sessions.write();
        sessions.remove(session_id)
            .ok_or_else(|| SbmumcError::Admin("Session not found".to_string()))?;
        Ok(())
    }

    /// Check session permissions
    pub fn check_permission(&self, session_id: &str, permission: AdminPermission) -> Result<bool> {
        let sessions = self.active_sessions.read();
        let session = sessions.get(session_id)
            .ok_or_else(|| SbmumcError::Admin("Session not found".to_string()))?;

        Ok(session.permissions.contains(&permission))
    }
}

impl Default for CommandResult {
    fn default() -> Self {
        Self {
            success: false,
            output: String::new(),
            errors: Vec::new(),
        }
    }
}

impl DocumentCompiler {
    /// Create a new document compiler
    pub fn new() -> Result<Self> {
        info!("Initializing Document Compiler");

        let supported_formats = vec![
            "txt".to_string(),
            "md".to_string(),
            "json".to_string(),
            "yaml".to_string(),
            "yml".to_string(),
            "xml".to_string(),
        ].into_iter().collect();

        // Initialize default patterns
        let patterns = vec![
            CompilationPattern {
                pattern: r"ADD CONCEPT: (.+)".to_string(),
                pattern_type: PatternType::Knowledge,
                action: CompilationAction::AddConcept {
                    name: String::new(),
                    description: String::new(),
                },
            },
            CompilationPattern {
                pattern: r"SET CONFIG: (.+) = (.+)".to_string(),
                pattern_type: PatternType::Configuration,
                action: CompilationAction::SetConfiguration {
                    path: String::new(),
                    value: String::new(),
                },
            },
        ];

        Ok(Self {
            supported_formats,
            patterns,
            max_file_size: 10 * 1024 * 1024, // 10MB
        })
    }

    /// Compile a file
    pub fn compile_file(&self, path: &str) -> Result<CompilationResult> {
        let path = PathBuf::from(path);

        if !path.exists() {
            return Err(SbmumcError::FileNotFound(path.display().to_string()));
        }

        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        if !self.supported_formats.contains(&extension) {
            return Err(SbmumcError::DocumentFormatNotSupported(extension));
        }

        let content = std::fs::read_to_string(&path)?;
        self.compile_content(&content, &extension)
    }

    /// Compile content
    pub fn compile_content(&self, content: &str, format: &str) -> Result<CompilationResult> {
        debug!("Compiling {} content: {} chars", format, content.len());

        let mut changes = Vec::new();

        // Parse content based on format
        match format {
            "json" | "yaml" | "yml" => {
                // Structured format - parse as config
                for line in content.lines() {
                    if !line.trim().is_empty() && !line.trim().starts_with('#') {
                        changes.push(CompilationChange {
                            change_type: ChangeType::Configuration,
                            description: line.to_string(),
                            applied: true,
                        });
                    }
                }
            }
            "md" | "txt" => {
                // Text format - extract patterns
                for line in content.lines() {
                    let line = line.trim();
                    if line.starts_with("## ") || line.starts_with("### ") {
                        changes.push(CompilationChange {
                            change_type: ChangeType::Knowledge,
                            description: format!("Section: {}", line.trim_start_matches('#').trim()),
                            applied: true,
                        });
                    } else if line.to_uppercase().starts_with("ADD:") {
                        changes.push(CompilationChange {
                            change_type: ChangeType::Instruction,
                            description: line[4..].trim().to_string(),
                            applied: true,
                        });
                    }
                }
            }
            _ => {
                return Err(SbmumcError::DocumentFormatNotSupported(format.to_string()));
            }
        }

        Ok(CompilationResult {
            success: true,
            format: format.to_string(),
            changes_count: changes.len(),
            changes,
        })
    }

    /// Add a compilation pattern
    pub fn add_pattern(&mut self, pattern: CompilationPattern) {
        self.patterns.push(pattern);
    }
}

/// Compilation result
#[derive(Debug, Clone)]
pub struct CompilationResult {
    pub success: bool,
    pub format: String,
    pub changes_count: usize,
    pub changes: Vec<CompilationChange>,
}

/// Compilation change
#[derive(Debug, Clone)]
pub struct CompilationChange {
    pub change_type: ChangeType,
    pub description: String,
    pub applied: bool,
}

/// Change types
#[derive(Debug, Clone, Copy)]
pub enum ChangeType {
    Configuration,
    Knowledge,
    Instruction,
    Command,
}

impl ConfigManager {
    /// Create a new config manager
    pub fn new() -> Self {
        Self {
            config: RwLock::new(HashMap::new()),
            pending_changes: RwLock::new(Vec::new()),
        }
    }

    /// Get a configuration value
    pub fn get(&self, path: &str) -> Option<PropertyValue> {
        let config = self.config.read();
        config.get(path).cloned()
    }

    /// Set a configuration value
    pub fn set(&self, path: &str, value: PropertyValue) {
        let mut config = self.config.write();
        let old_value = config.get(path).cloned();

        config.insert(path.to_string(), value.clone());

        // Record change
        let change = ConfigChange {
            path: path.to_string(),
            old_value,
            new_value: value,
            timestamp: crate::core::Timestamp::now(),
            approved: false,
        };

        self.pending_changes.write().push(change);
    }

    /// List all configurations
    pub fn list_all(&self) -> HashMap<String, PropertyValue> {
        self.config.read().clone()
    }

    /// Approve a pending change
    pub fn approve_change(&self, path: &str) -> Result<()> {
        let mut changes = self.pending_changes.write();
        if let Some(change) = changes.iter_mut().find(|c| c.path == path) {
            change.approved = true;
            Ok(())
        } else {
            Err(SbmumcError::Admin("Pending change not found".to_string()))
        }
    }
}

impl FileWatcher {
    /// Stop watching
    pub async fn stop(&self) -> Result<()> {
        *self.running.write() = false;
        Ok(())
    }

    /// Add a path to watch
    pub fn watch(&mut self, path: PathBuf) {
        self.watched_paths.insert(path);
    }

    /// Remove a path from watch
    pub fn unwatch(&mut self, path: &PathBuf) {
        self.watched_paths.remove(path);
    }
}
