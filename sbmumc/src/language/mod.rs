//! Language Module - Natural Language Processing for SBMUMC
//!
//! This module handles all language-related processing including:
//! - Natural language understanding
//! - Text generation
//! - Translation
//! - Sentiment analysis
//! - Entity extraction

use crate::core::{SbmumcError, Result, PropertyValue};
use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use tracing::{debug, info};

/// NLP Engine - Main natural language processing system
pub struct NlpEngine {
    /// Language models
    language_models: HashMap<String, LanguageModel>,

    /// Tokenizer
    tokenizer: Tokenizer,

    /// Parser
    parser: Parser,

    /// Configuration
    config: NlpConfig,
}

/// Language model wrapper
#[derive(Debug, Clone)]
pub struct LanguageModel {
    pub id: String,
    pub name: String,
    pub language: String,
    pub model_type: ModelType,
    pub vocabulary_size: usize,
}

/// Model types
#[derive(Debug, Clone, Copy)]
pub enum ModelType {
    Transformer,
    LSTM,
    GRU,
    RuleBased,
}

/// Tokenizer configuration
pub struct Tokenizer {
    /// Vocabulary
    vocabulary: HashMap<String, usize>,

    /// Special tokens
    special_tokens: Vec<String>,

    /// Maximum token length
    max_length: usize,
}

/// Parser for syntax analysis
pub struct Parser {
    /// Parsing rules
    rules: Vec<ParsingRule>,

    /// Parse tree cache
    parse_cache: RwLock<lru_cache::LruCache<String, ParseTree>>,
}

/// Parsing rule
#[derive(Debug, Clone)]
pub struct ParsingRule {
    pub pattern: String,
    pub rule_type: RuleType,
    pub action: String,
}

/// Rule types
#[derive(Debug, Clone, Copy)]
pub enum RuleType {
    Terminal,
    NonTerminal,
    Optional,
    Repetition,
}

/// Parse tree node
#[derive(Debug, Clone)]
pub struct ParseTree {
    pub root: TreeNode,
    pub confidence: f64,
}

/// Tree node in parse tree
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub node_type: String,
    pub value: Option<String>,
    pub children: Vec<TreeNode>,
    pub span: (usize, usize),
}

/// NLP configuration
#[derive(Debug, Clone)]
pub struct NlpConfig {
    pub max_context_length: usize,
    pub enable_emotion_detection: bool,
    pub enable_intent_recognition: bool,
    pub enable_entity_extraction: bool,
    pub default_language: String,
}

impl Default for NlpConfig {
    fn default() -> Self {
        Self {
            max_context_length: 4096,
            enable_emotion_detection: true,
            enable_intent_recognition: true,
            enable_entity_extraction: true,
            default_language: "en".to_string(),
        }
    }
}

impl NlpEngine {
    /// Create a new NLP engine
    pub fn new() -> Result<Self> {
        info!("Initializing NLP Engine");

        let mut language_models = HashMap::new();

        // Add default English model
        language_models.insert("en".to_string(), LanguageModel {
            id: "en_default".to_string(),
            name: "English Model".to_string(),
            language: "en".to_string(),
            model_type: ModelType::Transformer,
            vocabulary_size: 50000,
        });

        Ok(Self {
            language_models,
            tokenizer: Tokenizer::new()?,
            parser: Parser::new()?,
            config: NlpConfig::default(),
        })
    }

    /// Process text input
    pub fn process(&self, text: &str, language: Option<&str>) -> Result<NlpResult> {
        debug!("Processing text: {} chars", text.len());

        let lang = language.unwrap_or(&self.config.default_language);

        // Tokenize
        let tokens = self.tokenize(text, lang)?;

        // Parse
        let parse_tree = self.parse(&tokens)?;

        // Extract entities
        let entities = if self.config.enable_entity_extraction {
            self.extract_entities(&tokens)?
        } else {
            Vec::new()
        };

        // Detect sentiment
        let sentiment = self.detect_sentiment(&tokens)?;

        // Recognize intent
        let intent = if self.config.enable_intent_recognition {
            self.recognize_intent(&tokens)?
        } else {
            Intent::default()
        };

        Ok(NlpResult {
            text: text.to_string(),
            language: lang.to_string(),
            tokens,
            parse_tree,
            entities,
            sentiment,
            intent,
            confidence: 0.85,
        })
    }

