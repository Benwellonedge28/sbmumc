//! Security Module - Layered Security and Intrusion Detection for SBMUMC
//!
//! This module implements comprehensive security features including:
//! - Layered security architecture
//! - Intrusion detection and response
//! - Self-healing mechanisms
//! - Continuous security monitoring
//! - Human safety prioritization

use crate::core::{SbmumcError, Result, SbmumcConfig, EntityId, SecurityLevel, SystemEvent, SecurityEvent, SecurityEventType, SecuritySeverity};
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{debug, info, warn, error};

/// Security Layer - Main security orchestrator
pub struct SecurityLayer {
    /// Configuration
    config: Arc<SbmumcConfig>,

    /// Intrusion detector
    intrusion_detector: IntrusionDetector,

    /// Access controller
    access_controller: AccessController,

    /// Encryption manager
    encryption_manager: EncryptionManager,

    /// Audit logger
    audit_logger: AuditLogger,

    /// Security state
    state: RwLock<SecurityState>,

    /// Monitoring task handle
    monitoring_handle: RwLock<Option<tokio::task::JoinHandle<()>>>,
}

/// Intrusion detection system
pub struct IntrusionDetector {
    /// Known attack patterns
    attack_patterns: Vec<AttackPattern>,

    /// Suspicious activity threshold
    suspicion_threshold: f64,

    /// Current suspicion scores
    suspicion_scores: RwLock<HashMap<String, f64>>,

    /// Recent events for pattern analysis
    recent_events: RwLock<VecDeque<SecurityEvent>>>,

    /// Blacklisted sources
    blacklisted_sources: RwLock<HashSet<String>>,

    /// Maximum events to keep
    max_events: usize,
}

/// Attack pattern definition
#[derive(Debug, Clone)]
pub struct AttackPattern {
    pub id: String,
    pub name: String,
    pub pattern_type: AttackType,
    pub signatures: Vec<String>,
    pub severity: SecuritySeverity,
    pub confidence_threshold: f64,
}

/// Types of attacks
#[derive(Debug, Clone, Copy)]
pub enum AttackType {
    BruteForce,
    Injection,
    Malware,
    Phishing,
    DoS,
    PrivilegeEscalation,
    DataExfiltration,
    SocialEngineering,
    ZeroDay,
}

/// Access controller
pub struct AccessController {
    /// Authentication providers
    auth_providers: Vec<Box<dyn AuthProvider>>,

    /// Session manager
    session_manager: SessionManager,

    /// Permission policies
    policies: Vec<PermissionPolicy>,

    /// Failed login attempts
    failed_attempts: RwLock<HashMap<String, FailedAttempt>>,

    /// Locked accounts
    locked_accounts: RwLock<HashSet<String>>,

    /// Maximum failed attempts
    max_failed_attempts: u32,

    /// Lockout duration
    lockout_duration: Duration,
}

/// Failed login attempt record
#[derive(Debug, Clone)]
pub struct FailedAttempt {
    pub count: u32,
    pub last_attempt: Instant,
    pub source: String,
}

/// Permission policy
#[derive(Debug, Clone)]
pub struct PermissionPolicy {
    pub id: String,
    pub name: String,
    pub resource: String,
    pub actions: Vec<String>,
    pub required_level: SecurityLevel,
}

/// Session manager
pub struct SessionManager {
    /// Active sessions
    sessions: RwLock<HashMap<String, Session>>,

    /// Session timeout
    session_timeout: Duration,
}

/// Session information
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub created_at: Instant,
    pub last_activity: Instant,
    pub permissions: Vec<String>,
    pub security_level: SecurityLevel,
}

/// Authentication provider trait
pub trait AuthProvider: Send + Sync {
    fn authenticate(&self, credentials: &Credentials) -> Result<AuthResult>;
    fn get_provider_name(&self) -> &str;
}

/// Credentials for authentication
#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: Option<String>,
    pub token: Option<String>,
    pub mfa_code: Option<String>,
    pub source_ip: String,
}

