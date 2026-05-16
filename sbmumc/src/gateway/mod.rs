//!
//! # SBMUMC Module 1571: API Gateway and Gateway Management
//!
//! Central API gateway with routing, rate limiting, authentication,
//! request transformation, and service orchestration.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// API route configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRoute {
    pub id: String,
    pub path: String,
    pub method: HttpMethod,
    pub target_service: String,
    pub target_path: String,
    pub plugins: Vec<String>,
    pub rate_limit: Option<RateLimitConfig>,
    pub auth: Option<AuthConfig>,
    pub transformation: Option<TransformConfig>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
    pub key_by: RateLimitKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RateLimitKey {
    IP,
    User,
    APIKey,
    Custom(String),
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub auth_type: AuthType,
    pub provider: Option<String>,
    pub jwt_secret: Option<String>,
    pub api_key_header: Option<String>,
    pub require_https: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthType {
    None,
    Basic,
    Bearer,
    APIKey,
    JWT,
    OAuth2,
    Custom,
}

/// Request/Response transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformConfig {
    pub request_headers: Vec<HeaderTransform>,
    pub request_body: Option<BodyTransform>,
    pub response_headers: Vec<HeaderTransform>,
    pub response_body: Option<BodyTransform>,
}

/// Header transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderTransform {
    pub operation: TransformOperation,
    pub header: String,
    pub value: Option<String>,
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransformOperation {
    Add,
    Remove,
    Update,
    Rename,
}

/// Body transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyTransform {
    pub transform_type: BodyTransformType,
    pub mappings: Vec<FieldMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BodyTransformType {
    Map,
    Filter,
    Merge,
    Extract,
}

/// Field mapping for transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMapping {
    pub source: String,
    pub target: String,
    pub transform: Option<String>,
}

/// API Gateway service
pub struct ApiGateway {
    routes: Arc<RwLock<HashMap<String, ApiRoute>>>,
    services: Arc<RwLock<HashMap<String, ServiceConfig>>>,
    plugins: Arc<RwLock<HashMap<String, Plugin>>>,
    rate_limiter: Arc<RwLock<RateLimiter>>,
    auth_handler: Arc<RwLock<AuthHandler>>,
}