    /// Tokenize text
    pub fn tokenize(&self, text: &str, language: &str) -> Result<Vec<Token>> {
        self.tokenizer.tokenize(text, language)
    }

    /// Parse tokens into syntax tree
    pub fn parse(&self, tokens: &[Token]) -> Result<ParseTree> {
        self.parser.parse(tokens)
    }

    /// Extract entities from text
    pub fn extract_entities(&self, tokens: &[Token]) -> Result<Vec<Entity>> {
        let mut entities = Vec::new();
        let mut current_entity: Option<Entity> = None;

        for (i, token) in tokens.iter().enumerate() {
            if let Some(entity_type) = self.classify_entity(token) {
                if let Some(ref mut entity) = current_entity {
                    if entity.entity_type == entity_type {
                        entity.tokens.push(token.clone());
                        entity.end = i;
                    } else {
                        entities.push(current_entity.take().unwrap());
                        current_entity = Some(Entity {
                            entity_type,
                            text: token.text.clone(),
                            tokens: vec![token.clone()],
                            start: i,
                            end: i,
                            confidence: 0.8,
                        });
                    }
                } else {
                    current_entity = Some(Entity {
                        entity_type,
                        text: token.text.clone(),
                        tokens: vec![token.clone()],
                        start: i,
                        end: i,
                        confidence: 0.8,
                    });
                }
            }
        }

        if let Some(entity) = current_entity {
            entities.push(entity);
        }

        Ok(entities)
    }

    /// Classify a token as an entity type
    fn classify_entity(&self, token: &Token) -> Option<String> {
        // Simple entity classification
        if token.text.chars().all(|c| c.is_uppercase()) && token.text.len() > 1 {
            return Some("ORGANIZATION".to_string());
        }
        if token.text.chars().all(|c| c.is_uppercase() || c == '\'' ) && token.text.len() > 1 {
            return Some("PERSON".to_string());
        }
        if token.text.chars().all(|c| c.is_numeric()) {
            return Some("NUMBER".to_string());
        }
        None
    }

    /// Detect sentiment
    pub fn detect_sentiment(&self, tokens: &[Token]) -> Result<Sentiment> {
        let positive_words: HashSet<&str> = [
            "good", "great", "excellent", "amazing", "wonderful", "fantastic", "love", "like", "happy", "joy"
        ].into_iter().collect();

        let negative_words: HashSet<&str> = [
            "bad", "terrible", "awful", "hate", "dislike", "sad", "angry", "horrible", "worst", "poor"
        ].into_iter().collect();

        let mut positive_count = 0;
        let mut negative_count = 0;

        for token in tokens {
            let lower = token.text.to_lowercase();
            if positive_words.contains(lower.as_str()) {
                positive_count += 1;
            }
            if negative_words.contains(lower.as_str()) {
                negative_count += 1;
            }
        }

        let total = positive_count + negative_count;
        if total == 0 {
            return Ok(Sentiment::default());
        }

        let positive_ratio = positive_count as f64 / total as f64;

        Ok(Sentiment {
            polarity: if positive_ratio > 0.6 {
                Polarity::Positive
            } else if positive_ratio < 0.4 {
                Polarity::Negative
            } else {
                Polarity::Neutral
            },
            score: positive_ratio,
            confidence: 0.7,
        })
    }

    /// Recognize intent
    pub fn recognize_intent(&self, tokens: &[Token]) -> Result<Intent> {
        let text: String = tokens.iter().map(|t| t.text.to_lowercase()).collect();
        let text = text.as_str();

        // Simple intent classification
        if text.contains("what") || text.contains("how") || text.contains("why") || text.contains("when") || text.contains("where") {
            return Ok(Intent {
                intent_type: IntentType::Question,
                confidence: 0.8,
                entities: HashMap::new(),
            });
        }

        if text.contains("create") || text.contains("make") || text.contains("generate") || text.contains("build") {
            return Ok(Intent {
                intent_type: IntentType::Create,
                confidence: 0.8,
                entities: HashMap::new(),
            });
        }

        if text.contains("delete") || text.contains("remove") || text.contains("destroy") {
            return Ok(Intent {
                intent_type: IntentType::Delete,
                confidence: 0.8,
                entities: HashMap::new(),
            });
        }

        if text.contains("find") || text.contains("search") || text.contains("look") {
            return Ok(Intent {
                intent_type: IntentType::Search,
                confidence: 0.8,
                entities: HashMap::new(),
            });
        }

        Ok(Intent {
            intent_type: IntentType::Other,
            confidence: 0.5,
            entities: HashMap::new(),
        })
    }

