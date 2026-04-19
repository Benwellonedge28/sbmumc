//! Error Types for SBMUMC
//!
//! Comprehensive error handling for the SBMUMC system.

use thiserror::Error;
use std::fmt;

/// Result type alias for SBMUMC operations
pub type Result<T> = std::result::Result<T, SbmumcError>;

/// Main error type for SBMUMC
#[derive(Debug, Error)]
pub enum SbmumcError {
    // =========================================================================
    // System Errors
    // =========================================================================

    /// System is already running
    #[error("System is already running")]
    SystemAlreadyRunning,

    /// System is not running
    #[error("System is not running")]
    SystemNotRunning,

    /// Shutdown failed
    #[error("Failed to shutdown system")]
    ShutdownFailed,

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Initialization failed
    #[error("Initialization failed: {0}")]
    Initialization(String),

    // =========================================================================
    // Knowledge & Reasoning Errors
    // =========================================================================

    /// Knowledge graph error
    #[error("Knowledge error: {0}")]
    Knowledge(String),

    /// Concept not found
    #[error("Concept not found: {0}")]
    ConceptNotFound(String),

    /// Reasoning error
    #[error("Reasoning error: {0}")]
    Reasoning(String),

    /// Planning error
    #[error("Planning error: {0}")]
    Planning(String),

    // =========================================================================
    // Learning Errors
    // =========================================================================

    /// Learning error
    #[error("Learning error: {0}")]
    Learning(String),

    /// Training failed
    #[error("Training failed: {0}")]
    TrainingFailed(String),

    /// Model error
    #[error("Model error: {0}")]
    Model(String),

    // =========================================================================
    // Security Errors
    // =========================================================================

    /// Security error
    #[error("Security error: {0}")]
    Security(String),

    /// Authentication failed
    #[error("Authentication failed")]
    AuthenticationFailed,

    /// Authorization failed
    #[error("Authorization failed: {0}")]
    Authorization(String),

    /// Intrusion detected
    #[error("Intrusion detected: {0}")]
    IntrusionDetected(String),

    /// Validation failed
    #[error("Validation failed: {0}")]
    Validation(String),

    // =========================================================================
    // Input/Output Errors
    // =========================================================================

    /// Input error
    #[error("Input error: {0}")]
    Input(String),

    /// Output error
    #[error("Output error: {0}")]
    Output(String),

    /// Parsing error
    #[error("Parsing error: {0}")]
    Parsing(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    // =========================================================================
    // Language & NLP Errors
    // =========================================================================

    /// Language error
    #[error("Language error: {0}")]
    Language(String),

    /// Translation error
    #[error("Translation error: {0}")]
    Translation(String),

    /// Parsing language error
    #[error("Language parsing error: {0}")]
    LanguageParsing(String),

    /// NLP error
    #[error("NLP error: {0}")]
    Nlp(String),

    // =========================================================================
    // Compilation Errors
    // =========================================================================

    /// Compilation error
    #[error("Compilation error: {0}")]
    Compilation(String),

    /// Grammar error
    #[error("Grammar error: {0}")]
    Grammar(String),

    /// Lexical error
    #[error("Lexical error: {0}")]
    Lexical(String),

    /// Syntax error
    #[error("Syntax error: {0}")]
    Syntax(String),

    /// Semantic error
    #[error("Semantic error: {0}")]
    Semantic(String),

    /// Code generation error
    #[error("Code generation error: {0}")]
    CodeGeneration(String),

    /// Target not supported
    #[error("Target not supported: {0}")]
    TargetNotSupported(String),

    // =========================================================================
    // Admin Errors
    // =========================================================================

    /// Admin error
    #[error("Admin error: {0}")]
    Admin(String),

    /// Document compilation error
    #[error("Document compilation error: {0}")]
    DocumentCompilation(String),

    /// Document format not supported
    #[error("Document format not supported: {0}")]
    DocumentFormatNotSupported(String),

    // =========================================================================
    // Communication Errors
    // =========================================================================

    /// Channel error
    #[error("Channel error: {0}")]
    Channel(String),

    /// Timeout error
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    // =========================================================================
    // Storage Errors
    // =========================================================================

    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),

    /// File not found
    #[error("File not found: {0}")]
    FileNotFound(String),

    /// File access denied
    #[error("File access denied: {0}")]
    FileAccessDenied(String),

    // =========================================================================
    // Generic Errors
    // =========================================================================

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Not implemented
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Invalid state
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Resource exhausted
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),
}

impl SbmumcError {
    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::Timeout(_)
                | Self::Validation(_)
                | Self::TrainingFailed(_)
                | Self::Connection(_)
        )
    }

    /// Check if this is a critical error
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            Self::IntrusionDetected(_)
                | Self::Security(_)
                | Self::Internal(_)
                | Self::ShutdownFailed
        )
    }

    /// Get error category
    pub fn category(&self) -> &'static str {
        match self {
            // System
            Self::SystemAlreadyRunning | Self::SystemNotRunning | Self::ShutdownFailed | Self::Configuration(_) | Self::Initialization(_) => "system",

            // Knowledge
            Self::Knowledge(_) | Self::ConceptNotFound(_) | Self::Reasoning(_) | Self::Planning(_) => "knowledge",

            // Learning
            Self::Learning(_) | Self::TrainingFailed(_) | Self::Model(_) => "learning",

            // Security
            Self::Security(_) | Self::AuthenticationFailed | Self::Authorization(_) | Self::IntrusionDetected(_) | Self::Validation(_) => "security",

            // IO
            Self::Input(_) | Self::Output(_) | Self::Parsing(_) | Self::Serialization(_) => "io",

            // Language
            Self::Language(_) | Self::Translation(_) | Self::LanguageParsing(_) | Self::Nlp(_) => "language",

            // Compilation
            Self::Compilation(_) | Self::Grammar(_) | Self::Lexical(_) | Self::Syntax(_) | Self::Semantic(_) | Self::CodeGeneration(_) | Self::TargetNotSupported(_) => "compilation",

            // Admin
            Self::Admin(_) | Self::DocumentCompilation(_) | Self::DocumentFormatNotSupported(_) => "admin",

            // Communication
            Self::Channel(_) | Self::Timeout(_) | Self::Connection(_) => "communication",

            // Storage
            Self::Storage(_) | Self::FileNotFound(_) | Self::FileAccessDenied(_) => "storage",

            // Generic
            Self::Internal(_) | Self::NotImplemented(_) | Self::InvalidState(_) | Self::ResourceExhausted(_) => "generic",
        }
    }
}

impl fmt::Display for SbmumcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.category(), self)
    }
}

/// Error context for adding additional information to errors
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub code: String,
    pub message: String,
    pub source: Option<Box<SbmumcError>>,
    pub context: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            source: None,
            context: std::collections::HashMap::new(),
        }
    }

    pub fn with_source(mut self, source: SbmumcError) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }
}

/// Convert any error to SbmumcError
impl From<std::io::Error> for SbmumcError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Self::FileNotFound(err.to_string()),
            std::io::ErrorKind::PermissionDenied => Self::FileAccessDenied(err.to_string()),
            _ => Self::Internal(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for SbmumcError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}

impl From<tokio::task::JoinError> for SbmumcError {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::Internal(err.to_string())
    }
}
