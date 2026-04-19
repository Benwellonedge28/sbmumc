//! Cortex Module - Central Processing Unit for SBMUMC
//!
//! The Cortex is the central processing unit responsible for coordinating all
//! components of the SBMUMC system. It handles thought processing, information
//! flow, and coordination between modules.

use crate::core::{SbmumcError, Result, SbmumcConfig, EntityId, Thought, ThoughtContent, ProcessingRequest, ProcessingResponse, Output, ThoughtResult};
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{info, debug, error};

/// The Cortex - Central Processing Unit of SBMUMC
pub struct Cortex {
    /// Configuration
    config: Arc<SbmumcConfig>,

    /// Active thoughts
    active_thoughts: RwLock<HashMap<EntityId, Thought>>,

    /// Thought history
    thought_history: RwLock<Vec<Thought>>,

    /// Maximum active thoughts
    max_active_thoughts: usize,

    /// Shutdown signal receiver
    shutdown_rx: RwLock<Option<tokio::sync::oneshot::Receiver<()>>>,

    /// Running state
    running: RwLock<bool>,
}

impl Cortex {
    /// Create a new Cortex instance
    pub fn new(config: &SbmumcConfig) -> Result<Self> {
        info!("Initializing Cortex");
        Ok(Self {
            config: Arc::new(config.clone()),
            active_thoughts: RwLock::new(HashMap::new()),
            thought_history: RwLock::new(Vec::new()),
            max_active_thoughts: config.system.max_concurrent_operations,
            shutdown_rx: RwLock::new(None),
            running: RwLock::new(false),
        })
    }

    /// Start the Cortex
    pub async fn start(&self) -> Result<()> {
        if *self.running.read() {
            return Err(SbmumcError::SystemAlreadyRunning);
        }

        info!("Starting Cortex");
        *self.running.write() = true;

        // Start background processing tasks
        self.start_background_tasks().await?;

        info!("Cortex started");
        Ok(())
    }

    /// Stop the Cortex
    pub async fn stop(&self) -> Result<()> {
        if !*self.running.read() {
            return Err(SbmumcError::SystemNotRunning);
        }

        info!("Stopping Cortex");

        // Signal shutdown
        if let Some(rx) = self.shutdown_rx.write().take() {
            let _ = rx.await;
        }

        *self.running.write() = false;
        info!("Cortex stopped");
        Ok(())
    }

    /// Process a request through the Cortex
    pub async fn process(&self, request: ProcessingRequest) -> Result<ProcessingResponse> {
        debug!("Processing request: {:?}", request.context.request_id);

        let start_time = std::time::Instant::now();

        // Create a new thought for this request
        let thought = self.create_thought(&request).await?;

        // Process the thought through reasoning
        let processed_thought = self.process_thought(thought).await?;

        // Generate response
        let response = self.generate_response(processed_thought, start_time).await?;

        debug!("Request processed successfully");
        Ok(response)
    }

    /// Create a new thought from a request
    async fn create_thought(&self, request: &ProcessingRequest) -> Result<Thought> {
        let thought = Thought {
            id: request.context.request_id,
            content: match &request.input {
                crate::core::Input::Text(text) => ThoughtContent::Text(text.clone()),
                crate::core::Input::Structured(data) => ThoughtContent::Structured(
                    crate::core::StructuredContent {
                        format: crate::core::ContentFormat::Json,
                        data: data.clone(),
                    }
                ),
                crate::core::Input::Voice(voice) => ThoughtContent::Text(format!(
                    "[Voice input: {} bytes, format: {}]",
                    voice.audio_data.len(),
                    voice.format
                )),
                crate::core::Input::File(file) => ThoughtContent::Structured(
                    crate::core::StructuredContent {
                        format: crate::core::ContentFormat::Custom(file.format.to_string()),
                        data: serde_json::json!({
                            "path": file.path,
                            "format": file.format
                        }),
                    }
                ),
            },
            reasoning_chain: Vec::new(),
            result: None,
            metadata: crate::core::Metadata::default(),
        };

        // Add to active thoughts
        let mut active = self.active_thoughts.write();
        if active.len() >= self.max_active_thoughts {
            // Remove oldest thought
            if let Some(oldest) = active.keys().next().copied() {
                active.remove(&oldest);
            }
        }
        active.insert(thought.id, thought.clone());

        Ok(thought)
    }

