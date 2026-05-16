//! Module 1588: API Management & Rate Limiting
//!
//! Comprehensive API management including rate limiting, quota management,
//! API versioning, and request/response transformation.
//!
//! # Features
//!
//! - Rate Limiting - Token bucket, leaky bucket, sliding window
//! - Quota Management - Usage quotas per client/application
//! - API Versioning - Multi-version support with deprecation
//! - Request Transformation - Header/body manipulation
//! - API Analytics - Usage tracking and insights

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

/// Rate limiting algorithm types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RateLimitAlgorithm {
    TokenBucket,
    LeakyBucket,
    SlidingWindow,
    FixedWindow,
}

/// Rate limit tier levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RateLimitTier {
    Free,
    Basic,
    Pro,
    Enterprise,
    Unlimited,
}

/// Rate limit result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitResult {
    pub allowed: bool,
    pub remaining: u64,
    pub limit: u64,
    pub reset_at: u64,
    pub retry_after_ms: Option<u64>,
    pub quota_type: QuotaType,
}

/// Quota tracking types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuotaType {
    Requests,
    Bandwidth,
    Compute,
    Storage,
}

/// API rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub id: String,
    pub name: String,
    pub algorithm: RateLimitAlgorithm,
    pub requests_per_second: u64,
    pub requests_per_minute: Option<u64>,
    pub requests_per_hour: Option<u64>,
    pub requests_per_day: Option<u64>,
    pub burst_size: u64,
    pub key_extractors: Vec<KeyExtractor>,
    pub response_type: ResponseType,
    pub enabled: bool,
}

/// Key extraction for rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyExtractor {
    pub source: KeySource,
    pub field: String,
    pub transform: Option<String>,
}

/// Key source types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeySource {
    IpAddress,
    ApiKey,
    UserId,
    Header,
    QueryParam,
    JwtClaim,
}

/// Response type when limit exceeded
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResponseType {
    Http429,
    Http503,
    CustomBody,
    JsonError,
}

/// API key definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub key: String,
    pub name: String,
    pub client_id: String,
    pub scopes: Vec<String>,
    pub rate_limit_tier: RateLimitTier,
    pub quotas: HashMap<QuotaType, QuotaConfig>,
    pub metadata: HashMap<String, String>,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub last_used_at: Option<u64>,
    pub is_active: bool,
}

/// Quota configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaConfig {
    pub limit: u64,
    pub period: QuotaPeriod,
    pub reset_strategy: ResetStrategy,
}

/// Quota time periods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuotaPeriod {
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    Unlimited,
}

/// Quota reset strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResetStrategy {
    Rolling,
    Fixed,
    Calendar,
}

/// API version definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiVersion {
    pub id: String,
    pub version: String,
    pub status: VersionStatus,
    pub deprecated_at: Option<u64>,
    pub sunset_at: Option<u64>,
    pub migrations: Vec<Migration>,
    pub docs_url: Option<String>,
    pub created_at: u64,
}

/// Version lifecycle status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VersionStatus {
    Draft,
    Beta,
    Stable,
    Deprecated,
    Sunset,
}

/// Version migration path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Migration {
    pub from_version: String,
    pub to_version: String,
    pub steps: Vec<MigrationStep>,
    pub automatic: bool,
}

/// Migration step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStep {
    pub order: u32,
    pub description: String,
    pub transformation: Option<Transformation>,
    pub documentation: Option<String>,
}

/// Request/response transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transformation {
    pub transform_type: TransformType,
    pub field_path: String,
    pub operation: TransformOperation,
    pub value: Option<String>,
    pub conditions: Vec<Condition>,
}

/// Transformation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransformType {
    RequestHeader,
    ResponseHeader,
    RequestBody,
    ResponseBody,
    QueryParam,
    PathParam,
}

/// Transform operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransformOperation {
    Add,
    Remove,
    Rename,
    Modify,
    AddIfMissing,
    Convert,
}

/// Transformation condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub field: String,
    pub operator: String,
    pub value: String,
}

