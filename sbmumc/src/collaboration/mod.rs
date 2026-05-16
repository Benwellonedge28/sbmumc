//! # SBMUMC Module 1561: Real-Time Collaboration System
//!
//! Enables real-time collaborative development with presence awareness,
//! cursor tracking, shared editing sessions, and team coordination.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a user in a collaborative session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collaborator {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub cursor_position: CursorPosition,
    pub selection: Option<TextSelection>,
    pub status: CollaboratorStatus,
    pub joined_at: u64,
    pub last_active: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPosition {
    pub file_path: String,
    pub line: u32,
    pub column: u32,
    pub viewport_top: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSelection {
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CollaboratorStatus {
    Online,
    Away,
    Busy,
    Offline,
}

/// A collaborative editing session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub id: String,
    pub name: String,
    pub project_path: String,
    pub created_by: String,
    pub created_at: u64,
    pub collaborators: Vec<Collaborator>,
    pub settings: SessionSettings,
    pub permissions: HashMap<String, PermissionLevel>,
}

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSettings {
    pub allow_guest_access: bool,
    pub max_collaborators: u32,
    pub require_approval: bool,
    pub enable_voice_chat: bool,
    pub enable_screen_sharing: bool,
    pub auto_save_interval_secs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionLevel {
    Owner,
    Admin,
    Editor,
    Commenter,
    Viewer,
}

/// Collaboration change operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationChange {
    pub id: String,
    pub session_id: String,
    pub user_id: String,
    pub file_path: String,
    pub operation: ChangeOperation,
    pub timestamp: u64,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeOperation {
    Insert { position: Position, text: String },
    Delete { start: Position, end: Position },
    Replace { start: Position, end: Position, text: String },
    Format { range: Range, format_type: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolution {
    LastWriteWins,
    Merge,
    Manual,
    OperationalTransform,
}

/// Collaboration event for real-time updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationEvent {
    pub event_type: EventType,
    pub session_id: String,
    pub user_id: String,
    pub data: serde_json::Value,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    UserJoined,
    UserLeft,
    CursorMoved,
    SelectionChanged,
    TextChanged,
    FileOpened,
    FileClosed,
    CommentAdded,
    CommentResolved,
    ReviewRequested,
    ApprovalGranted,
}

/// Real-time presence tracker
pub struct PresenceTracker {
    sessions: Arc<RwLock<HashMap<String, CollaborationSession>>>,
    changes: Arc<RwLock<Vec<CollaborationChange>>>,
    conflict_resolution: ConflictResolution,
}

impl PresenceTracker {
    pub fn new(conflict_resolution: ConflictResolution) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            changes: Arc::new(RwLock::new(Vec::new())),
            conflict_resolution,
        }
    }

    /// Create a new collaboration session
    pub fn create_session(&self, name: String, project_path: String, user_id: String) -> String {
        let session_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let session = CollaborationSession {
            id: session_id.clone(),
            name,
            project_path,
            created_by: user_id.clone(),
            created_at: timestamp,
            collaborators: vec![],
            settings: SessionSettings {
                allow_guest_access: false,
                max_collaborators: 10,
                require_approval: false,
                enable_voice_chat: false,
                enable_screen_sharing: false,
                auto_save_interval_secs: 30,
            },
            permissions: HashMap::new(),
        };

        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(session_id.clone(), session);

        session_id
    }

    /// Join an existing session
    pub fn join_session(&self, session_id: &str, user: Collaborator) -> Result<(), CollaborationError> {
        let mut sessions = self.sessions.write().unwrap();

        if let Some(session) = sessions.get_mut(session_id) {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            let mut collaborator = user;
            collaborator.joined_at = timestamp;
            collaborator.last_active = timestamp;

            session.collaborators.push(user);

            Ok(())
        } else {
            Err(CollaborationError::SessionNotFound)
        }
    }

    /// Leave a session
    pub fn leave_session(&self, session_id: &str, user_id: &str) -> Result<(), CollaborationError> {
        let mut sessions = self.sessions.write().unwrap();

        if let Some(session) = sessions.get_mut(session_id) {
            session.collaborators.retain(|c| c.id != user_id);
            Ok(())
        } else {
            Err(CollaborationError::SessionNotFound)
        }
    }

    /// Update cursor position
    pub fn update_cursor(&self, session_id: &str, user_id: &str, position: CursorPosition) -> Result<(), CollaborationError> {
        let mut sessions = self.sessions.write().unwrap();

        if let Some(session) = sessions.get_mut(session_id) {
            if let Some(collaborator) = session.collaborators.iter_mut().find(|c| c.id == user_id) {
                collaborator.cursor_position = position;
                collaborator.last_active = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
                Ok(())
            } else {
                Err(CollaborationError::UserNotFound)
            }
        } else {
            Err(CollaborationError::SessionNotFound)
        }
    }

    /// Record a text change
    pub fn record_change(&self, session_id: &str, user_id: &str, file_path: String, operation: ChangeOperation) -> String {
        let change_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let change = CollaborationChange {
            id: change_id.clone(),
            session_id: session_id.to_string(),
            user_id: user_id.to_string(),
            file_path,
            operation,
            timestamp,
            version: 1,
        };

        let mut changes = self.changes.write().unwrap();
        changes.push(change);

        change_id
    }

    /// Get active collaborators in a session
    pub fn get_active_collaborators(&self, session_id: &str) -> Result<Vec<Collaborator>, CollaborationError> {
        let sessions = self.sessions.read().unwrap();

        if let Some(session) = sessions.get(session_id) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            let timeout = 60000; // 60 seconds

            let active: Vec<Collaborator> = session.collaborators
                .iter()
                .filter(|c| now - c.last_active < timeout)
                .cloned()
                .collect();

            Ok(active)
        } else {
            Err(CollaborationError::SessionNotFound)
        }
    }

    /// Resolve conflicts between changes
    pub fn resolve_conflicts(&self, changes: Vec<CollaborationChange>) -> Vec<CollaborationChange> {
        match self.conflict_resolution {
            ConflictResolution::LastWriteWins => {
                let mut sorted = changes;
                sorted.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                sorted
            }
            ConflictResolution::OperationalTransform => {
                // Apply operational transform algorithm
                self.apply_operational_transform(changes)
            }
            ConflictResolution::Merge | ConflictResolution::Manual => {
                changes
            }
        }
    }

    fn apply_operational_transform(&self, changes: Vec<CollaborationChange>) -> Vec<CollaborationChange> {
        // Simplified OT implementation
        changes
    }

    /// Broadcast session event
    pub fn broadcast_event(&self, session_id: &str, event: CollaborationEvent) -> Result<(), CollaborationError> {
        let sessions = self.sessions.read().unwrap();

        if sessions.contains_key(session_id) {
            // In real implementation, this would broadcast via WebSocket
            Ok(())
        } else {
            Err(CollaborationError::SessionNotFound)
        }
    }

    /// Get session history
    pub fn get_session_history(&self, session_id: &str, limit: usize) -> Result<Vec<CollaborationChange>, CollaborationError> {
        let changes = self.changes.read().unwrap();

        let filtered: Vec<CollaborationChange> = changes
            .iter()
            .filter(|c| c.session_id == session_id)
            .rev()
            .take(limit)
            .cloned()
            .collect();

        Ok(filtered)
    }
}

