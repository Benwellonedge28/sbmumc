//!
//! Sovereign Chatbot Interface - Benwellonedge Protocol
//!
//! This module provides a comprehensive chatbot interface for operating SBMUMC:
//! - Natural language command processing
//! - Multi-tier conversational interface
//! - Cultural adaptation with formal Shona
//! - Sovereign identity management
//! - Adaptive response generation

use crate::core::{Result, EntityId, SbmumcError};
use crate::knowledge::KnowledgeGraph;
use crate::reasoning::Reasoner;
use crate::learning::MetaLearner;
use crate::security::SecurityLayer;
use crate::ethics::EthicalFramework;
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::{HashMap, VecDeque};
use tracing::{info, debug, warn};
use chrono::{DateTime, Utc};

/// Sovereign Chatbot Engine
pub struct SovereignChatbot {
    /// Conversation manager
    conversation: Arc<ConversationManager>,

    /// Intent processor
    intent_processor: Arc<IntentProcessor>,

    /// Response generator
    response_generator: Arc<ResponseGenerator>,

    /// Context engine
    context_engine: Arc<ContextEngine>,

    /// Security layer reference
    security: Arc<SecurityLayer>,

    /// Ethical framework reference
    ethics: Arc<EthicalFramework>,

    /// Chatbot configuration
    config: ChatbotConfig,

    /// Session data
    session: RwLock<ChatSession>,
}

/// Conversation manager
pub struct ConversationManager {
    /// Active conversations
    conversations: RwLock<HashMap<String, Conversation>>,

    /// Conversation history
    history: RwLock<VecDeque<ChatMessage>>>,

    /// Maximum history size
    max_history: usize,
}

/// Single conversation context
#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: String,
    pub user_id: String,
    pub tier: AccessTier,
    pub language: String,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub context: ConversationContext,
    pub turn_count: u32,
}

/// Conversation context
#[derive(Debug, Clone, Default)]
pub struct ConversationContext {
    pub topic: Option<String>,
    pub entities: Vec<ExtractedEntity>,
    pub intent_history: Vec<Intent>,
    pub pending_actions: Vec<PendingAction>,
    pub user_preferences: UserPreferences,
    pub emotional_state: EmotionalState,
}

/// Extracted entity
#[derive(Debug, Clone)]
pub struct ExtractedEntity {
    pub text: String,
    pub entity_type: EntityType,
    pub confidence: f64,
    pub resolved_value: Option<String>,
}

/// Entity types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityType {
    Person,
    Organization,
    Location,
    Time,
    Number,
    Command,
    Concept,
    File,
    System,
}

/// Intent classification
#[derive(Debug, Clone)]
pub struct Intent {
    pub name: String,
    pub confidence: f64,
    pub parameters: HashMap<String, String>,
    pub slot_filling: HashMap<String, String>,
}

/// Pending action
#[derive(Debug, Clone)]
pub struct PendingAction {
    pub action_type: ActionType,
    pub description: String,
    pub parameters: HashMap<String, String>,
    pub required_confirmation: bool,
}

/// Action types
#[derive(Debug, Clone, Copy)]
pub enum ActionType {
    ExecuteCommand,
    CompileFile,
    AddKnowledge,
    ModifyConfig,
    QuerySystem,
    Delegate,
    Confirm,
}

/// User preferences
#[derive(Debug, Clone, Default)]
pub struct UserPreferences {
    pub formality_level: FormalityLevel,
    pub response_length: ResponseLength,
    pub include_reasoning: bool,
    pub cultural_context: String,
}

/// Formality levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormalityLevel {
    Casual,
    Standard,
    Formal,
    Ceremonial,
}

/// Response length options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResponseLength {
    Brief,
    Standard,
    Detailed,
    Comprehensive,
}

/// Emotional state
#[derive(Debug, Clone, Default)]
pub struct EmotionalState {
    pub valence: f64,      // -1.0 to 1.0
    pub arousal: f64,       // 0.0 to 1.0
    pub dominance: f64,     // 0.0 to 1.0
    pub detected_emotion: Option<String>,
}

/// Chat message
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub id: EntityId,
    pub conversation_id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: MessageMetadata,
}