    /// Generate text
    pub fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        debug!("Generating text for prompt: {} chars", prompt.len());

        // Simple text generation (in real implementation, use a language model)
        Ok(format!("Generated response to: {}", prompt))
    }

    /// Add a language model
    pub fn add_model(&mut self, model: LanguageModel) {
        self.language_models.insert(model.language.clone(), model);
    }

    /// Get supported languages
    pub fn supported_languages(&self) -> Vec<String> {
        self.language_models.keys().cloned().collect()
    }
}

impl Default for NlpEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create NlpEngine")
    }
}

// ============================================================================
// Tokenizer Implementation
// ============================================================================

impl Tokenizer {
    pub fn new() -> Result<Self> {
        let mut vocabulary = HashMap::new();
        // Add common words to vocabulary
        let common_words = [
            "the", "a", "an", "is", "are", "was", "were", "be", "been", "being",
            "have", "has", "had", "do", "does", "did", "will", "would", "could", "should",
            "may", "might", "must", "shall", "can", "need", "dare", "ought", "used",
            "to", "of", "in", "for", "on", "with", "at", "by", "from", "as",
            "into", "through", "during", "before", "after", "above", "below", "between",
            "under", "again", "further", "then", "once", "here", "there", "when", "where",
            "why", "how", "all", "each", "few", "more", "most", "other", "some", "such",
            "no", "nor", "not", "only", "own", "same", "so", "than", "too", "very",
        ];

        for (i, word) in common_words.iter().enumerate() {
            vocabulary.insert(word.to_string(), i);
        }

        Ok(Self {
            vocabulary,
            special_tokens: vec![
                "[PAD]".to_string(),
                "[UNK]".to_string(),
                "[CLS]".to_string(),
                "[SEP]".to_string(),
                "[MASK]".to_string(),
            ],
            max_length: 4096,
        })
    }

    pub fn tokenize(&self, text: &str, language: &str) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut position = 0;

        for c in text.chars() {
            if c.is_whitespace() {
                if !current_token.is_empty() {
                    tokens.push(self.create_token(&current_token, position));
                    position += 1;
                    current_token.clear();
                }
            } else if c.is_alphanumeric() || c == '\'' || c == '-' {
                current_token.push(c);
            } else {
                // Punctuation
                if !current_token.is_empty() {
                    tokens.push(self.create_token(&current_token, position));
                    position += 1;
                    current_token.clear();
                }
                tokens.push(Token {
                    text: c.to_string(),
                    token_type: TokenType::Punctuation,
                    position,
                    embedding: None,
                });
                position += 1;
            }
        }

        if !current_token.is_empty() {
            tokens.push(self.create_token(&current_token, position));
        }

        Ok(tokens)
    }

    fn create_token(&self, text: &str, position: usize) -> Token {
        let token_type = if self.vocabulary.contains_key(text.to_lowercase().as_str()) {
            TokenType::Word
        } else if text.chars().all(|c| c.is_numeric()) {
            TokenType::Number
        } else {
            TokenType::Word
        };

        Token {
            text: text.to_string(),
            token_type,
            position,
            embedding: None,
        }
    }
}

// ============================================================================
// Parser Implementation
// ============================================================================

impl Parser {
    pub fn new() -> Result<Self> {
        Ok(Self {
            rules: Vec::new(),
            parse_cache: RwLock::new(lru_cache::LruCache::new(1000)),
        })
    }

    pub fn parse(&self, tokens: &[Token]) -> Result<ParseTree> {
        // Simple recursive descent parsing
        let root = self.parse_sentence(tokens, 0)?;

        Ok(ParseTree {
            root,
            confidence: 0.8,
        })
    }

