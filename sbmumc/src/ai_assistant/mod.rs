//!
//! # SBMUMC Module 1578: AI Assistant Integration
//!
//! Integration with AI assistants (ChatGPT, Claude, etc.) for code
//! generation, debugging, documentation, and pair programming.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// AI provider
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AIProvider {
    OpenAI,
    Anthropic,
    Google,
    Meta,
    Local,
    Custom(String),
}

/// AI model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModel {
    pub id: String,
    pub provider: AIProvider,
    pub name: String,
    pub version: String,
    pub capabilities: Vec<ModelCapability>,
    pub max_tokens: u32,
    pub supports_streaming: bool,
}

/// Model capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelCapability {
    CodeGeneration,
    CodeReview,
    Debugging,
    Documentation,
    Translation,
    Refactoring,
    Testing,
    Explanation,
    PairProgramming,
}

/// AI conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConversation {
    pub id: String,
    pub user_id: String,
    pub model_id: String,
    pub messages: Vec<AIMessage>,
    pub context: ConversationContext,
    pub created_at: u64,
    pub updated_at: u64,
}

/// AI message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMessage {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: u64,
    pub attachments: Vec<MessageAttachment>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

/// Message attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttachment {
    pub name: String,
    pub mime_type: String,
    pub content: String,
    pub size_bytes: u64,
}

/// Conversation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub files: Vec<FileContext>,
    pub repository: Option<RepositoryContext>,
    pub language: Option<String>,
    pub task_type: Option<TaskType>,
}

/// File context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContext {
    pub path: String,
    pub language: String,
    pub content: String,
    pub line_count: u32,
}

/// Repository context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryContext {
    pub name: String,
    pub url: String,
    pub branch: String,
    pub commit_hash: String,
}

/// Task types for AI
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskType {
    CodeGeneration,
    CodeReview,
    BugFix,
    Refactoring,
    Documentation,
    Testing,
    Explanation,
    Optimization,
    SecurityAudit,
    PairProgramming,
}

/// AI request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    pub model: String,
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub temperature: f32,
    pub max_tokens: u32,
    pub top_p: f32,
    pub stop_sequences: Vec<String>,
    pub stream: bool,
    pub context: Option<ConversationContext>,
}

/// AI response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub content: String,
    pub model: String,
    pub finish_reason: FinishReason,
    pub tokens_used: u32,
    pub latency_ms: u64,
    pub citations: Vec<Citation>,
}

/// Finish reason
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    Error,
}

/// Citation from code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub file: String,
    pub line_start: u32,
    pub line_end: u32,
    pub snippet: String,
}

/// AI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: AIProvider,
    pub api_endpoint: String,
    pub api_key: String,
    pub default_model: String,
    pub rate_limit: RateLimitConfig,
    pub retry_config: RetryConfig,
}

/// Rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub tokens_per_minute: u64,
    pub concurrent_requests: u32,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

/// AI assistant service
pub struct AIAssistantService {
    providers: Arc<RwLock<HashMap<AIProvider, ProviderConfig>>>,
    models: Arc<RwLock<HashMap<String, AIModel>>>,
    conversations: Arc<RwLock<HashMap<String, AIConversation>>>,
    prompt_templates: Arc<RwLock<HashMap<String, PromptTemplate>>>,
}

impl AIAssistantService {
    pub fn new() -> Self {
        let service = Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
            models: Arc::new(RwLock::new(HashMap::new())),
            conversations: Arc::new(RwLock::new(HashMap::new())),
            prompt_templates: Arc::new(RwLock::new(HashMap::new())),
        };

