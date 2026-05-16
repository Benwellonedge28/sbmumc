//!
//! # SBMUMC Module 1582: External System Integration
//!
//! Integration adapters for external systems including cloud providers,
//! third-party APIs, and enterprise software.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Integration adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationAdapter {
    pub id: String,
    pub name: String,
    pub adapter_type: AdapterType,
    pub config: AdapterConfig,
    pub auth: IntegrationAuth,
    pub status: IntegrationStatus,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Adapter types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AdapterType {
    CloudProvider,
    CRM,
    ERP,
    Communication,
    Payment,
    Analytics,
    Storage,
    Security,
    Monitoring,
    Custom,
}

/// Adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    pub endpoint: String,
    pub timeout_secs: u32,
    pub retry_count: u32,
    pub retry_delay_ms: u64,
    pub custom_settings: HashMap<String, serde_json::Value>,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationAuth {
    pub auth_type: AuthType,
    pub credentials: Option<AuthCredentials>,
    pub oauth_config: Option<OAuthConfig>,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthType {
    None,
    Basic,
    APIKey,
    OAuth2,
    JWT,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentials {
    pub username: Option<String>,
    pub password: Option<String>,
}

/// OAuth configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

/// Integration status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IntegrationStatus {
    Active,
    Inactive,
    Error,
    Pending,
}

/// Cloud provider integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudIntegration {
    pub provider: CloudProvider,
    pub regions: Vec<String>,
    pub services: Vec<CloudService>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CloudProvider {
    AWS,
    Azure,
    GCP,
    DigitalOcean,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CloudService {
    Compute,
    Storage,
    Database,
    Networking,
    AI_ML,
    Analytics,
    Security,
}

/// CRM integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CRMIntegration {
    pub crm_type: CRMType,
    pub sync_contacts: bool,
    pub sync_deals: bool,
    pub sync_activities: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CRMType {
    Salesforce,
    HubSpot,
    Zoho,
    Pipedrive,
    Custom,
}

/// Communication integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationIntegration {
    pub comm_type: CommType,
    pub channels: Vec<CommChannel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommType {
    Email,
    SMS,
    Chat,
    Video,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommChannel {
    Email,
    SMS,
    Slack,
    Teams,
    Discord,
    WhatsApp,
    Custom,
}

/// Payment integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentIntegration {
    pub payment_provider: PaymentProvider,
    pub currencies: Vec<String>,
    pub payment_methods: Vec<PaymentMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentProvider {
    Stripe,
    PayPal,
    Square,
    Braintree,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    BankTransfer,
    PayPal,
    Crypto,
}

/// Integration event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationEvent {
    pub id: String,
    pub adapter_id: String,
    pub event_type: EventType,
    pub payload: serde_json::Value,
    pub timestamp: u64,
    pub status: EventStatus,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    Created,
    Updated,
    Deleted,
    Synced,
    Error,
    Webhook,
}

/// Event status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventStatus {
    Pending,
    Processed,
    Failed,
    Retrying,
}

/// Integration sync job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncJob {
    pub id: String,
    pub adapter_id: String,
    pub sync_type: SyncType,
    pub direction: SyncDirection,
    pub status: SyncStatus,
    pub records_synced: usize,
    pub errors: Vec<SyncError>,
    pub started_at: u64,
    pub completed_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncType {
    Full,
    Incremental,
    Delta,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncDirection {
    Import,
    Export,
    Bidirectional,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    Pending,
    Running,
    Completed,
    Failed,
    PartiallyFailed,
}

/// Sync error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncError {
    pub record_id: String,
    pub error_message: String,
    pub timestamp: u64,
}

/// External API client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAPIClient {
    pub id: String,
    pub name: String,
    pub base_url: String,
    pub auth: IntegrationAuth,
    pub rate_limit: RateLimit,
    pub timeout_secs: u32,
}

/// Rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_second: u32,
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
}