    fn parse_sentence(&self, tokens: &[Token], start: usize) -> Result<TreeNode> {
        let mut children = Vec::new();

        // Simple parsing: group tokens into Subject, Verb, Object
        let subject_end = self.find_boundary(tokens, start, &["is", "are", "was", "were", "will", "would"])?;
        let verb_end = self.find_boundary(tokens, subject_end, &["to", "and", "or", "."])?;

        if start < subject_end {
            children.push(TreeNode {
                node_type: "SUBJECT".to_string(),
                value: None,
                children: tokens[start..subject_end].iter().map(|t| TreeNode {
                    node_type: "WORD".to_string(),
                    value: Some(t.text.clone()),
                    children: Vec::new(),
                    span: (t.position, t.position),
                }).collect(),
                span: (tokens[start].position, tokens[subject_end - 1].position),
            });
        }

        if subject_end < verb_end {
            children.push(TreeNode {
                node_type: "VERB_PHRASE".to_string(),
                value: None,
                children: tokens[subject_end..verb_end].iter().map(|t| TreeNode {
                    node_type: "WORD".to_string(),
                    value: Some(t.text.clone()),
                    children: Vec::new(),
                    span: (t.position, t.position),
                }).collect(),
                span: (tokens[subject_end].position, tokens[verb_end - 1].position),
            });
        }

        if verb_end < tokens.len() {
            children.push(TreeNode {
                node_type: "OBJECT".to_string(),
                value: None,
                children: tokens[verb_end..].iter().map(|t| TreeNode {
                    node_type: "WORD".to_string(),
                    value: Some(t.text.clone()),
                    children: Vec::new(),
                    span: (t.position, t.position),
                }).collect(),
                span: (tokens[verb_end].position, tokens.last().map(|t| t.position).unwrap_or(0)),
            });
        }

        Ok(TreeNode {
            node_type: "SENTENCE".to_string(),
            value: None,
            children,
            span: (0, tokens.last().map(|t| t.position).unwrap_or(0)),
        })
    }

    fn find_boundary(&self, tokens: &[Token], start: usize, markers: &[&str]) -> usize {
        let mut end = start;

        while end < tokens.len() {
            let lower = tokens[end].text.to_lowercase();
            if markers.contains(&lower.as_str()) {
                break;
            }
            end += 1;
        }

        end.min(tokens.len())
    }
}

// ============================================================================
// NLP Result Types
// ============================================================================

/// NLP processing result
#[derive(Debug, Clone)]
pub struct NlpResult {
    pub text: String,
    pub language: String,
    pub tokens: Vec<Token>,
    pub parse_tree: ParseTree,
    pub entities: Vec<Entity>,
    pub sentiment: Sentiment,
    pub intent: Intent,
    pub confidence: f64,
}

/// Token representation
#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
    pub position: usize,
    pub embedding: Option<Vec<f32>>,
}

/// Token types
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Word,
    Number,
    Punctuation,
    Special,
    Unknown,
}

/// Entity extraction result
#[derive(Debug, Clone)]
pub struct Entity {
    pub entity_type: String,
    pub text: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub end: usize,
    pub confidence: f64,
}

/// Sentiment analysis result
#[derive(Debug, Clone, Default)]
pub struct Sentiment {
    pub polarity: Polarity,
    pub score: f64,
    pub confidence: f64,
}

/// Polarity types
#[derive(Debug, Clone, Copy, Default)]
pub enum Polarity {
    Positive,
    Negative,
    Neutral,
    #[default]
    Unknown,
}

/// Intent recognition result
#[derive(Debug, Clone)]
pub struct Intent {
    pub intent_type: IntentType,
    pub confidence: f64,
    pub entities: HashMap<String, String>,
}

impl Default for Intent {
    fn default() -> Self {
        Self {
            intent_type: IntentType::Other,
            confidence: 0.5,
            entities: HashMap::new(),
        }
    }
}

/// Intent types
#[derive(Debug, Clone, Copy, Default)]
pub enum IntentType {
    Question,
    Statement,
    Command,
    Create,
    Delete,
    Search,
    Other,
    #[default]
    Unknown,
}

// ============================================================================
// Translator
// ============================================================================

/// Translator for multilingual support
pub struct Translator {
    /// Supported language pairs
    language_pairs: HashMap<String, HashMap<String, f64>>,