        service.init_default_templates();
        service
    }

    /// Configure AI provider
    pub fn configure_provider(&self, config: ProviderConfig) -> Result<(), AIError> {
        let mut providers = self.providers.write().unwrap();
        providers.insert(config.provider.clone(), config);
        Ok(())
    }

    /// Register model
    pub fn register_model(&self, model: AIModel) {
        let mut models = self.models.write().unwrap();
        models.insert(model.id.clone(), model);
    }

    /// Create conversation
    pub fn create_conversation(&self, user_id: String, model_id: String, context: ConversationContext) -> String {
        let conversation_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let conversation = AIConversation {
            id: conversation_id.clone(),
            user_id,
            model_id,
            messages: vec![],
            context,
            created_at: timestamp,
            updated_at: timestamp,
        };

        let mut conversations = self.conversations.write().unwrap();
        conversations.insert(conversation_id, conversation);

        conversation_id
    }

    /// Send message
    pub async fn send_message(&self, conversation_id: &str, content: String) -> Result<AIResponse, AIError> {
        let mut conversations = self.conversations.write().unwrap();

        if let Some(conversation) = conversations.get_mut(conversation_id) {
            // Add user message
            let user_message = AIMessage {
                id: Uuid::new_v4().to_string(),
                role: MessageRole::User,
                content: content.clone(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                attachments: vec![],
                metadata: HashMap::new(),
            };

            conversation.messages.push(user_message);

            // Build prompt from conversation history
            let prompt = self.build_prompt(conversation)?;

            // Get provider config
            let providers = self.providers.read().unwrap();
            let models = self.models.read().unwrap();

            let model = models.get(&conversation.model_id)
                .ok_or(AIError::ModelNotFound)?;

            let provider = providers.get(&model.provider)
                .ok_or(AIError::ProviderNotConfigured)?;

            // Call AI provider (simulated)
            let response = self.call_provider(provider, &prompt).await?;

            // Add assistant message
            let assistant_message = AIMessage {
                id: Uuid::new_v4().to_string(),
                role: MessageRole::Assistant,
                content: response.content.clone(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                attachments: vec![],
                metadata: HashMap::new(),
            };

            conversation.messages.push(assistant_message);
            conversation.updated_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            drop(conversations);

            Ok(response)
        } else {
            Err(AIError::ConversationNotFound)
        }
    }

    /// Generate code using AI
    pub async fn generate_code(&self, request: CodeGenerationRequest) -> Result<CodeGenerationResult, AIError> {
        let conversation_id = self.create_conversation(
            request.user_id.clone(),
            request.model_id.clone(),
            ConversationContext {
                files: vec![FileContext {
                    path: request.file_path.clone(),
                    language: request.language.clone(),
                    content: request.context_code.clone().unwrap_or_default(),
                    line_count: 0,
                }],
                repository: None,
                language: Some(request.language.clone()),
                task_type: Some(TaskType::CodeGeneration),
            },
        );

        let prompt = self.build_code_generation_prompt(&request)?;

        // Send request
        let response = self.send_message(&conversation_id, prompt).await?;

        Ok(CodeGenerationResult {
            conversation_id,
            generated_code: response.content,
            language: request.language,
            file_path: request.file_path,
            tokens_used: response.tokens_used,
        })
    }

    /// Review code using AI
    pub async fn review_code(&self, request: CodeReviewRequest) -> Result<CodeReviewResult, AIError> {
        let conversation_id = self.create_conversation(
            request.user_id.clone(),
            request.model_id.clone(),
            ConversationContext {
                files: vec![FileContext {
                    path: request.file_path.clone(),
                    language: request.language.clone(),
                    content: request.code.clone(),
                    line_count: request.code.lines().count() as u32,
                }],
                repository: None,
                language: Some(request.language.clone()),
                task_type: Some(TaskType::CodeReview),
            },
        );

        let prompt = self.build_code_review_prompt(&request)?;
        let response = self.send_message(&conversation_id, prompt).await?;

        // Parse review comments from response
        let comments = self.parse_review_comments(&response.content)?;

        Ok(CodeReviewResult {
            conversation_id,
            overall_rating: self.calculate_rating(&comments),
            issues: comments,
            suggestions: vec![],
            tokens_used: response.tokens_used,
        })
    }

    /// Explain code
    pub async fn explain_code(&self, request: ExplainCodeRequest) -> Result<String, AIError> {
        let conversation_id = self.create_conversation(
            request.user_id.clone(),
            request.model_id.clone(),
            ConversationContext {
                files: vec![FileContext {
                    path: request.file_path.clone(),
                    language: request.language.clone(),
                    content: request.code.clone(),
                    line_count: 0,
                }],
                repository: None,
                language: Some(request.language.clone()),
                task_type: Some(TaskType::Explanation),
            },
        );

        let prompt = format!(
            "Explain the following {} code in detail:\n\n```{}\n{}\n```\n\nProvide a clear explanation including:\n1. What the code does\n2. How it works\n3. Key concepts and patterns used",
            request.language,
            request.language,
            request.code
        );

        let response = self.send_message(&conversation_id, prompt).await?;
        Ok(response.content)
    }

    fn build_prompt(&self, conversation: &AIConversation) -> Result<String, AIError> {
        let mut prompt = String::new();

        // Add context
        if !conversation.context.files.is_empty() {
            prompt.push_str("Context files:\n");
            for file in &conversation.context.files {
                prompt.push_str(&format!("\n// {} ({})\n{}\n", file.path, file.language, file.content));
            }
        }

        // Add conversation history
        for message in &conversation.messages {
            match message.role {
                MessageRole::System => prompt.push_str(&format!("System: {}\n", message.content)),
                MessageRole::User => prompt.push_str(&format!("User: {}\n", message.content)),
                MessageRole::Assistant => prompt.push_str(&format!("Assistant: {}\n", message.content)),
                _ => {}
            }
        }

        Ok(prompt)
    }

    fn build_code_generation_prompt(&self, request: &CodeGenerationRequest) -> Result<String, AIError> {
        let mut prompt = String::new();

        if let Some(context) = &request.context_code {
            prompt.push_str(&format!("Existing code context:\n```{}\n{}\n```\n\n", request.language, context));
        }

        prompt.push_str(&format!("Generate {} code for the following requirement:\n{}\n", request.language, request.requirement));

        if let Some(framework) = &request.framework {
            prompt.push_str(&format!("Use the {} framework\n", framework));
        }

        if !request.requirements.is_empty() {
            prompt.push_str("\nRequirements:\n");
            for req in &request.requirements {
                prompt.push_str(&format!("- {}\n", req));
            }
        }

        Ok(prompt)
    }

    fn build_code_review_prompt(&self, request: &CodeReviewRequest) -> Result<String, AIError> {
        let mut prompt = String::new();

        prompt.push_str(&format!("Review the following {} code:\n\n```{}\n{}\n```\n\n", request.language, request.language, request.code));

        if request.focus_areas.is_empty() {
            prompt.push_str("\nProvide a comprehensive review including:\n");
            prompt.push_str("- Potential bugs and issues\n");
            prompt.push_str("- Performance improvements\n");
            prompt.push_str("- Security concerns\n");
            prompt.push_str("- Code quality and style\n");
        } else {
            prompt.push_str(&format!("Focus on: {}\n", request.focus_areas.join(", ")));
        }

        Ok(prompt)
    }

    async fn call_provider(&self, provider: &ProviderConfig, prompt: &str) -> Result<AIResponse, AIError> {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // Simulated AI call - in production would use actual API
        let content = format!("AI response to: {}...", &prompt[..prompt.len().min(100)]);

        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Ok(AIResponse {
            content,
            model: provider.default_model.clone(),
            finish_reason: FinishReason::Stop,
            tokens_used: 100,
            latency_ms: end - start,
            citations: vec![],
        })
    }

    fn parse_review_comments(&self, content: &str) -> Result<Vec<ReviewComment>, AIError> {
        // Simplified parsing - in production would use structured output
        let mut comments = Vec::new();

        // Look for line number patterns like "Line 42:"
        for line in content.lines() {
            if line.contains("Line ") || line.contains("line ") {
                // Create comment from line
                comments.push(ReviewComment {
                    line: 0,
                    severity: "info".to_string(),
                    message: line.to_string(),
                    suggestion: None,
                });
            }
        }

        Ok(comments)
    }

    fn calculate_rating(&self, comments: &[ReviewComment]) -> f32 {
        if comments.is_empty() {
            return 10.0;
        }

        let critical = comments.iter().filter(|c| c.severity == "critical").count() as f32;
        let major = comments.iter().filter(|c| c.severity == "major").count() as f32;
        let minor = comments.iter().filter(|c| c.severity == "minor").count() as f32;

        let score = 10.0 - (critical * 2.0 + major * 1.0 + minor * 0.25);
        score.max(0.0).min(10.0)
    }

    fn init_default_templates(&self) {
        let mut templates = self.prompt_templates.write().unwrap();

        templates.insert("code_generation".to_string(), PromptTemplate {
            name: "Code Generation".to_string(),
            template_type: TemplateType::CodeGeneration,
            system_prompt: "You are an expert programmer. Generate clean, efficient, and well-documented code.".to_string(),
            user_template: "Generate {{language}} code for: {{requirement}}".to_string(),
        });

        templates.insert("code_review".to_string(), PromptTemplate {
            name: "Code Review".to_string(),
            template_type: TemplateType::CodeReview,
            system_prompt: "You are an expert code reviewer. Provide constructive feedback on code quality.".to_string(),
            user_template: "Review the following {{language}} code:\n{{code}}".to_string(),
        });
    }

    /// Get conversation history
    pub fn get_conversation(&self, conversation_id: &str) -> Option<AIConversation> {
        let conversations = self.conversations.read().unwrap();
        conversations.get(conversation_id).cloned()
    }

    /// List models
    pub fn list_models(&self, provider: Option<AIProvider>) -> Vec<AIModel> {
        let models = self.models.read().unwrap();

        match provider {
            Some(p) => models.values().filter(|m| m.provider == p).cloned().collect(),
            None => models.values().cloned().collect(),
        }
    }
}

/// Prompt template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub name: String,
    pub template_type: TemplateType,
    pub system_prompt: String,
    pub user_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateType {
    CodeGeneration,
    CodeReview,
    Debugging,
    Documentation,
    General,
}

/// Code generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenerationRequest {
    pub user_id: String,
    pub model_id: String,
    pub language: String,
    pub file_path: String,
    pub requirement: String,
    pub framework: Option<String>,
    pub context_code: Option<String>,
    pub requirements: Vec<String>,
}