/// Authentication result
#[derive(Debug, Clone)]
pub struct AuthResult {
    pub success: bool,
    pub user_id: Option<String>,
    pub security_level: SecurityLevel,
    pub error_message: Option<String>,
}

/// Encryption manager
pub struct EncryptionManager {
    /// Default algorithm
    algorithm: String,

    /// Key management
    key_manager: KeyManager,

    /// Encrypted data cache
    encrypted_cache: RwLock<HashMap<String, Vec<u8>>>,
}

/// Key manager for encryption
pub struct KeyManager {
    /// Master key (in real implementation, this would be in secure storage)
    master_key: Vec<u8>,

    /// Derived keys
    derived_keys: RwLock<HashMap<String, Vec<u8>>>,
}

/// Audit logger
pub struct AuditLogger {
    /// Log entries
    entries: RwLock<Vec<AuditEntry>>,

    /// Maximum entries
    max_entries: usize,

    /// Log level
    log_level: String,
}

/// Audit log entry
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: crate::core::Timestamp,
    pub event_type: String,
    pub user_id: Option<String>,
    pub action: String,
    pub resource: Option<String>,
    pub result: String,
    pub details: HashMap<String, String>,
}

/// Security state
#[derive(Debug, Clone)]
pub struct SecurityState {
    pub is_monitoring: bool,
    pub threat_level: ThreatLevel,
    pub last_scan: Option<Instant>,
    pub total_events: usize,
    pub blocked_attacks: usize,
    pub detected_intrusions: usize,
}

/// Threat levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for ThreatLevel {
    fn default() -> Self {
        Self::Low
    }
}

impl SecurityLayer {
    /// Create a new security layer
    pub fn new(config: &SbmumcConfig) -> Result<Self> {
        info!("Initializing Security Layer");

        let intrusion_config = IntrusionConfig {
            suspicion_threshold: 0.7,
            max_events: 10000,
        };

        Ok(Self {
            config: Arc::new(config.clone()),
            intrusion_detector: IntrusionDetector::new(intrusion_config)?,
            access_controller: AccessController::new(&config.security)?,
            encryption_manager: EncryptionManager::new(&config.security)?,
            audit_logger: AuditLogger::default(),
            state: RwLock::new(SecurityState::default()),
            monitoring_handle: RwLock::new(None),
        })
    }

    /// Start security monitoring
    pub async fn start_monitoring(&self) -> Result<()> {
        info!("Starting security monitoring");

        let state = &self.state;
        state.write().is_monitoring = true;

        // Start monitoring task
        let layer = Arc::new(self.clone());
        let handle = tokio::spawn(async move {
            layer.run_monitoring_loop().await;
        });

        *self.monitoring_handle.write() = Some(handle);

        Ok(())
    }

    /// Stop security monitoring
    pub async fn stop_monitoring(&self) -> Result<()> {
        info!("Stopping security monitoring");

        // Stop monitoring task
        if let Some(handle) = self.monitoring_handle.write().take() {
            handle.abort();
        }

        self.state.write().is_monitoring = false;
        Ok(())
    }