/// Integration service
pub struct IntegrationService {
    adapters: Arc<RwLock<HashMap<String, IntegrationAdapter>>>,
    events: Arc<RwLock<Vec<IntegrationEvent>>>,
    sync_jobs: Arc<RwLock<HashMap<String, SyncJob>>>,
    webhooks: Arc<RwLock<HashMap<String, WebhookConfig>>>,
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub id: String,
    pub adapter_id: String,
    pub url: String,
    pub events: Vec<String>,
    pub secret: Option<String>,
    pub retry_policy: RetryPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

impl IntegrationService {
    pub fn new() -> Self {
        Self {
            adapters: Arc::new(RwLock::new(HashMap::new())),
            events: Arc::new(RwLock::new(Vec::new())),
            sync_jobs: Arc::new(RwLock::new(HashMap::new())),
            webhooks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register adapter
    pub fn register_adapter(&self, adapter: IntegrationAdapter) -> Result<String, IntegrationError> {
        let mut adapters = self.adapters.write().unwrap();

        if adapters.contains_key(&adapter.id) {
            return Err(IntegrationError::AdapterAlreadyExists);
        }

        adapters.insert(adapter.id.clone(), adapter);
        Ok(adapter.id)
    }

    /// Update adapter
    pub fn update_adapter(&self, adapter_id: &str, updates: AdapterUpdates) -> Result<(), IntegrationError> {
        let mut adapters = self.adapters.write().unwrap();

        if let Some(adapter) = adapters.get_mut(adapter_id) {
            if let Some(config) = updates.config {
                adapter.config = config;
            }
            if let Some(auth) = updates.auth {
                adapter.auth = auth;
            }
            if let Some(status) = updates.status {
                adapter.status = status;
            }
            adapter.updated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            Ok(())
        } else {
            Err(IntegrationError::AdapterNotFound)
        }
    }

    /// Test adapter connection
    pub async fn test_connection(&self, adapter_id: &str) -> Result<ConnectionTest, IntegrationError> {
        let adapters = self.adapters.read().unwrap();
        let adapter = adapters.get(adapter_id)
            .ok_or(IntegrationError::AdapterNotFound)?
            .clone();
        drop(adapters);

        // Simulated connection test
        Ok(ConnectionTest {
            success: true,
            latency_ms: 50,
            message: "Connection successful".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        })
    }

    /// Call external API
    pub async fn call_api(&self, client_id: &str, endpoint: &str, method: &str, body: Option<serde_json::Value>) -> Result<APIResponse, IntegrationError> {
        // Simulated API call
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // In production, would make actual HTTP request
        let response = APIResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: Some(serde_json::json!({ "status": "ok" })),
            took_ms: 100,
        };

        // Log event
        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let event = IntegrationEvent {
            id: Uuid::new_v4().to_string(),
            adapter_id: client_id.to_string(),
            event_type: EventType::Synced,
            payload: serde_json::json!({
                "endpoint": endpoint,
                "method": method
            }),
            timestamp: end,
            status: EventStatus::Processed,
            retry_count: 0,
        };

        let mut events = self.events.write().unwrap();
        events.push(event);

        Ok(response)
    }

    /// Create sync job
    pub fn create_sync_job(&self, job: SyncJob) -> String {
        let mut jobs = self.sync_jobs.write().unwrap();
        jobs.insert(job.id.clone(), job.clone());
        job.id
    }

    /// Execute sync job
    pub async fn execute_sync(&self, job_id: &str) -> Result<SyncJob, IntegrationError> {
        let mut jobs = self.sync_jobs.write().unwrap();

        if let Some(job) = jobs.get_mut(job_id) {
            job.status = SyncStatus::Running;

            // Simulate sync operation
            job.records_synced = 100;
            job.status = SyncStatus::Completed;
            job.completed_at = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
            );

            Ok(job.clone())
        } else {
            Err(IntegrationError::SyncJobNotFound)
        }
    }

    /// Register webhook
    pub fn register_webhook(&self, webhook: WebhookConfig) -> String {
        let mut webhooks = self.webhooks.write().unwrap();
        webhooks.insert(webhook.id.clone(), webhook.clone());
        webhook.id
    }

    /// Trigger webhook
    pub async fn trigger_webhook(&self, webhook_id: &str, payload: serde_json::Value) -> Result<(), IntegrationError> {
        let webhooks = self.webhooks.read().unwrap();
        let webhook = webhooks.get(webhook_id)
            .ok_or(IntegrationError::WebhookNotFound)?
            .clone();
        drop(webhooks);

        // In production, would make HTTP request to webhook URL
        println!("Triggering webhook {} with payload: {}", webhook.url, payload);

        Ok(())
    }

    /// Get adapter status
    pub fn get_adapter_status(&self, adapter_id: &str) -> Option<IntegrationStatus> {
        let adapters = self.adapters.read().unwrap();
        adapters.get(adapter_id).map(|a| a.status.clone())
    }

    /// List adapters
    pub fn list_adapters(&self) -> Vec<IntegrationAdapter> {
        let adapters = self.adapters.read().unwrap();
        adapters.values().cloned().collect()
    }

    /// Get events
    pub fn get_events(&self, adapter_id: &str, limit: usize) -> Vec<IntegrationEvent> {
        let events = self.events.read().unwrap();
        events
            .iter()
            .filter(|e| e.adapter_id == adapter_id)
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }
}

/// Adapter updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterUpdates {
    pub config: Option<AdapterConfig>,
    pub auth: Option<IntegrationAuth>,
    pub status: Option<IntegrationStatus>,
}

/// Connection test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionTest {
    pub success: bool,
    pub latency_ms: u64,
    pub message: String,
    pub timestamp: u64,
}

/// API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
    pub took_ms: u64,
}

/// Integration error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationError {
    AdapterNotFound,
    AdapterAlreadyExists,
    ConnectionFailed,
    AuthenticationFailed,
    RateLimitExceeded,
    SyncJobNotFound,
    WebhookNotFound,
    APIError(String),
}

impl std::fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegrationError::AdapterNotFound => write!(f, "Adapter not found"),
            IntegrationError::AdapterAlreadyExists => write!(f, "Adapter already exists"),
            IntegrationError::ConnectionFailed => write!(f, "Connection failed"),
            IntegrationError::AuthenticationFailed => write!(f, "Authentication failed"),
            IntegrationError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            IntegrationError::SyncJobNotFound => write!(f, "Sync job not found"),
            IntegrationError::WebhookNotFound => write!(f, "Webhook not found"),
            IntegrationError::APIError(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl std::error::Error for IntegrationError {}

// Re-export types
pub use IntegrationAdapter;
pub use IntegrationEvent;
pub use SyncJob;
pub use CloudIntegration;
pub use CRMIntegration;
pub use CommunicationIntegration;
pub use PaymentIntegration;
pub use IntegrationService;