/// Code generation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenerationResult {
    pub conversation_id: String,
    pub generated_code: String,
    pub language: String,
    pub file_path: String,
    pub tokens_used: u32,
}

/// Code review request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReviewRequest {
    pub user_id: String,
    pub model_id: String,
    pub language: String,
    pub file_path: String,
    pub code: String,
    pub focus_areas: Vec<String>,
}

/// Code review result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReviewResult {
    pub conversation_id: String,
    pub overall_rating: f32,
    pub issues: Vec<ReviewComment>,
    pub suggestions: Vec<String>,
    pub tokens_used: u32,
}

/// Review comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewComment {
    pub line: u32,
    pub severity: String,
    pub message: String,
    pub suggestion: Option<String>,
}

/// Explain code request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainCodeRequest {
    pub user_id: String,
    pub model_id: String,
    pub language: String,
    pub file_path: String,
    pub code: String,
}

/// AI error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIError {
    ProviderNotConfigured,
    ModelNotFound,
    ConversationNotFound,
    RateLimitExceeded,
    InvalidRequest,
    APIError(String),
}

impl std::fmt::Display for AIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIError::ProviderNotConfigured => write!(f, "AI provider not configured"),
            AIError::ModelNotFound => write!(f, "Model not found"),
            AIError::ConversationNotFound => write!(f, "Conversation not found"),
            AIError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            AIError::InvalidRequest => write!(f, "Invalid request"),
            AIError::APIError(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl std::error::Error for AIError {}

// Re-export types
pub use AIProvider;
pub use AIModel;
pub use AIConversation;
pub use AIRequest;
pub use AIResponse;
pub use AIAssistantService;