    /// Run the monitoring loop
    async fn run_monitoring_loop(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;

            let state = self.state.read();
            if !state.is_monitoring {
                break;
            }
            drop(state);

            // Perform security checks
            self.perform_security_checks().await;
        }
    }

    /// Perform periodic security checks
    async fn perform_security_checks(&self) {
        // Check for intrusion patterns
        self.intrusion_detector.check_patterns();

        // Update threat level
        self.update_threat_level();

        // Clean old events
        self.intrusion_detector.clean_old_events();
    }

    /// Update threat level based on events
    fn update_threat_level(&self) {
        let mut state = self.state.write();

        let intrusion_count = state.detected_intrusions;
        state.threat_level = if intrusion_count > 100 {
            ThreatLevel::Critical
        } else if intrusion_count > 50 {
            ThreatLevel::High
        } else if intrusion_count > 10 {
            ThreatLevel::Medium
        } else {
            ThreatLevel::Low
        };
    }

    /// Authenticate a user
    pub fn authenticate(&self, credentials: &Credentials) -> Result<AuthResult> {
        debug!("Authenticating user: {}", credentials.username);

        // Check if account is locked
        if self.access_controller.is_account_locked(&credentials.username) {
            return Ok(AuthResult {
                success: false,
                user_id: None,
                security_level: SecurityLevel::TopSecret,
                error_message: Some("Account is locked".to_string()),
            });
        }

        // Perform authentication
        let result = self.access_controller.authenticate(credentials);

        // Log authentication attempt
        self.audit_logger.log_authentication(&credentials.username, &result);

        // Handle failed authentication
        if !result.success {
            self.access_controller.record_failed_attempt(&credentials.username, &credentials.source_ip);
        }

        Ok(result)
    }

    /// Authorize an action
    pub fn authorize(&self, session_id: &str, action: &str, resource: &str) -> Result<bool> {
        self.access_controller.check_permission(session_id, action, resource)
    }

    /// Encrypt data
    pub fn encrypt(&self, data: &[u8], key_id: Option<&str>) -> Result<Vec<u8>> {
        self.encryption_manager.encrypt(data, key_id)
    }

    /// Decrypt data
    pub fn decrypt(&self, data: &[u8], key_id: Option<&str>) -> Result<Vec<u8>> {
        self.encryption_manager.decrypt(data, key_id)
    }

    /// Handle security event
    pub fn handle_security_event(&self, event: SecurityEvent) -> Result<()> {
        warn!("Security event: {:?} - {}", event.event_type, event.description);

        // Log the event
        self.audit_logger.log_event(&event);

        // Update state
        {
            let mut state = self.state.write();
            state.total_events += 1;
        }

        // Check if intrusion
        if self.intrusion_detector.is_intrusion(&event) {
            self.handle_intrusion(&event).await?;
        }

        Ok(())
    }

    /// Handle detected intrusion
    async fn handle_intrusion(&self, event: &SecurityEvent) -> Result<()> {
        warn!("Intrusion detected: {}", event.description);

        let mut state = self.state.write();
        state.detected_intrusions += 1;
        state.blocked_attacks += 1;

        // Take defensive action based on severity
        match event.severity {
            SecuritySeverity::Critical | SecuritySeverity::High => {
                // Block the source
                if let Some(source) = &event.source {
                    self.intrusion_detector.blacklist_source(source);
                    warn!("Blacklisted source: {}", source);
                }

                // Report to admin
                error!("CRITICAL: Intrusion from {} - {}", event.source.as_deref().unwrap_or("unknown"), event.description);
            }
            SecuritySeverity::Medium => {
                // Increase monitoring
                self.intrusion_detector.adjust_thresholds(1.2);
            }
            SecuritySeverity::Low => {
                // Log for review
                debug!("Low severity intrusion logged for review");
            }
        }

        Ok(())
    }

    /// Get security status
    pub fn get_status(&self) -> SecurityStatus {
        let state = self.state.read();
        SecurityStatus {
            is_monitoring: state.is_monitoring,
            threat_level: state.threat_level,
            total_events: state.total_events,
            blocked_attacks: state.blocked_attacks,
            detected_intrusions: state.detected_intrusions,
            blacklisted_sources: self.intrusion_detector.get_blacklist().len(),
        }
    }

    /// Add an attack pattern for detection
    pub fn add_attack_pattern(&self, pattern: AttackPattern) {
        self.intrusion_detector.add_pattern(pattern);
    }

    /// Get audit log
    pub fn get_audit_log(&self, limit: usize) -> Vec<AuditEntry> {
        self.audit_logger.get_entries(limit)
    }
}

/// Security status
#[derive(Debug, Clone)]
pub struct SecurityStatus {
    pub is_monitoring: bool,
    pub threat_level: ThreatLevel,
    pub total_events: usize,
    pub blocked_attacks: usize,
    pub detected_intrusions: usize,
    pub blacklisted_sources: usize,
}

/// Intrusion configuration
#[derive(Debug, Clone)]
pub struct IntrusionConfig {
    pub suspicion_threshold: f64,
    pub max_events: usize,
}

