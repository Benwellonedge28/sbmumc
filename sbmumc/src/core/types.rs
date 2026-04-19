//! Core Types for SBMUMC
//!
//! Fundamental types used throughout the SBMUMC system.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Unique identifier for entities in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(Uuid);

impl EntityId {
    /// Create a new unique entity ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create an entity ID from a UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID
    pub fn uuid(&self) -> Uuid {
        self.0
    }

    /// Create from string representation
    pub fn parse(s: &str) -> Option<Self> {
        Uuid::parse_str(s).ok().map(Self)
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Timestamp for tracking temporal information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    /// Get current timestamp
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Create from datetime
    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    /// Get underlying datetime
    pub fn datetime(&self) -> DateTime<Utc> {
        self.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

/// Represents a concept in the knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub id: EntityId,
    pub name: String,
    pub description: String,
    pub properties: HashMap<String, PropertyValue>,
    pub relationships: Vec<Relationship>,
    pub metadata: Metadata,
}

impl Concept {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            id: EntityId::new(),
            name: name.to_string(),
            description: description.to_string(),
            properties: HashMap::new(),
            relationships: Vec::new(),
            metadata: Metadata::default(),
        }
    }
}

/// Property value types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<PropertyValue>),
    Object(HashMap<String, PropertyValue>),
}

/// Represents a relationship between concepts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub target_id: EntityId,
    pub relation_type: RelationType,
    pub properties: HashMap<String, PropertyValue>,
    pub confidence: f64,
}

/// Types of relationships
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationType {
    IsA,
    PartOf,
    HasA,
    RelatedTo,
    Causes,
    Enables,
    Prevents,
    SimilarTo,
    OppositeOf,
    Custom(String),
}

impl RelationType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::IsA => "is_a",
            Self::PartOf => "part_of",
            Self::HasA => "has_a",
            Self::RelatedTo => "related_to",
            Self::Causes => "causes",
            Self::Enables => "enables",
            Self::Prevents => "prevents",
            Self::SimilarTo => "similar_to",
            Self::OppositeOf => "opposite_of",
            Self::Custom(s) => s,
        }
    }
}

/// Metadata for entities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metadata {
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub source: Option<String>,
    pub tags: Vec<String>,
    pub confidence: f64,
}

/// Represents a thought or processing unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thought {
    pub id: EntityId,
    pub content: ThoughtContent,
    pub reasoning_chain: Vec<ReasoningStep>,
    pub result: Option<ThoughtResult>,
    pub metadata: Metadata,
}

/// Content of a thought
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThoughtContent {
    Text(String),
    Structured(StructuredContent),
    Query(Query),
    Action(Action),
}

/// Structured content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredContent {
    pub format: ContentFormat,
    pub data: serde_json::Value,
}

/// Content formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentFormat {
    Json,
    Xml,
    Markdown,
    Html,
    Custom(String),
}

/// Query content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    pub query_type: QueryType,
    pub parameters: HashMap<String, PropertyValue>,
}

/// Query types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryType {
    Knowledge,
    Action,
    Explanation,
    Planning,
}

/// Action content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_type: ActionType,
    pub target: Option<EntityId>,
    pub parameters: HashMap<String, PropertyValue>,
}

/// Action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    Learn,
    Reason,
    Plan,
    Execute,
    Communicate,
    Compile,
    Analyze,
    Create,
    Modify,
    Delete,
}

/// Reasoning step in a chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    pub step_number: usize,
    pub premise: String,
    pub inference: String,
    pub confidence: f64,
}

/// Result of a thought
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtResult {
    pub success: bool,
    pub output: String,
    pub confidence: f64,
    pub explanation: Option<String>,
}

/// Context for processing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessingContext {
    pub request_id: EntityId,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub region: Option<String>,
    pub culture: Option<String>,
    pub language: Option<String>,
    pub preferences: HashMap<String, PropertyValue>,
}

/// Processing request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingRequest {
    pub input: Input,
    pub context: ProcessingContext,
    pub options: RequestOptions,
}

/// Input types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    Text(String),
    Voice(VoiceInput),
    Structured(serde_json::Value),
    File(FileInput),
}

/// Voice input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceInput {
    pub audio_data: Vec<u8>,
    pub format: String,
    pub language: Option<String>,
}

/// File input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInput {
    pub path: String,
    pub format: FileFormat,
    pub content: Option<Vec<u8>>,
}

/// File formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileFormat {
    Text,
    SourceCode,
    Grammar,
    Document,
    Image,
    Audio,
    Video,
    Data,
    Archive,
    Unknown,
}

/// Request options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestOptions {
    pub max_tokens: Option<usize>,
    pub temperature: Option<f64>,
    pub timeout_ms: Option<u64>,
    pub include_explanation: bool,
    pub include_reasoning: bool,
}

/// Processing response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResponse {
    pub request_id: EntityId,
    pub output: Output,
    pub reasoning: Option<Vec<ReasoningStep>>,
    pub metadata: ResponseMetadata,
}

/// Output types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Output {
    Text(String),
    Structured(serde_json::Value),
    File(FileOutput),
    Action(ActionResult),
}

/// File output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOutput {
    pub path: String,
    pub format: FileFormat,
    pub content: Vec<u8>,
}

/// Action result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub action_type: ActionType,
    pub success: bool,
    pub result_data: Option<serde_json::Value>,
    pub message: Option<String>,
}

/// Response metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub timestamp: Timestamp,
    pub processing_time_ms: u64,
    pub confidence: f64,
    pub warnings: Vec<String>,
}

/// Security level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    Public,
    Internal,
    Confidential,
    Secret,
    TopSecret,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::Internal
    }
}

/// Event types for the system
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SystemEvent {
    #[serde(rename = "processing_started")]
    ProcessingStarted { request_id: EntityId },
    #[serde(rename = "processing_completed")]
    ProcessingCompleted { request_id: EntityId, success: bool },
    #[serde(rename = "security_event")]
    SecurityEvent { event: SecurityEvent },
    #[serde(rename = "learning_event")]
    LearningEvent { event: LearningEvent },
    #[serde(rename = "compilation_event")]
    CompilationEvent { event: CompilationEvent },
}

/// Security events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_type: SecurityEventType,
    pub severity: SecuritySeverity,
    pub description: String,
    pub source: Option<String>,
    pub timestamp: Timestamp,
}

/// Security event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityEventType {
    IntrusionAttempt,
    UnauthorizedAccess,
    DataExfiltration,
    ManipulationAttempt,
    DenialOfService,
    VulnerabilityFound,
    PatchApplied,
}

/// Security severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Learning events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEvent {
    pub event_type: LearningEventType,
    pub description: String,
    pub improvement: Option<f64>,
}

/// Learning event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LearningEventType {
    ConceptLearned,
    PatternDiscovered,
    StrategyImproved,
    ModelUpdated,
    KnowledgeIntegrated,
}

/// Compilation events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationEvent {
    pub event_type: CompilationEventType,
    pub source: String,
    pub target: Option<String>,
    pub success: bool,
}

/// Compilation event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompilationEventType {
    CompilationStarted,
    CompilationCompleted,
    CompilationFailed,
    OptimizationApplied,
}