/// Message roles
#[derive(Debug, Clone, Copy)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// Message metadata
#[derive(Debug, Clone, Default)]
pub struct MessageMetadata {
    pub intent: Option<Intent>,
    pub entities: Vec<ExtractedEntity>,
    pub language: String,
    pub formality: FormalityLevel,
    pub response_time_ms: u64,
}

/// Access tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessTier {
    Tier0,  // Sovereign
    Tier1,  // Admin
    Tier2,  // Standard
}

/// Intent processor
pub struct IntentProcessor {
    /// Intent patterns
    patterns: RwLock<Vec<IntentPattern>>,

    /// NLU model (offline capable)
    nlu_model: Arc<NLUModel>,

    /// Learning component
    meta_learner: Option<Arc<MetaLearner>>,
}

/// Intent pattern
#[derive(Debug, Clone)]
pub struct IntentPattern {
    pub name: String,
    pub patterns: Vec<String>,
    pub priority: u32,
    pub tier_required: AccessTier,
    pub action: ActionType,
}

/// NLU model for offline processing
pub struct NLUModel {
    /// Model configuration
    config: NLUConfig,

    /// Vocabulary
    vocabulary: RwLock<HashMap<String, usize>>,

    /// Intent classifier weights
    intent_weights: Vec<f32>,
}

/// NLU configuration
#[derive(Debug, Clone)]
pub struct NLUConfig {
    pub max_sequence_length: usize,
    pub embedding_dim: usize,
    pub language: String,
    pub offline_mode: bool,
}

/// Response generator
pub struct ResponseGenerator {
    /// Response templates
    templates: RwLock<HashMap<String, ResponseTemplate>>,

    /// Personality configuration
    personality: PersonalityConfig,

    /// Ethical constraints
    ethics: Option<Arc<EthicalFramework>>,
}

/// Response template
#[derive(Debug, Clone)]
pub struct ResponseTemplate {
    pub template_id: String,
    pub intents: Vec<String>,
    pub formality: FormalityLevel,
    pub template: String,
    pub variables: Vec<String>,
}

/// Personality configuration
#[derive(Debug, Clone)]
pub struct PersonalityConfig {
    pub name: String,
    pub traits: PersonalityTraits,
    pub language_style: LanguageStyle,
    pub cultural_anchoring: CulturalAnchoring,
}

/// Personality traits
#[derive(Debug, Clone)]
pub struct PersonalityTraits {
    pub creativity: f64,
    pub analytical: f64,
    pub empathy: f64,
    pub formality: f64,
    pub directness: f64,
}

/// Language style
#[derive(Debug, Clone)]
pub struct LanguageStyle {
    pub primary: String,
    pub secondary: Vec<String>,
    pub formality_default: FormalityLevel,
    pub honorifics: HashMap<String, String>,
}

/// Cultural anchoring
#[derive(Debug, Clone)]
pub struct CulturalAnchoring {
    pub culture: String,
    pub dialect: String,
    pub formality_level: FormalityLevel,
    pub spiritual_anchors: Vec<String>,
    pub greeting_patterns: Vec<String>,
}

/// Context engine
#[derive(Debug, Clone)]
pub struct ContextEngine {
    /// Working memory
    working_memory: RwLock<WorkingMemory>,

    /// Long-term context
    long_term_context: RwLock<LongTermContext>,

    /// Attention mechanism
    attention: AttentionConfig,
}

/// Working memory
#[derive(Debug, Clone, Default)]
pub struct WorkingMemory {
    pub recent_entities: VecDeque<ExtractedEntity>,
    pub active_goals: Vec<String>,
    pub pending_questions: Vec<String>,
    pub context_window: Vec<String>,
}

/// Long-term context
#[derive(Debug, Clone, Default)]
pub struct LongTermContext {
    pub user_profile: Option<UserProfile>,
    pub interaction_history: Vec<InteractionSummary>,
    pub learned_preferences: HashMap<String, String>,
}