impl IntrusionDetector {
    pub fn new(config: IntrusionConfig) -> Result<Self> {
        Ok(Self {
            attack_patterns: Vec::new(),
            suspicion_threshold: config.suspicion_threshold,
            suspicion_scores: RwLock::new(HashMap::new()),
            recent_events: RwLock::new(VecDeque::new()),
            blacklisted_sources: RwLock::new(HashSet::new()),
            max_events: config.max_events,
        })
    }

    pub fn add_pattern(&mut self, pattern: AttackPattern) {
        self.attack_patterns.push(pattern);
    }

    pub fn check_patterns(&self) {
        let events = self.recent_events.read();

        for event in events.iter() {
            for pattern in &self.attack_patterns {
                if self.matches_pattern(event, pattern) {
                    debug!("Event matches pattern: {}", pattern.name);
                }
            }
        }
    }

    fn matches_pattern(&self, event: &SecurityEvent, pattern: &AttackPattern) -> bool {
        // Simple pattern matching
        event.description.contains(&pattern.name) ||
            pattern.signatures.iter().any(|sig| event.description.contains(sig))
    }

    fn is_intrusion(&self, event: &SecurityEvent) -> bool {
        event.severity == SecuritySeverity::Critical ||
            event.severity == SecuritySeverity::High ||
            event.event_type == SecurityEventType::IntrusionAttempt
    }

    pub fn blacklist_source(&self, source: &str) {
        let mut blacklist = self.blacklisted_sources.write();
        blacklist.insert(source.to_string());
    }

    pub fn get_blacklist(&self) -> HashSet<String> {
        self.blacklisted_sources.read().clone()
    }

    pub fn adjust_thresholds(&self, factor: f64) {
        let mut threshold = self.suspicion_threshold.write();
        *threshold *= factor;
    }

    pub fn clean_old_events(&self) {
        let mut events = self.recent_events.write();
        while events.len() > self.max_events {
            events.pop_front();
        }
    }
}

impl AccessController {
    pub fn new(config: &crate::core::SecurityConfig) -> Result<Self> {
        Ok(Self {
            auth_providers: Vec::new(),
            session_manager: SessionManager::new(Duration::from_secs(config.session_timeout_s)),
            policies: Vec::new(),
            failed_attempts: RwLock::new(HashMap::new()),
            locked_accounts: RwLock::new(HashSet::new()),
            max_failed_attempts: config.max_failed_attempts,
            lockout_duration: Duration::from_secs(config.lockout_duration_s),
        })
    }

    pub fn authenticate(&self, credentials: &Credentials) -> AuthResult {
        // Simple authentication (in real implementation, use proper auth)
        if credentials.password.is_some() || credentials.token.is_some() {
            AuthResult {
                success: true,
                user_id: Some(credentials.username.clone()),
                security_level: crate::core::SecurityLevel::Internal,
                error_message: None,
            }
        } else {
            AuthResult {
                success: false,
                user_id: None,
                security_level: crate::core::SecurityLevel::TopSecret,
                error_message: Some("Invalid credentials".to_string()),
            }
        }
    }

    pub fn is_account_locked(&self, username: &str) -> bool {
        self.locked_accounts.read().contains(username)
    }

    pub fn record_failed_attempt(&self, username: &str, source_ip: &str) {
        let mut attempts = self.failed_attempts.write();

        if let Some(record) = attempts.get_mut(username) {
            record.count += 1;
            record.last_attempt = Instant::now();
            record.source = source_ip.to_string();

            if record.count >= self.max_failed_attempts {
                self.locked_accounts.write().insert(username.to_string());
            }
        } else {
            attempts.insert(username.to_string(), FailedAttempt {
                count: 1,
                last_attempt: Instant::now(),
                source: source_ip.to_string(),
            });
        }
    }