impl ApiGateway {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
            services: Arc::new(RwLock::new(HashMap::new())),
            plugins: Arc::new(RwLock::new(HashMap::new())),
            rate_limiter: Arc::new(RwLock::new(RateLimiter::new())),
            auth_handler: Arc::new(RwLock::new(AuthHandler::new())),
        }
    }

    /// Register route
    pub fn register_route(&self, route: ApiRoute) -> String {
        let mut routes = self.routes.write().unwrap();
        routes.insert(route.id.clone(), route.clone());
        route.id
    }

    /// Register service
    pub fn register_service(&self, service: ServiceConfig) -> String {
        let mut services = self.services.write().unwrap();
        services.insert(service.id.clone(), service.clone());
        service.id
    }

    /// Register plugin
    pub fn register_plugin(&self, plugin: Plugin) {
        let mut plugins = self.plugins.write().unwrap();
        plugins.insert(plugin.id.clone(), plugin);
    }

    /// Route request
    pub async fn route(&self, request: RouteRequest) -> Result<RouteResponse, GatewayError> {
        // Find matching route
        let routes = self.routes.read().unwrap();
        let route = self.find_route(&routes, &request.path, request.method.clone())?;

        // Check rate limit
        let rate_config = route.rate_limit.as_ref();
        if let Some(config) = rate_config {
            if !self.check_rate_limit(&request, config)? {
                return Err(GatewayError::RateLimitExceeded);
            }
        }

        // Authenticate
        if let Some(auth) = &route.auth {
            if !self.authenticate(&request, auth)? {
                return Err(GatewayError::Unauthorized);
            }
        }

        // Transform request
        let transformed = self.transform_request(&request, route.as_ref());

        // Find target service
        let services = self.services.read().unwrap();
        let service = services.get(&route.target_service)
            .ok_or(GatewayError::ServiceNotFound)?;

        // Execute request
        let response = self.execute_request(service, &transformed, &route.target_path).await?;

        // Transform response
        let final_response = self.transform_response(response, route.as_ref());

        Ok(final_response)
    }

    fn find_route(&self, routes: &HashMap<String, ApiRoute>, path: &str, method: HttpMethod) -> Result<ApiRoute, GatewayError> {
        routes.values()
            .find(|r| r.enabled && self.match_path(&r.path, path) && r.method == method)
            .cloned()
            .ok_or(GatewayError::RouteNotFound)
    }

    fn match_path(&self, pattern: &str, path: &str) -> bool {
        // Simple path matching with parameters
        let pattern_parts: Vec<&str> = pattern.split('/').collect();
        let path_parts: Vec<&str> = path.split('/').collect();

        if pattern_parts.len() != path_parts.len() {
            return false;
        }

        pattern_parts.iter().zip(path_parts.iter()).all(|(p, pp)| {
            if p.starts_with(':') || p.starts_with('{') {
                true
            } else {
                p == pp
            }
        })
    }

    fn check_rate_limit(&self, request: &RouteRequest, config: &RateLimitConfig) -> Result<bool, GatewayError> {
        let key = match config.key_by {
            RateLimitKey::IP => request.client_ip.clone(),
            RateLimitKey::User => request.user_id.clone().unwrap_or_default(),
            RateLimitKey::APIKey => request.api_key.clone().unwrap_or_default(),
            RateLimitKey::Custom(ref key) => request.headers.get(key).cloned().unwrap_or_default(),
        };

        let mut limiter = self.rate_limiter.write().unwrap();
        Ok(limiter.check(&key, config.requests_per_second, config.burst_size))
    }

    fn authenticate(&self, request: &RouteRequest, auth: &AuthConfig) -> Result<bool, GatewayError> {
        let mut handler = self.auth_handler.write().unwrap();
        handler.authenticate(request, auth)
    }

    fn transform_request(&self, request: &RouteRequest, route: &ApiRoute) -> TransformedRequest {
        let mut transformed = TransformedRequest {
            method: request.method.clone(),
            path: route.target_path.clone(),
            headers: request.headers.clone(),
            body: request.body.clone(),
            query_params: request.query_params.clone(),
        };

        if let Some(config) = &route.transformation {
            // Apply header transformations
            for header_transform in &config.request_headers {
                match header_transform.operation {
                    TransformOperation::Add => {
                        if let Some(value) = &header_transform.value {
                            transformed.headers.insert(header_transform.header.clone(), value.clone());
                        }
                    }
                    TransformOperation::Remove => {
                        transformed.headers.remove(&header_transform.header);
                    }
                    TransformOperation::Update => {
                        if let Some(value) = &header_transform.value {
                            transformed.headers.insert(header_transform.header.clone(), value.clone());
                        }
                    }
                    TransformOperation::Rename => {
                        if let Some(old) = transformed.headers.remove(&header_transform.header) {
                            if let Some(value) = &header_transform.value {
                                transformed.headers.insert(value.clone(), old);
                            }
                        }
                    }
                }
            }
        }

        transformed
    }

    fn transform_response(&self, response: ServiceResponse, route: &ApiRoute) -> RouteResponse {
        let mut final_response = RouteResponse {
            status_code: response.status_code,
            headers: response.headers,
            body: response.body,
        };

        if let Some(config) = &route.transformation {
            for header_transform in &config.response_headers {
                match header_transform.operation {
                    TransformOperation::Add => {
                        if let Some(value) = &header_transform.value {
                            final_response.headers.insert(header_transform.header.clone(), value.clone());
                        }
                    }
                    TransformOperation::Remove => {
                        final_response.headers.remove(&header_transform.header);
                    }
                    _ => {}
                }
            }
        }

        final_response
    }

    async fn execute_request(&self, service: &ServiceConfig, request: &TransformedRequest, target_path: &str) -> Result<ServiceResponse, GatewayError> {
        // Simplified - in production would make actual HTTP request
        Ok(ServiceResponse {
            status_code: 200,
            headers: HashMap::new(),
            body: Some(serde_json::json!({ "status": "ok" }).to_string()),
        })
    }

    /// Get route metrics
    pub fn get_metrics(&self) -> GatewayMetrics {
        let routes = self.routes.read().unwrap();
        let services = self.services.read().unwrap();

        GatewayMetrics {
            total_routes: routes.len(),
            total_services: services.len(),
            active_routes: routes.values().filter(|r| r.enabled).count(),
        }
    }
}

/// Route request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub query_params: HashMap<String, String>,
    pub client_ip: String,
    pub user_id: Option<String>,
    pub api_key: Option<String>,
}

/// Transformed request
#[derive(Debug, Clone)]
pub struct TransformedRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub query_params: HashMap<String, String>,
}