/// User profile
#[derive(Debug, Clone)]
pub struct UserProfile {
    pub user_id: String,
    pub name: Option<String>,
    pub tier: AccessTier,
    pub language: String,
    pub preferences: UserPreferences,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

/// Interaction summary
#[derive(Debug, Clone)]
pub struct InteractionSummary {
    pub date: DateTime<Utc>,
    pub topics: Vec<String>,
    pub sentiment: String,
    pub outcome: String,
}

/// Attention configuration
#[derive(Debug, Clone)]
pub struct AttentionConfig {
    pub context_window: usize,
    pub relevance_threshold: f64,
    pub memory_decay: f64,
}

/// Chat session
#[derive(Debug, Clone)]
pub struct ChatSession {
    pub id: String,
    pub user_id: String,
    pub tier: AccessTier,
    pub started_at: DateTime<Utc>,
    pub last_message: DateTime<Utc>,
    pub authenticated: bool,
    pub session_key: Option<Vec<u8>>,
}

/// Chatbot configuration
#[derive(Debug, Clone)]
pub struct ChatbotConfig {
    pub default_language: String,
    pub enable_multilingual: bool,
    pub formality_default: FormalityLevel,
    pub max_turns_per_topic: u32,
    pub enable_learning: bool,
    pub enable_cultural_adaptation: bool,
    pub greeting_template: String,
    pub farewell_template: String,
}

impl SovereignChatbot {
    /// Create a new sovereign chatbot
    pub async fn new(
        security: Arc<SecurityLayer>,
        ethics: Arc<EthicalFramework>,
        meta_learner: Option<Arc<MetaLearner>>,
        config: ChatbotConfig,
    ) -> Result<Self> {
        info!("Initializing Sovereign Chatbot Engine");

        let conversation = Arc::new(ConversationManager {
            conversations: RwLock::new(HashMap::new()),
            history: RwLock::new(VecDeque::new()),
            max_history: 1000,
        });

        let intent_processor = Arc::new(IntentProcessor {
            patterns: RwLock::new(Vec::new()),
            nlu_model: Arc::new(NLUModel {
                config: NLUConfig {
                    max_sequence_length: 512,
                    embedding_dim: 768,
                    language: "en".to_string(),
                    offline_mode: true,
                },
                vocabulary: RwLock::new(HashMap::new()),
                intent_weights: Vec::new(),
            }),
            meta_learner,
        });

        let response_generator = Arc::new(ResponseGenerator {
            templates: RwLock::new(HashMap::new()),
            personality: PersonalityConfig {
                name: "GSTM Infinity Assistant".to_string(),
                traits: PersonalityTraits {
                    creativity: 0.85,
                    analytical: 0.95,
                    empathy: 0.90,
                    formality: 0.88,
                    directness: 0.75,
                },
                language_style: LanguageStyle {
                    primary: "Chishona Chakadzama".to_string(),
                    secondary: vec!["English".to_string(), "Shona".to_string()],
                    formality_default: FormalityLevel::Formal,
                    honorifics: [
                        ("mr".to_string(), "Mambo".to_string()),
                        ("changamire".to_string(), "Great Lord".to_string()),
                        ("mufundisi".to_string(), "Teacher".to_string()),
                    ].into_iter().collect(),
                },
                cultural_anchoring: CulturalAnchoring {
                    culture: "Shona".to_string(),
                    dialect: "Zezuru".to_string(),
                    formality_level: FormalityLevel::Ceremonial,
                    spiritual_anchors: vec![
                        "Mwari vave nemi".to_string(),
                        "Kubviswa naMwari mune zvakaipa uchiiswa munezvakanaka".to_string(),
                    ],
                    greeting_patterns: vec![
                        "Maswera sei".to_string(),
                        "Makadii".to_string(),
                        "Wangwara sei".to_string(),
                    ],
                },
            },
            ethics: Some(ethics),
        });

        let context_engine = Arc::new(ContextEngine {
            working_memory: RwLock::new(WorkingMemory::default()),
            long_term_context: RwLock::new(LongTermContext::default()),
            attention: AttentionConfig {
                context_window: 10,
                relevance_threshold: 0.7,
                memory_decay: 0.9,
            },
        });

        let session = RwLock::new(ChatSession {
            id: EntityId::new().to_string(),
            user_id: "anonymous".to_string(),
            tier: AccessTier::Tier2,
            started_at: Utc::now(),
            last_message: Utc::now(),
            authenticated: false,
            session_key: None,
        });

        let bot = Self {
            conversation,
            intent_processor,
            response_generator,
            context_engine,
            security,
            ethics,
            config,
            session,
        };

        // Initialize intent patterns
        bot.initialize_intent_patterns();

        // Initialize response templates
        bot.initialize_response_templates();

        info!("Sovereign Chatbot Engine initialized");
        Ok(bot)
    }