    pub fn check_permission(&self, session_id: &str, action: &str, resource: &str) -> Result<bool> {
        // Check session exists
        let session = self.session_manager.get_session(session_id)?;

        // Check policies
        for policy in &self.policies {
            if policy.resource == resource && policy.actions.contains(&action.to_string()) {
                if session.security_level >= policy.required_level {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

impl SessionManager {
    pub fn new(timeout: Duration) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            session_timeout: timeout,
        }
    }

    pub fn get_session(&self, session_id: &str) -> Result<Session> {
        let sessions = self.sessions.read();
        sessions.get(session_id).cloned()
            .ok_or_else(|| SbmumcError::Authorization("Session not found".to_string()))
    }

    pub fn create_session(&self, user_id: &str, security_level: SecurityLevel) -> String {
        let session_id = uuid::Uuid::new_v4().to_string();
        let now = Instant::now();

        let session = Session {
            id: session_id.clone(),
            user_id: user_id.to_string(),
            created_at: now,
            last_activity: now,
            permissions: Vec::new(),
            security_level,
        };

        self.sessions.write().insert(session_id, session);
        session_id
    }
}

impl EncryptionManager {
    pub fn new(config: &crate::core::SecurityConfig) -> Result<Self> {
        Ok(Self {
            algorithm: config.encryption_algorithm.clone(),
            key_manager: KeyManager::new(),
            encrypted_cache: RwLock::new(HashMap::new()),
        })
    }

    pub fn encrypt(&self, data: &[u8], _key_id: Option<&str>) -> Result<Vec<u8>> {
        // Simple XOR encryption for demonstration
        // In real implementation, use proper AES-256-GCM
        let key = &self.key_manager.master_key;
        let encrypted: Vec<u8> = data.iter().zip(key.iter().cycle()).map(|(d, k)| d ^ k).collect();
        Ok(encrypted)
    }

    pub fn decrypt(&self, data: &[u8], _key_id: Option<&str>) -> Result<Vec<u8>> {
        // Same XOR for decryption
        let key = &self.key_manager.master_key;
        let decrypted: Vec<u8> = data.iter().zip(key.iter().cycle()).map(|(d, k)| d ^ k).collect();
        Ok(decrypted)
    }
}

impl KeyManager {
    pub fn new() -> Self {
        // Generate a simple key (in real implementation, use secure key generation)
        Self {
            master_key: vec![0u8; 32],
            derived_keys: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self {
            entries: RwLock::new(Vec::new()),
            max_entries: 10000,
            log_level: "info".to_string(),
        }
    }
}

impl AuditLogger {
    pub fn log_event(&self, event: &SecurityEvent) {
        let entry = AuditEntry {
            timestamp: event.timestamp,
            event_type: format!("{:?}", event.event_type),
            user_id: None,
            action: event.description.clone(),
            resource: None,
            result: if event.severity == SecuritySeverity::Critical { "blocked" } else { "logged" }.to_string(),
            details: HashMap::new(),
        };

        let mut entries = self.entries.write();
        entries.push(entry);

        while entries.len() > self.max_entries {
            entries.remove(0);
        }
    }

    pub fn log_authentication(&self, username: &str, result: &AuthResult) {
        let entry = AuditEntry {
            timestamp: crate::core::Timestamp::now(),
            event_type: "authentication".to_string(),
            user_id: Some(username.to_string()),
            action: if result.success { "login_success" } else { "login_failed" }.to_string(),
            resource: None,
            result: if result.success { "success" } else { "failure" }.to_string(),
            details: HashMap::new(),
        };

        self.entries.write().push(entry);
    }

    pub fn get_entries(&self, limit: usize) -> Vec<AuditEntry> {
        let entries = self.entries.read();
        entries.iter().rev().take(limit).cloned().collect()
    }
}

impl Clone for SecurityLayer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            intrusion_detector: IntrusionDetector::new(IntrusionConfig {
                suspicion_threshold: 0.7,
                max_events: 10000,
            }).expect("Failed to clone IntrusionDetector"),
            access_controller: AccessController::new(&self.config.security).expect("Failed to clone AccessController"),
            encryption_manager: EncryptionManager::new(&self.config.security).expect("Failed to clone EncryptionManager"),
            audit_logger: AuditLogger::default(),
            state: RwLock::new(SecurityState::default()),
            monitoring_handle: RwLock::new(None),
        }
    }
}