/// API endpoint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpoint {
    pub id: String,
    pub path: String,
    pub method: HttpMethod,
    pub version: String,
    pub handler: String,
    pub auth_required: bool,
    pub scopes: Vec<String>,
    pub rate_limit_config: Option<String>,
    pub transformations: Vec<Transformation>,
    pub response_schema: Option<String>,
    pub request_schema: Option<String>,
    pub tags: Vec<String>,
    pub summary: String,
    pub deprecated: bool,
}

/// HTTP methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

/// API usage analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiUsageAnalytics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub rate_limited_requests: u64,
    pub total_latency_ms: u64,
    pub avg_latency_ms: u64,
    pub p50_latency_ms: u64,
    pub p90_latency_ms: u64,
    pub p99_latency_ms: u64,
    pub bandwidth_bytes: u64,
    pub top_endpoints: Vec<EndpointUsage>,
    pub top_clients: Vec<ClientUsage>,
}

/// Endpoint usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointUsage {
    pub endpoint: String,
    pub method: HttpMethod,
    pub request_count: u64,
    pub avg_latency_ms: u64,
    pub error_rate: f64,
}

/// Client usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientUsage {
    pub client_id: String,
    pub request_count: u64,
    pub bandwidth_bytes: u64,
    pub error_count: u64,
}

/// Request context for rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestContext {
    pub request_id: String,
    pub ip_address: String,
    pub path: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub api_key: Option<String>,
    pub user_id: Option<String>,
    pub timestamp: u64,
}

/// API gateway state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiGatewayState {
    pub rate_limits: HashMap<String, RateLimitConfig>,
    pub api_keys: HashMap<String, ApiKey>,
    pub versions: HashMap<String, ApiVersion>,
    pub endpoints: HashMap<String, ApiEndpoint>,
    pub token_buckets: HashMap<String, TokenBucket>,
    pub usage_records: HashMap<String, VecDeque<UsageRecord>>,
}

/// Token bucket for rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBucket {
    pub key: String,
    pub tokens: f64,
    pub max_tokens: f64,
    pub refill_rate: f64,
    pub last_refill: u64,
}

/// Usage record for quota tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub timestamp: u64,
    pub value: u64,
    pub request_id: String,
}

/// API gateway service
pub struct ApiGatewayService {
    state: ApiGatewayState,
    analytics: ApiUsageAnalytics,
}