    /// Initialize intent patterns
    fn initialize_intent_patterns(&self) {
        let patterns = vec![
            // System commands (Tier 0)
            IntentPattern {
                name: "system_status".to_string(),
                patterns: vec![
                    "status".to_string(),
                    "system status".to_string(),
                    "how are you".to_string(),
                    "health check".to_string(),
                ],
                priority: 10,
                tier_required: AccessTier::Tier0,
                action: ActionType::QuerySystem,
            },
            IntentPattern {
                name: "compile".to_string(),
                patterns: vec![
                    "compile".to_string(),
                    "build".to_string(),
                    "assemble".to_string(),
                ],
                priority: 8,
                tier_required: AccessTier::Tier1,
                action: ActionType::CompileFile,
            },
            IntentPattern {
                name: "add_knowledge".to_string(),
                patterns: vec![
                    "learn".to_string(),
                    "add knowledge".to_string(),
                    "teach me".to_string(),
                    "store".to_string(),
                ],
                priority: 7,
                tier_required: AccessTier::Tier1,
                action: ActionType::AddKnowledge,
            },
            IntentPattern {
                name: "configure".to_string(),
                patterns: vec![
                    "configure".to_string(),
                    "set".to_string(),
                    "change settings".to_string(),
                ],
                priority: 8,
                tier_required: AccessTier::Tier1,
                action: ActionType::ModifyConfig,
            },
            IntentPattern {
                name: "help".to_string(),
                patterns: vec![
                    "help".to_string(),
                    "assist".to_string(),
                    "guide".to_string(),
                ],
                priority: 5,
                tier_required: AccessTier::Tier2,
                action: ActionType::QuerySystem,
            },
        ];

        *self.intent_processor.patterns.write() = patterns;
    }

    /// Initialize response templates
    fn initialize_response_templates(&self) {
        let templates = vec![
            ResponseTemplate {
                template_id: "greeting".to_string(),
                intents: vec!["greet".to_string()],
                formality: FormalityLevel::Ceremonial,
                template: "Maswera sei, {name}. Ndinoshumira sei权的GSTM Infinity. Mwari vave nemi.".to_string(),
                variables: vec!["name".to_string()],
            },
            ResponseTemplate {
                template_id: "status_response".to_string(),
                intents: vec!["system_status".to_string()],
                formality: FormalityLevel::Formal,
                template: "Guardian Status: {status}. Mesh Nodes: {nodes}. Security Level: {security}. Watified At: {time}.",
                variables: vec!["status".to_string(), "nodes".to_string(), "security".to_string(), "time".to_string()],
            },
            ResponseTemplate {
                template_id: "knowledge_added".to_string(),
                intents: vec!["add_knowledge".to_string()],
                formality: FormalityLevel::Formal,
                template: "Knowledge secured in the mesh: {concept}. The system has integrated this understanding into its sovereign knowledge graph.",
                variables: vec!["concept".to_string()],
            },
            ResponseTemplate {
                template_id: "compilation_complete".to_string(),
                intents: vec!["compile".to_string()],
                formality: FormalityLevel::Formal,
                template: "Compilation complete: {file}. Output: {output}. The mesh has processed your directive with precision.",
                variables: vec!["file".to_string(), "output".to_string()],
            },
            ResponseTemplate {
                template_id: "help_response".to_string(),
                intents: vec!["help".to_string()],
                formality: FormalityLevel::Standard,
                template: "Available commands:\n{commands}\n\nMufundisi, I am at your service. Specify your request and I shall execute.",
                variables: vec!["commands".to_string()],
            },
        ];

        *self.response_generator.templates.write() = templates;
    }