/// Error types for collaboration operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollaborationError {
    SessionNotFound,
    UserNotFound,
    PermissionDenied,
    ConflictDetected,
    InvalidOperation,
}

impl std::fmt::Display for CollaborationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollaborationError::SessionNotFound => write!(f, "Session not found"),
            CollaborationError::UserNotFound => write!(f, "User not found in session"),
            CollaborationError::PermissionDenied => write!(f, "Permission denied"),
            CollaborationError::ConflictDetected => write!(f, "Conflict detected in changes"),
            CollaborationError::InvalidOperation => write!(f, "Invalid operation"),
        }
    }
}

impl std::error::Error for CollaborationError {}

/// Collaboration service for managing sessions
pub struct CollaborationService {
    presence_tracker: Arc<PresenceTracker>,
    max_sessions: usize,
    default_permissions: HashMap<String, PermissionLevel>,
}

impl CollaborationService {
    pub fn new(max_sessions: usize) -> Self {
        let mut default = HashMap::new();
        default.insert("owner".to_string(), PermissionLevel::Owner);
        default.insert("admin".to_string(), PermissionLevel::Admin);

        Self {
            presence_tracker: Arc::new(PresenceTracker::new(ConflictResolution::OperationalTransform)),
            max_sessions,
            default_permissions: default,
        }
    }

    /// Start a new collaboration session
    pub fn start_session(&self, name: String, project_path: String, user_id: String) -> String {
        self.presence_tracker.create_session(name, project_path, user_id)
    }

    /// Get session info
    pub fn get_session(&self, session_id: &str) -> Option<CollaborationSession> {
        let sessions = self.presence_tracker.sessions.read().unwrap();
        sessions.get(session_id).cloned()
    }

    /// List all active sessions
    pub fn list_sessions(&self) -> Vec<CollaborationSession> {
        let sessions = self.presence_tracker.sessions.read().unwrap();
        sessions.values().cloned().collect()
    }

    /// End a session
    pub fn end_session(&self, session_id: &str) -> Result<(), CollaborationError> {
        let mut sessions = self.presence_tracker.sessions.write().unwrap();
        sessions.remove(session_id);
        Ok(())
    }
}

// Re-export types
pub use Collaborator;
pub use CollaborationSession;
pub use CollaborationChange;
pub use CollaborationEvent;
pub use SessionSettings;
pub use PermissionLevel;
pub use PresenceTracker;
pub use CollaborationService;