/// Route response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

/// Service response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub id: String,
    pub name: String,
    pub url: String,
    pub health_check_path: Option<String>,
    pub timeout_secs: u32,
    pub max_retries: u32,
    pub load_balancing: LoadBalancingStrategy,
}

/// Load balancing strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    IPHash,
    Random,
    Weighted,
}

/// Plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub plugin_type: PluginType,
    pub config: HashMap<String, serde_json::Value>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginType {
    Authentication,
    RateLimiting,
    Logging,
    Caching,
    Transformation,
    Monitoring,
    Security,
}

/// Rate limiter
pub struct RateLimiter {
    buckets: HashMap<String, RateBucket>,
}

struct RateBucket {
    tokens: f64,
    last_refill: u64,
    refill_rate: f64,
    capacity: f64,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            buckets: HashMap::new(),
        }
    }

    pub fn check(&mut self, key: &str, rate: u32, burst: u32) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let bucket = self.buckets.entry(key.to_string())
            .or_insert(RateBucket {
                tokens: burst as f64,
                last_refill: now,
                refill_rate: rate as f64,
                capacity: burst as f64,
            });

        // Refill tokens
        let elapsed = (now - bucket.last_refill) as f64 / 1000.0;
        bucket.tokens = (bucket.tokens + elapsed * bucket.refill_rate).min(bucket.capacity);
        bucket.last_refill = now;

        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

/// Auth handler
pub struct AuthHandler {
    validators: HashMap<AuthType, fn(&RouteRequest, &AuthConfig) -> bool>,
}

impl AuthHandler {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
        }
    }

    pub fn authenticate(&mut self, request: &RouteRequest, auth: &AuthConfig) -> Result<bool, GatewayError> {
        match auth.auth_type {
            AuthType::None => Ok(true),
            AuthType::Basic => self.validate_basic(request),
            AuthType::Bearer => self.validate_bearer(request, auth),
            AuthType::JWT => self.validate_jwt(request, auth),
            AuthType::APIKey => self.validate_api_key(request, auth),
            _ => Ok(true),
        }
    }

    fn validate_basic(&self, request: &RouteRequest) -> Result<bool, GatewayError> {
        if let Some(auth) = request.headers.get("Authorization") {
            if auth.starts_with("Basic ") {
                return Ok(true);
            }
        }
        Err(GatewayError::Unauthorized)
    }

    fn validate_bearer(&self, request: &RouteRequest, auth: &AuthConfig) -> Result<bool, GatewayError> {
        if let Some(auth_header) = request.headers.get("Authorization") {
            if auth_header.starts_with("Bearer ") {
                return Ok(true);
            }
        }
        Err(GatewayError::Unauthorized)
    }

    fn validate_jwt(&self, request: &RouteRequest, auth: &AuthConfig) -> Result<bool, GatewayError> {
        if let Some(token) = request.headers.get("Authorization").and_then(|h| h.strip_prefix("Bearer ")) {
            // In production, validate JWT signature
            Ok(!token.is_empty())
        } else {
            Err(GatewayError::Unauthorized)
        }
    }

    fn validate_api_key(&self, request: &RouteRequest, auth: &AuthConfig) -> Result<bool, GatewayError> {
        let header = auth.api_key_header.as_deref().unwrap_or("X-API-Key");
        if let Some(api_key) = request.headers.get(header) {
            return Ok(!api_key.is_empty());
        }
        Err(GatewayError::Unauthorized)
    }
}

/// Gateway metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayMetrics {
    pub total_routes: usize,
    pub total_services: usize,
    pub active_routes: usize,
}

/// Gateway errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GatewayError {
    RouteNotFound,
    ServiceNotFound,
    Unauthorized,
    RateLimitExceeded,
    Timeout,
    TransformationFailed,
}

impl std::fmt::Display for GatewayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GatewayError::RouteNotFound => write!(f, "Route not found"),
            GatewayError::ServiceNotFound => write!(f, "Service not found"),
            GatewayError::Unauthorized => write!(f, "Unauthorized"),
            GatewayError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            GatewayError::Timeout => write!(f, "Request timeout"),
            GatewayError::TransformationFailed => write!(f, "Transformation failed"),
        }
    }
}

impl std::error::Error for GatewayError {}

// Re-export types
pub use ApiRoute;
pub use ServiceConfig;
pub use Plugin;
pub use ApiGateway;