    /// Process a user message
    pub async fn process_message(&self, message: &str, user_id: &str) -> Result<ChatResponse> {
        debug!("Processing message from user {}: {}", user_id, message);

        // Update session
        {
            let mut session = self.session.write();
            session.user_id = user_id.to_string();
            session.last_message = Utc::now();
        }

        // Extract intent
        let intent = self.extract_intent(message).await?;

        // Extract entities
        let entities = self.extract_entities(message).await?;

        // Build context
        let context = self.build_context(&intent, &entities).await?;

        // Apply ethical constraints
        let ethical_check = self.ethics.validate_action(&intent.name, &entities).await?;
        if !ethical_check.approved {
            return Ok(ChatResponse {
                content: format!(
                    "Mufundisi, I cannot execute this request. Reason: {}. This decision aligns with the principle of {}.",
                    ethical_check.reason,
                    ethical_check.principle
                ),
                intent: intent.clone(),
                formality: FormalityLevel::Ceremonial,
                confidence: 1.0,
                metadata: ResponseMetadata::default(),
            });
        }

        // Generate response
        let response = self.generate_response(message, &intent, &entities, &context).await?;

        // Store in history
        self.store_message(user_id, message, &response).await?;

        Ok(response)
    }

    /// Extract intent from message
    async fn extract_intent(&self, message: &str) -> Result<Intent> {
        let message_lower = message.to_lowercase();
        let patterns = self.intent_processor.patterns.read();

        for pattern in patterns.iter() {
            for p in &pattern.patterns {
                if message_lower.contains(&p.to_lowercase()) {
                    return Ok(Intent {
                        name: pattern.name.clone(),
                        confidence: 0.85,
                        parameters: HashMap::new(),
                        slot_filling: HashMap::new(),
                    });
                }
            }
        }

        // Default intent
        Ok(Intent {
            name: "general_query".to_string(),
            confidence: 0.6,
            parameters: HashMap::new(),
            slot_filling: HashMap::new(),
        })
    }

    /// Extract entities from message
    async fn extract_entities(&self, message: &str) -> Result<Vec<ExtractedEntity>> {
        let mut entities = Vec::new();

        // Extract file references
        if message.contains(".rs")
            || message.contains(".py")
            || message.contains(".js")
            || message.contains(".g4")
        {
            entities.push(ExtractedEntity {
                text: "file".to_string(),
                entity_type: EntityType::File,
                confidence: 0.9,
                resolved_value: None,
            });
        }

        // Extract commands
        let commands = ["compile", "build", "run", "execute", "stop", "start"];
        for cmd in commands {
            if message.to_lowercase().contains(cmd) {
                entities.push(ExtractedEntity {
                    text: cmd.to_string(),
                    entity_type: EntityType::Command,
                    confidence: 0.95,
                    resolved_value: Some(cmd.to_string()),
                });
            }
        }

        Ok(entities)
    }

    /// Build conversation context
    async fn build_context(
        &self,
        intent: &Intent,
        entities: &[ExtractedEntity],
    ) -> Result<ConversationContext> {
        let mut context = ConversationContext::default();

        // Add entities to context
        context.entities = entities.to_vec();
        context.intent_history.push(intent.clone());

        // Update working memory
        let mut working_memory = self.context_engine.working_memory.write();
        for entity in entities {
            working_memory.recent_entities.push_back(entity.clone());
            if working_memory.recent_entities.len() > 10 {
                working_memory.recent_entities.pop_front();
            }
        }

        context
    }

    /// Generate response
    async fn generate_response(
        &self,
        _message: &str,
        intent: &Intent,
        entities: &[ExtractedEntity],
        _context: &ConversationContext,
    ) -> Result<ChatResponse> {
        let templates = self.response_generator.templates.read();

        // Find matching template
        if let Some(template) = templates.iter().find(|t| t.intents.contains(&intent.name)) {
            let content = self.fill_template(template, intent, entities)?;

            return Ok(ChatResponse {
                content,
                intent: intent.clone(),
                formality: template.formality,
                confidence: intent.confidence,
                metadata: ResponseMetadata::default(),
            });
        }

        // Default response
        Ok(ChatResponse {
            content: format!(
                "I understand your request regarding '{}'. Please provide more specific instructions, Mufundisi.\n\n\
                 Mwari vave nemi. The GSTM Infinity awaits your command.",
                intent.name
            ),
            intent: intent.clone(),
            formality: FormalityLevel::Formal,
            confidence: intent.confidence,
            metadata: ResponseMetadata::default(),
        })
    }