    /// Current translation model
    translation_model: Option<String>,
}

impl Translator {
    /// Create a new translator
    pub fn new() -> Result<Self> {
        info!("Initializing Translator");

        Ok(Self {
            language_pairs: HashMap::new(),
            translation_model: None,
        })
    }

    /// Translate text between languages
    pub fn translate(&self, text: &str, from: &str, to: &str) -> Result<String> {
        debug!("Translating from {} to {}: {} chars", from, to, text.len());

        if from == to {
            return Ok(text.to_string());
        }

        // In a full implementation, this would use a translation model
        Ok(format!("[Translated from {} to {}]: {}", from, to, text))
    }

    /// Detect language
    pub fn detect_language(&self, text: &str) -> Result<LanguageDetection> {
        // Simple language detection based on common patterns
        let mut scores: HashMap<&str, f64> = HashMap::new();

        // Check for Chinese characters
        if text.chars().any(|c| matches!(c, '\u{4e00}'..='\u{9fff}')) {
            scores.insert("zh", 0.9);
        }

        // Check for Japanese characters
        if text.chars().any(|c| matches!(c, '\u{3040}'..='\u{309f}') || matches!(c, '\u{30a0}'..='\u{30ff}')) {
            *scores.entry("ja").or_insert(0.0) = 0.8;
        }

        // Check for Korean characters
        if text.chars().any(|c| matches!(c, '\u{ac00}'..='\u{d7af}')) {
            *scores.entry("ko").or_insert(0.0) = 0.8;
        }

        // Check for Arabic
        if text.chars().any(|c| matches!(c, '\u{0600}'..='\u{06ff}')) {
            *scores.entry("ar").or_insert(0.0) = 0.9;
        }

        // Default to English
        let detected = scores.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).map(|(k, _)| *k).unwrap_or("en");

        Ok(LanguageDetection {
            language: detected.to_string(),
            confidence: *scores.get(detected).unwrap_or(&0.5),
        })
    }

    /// Get supported languages
    pub fn supported_languages(&self) -> Vec<String> {
        let mut languages: HashSet<String> = HashSet::new();
        for (from, tos) in &self.language_pairs {
            languages.insert(from.clone());
            for to in tos.keys() {
                languages.insert(to.clone());
            }
        }
        // Add common languages
        languages.extend(vec![
            "en".to_string(), "zh".to_string(), "es".to_string(),
            "fr".to_string(), "de".to_string(), "ja".to_string(),
            "ko".to_string(), "ar".to_string(), "hi".to_string(),
            "pt".to_string(),
        ]);
        languages.into_iter().collect()
    }

    /// Add translation pair
    pub fn add_pair(&mut self, from: &str, to: &str, confidence: f64) {
        self.language_pairs
            .entry(from.to_string())
            .or_insert_with(HashMap::new)
            .insert(to.to_string(), confidence);
    }
}

impl Default for Translator {
    fn default() -> Self {
        Self::new().expect("Failed to create Translator")
    }
}

/// Language detection result
#[derive(Debug, Clone)]
pub struct LanguageDetection {
    pub language: String,
    pub confidence: f64,
}

// ============================================================================
// Simple LRU Cache
// ============================================================================

mod lru_cache {
    use std::collections::{HashMap, VecDeque};
    use std::hash::Hash;

    pub struct LruCache<K, V> {
        capacity: usize,
        cache: HashMap<K, V>,
        order: VecDeque<K>,
    }

    impl<K: Eq + Hash + Clone, V: Clone> LruCache<K, V> {
        pub fn new(capacity: usize) -> Self {
            Self {
                capacity,
                cache: HashMap::new(),
                order: VecDeque::new(),
            }
        }

        pub fn get(&self, key: &K) -> Option<V> {
            self.cache.get(key).cloned()
        }

        pub fn insert(&mut self, key: K, value: V) {
            if self.cache.contains_key(&key) {
                self.order.retain(|k| k != &key);
            } else if self.cache.len() >= self.capacity {
                if let Some(lru) = self.order.pop_front() {
                    self.cache.remove(&lru);
                }
            }

            self.cache.insert(key.clone(), value);
            self.order.push_back(key);
        }
    }
}
