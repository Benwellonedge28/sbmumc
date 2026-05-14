//! # SBMUMC API Server
//!
//! REST API and WebSocket support for remote access

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServer {
    pub host: String,
    pub port: u16,
    pub routes: Vec<Route>,
    pub middleware: Vec<Middleware>,
    pub websocket_enabled: bool,
    pub graphql_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub path: String,
    pub method: HttpMethod,
    pub handler: String,
    pub auth_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Middleware {
    pub name: String,
    pub enabled: bool,
    pub config: HashMap<String, String>,
}

impl ApiServer {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            routes: Self::default_routes(),
            middleware: Self::default_middleware(),
            websocket_enabled: true,
            graphql_enabled: true,
        }
    }

    fn default_routes() -> Vec<Route> {
        vec![
            Route {
                path: "/api/v1/status".to_string(),
                method: HttpMethod::GET,
                handler: "status_handler".to_string(),
                auth_required: false,
            },
            Route {
                path: "/api/v1/omnidev".to_string(),
                method: HttpMethod::POST,
                handler: "omnidev_handler".to_string(),
                auth_required: true,
            },
            Route {
                path: "/api/v1/graph/search".to_string(),
                method: HttpMethod::POST,
                handler: "graph_search_handler".to_string(),
                auth_required: true,
            },
            Route {
                path: "/api/v1/config".to_string(),
                method: HttpMethod::GET,
                handler: "config_handler".to_string(),
                auth_required: false,
            },
            Route {
                path: "/api/v1/transaction".to_string(),
                method: HttpMethod::POST,
                handler: "transaction_handler".to_string(),
                auth_required: true,
            },
            Route {
                path: "/api/v1/audit".to_string(),
                method: HttpMethod::GET,
                handler: "audit_handler".to_string(),
                auth_required: true,
            },
        ]
    }

    fn default_middleware() -> Vec<Middleware> {
        vec![
            Middleware {
                name: "cors".to_string(),
                enabled: true,
                config: HashMap::from([
                    ("allowed_origins".to_string(), "*".to_string()),
                    ("allowed_methods".to_string(), "GET,POST,PUT,DELETE".to_string()),
                ]),
            },
            Middleware {
                name: "rate_limit".to_string(),
                enabled: true,
                config: HashMap::from([
                    ("requests_per_minute".to_string(), "100".to_string()),
                ]),
            },
            Middleware {
                name: "auth".to_string(),
                enabled: true,
                config: HashMap::from([
                    ("jwt_secret".to_string(), "sbmumc-secret".to_string()),
                ]),
            },
            Middleware {
                name: "logging".to_string(),
                enabled: true,
                config: HashMap::new(),
            },
        ]
    }

    pub fn get_endpoint(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    pub fn ws_endpoint(&self) -> String {
        format!("ws://{}:{}/ws", self.host, self.port)
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn remove_route(&mut self, path: &str) {
        self.routes.retain(|r| r.path != path);
    }

    pub fn enable_websocket(&mut self) {
        self.websocket_enabled = true;
    }

    pub fn disable_websocket(&mut self) {
        self.websocket_enabled = false;
    }

    pub fn get_openapi_spec(&self) -> String {
        r#"openapi: 3.0.0
info:
  title: SBMUMC API
  version: 1.0.0
paths:
  /api/v1/status:
    get:
      summary: System status
      responses:
        '200':
          description: OK
  /api/v1/omnidev:
    post:
      summary: OmniDev AGI operations
      requestBody:
        content:
          application/json:
            schema:
              type: object
      responses:
        '200':
          description: OK
"#.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: i64,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn error(msg: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLRequest {
    pub query: String,
    pub variables: Option<HashMap<String, serde_json::Value>>,
    pub operation_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse {
    pub data: Option<serde_json::Value>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub locations: Option<Vec<Location>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_server_creation() {
        let server = ApiServer::new("0.0.0.0".to_string(), 8080);
        assert_eq!(server.port, 8080);
        assert!(!server.routes.is_empty());
    }

    #[test]
    fn test_api_response() {
        let response: ApiResponse<String> = ApiResponse::success("test".to_string());
        assert!(response.success);
        assert!(response.data.is_some());
    }

    #[test]
    fn test_api_error_response() {
        let response: ApiResponse<String> = ApiResponse::error("Test error".to_string());
        assert!(!response.success);
        assert!(response.error.is_some());
    }
}