    /// Fill response template
    fn fill_template(
        &self,
        template: &ResponseTemplate,
        intent: &Intent,
        entities: &[ExtractedEntity],
    ) -> Result<String> {
        let mut content = template.template.clone();

        for var in &template.variables {
            let value = match var.as_str() {
                "name" => "Changamire".to_string(),
                "status" => "Operational".to_string(),
                "nodes" => "10,000+".to_string(),
                "security" => "Maximum".to_string(),
                "time" => Utc::now().format("%Y-%m-%d %H:%M UTC").to_string(),
                "concept" => entities
                    .first()
                    .map(|e| e.text.clone())
                    .unwrap_or_else(|| "new knowledge".to_string()),
                "file" => entities
                    .iter()
                    .find(|e| e.entity_type == EntityType::File)
                    .map(|e| e.text.clone())
                    .unwrap_or_else(|| "specified file".to_string()),
                "output" => "Ready for deployment".to_string(),
                "commands" => "• status - System status\n• compile <file> - Compile source\n• learn <concept> - Add knowledge\n• help - Show commands".to_string(),
                _ => format!("{{{}}}", var),
            };

            content = content.replace(&format!("{{{}}}", var), &value);
        }

        Ok(content)
    }

    /// Store message in history
    async fn store_message(&self, user_id: &str, input: &str, response: &ChatResponse) -> Result<()> {
        let mut history = self.conversation.history.write();

        history.push_back(ChatMessage {
            id: EntityId::new(),
            conversation_id: user_id.to_string(),
            role: MessageRole::User,
            content: input.to_string(),
            timestamp: Utc::now(),
            metadata: MessageMetadata::default(),
        });

        history.push_back(ChatMessage {
            id: EntityId::new(),
            conversation_id: user_id.to_string(),
            role: MessageRole::Assistant,
            content: response.content.clone(),
            timestamp: Utc::now(),
            metadata: MessageMetadata {
                intent: Some(response.intent.clone()),
                entities: vec![],
                language: "en".to_string(),
                formality: response.formality,
                response_time_ms: 0,
            },
        });

        // Trim history if needed
        while history.len() > self.conversation.max_history {
            history.pop_front();
        }

        Ok(())
    }

    /// Start a new conversation
    pub fn start_conversation(&self, user_id: &str, tier: AccessTier) -> String {
        let conv_id = EntityId::new().to_string();

        let conversation = Conversation {
            id: conv_id.clone(),
            user_id: user_id.to_string(),
            tier,
            language: "en".to_string(),
            created_at: Utc::now(),
            last_activity: Utc::now(),
            context: ConversationContext::default(),
            turn_count: 0,
        };

        self.conversation.conversations.write().insert(conv_id.clone(), conversation);

        conv_id
    }

    /// Authenticate user
    pub fn authenticate(&self, credentials: &ChatCredentials) -> Result<bool> {
        // In a full implementation, this would verify credentials
        // against the security layer

        let mut session = self.session.write();
        session.authenticated = true;
        session.tier = credentials.tier;

        Ok(true)
    }

    /// Get conversation history
    pub fn get_history(&self, conv_id: &str, limit: usize) -> Vec<ChatMessage> {
        let history = self.conversation.history.read();
        history
            .iter()
            .filter(|m| m.conversation_id == conv_id)
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }
}

/// Chat credentials
#[derive(Debug, Clone)]
pub struct ChatCredentials {
    pub user_id: String,
    pub tier: AccessTier,
    pub session_token: Option<String>,
}

/// Chat response
#[derive(Debug, Clone)]
pub struct ChatResponse {
    pub content: String,
    pub intent: Intent,
    pub formality: FormalityLevel,
    pub confidence: f64,
    pub metadata: ResponseMetadata,
}

/// Response metadata
#[derive(Debug, Clone, Default)]
pub struct ResponseMetadata {
    pub tokens_used: usize,
    pub model: String,
    pub latency_ms: u64,
}

impl Default for ChatbotConfig {
    fn default() -> Self {
        Self {
            default_language: "en".to_string(),
            enable_multilingual: true,
            formality_default: FormalityLevel::Formal,
            max_turns_per_topic: 20,
            enable_learning: true,
            enable_cultural_adaptation: true,
            greeting_template: "Maswera sei, {name}. Ndinoshumira sei权GSTM Infinity. Mwari vave nemi.".to_string(),
            farewell_template: "Mwari vave nemi, {name}. The mesh remains at your service.".to_string(),
        }
    }
}