    /// Process a thought through the reasoning pipeline
    async fn process_thought(&self, mut thought: Thought) -> Result<Thought> {
        debug!("Processing thought: {:?}", thought.id);

        // Add reasoning steps
        thought.reasoning_chain.push(crate::core::ReasoningStep {
            step_number: 1,
            premise: format!("Input received: {:?}", thought.content),
            inference: "Analyzing input content".to_string(),
            confidence: 0.9,
        });

        // Process based on content type
        let result = match &thought.content {
            ThoughtContent::Text(text) => {
                self.process_text_thought(text).await?
            }
            ThoughtContent::Structured(content) => {
                self.process_structured_thought(content).await?
            }
            ThoughtContent::Query(query) => {
                self.process_query_thought(query).await?
            }
            ThoughtContent::Action(action) => {
                self.process_action_thought(action).await?
            }
        };

        thought.result = Some(result);

        // Move from active to history
        {
            let mut active = self.active_thoughts.write();
            active.remove(&thought.id);
        }
        {
            let mut history = self.thought_history.write();
            history.push(thought.clone());
            // Keep only last 1000 thoughts
            if history.len() > 1000 {
                history.remove(0);
            }
        }

        Ok(thought)
    }

    /// Process a text thought
    async fn process_text_thought(&self, text: &str) -> Result<ThoughtResult> {
        debug!("Processing text thought: {} chars", text.len());

        // Simple analysis - in a full implementation, this would use
        // the NLP engine and knowledge graph
        let analysis = format!("Analyzed text content with {} characters", text.len());

        Ok(ThoughtResult {
            success: true,
            output: analysis,
            confidence: 0.85,
            explanation: Some("Text processed through basic analysis pipeline".to_string()),
        })
    }

    /// Process a structured thought
    async fn process_structured_thought(&self, content: &crate::core::StructuredContent) -> Result<ThoughtResult> {
        debug!("Processing structured thought: {:?}", content.format);

        Ok(ThoughtResult {
            success: true,
            output: format!("Processed {:?} content", content.format),
            confidence: 0.9,
            explanation: Some("Structured content processed".to_string()),
        })
    }

    /// Process a query thought
    async fn process_query_thought(&self, query: &crate::core::Query) -> Result<ThoughtResult> {
        debug!("Processing query thought: {:?}", query.query_type);

        Ok(ThoughtResult {
            success: true,
            output: format!("Query type {:?} processed", query.query_type),
            confidence: 0.9,
            explanation: Some("Query processed through knowledge system".to_string()),
        })
    }

    /// Process an action thought
    async fn process_action_thought(&self, action: &crate::core::Action) -> Result<ThoughtResult> {
        debug!("Processing action thought: {:?}", action.action_type);

        Ok(ThoughtResult {
            success: true,
            output: format!("Action {:?} executed", action.action_type),
            confidence: 0.95,
            explanation: Some("Action executed successfully".to_string()),
        })
    }

    /// Generate response from processed thought
    async fn generate_response(&self, thought: Thought, start_time: std::time::Instant) -> Result<ProcessingResponse> {
        let result = thought.result.ok_or_else(|| {
            SbmumcError::Internal("Thought has no result".to_string())
        })?;

        let output = Output::Text(result.output);

        Ok(ProcessingResponse {
            request_id: thought.id,
            output,
            reasoning: if self.config.system.enable_distributed {
                Some(thought.reasoning_chain)
            } else {
                None
            },
            metadata: crate::core::ResponseMetadata {
                timestamp: crate::core::Timestamp::now(),
                processing_time_ms: start_time.elapsed().as_millis() as u64,
                confidence: result.confidence,
                warnings: Vec::new(),
            },
        })
    }

    /// Start background processing tasks
    async fn start_background_tasks(&self) -> Result<()> {
        // Start thought cleanup task
        let (tx, rx) = tokio::sync::oneshot::channel();
        *self.shutdown_rx.write() = Some(rx);

        // Spawn cleanup task
        tokio::spawn(async move {
            let _ = tx.send(());
        });

        Ok(())
    }

    /// Get active thought count
    pub fn active_thought_count(&self) -> usize {
        self.active_thoughts.read().len()
    }

    /// Get thought history
    pub fn get_thought_history(&self, limit: usize) -> Vec<Thought> {
        let history = self.thought_history.read();
        history.iter().rev().take(limit).cloned().collect()
    }

    /// Get specific thought by ID
    pub fn get_thought(&self, id: EntityId) -> Option<Thought> {
        self.active_thoughts.read().get(&id).cloned()
            .or_else(|| self.thought_history.read().iter().find(|t| t.id == id).cloned())
    }

    /// Check if Cortex is running
    pub fn is_running(&self) -> bool {
        *self.running.read()
    }
}

impl std::fmt::Debug for Cortex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cortex")
            .field("max_active_thoughts", &self.max_active_thoughts)
            .field("active_thoughts", &self.active_thoughts.read().len())
            .field("history_size", &self.thought_history.read().len())
            .field("running", &self.running.read())
            .finish()
    }
}