impl ApiGatewayService {
    /// Create new API gateway service
    pub fn new() -> Self {
        Self {
            state: ApiGatewayState::default(),
            analytics: ApiUsageAnalytics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                rate_limited_requests: 0,
                total_latency_ms: 0,
                avg_latency_ms: 0,
                p50_latency_ms: 0,
                p90_latency_ms: 0,
                p99_latency_ms: 0,
                bandwidth_bytes: 0,
                top_endpoints: Vec::new(),
                top_clients: Vec::new(),
            },
        }
    }

    /// Create rate limit configuration
    pub fn create_rate_limit(&mut self, config: RateLimitConfig) -> Result<String, ApiError> {
        let id = config.id.clone();
        self.state.rate_limits.insert(id.clone(), config);
        Ok(id)
    }

    /// Check rate limit for request
    pub fn check_rate_limit(&mut self, context: &RequestContext, limit_id: &str) -> Result<RateLimitResult, ApiError> {
        let config = self.state.rate_limits.get(limit_id)
            .ok_or(ApiError::RateLimitNotFound)?;

        let key = self.extract_key(context, &config.key_extractors);

        match config.algorithm {
            RateLimitAlgorithm::TokenBucket => {
                self.check_token_bucket(&key, config.requests_per_second, config.burst_size)
            }
            RateLimitAlgorithm::SlidingWindow => {
                self.check_sliding_window(&key, config.requests_per_minute.unwrap_or(60))
            }
            RateLimitAlgorithm::FixedWindow => {
                self.check_fixed_window(&key, config.requests_per_hour.unwrap_or(3600))
            }
            RateLimitAlgorithm::LeakyBucket => {
                self.check_leaky_bucket(&key, config.requests_per_second)
            }
        }
    }

    /// Extract rate limit key from request
    fn extract_key(&self, context: &RequestContext, extractors: &[KeyExtractor]) -> String {
        for extractor in extractors {
            let value = match extractor.source {
                KeySource::IpAddress => &context.ip_address,
                KeySource::ApiKey => context.api_key.as_deref().unwrap_or(""),
                KeySource::UserId => context.user_id.as_deref().unwrap_or(""),
                KeySource::Header => context.headers.get(&extractor.field).map(|s| s.as_str()).unwrap_or(""),
                KeySource::QueryParam => context.query_params.get(&extractor.field).map(|s| s.as_str()).unwrap_or(""),
                KeySource::JwtClaim => "",
            };

            if !value.is_empty() {
                return value.to_string();
            }
        }

        context.ip_address.clone()
    }

    /// Token bucket algorithm
    fn check_token_bucket(&mut self, key: &str, refill_rate: u64, max_tokens: u64) -> Result<RateLimitResult, ApiError> {
        let now = current_timestamp();
        let bucket_key = format!("token:{}", key);

        let bucket = self.state.token_buckets.entry(bucket_key.clone())
            .or_insert(TokenBucket {
                key: key.to_string(),
                tokens: max_tokens as f64,
                max_tokens: max_tokens as f64,
                refill_rate: refill_rate as f64,
                last_refill: now,
            });

        // Refill tokens
        let elapsed = (now - bucket.last_refill) as f64 / 1000.0;
        let new_tokens = (elapsed * bucket.refill_rate).min(bucket.max_tokens - bucket.tokens);
        bucket.tokens += new_tokens;
        bucket.last_refill = now;

        // Check if request allowed
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            Ok(RateLimitResult {
                allowed: true,
                remaining: bucket.tokens as u64,
                limit: max_tokens,
                reset_at: now + 1000,
                retry_after_ms: None,
                quota_type: QuotaType::Requests,
            })
        } else {
            let retry_after = (1000.0 / bucket.refill_rate) as u64;
            Ok(RateLimitResult {
                allowed: false,
                remaining: 0,
                limit: max_tokens,
                reset_at: now + retry_after,
                retry_after_ms: Some(retry_after),
                quota_type: QuotaType::Requests,
            })
        }
    }

    /// Sliding window algorithm
    fn check_sliding_window(&mut self, key: &str, window_requests: u64) -> Result<RateLimitResult, ApiError> {
        let now = current_timestamp();
        let window_ms = 60000; // 1 minute
        let key_name = format!("sliding:{}", key);

        let records = self.state.usage_records.entry(key_name)
            .or_insert_with(VecDeque::new);

        // Remove old records
        let cutoff = now - window_ms;
        while records.front().map(|r| r.timestamp < cutoff).unwrap_or(false) {
            records.pop_front();
        }

        let current_count = records.len() as u64;

        if current_count < window_requests {
            records.push_back(UsageRecord {
                timestamp: now,
                value: 1,
                request_id: format!("req_{}", now),
            });

            Ok(RateLimitResult {
                allowed: true,
                remaining: window_requests - current_count - 1,
                limit: window_requests,
                reset_at: cutoff + window_ms,
                retry_after_ms: None,
                quota_type: QuotaType::Requests,
            })
        } else {
            let oldest = records.front().map(|r| r.timestamp).unwrap_or(now);
            Ok(RateLimitResult {
                allowed: false,
                remaining: 0,
                limit: window_requests,
                reset_at: oldest + window_ms,
                retry_after_ms: Some(oldest + window_ms - now),
                quota_type: QuotaType::Requests,
            })
        }
    }

    /// Fixed window algorithm
    fn check_fixed_window(&mut self, key: &str, window_requests: u64) -> Result<RateLimitResult, ApiError> {
        let now = current_timestamp();
        let window_ms = 3600000; // 1 hour
        let key_name = format!("fixed:{}", key);

        let records = self.state.usage_records.entry(key_name)
            .or_insert_with(VecDeque::new);

        // Remove old records outside current window
        let window_start = (now / window_ms) * window_ms;
        let cutoff = window_start;
        while records.front().map(|r| r.timestamp < cutoff).unwrap_or(false) {
            records.pop_front();
        }

        let current_count = records.len() as u64;

        if current_count < window_requests {
            records.push_back(UsageRecord {
                timestamp: now,
                value: 1,
                request_id: format!("req_{}", now),
            });

            Ok(RateLimitResult {
                allowed: true,
                remaining: window_requests - current_count - 1,
                limit: window_requests,
                reset_at: window_start + window_ms,
                retry_after_ms: None,
                quota_type: QuotaType::Requests,
            })
        } else {
            Ok(RateLimitResult {
                allowed: false,
                remaining: 0,
                limit: window_requests,
                reset_at: window_start + window_ms,
                retry_after_ms: Some(window_start + window_ms - now),
                quota_type: QuotaType::Requests,
            })
        }
    }

    /// Leaky bucket algorithm
    fn check_leaky_bucket(&mut self, key: &str, rate: u64) -> Result<RateLimitResult, ApiError> {
        let now = current_timestamp();
        let key_name = format!("leaky:{}", key);

        let bucket = self.state.token_buckets.entry(key_name.clone())
            .or_insert(TokenBucket {
                key: key.to_string(),
                tokens: 0.0,
                max_tokens: rate * 10,
                refill_rate: rate as f64,
                last_refill: now,
            });

        // Leaky bucket: add 1 token per request
        if bucket.tokens < bucket.max_tokens {
            bucket.tokens += 1.0;
            Ok(RateLimitResult {
                allowed: true,
                remaining: (bucket.max_tokens - bucket.tokens) as u64,
                limit: bucket.max_tokens as u64,
                reset_at: now + 1000,
                retry_after_ms: None,
                quota_type: QuotaType::Requests,
            })
        } else {
            let leak_time = (bucket.tokens / bucket.refill_rate * 1000.0) as u64;
            Ok(RateLimitResult {
                allowed: false,
                remaining: 0,
                limit: bucket.max_tokens as u64,
                reset_at: now + leak_time,
                retry_after_ms: Some(leak_time),
                quota_type: QuotaType::Requests,
            })
        }
    }

    /// Create API key
    pub fn create_api_key(&mut self, key: ApiKey) -> Result<String, ApiError> {
        let id = key.id.clone();
        self.state.api_keys.insert(id.clone(), key);
        Ok(id)
    }

    /// Validate API key
    pub fn validate_api_key(&self, key: &str) -> Result<ApiKey, ApiError> {
        let api_key = self.state.api_keys.get(key)
            .ok_or(ApiError::InvalidApiKey)?;

        if !api_key.is_active {
            return Err(ApiError::ApiKeyDeactivated);
        }

        if let Some(expires) = api_key.expires_at {
            if current_timestamp() > expires {
                return Err(ApiError::ApiKeyExpired);
            }
        }

        Ok(api_key.clone())
    }

    /// Create API version
    pub fn create_version(&mut self, version: ApiVersion) -> Result<String, ApiError> {
        let id = version.id.clone();
        self.state.versions.insert(id.clone(), version);
        Ok(id)
    }

    /// Register endpoint
    pub fn register_endpoint(&mut self, endpoint: ApiEndpoint) -> Result<String, ApiError> {
        let id = endpoint.id.clone();
        self.state.endpoints.insert(id.clone(), endpoint);
        Ok(id)
    }

    /// Apply transformation
    pub fn apply_transformation(&self, data: &str, transform: &Transformation) -> Result<String, ApiError> {
        match transform.transform_type {
            TransformType::RequestBody | TransformType::ResponseBody => {
                // Simple JSON transformation
                Ok(data.to_string())
            }
            _ => Ok(data.to_string()),
        }
    }

    /// Get analytics
    pub fn get_analytics(&self) -> &ApiUsageAnalytics {
        &self.analytics
    }

    /// Update analytics with request
    pub fn record_request(&mut self, context: &RequestContext, latency_ms: u64, status: u16) {
        self.analytics.total_requests += 1;
        self.analytics.total_latency_ms += latency_ms;

        if status >= 200 && status < 300 {
            self.analytics.successful_requests += 1;
        } else if status == 429 {
            self.analytics.rate_limited_requests += 1;
        } else {
            self.analytics.failed_requests += 1;
        }

        self.analytics.avg_latency_ms = self.analytics.total_latency_ms / self.analytics.total_requests;
    }
}

/// API error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiError {
    RateLimitNotFound,
    InvalidApiKey,
    ApiKeyExpired,
    ApiKeyDeactivated,
    QuotaExceeded,
    InvalidVersion,
    TransformationFailed(String),
}

/// Helper functions
fn current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}

impl Default for ApiGatewayService {
    fn default() -> Self {
        Self::new()
    }
}