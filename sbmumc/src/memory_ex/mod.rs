//! Memory & Experience Management Module
//!
//! This module implements episodic memory system, semantic memory indexing,
//! procedural memory storage, memory consolidation, and forgetting mechanisms.
//!
//! Features:
//! - Episodic memory system
//! - Semantic memory indexing
//! - Procedural memory storage
//! - Memory consolidation
//! - Forgetting mechanisms with importance weighting

use crate::core::{SbmumcError, Result, EntityId, SecurityLevel};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

/// Memory type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryType {
    /// Episodic memory (events and experiences)
    Episodic,
    /// Semantic memory (facts and knowledge)
    Semantic,
    /// Procedural memory (skills and processes)
    Procedural,
    /// Working memory (short-term)
    Working,
    /// Long-term memory
    LongTerm,
    /// Sensory memory
    Sensory,
}

/// Memory content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// Memory identifier
    pub id: String,
    /// Memory type
    pub memory_type: MemoryType,
    /// Content
    pub content: MemoryContent,
    /// Metadata
    pub metadata: MemoryMetadata,
    /// Encoding strength
    pub encoding_strength: f64,
    /// Retrieval strength
    pub retrieval_strength: f64,
    /// Accessibility
    pub accessibility: f64,
    /// Emotional significance
    pub emotional_significance: f64,
    /// Last accessed
    pub last_accessed: u64,
    /// Access count
    pub access_count: u64,
    /// Importance score
    pub importance: f64,
    /// Forgetting threshold
    pub forgetting_threshold: f64,
}

impl Memory {
    /// Create a new memory
    pub fn new(id: &str, memory_type: MemoryType, content: MemoryContent) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Memory {
            id: id.to_string(),
            memory_type,
            content,
            metadata: MemoryMetadata::default(),
            encoding_strength: 1.0,
            retrieval_strength: 1.0,
            accessibility: 1.0,
            emotional_significance: 0.0,
            last_accessed: timestamp,
            access_count: 0,
            importance: 0.5,
            forgetting_threshold: 0.1,
        }
    }

    /// Access memory
    pub fn access(&mut self) {
        self.access_count += 1;
        self.last_accessed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.retrieval_strength = (self.retrieval_strength + 0.1).min(1.0);
    }

    /// Rehearse memory
    pub fn rehearse(&mut self, depth: u32) {
        let rehearsal_benefit = 1.0 - (0.5_f64.powi(depth as i32));
        self.encoding_strength = (self.encoding_strength + rehearsal_benefit * 0.1).min(1.0);
    }

    /// Calculate decay
    pub fn calculate_decay(&self, current_time: u64) -> f64 {
        let time_elapsed = (current_time - self.last_accessed) as f64;
        let base_decay = 0.01;

        // Exponential decay based on time
        let decay_rate = base_decay * (1.0 - self.importance * 0.5) * (1.0 - self.encoding_strength * 0.3);
        let decay = 1.0 - (decay_rate * time_elapsed.log10().max(1.0));

        decay.max(self.forgetting_threshold)
    }

    /// Update importance
    pub fn update_importance(&mut self, delta: f64) {
        self.importance = (self.importance + delta).clamp(0.0, 1.0);
    }
}

/// Memory content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryContent {
    /// Text content
    Text(String),
    /// Structured data
    Structured(Vec<MemoryField>),
    /// Binary data
    Binary(Vec<u8>),
    /// Reference to external storage
    Reference(MemoryReference),
    /// Compressed data
    Compressed { original_size: usize, compressed: Vec<u8> },
}

impl Default for MemoryContent {
    fn default() -> Self {
        MemoryContent::Text(String::new())
    }
}

/// Memory field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryField {
    /// Field name
    pub name: String,
    /// Field value
    pub value: String,
    /// Field type
    pub field_type: FieldType,
}

impl MemoryField {
    /// Create a new field
    pub fn new(name: &str, value: &str) -> Self {
        MemoryField {
            name: name.to_string(),
            value: value.to_string(),
            field_type: FieldType::Text,
        }
    }
}

/// Field type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Number,
    Date,
    Location,
    Person,
    Object,
    Event,
    Concept,
    Relation,
    Custom(String),
}

/// Memory reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryReference {
    /// Reference type
    pub reference_type: ReferenceType,
    /// Target ID
    pub target_id: String,
    /// Location hint
    pub location_hint: Option<String>,
}

impl MemoryReference {
    /// Create a new reference
    pub fn new(reference_type: ReferenceType, target_id: &str) -> Self {
        MemoryReference {
            reference_type,
            target_id: target_id.to_string(),
            location_hint: None,
        }
    }
}

/// Reference type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferenceType {
    /// Links to another memory
    Memory,
    /// Links to external file
    File,
    /// Links to external URL
    URL,
    /// Links to knowledge base
    KnowledgeBase,
    /// Links to sensor data
    Sensor,
}

/// Memory metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetadata {
    /// Creation time
    pub created_at: u64,
    /// Source
    pub source: MemorySource,
    /// Tags
    pub tags: Vec<String>,
    /// Context
    pub context: String,
    /// Related memories
    pub related_memories: Vec<String>,
    /// Confidence in accuracy
    pub confidence: f64,
    /// Spatial location (if applicable)
    pub spatial_location: Option<(f64, f64, f64)>,
    /// Temporal bounds
    pub temporal_bounds: Option<(u64, u64)>,
}

impl Default for MemoryMetadata {
    fn default() -> Self {
        MemoryMetadata {
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            source: MemorySource::Internal,
            tags: Vec::new(),
            context: String::new(),
            related_memories: Vec::new(),
            confidence: 0.8,
            spatial_location: None,
            temporal_bounds: None,
        }
    }
}

/// Memory source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemorySource {
    /// Internal generation
    Internal,
    /// User input
    UserInput,
    /// Observation
    Observation,
    /// Communication
    Communication,
    /// External Data,
    External,
    /// Generated
    Generated,
}

/// Episodic memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemory {
    /// Event description
    pub event: String,
    /// Participants
    pub participants: Vec<String>,
    /// Location
    pub location: Option<String>,
    /// Time period
    pub time_period: TimePeriod,
    /// Emotional state
    pub emotional_state: EmotionalState,
    /// Outcome
    pub outcome: Option<String>,
    /// Lessons learned
    pub lessons: Vec<String>,
    /// Sensory details
    pub sensory_details: SensoryDetails,
}

impl Default for EpisodicMemory {
    fn default() -> Self {
        EpisodicMemory {
            event: String::new(),
            participants: Vec::new(),
            location: None,
            time_period: TimePeriod::default(),
            emotional_state: EmotionalState::default(),
            outcome: None,
            lessons: Vec::new(),
            sensory_details: SensoryDetails::default(),
        }
    }
}

/// Time period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePeriod {
    /// Start time
    pub start: u64,
    /// End time
    pub end: u64,
    /// Duration (seconds)
    pub duration: Option<u64>,
    /// Relative time description
    pub relative: Option<String>,
}

impl Default for TimePeriod {
    fn default() -> Self {
        TimePeriod {
            start: 0,
            end: 0,
            duration: None,
            relative: None,
        }
    }
}

/// Emotional state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    /// Valence (-1 to 1)
    pub valence: f64,
    /// Arousal (0 to 1)
    pub arousal: f64,
    /// Dominance (0 to 1)
    pub dominance: f64,
    /// Primary emotion
    pub primary_emotion: Option<String>,
}

impl Default for EmotionalState {
    fn default() -> Self {
        EmotionalState {
            valence: 0.0,
            arousal: 0.5,
            dominance: 0.5,
            primary_emotion: None,
        }
    }
}

/// Sensory details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryDetails {
    /// Visual details
    pub visual: Option<String>,
    /// Auditory details
    pub auditory: Option<String>,
    /// Tactile details
    pub tactile: Option<String>,
    /// Olfactory details
    pub olfactory: Option<String>,
    /// Gustatory details
    pub gustatory: Option<String>,
}

impl Default for SensoryDetails {
    fn default() -> Self {
        SensoryDetails {
            visual: None,
            auditory: None,
            tactile: None,
            olfactory: None,
            gustatory: None,
        }
    }
}

/// Semantic memory concept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticConcept {
    /// Concept name
    pub name: String,
    /// Definition
    pub definition: String,
    /// Category
    pub category: String,
    /// Superordinate concepts
    pub superordinates: Vec<String>,
    /// Subordinate concepts
    pub subordinates: Vec<String>,
    /// Attributes
    pub attributes: HashMap<String, SemanticAttribute>,
    /// Relations
    pub relations: Vec<ConceptRelation>,
    /// Examples
    pub examples: Vec<String>,
    /// Confidence
    pub confidence: f64,
}

impl SemanticConcept {
    /// Create a new concept
    pub fn new(name: &str, definition: &str) -> Self {
        SemanticConcept {
            name: name.to_string(),
            definition: definition.to_string(),
            category: "General".to_string(),
            superordinates: Vec::new(),
            subordinates: Vec::new(),
            attributes: HashMap::new(),
            relations: Vec::new(),
            examples: Vec::new(),
            confidence: 0.8,
        }
    }

    /// Add attribute
    pub fn add_attribute(&mut self, name: &str, value: &str) {
        self.attributes.insert(name.to_string(), SemanticAttribute {
            name: name.to_string(),
            value: value.to_string(),
            confidence: 0.8,
        });
    }
}

/// Semantic attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticAttribute {
    /// Attribute name
    pub name: String,
    /// Attribute value
    pub value: String,
    /// Confidence
    pub confidence: f64,
}

/// Concept relation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptRelation {
    /// Related concept
    pub concept: String,
    /// Relation type
    pub relation_type: SemanticRelationType,
    /// Strength
    pub strength: f64,
}

/// Semantic relation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticRelationType {
    /// Is-a relation
    IsA,
    /// Part-of relation
    PartOf,
    /// Similar-to relation
    SimilarTo,
    /// Antonym-of relation
    AntonymOf,
    /// Causes relation
    Causes,
    /// Enables relation
    Enables,
    /// Related-to relation
    RelatedTo,
}

/// Procedural memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralMemory {
    /// Procedure name
    pub name: String,
    /// Description
    pub description: String,
    /// Steps
    pub steps: Vec<ProcedureStep>,
    /// Preconditions
    pub preconditions: Vec<String>,
    /// Postconditions
    pub postconditions: Vec<String>,
    /// Complexity score
    pub complexity: u32,
    /// Success rate
    pub success_rate: f64,
    /// Times performed
    pub times_performed: u32,
    /// Average duration
    pub average_duration: f64,
}

impl ProceduralMemory {
    /// Create a new procedure
    pub fn new(name: &str) -> Self {
        ProceduralMemory {
            name: name.to_string(),
            description: String::new(),
            steps: Vec::new(),
            preconditions: Vec::new(),
            postconditions: Vec::new(),
            complexity: 1,
            success_rate: 0.0,
            times_performed: 0,
            average_duration: 0.0,
        }
    }

    /// Add step
    pub fn add_step(&mut self, step: ProcedureStep) {
        self.steps.push(step);
        self.complexity = self.steps.len() as u32;
    }

    /// Record execution
    pub fn record_execution(&mut self, success: bool, duration: f64) {
        self.times_performed += 1;

        // Update success rate
        let old_rate = self.success_rate * (self.times_performed - 1) as f64;
        let new_rate = if success { 1.0 } else { 0.0 };
        self.success_rate = (old_rate + new_rate) / self.times_performed as f64;

        // Update average duration
        let total_duration = self.average_duration * (self.times_performed - 1) as f64;
        self.average_duration = (total_duration + duration) / self.times_performed as f64;
    }
}

/// Procedure step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureStep {
    /// Step number
    pub step_number: u32,
    /// Description
    pub description: String,
    /// Required resources
    pub resources: Vec<String>,
    /// Estimated duration
    pub duration: Option<f64>,
    /// Can parallelize with
    pub parallel_with: Vec<u32>,
}

impl ProcedureStep {
    /// Create a new step
    pub fn new(step_number: u32, description: &str) -> Self {
        ProcedureStep {
            step_number,
            description: description.to_string(),
            resources: Vec::new(),
            duration: None,
            parallel_with: Vec::new(),
        }
    }
}

/// Working memory buffer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemoryBuffer {
    /// Buffer ID
    pub id: String,
    /// Items in buffer
    pub items: Vec<WorkingMemoryItem>,
    /// Capacity
    pub capacity: usize,
    /// Attention focus
    pub focus_item: Option<String>,
    /// Timestamp
    pub timestamp: u64,
}

impl WorkingMemoryBuffer {
    /// Create a new buffer
    pub fn new(id: &str, capacity: usize) -> Self {
        WorkingMemoryBuffer {
            id: id.to_string(),
            items: Vec::new(),
            capacity,
            focus_item: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Add item
    pub fn add_item(&mut self, item: WorkingMemoryItem) -> bool {
        if self.items.len() >= self.capacity {
            // Remove least important item
            if let Some(pos) = self.find_least_important() {
                self.items.remove(pos);
            } else {
                return false;
            }
        }

        self.items.push(item);
        true
    }

    /// Find least important item
    fn find_least_important(&self) -> Option<usize> {
        let mut least_importance = f64::MAX;
        let mut least_pos = None;

        for (i, item) in self.items.iter().enumerate() {
            if item.importance < least_importance {
                least_importance = item.importance;
                least_pos = Some(i);
            }
        }

        least_pos
    }

    /// Focus on item
    pub fn focus_on(&mut self, item_id: &str) {
        if self.items.iter().any(|i| i.id == item_id) {
            self.focus_item = Some(item_id.to_string());
        }
    }
}

/// Working memory item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemoryItem {
    /// Item ID
    pub id: String,
    /// Content
    pub content: String,
    /// Importance
    pub importance: f64,
    /// Activation level
    pub activation: f64,
    /// Source (which memory type)
    pub source: MemoryType,
}

/// Memory consolidation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationRecord {
    /// Record ID
    pub id: String,
    /// Source memories
    pub source_memories: Vec<String>,
    /// Consolidated memory
    pub consolidated_id: String,
    /// Consolidation type
    pub consolidation_type: ConsolidationType,
    /// Quality score
    pub quality_score: f64,
    /// Timestamp
    pub timestamp: u64,
}

/// Consolidation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsolidationType {
    /// Episodic to semantic
    EpisodicToSemantic,
    /// Short-term to long-term
    ShortToLong,
    /// Generalization
    Generalization,
    /// Abstraction
    Abstraction,
    /// Integration
    Integration,
}

/// Forgetting event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgettingEvent {
    /// Memory ID
    pub memory_id: String,
    /// Reason
    pub reason: ForgettingReason,
    /// What was lost
    pub details: String,
    /// Timestamp
    pub timestamp: u64,
}

/// Forgetting reason
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForgettingReason {
    /// Natural decay
    Decay,
    /// Interference
    Interference,
    /// Motivated forgetting
    Motivated,
    /// Failure to consolidate
    FailedConsolidation,
    /// Storage limit
    StorageLimit,
    /// Corruption
    Corruption,
}

/// Memory indexing system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryIndex {
    /// Index by tags
    pub by_tags: HashMap<String, Vec<String>>,
    /// Index by time
    pub by_time: Vec<(u64, String)>,
    /// Index by type
    pub by_type: HashMap<MemoryType, Vec<String>>,
    /// Index by location
    pub by_location: HashMap<String, Vec<String>>,
    /// Semantic index
    pub semantic: HashMap<String, Vec<String>>,
    /// Full-text index
    pub full_text: HashMap<String, Vec<String>>,
}

impl MemoryIndex {
    /// Create a new index
    pub fn new() -> Self {
        MemoryIndex {
            by_tags: HashMap::new(),
            by_time: Vec::new(),
            by_type: HashMap::new(),
            by_location: HashMap::new(),
            semantic: HashMap::new(),
            full_text: HashMap::new(),
        }
    }

    /// Add memory to index
    pub fn add_memory(&mut self, memory: &Memory) {
        // Index by tags
        for tag in &memory.metadata.tags {
            self.by_tags
                .entry(tag.clone())
                .or_insert_with(Vec::new)
                .push(memory.id.clone());
        }

        // Index by time
        self.by_time.push((memory.metadata.created_at, memory.id.clone()));

        // Index by type
        self.by_type
            .entry(memory.memory_type)
            .or_insert_with(Vec::new)
            .push(memory.id.clone());
    }

    /// Search by tag
    pub fn search_tag(&self, tag: &str) -> Vec<&String> {
        self.by_tags.get(tag).map(|v| v.iter().collect()).unwrap_or_default()
    }

    /// Search by time range
    pub fn search_time_range(&self, start: u64, end: u64) -> Vec<&String> {
        self.by_time.iter()
            .filter(|(t, _)| *t >= start && *t <= end)
            .map(|(_, id)| id)
            .collect()
    }
}

impl Default for MemoryIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory management system
pub struct MemorySystem {
    /// Long-term memories
    pub long_term: HashMap<String, Memory>,
    /// Working memory
    pub working_memory: WorkingMemoryBuffer,
    /// Episodic memories
    pub episodic: Vec<EpisodicMemory>,
    /// Semantic memory
    pub semantic: HashMap<String, SemanticConcept>,
    /// Procedural memory
    pub procedural: Vec<ProceduralMemory>,
    /// Memory index
    pub index: MemoryIndex,
    /// Consolidation queue
    pub consolidation_queue: VecDeque<String>,
    /// Recent memories (cache)
    pub recent_cache: VecDeque<String>,
    /// Forgetting log
    pub forgetting_log: Vec<ForgettingEvent>,
    /// Memory statistics
    pub stats: MemoryStats,
    /// Maximum storage
    pub max_storage: usize,
    /// Current storage usage
    pub current_storage: usize,
}

impl MemorySystem {
    /// Create a new memory system
    pub fn new() -> Self {
        MemorySystem {
            long_term: HashMap::new(),
            working_memory: WorkingMemoryBuffer::new("main", 7), // Miller's 7 +/- 2
            episodic: Vec::new(),
            semantic: HashMap::new(),
            procedural: Vec::new(),
            index: MemoryIndex::new(),
            consolidation_queue: VecDeque::new(),
            recent_cache: VecDeque::new(),
            forgetting_log: Vec::new(),
            stats: MemoryStats::default(),
            max_storage: 1_000_000, // 1 million memory slots
            current_storage: 0,
        }
    }

    /// Store a memory
    pub fn store(&mut self, memory: Memory) -> Result<()> {
        if self.current_storage >= self.max_storage {
            self.trigger_forgetting()?;
        }

        // Index the memory
        self.index.add_memory(&memory);

        // Store based on type
        match memory.memory_type {
            MemoryType::Working => {
                let item = WorkingMemoryItem {
                    id: memory.id.clone(),
                    content: match &memory.content {
                        MemoryContent::Text(s) => s.clone(),
                        _ => memory.id.clone(),
                    },
                    importance: memory.importance,
                    activation: memory.encoding_strength,
                    source: memory.memory_type,
                };
                self.working_memory.add_item(item);
            },
            MemoryType::Episodic => {
                // Convert to episodic format
                let episodic = EpisodicMemory::default();
                self.episodic.push(episodic);
            },
            _ => {
                self.long_term.insert(memory.id.clone(), memory);
                self.current_storage += 1;
            },
        }

        // Add to recent cache
        self.recent_cache.push_front(memory.id.clone());
        if self.recent_cache.len() > 100 {
            self.recent_cache.pop_back();
        }

        // Update stats
        self.stats.total_memories += 1;

        Ok(())
    }

    /// Retrieve a memory
    pub fn retrieve(&mut self, memory_id: &str) -> Option<&Memory> {
        // Update working memory access
        if let Some(memory) = self.long_term.get_mut(memory_id) {
            memory.access();
            return Some(memory);
        }

        // Check recent cache
        if self.recent_cache.contains(&memory_id.to_string()) {
            // Move to front of cache
            self.recent_cache.retain(|id| id != memory_id);
            self.recent_cache.push_front(memory_id.to_string());
        }

        None
    }

    /// Search memories
    pub fn search(&self, query: &str, memory_type: Option<MemoryType>, limit: usize) -> Vec<&Memory> {
        let mut results: Vec<&Memory> = self.long_term.values()
            .filter(|m| {
                // Check text content
                let content_match = match &m.content {
                    MemoryContent::Text(s) => s.to_lowercase().contains(&query.to_lowercase()),
                    _ => false,
                };

                // Check tags
                let tag_match = m.metadata.tags.iter()
                    .any(|t| t.to_lowercase().contains(&query.to_lowercase()));

                content_match || tag_match
            })
            .filter(|m| {
                memory_type.map_or(true, |t| m.memory_type == t)
            })
            .collect();

        // Sort by relevance (access count and recency)
        results.sort_by(|a, b| {
            let score_a = a.access_count as f64 * a.encoding_strength;
            let score_b = b.access_count as f64 * b.encoding_strength;
            score_b.partial_cmp(&score_a).unwrap()
        });

        results.truncate(limit);
        results
    }

    /// Trigger forgetting process
    fn trigger_forgetting(&mut self) -> Result<()> {
        if self.long_term.is_empty() {
            return Ok(());
        }

        // Find memories below threshold
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut to_forget: Vec<String> = self.long_term.iter()
            .filter(|(_, m)| {
                m.calculate_decay(current_time) < m.forgetting_threshold
            })
            .map(|(id, _)| id.clone())
            .collect();

        // If no memories below threshold, remove lowest importance
        if to_forget.is_empty() {
            let mut memories: Vec<_> = self.long_term.iter().collect();
            memories.sort_by(|a, b| a.1.importance.partial_cmp(&b.1.importance).unwrap());

            if let Some((id, _)) = memories.first() {
                to_forget.push(id.clone());
            }
        }

        // Forget memories
        for memory_id in &to_forget {
            if let Some(memory) = self.long_term.remove(memory_id) {
                self.forgetting_log.push(ForgettingEvent {
                    memory_id: memory_id.clone(),
                    reason: ForgettingReason::StorageLimit,
                    details: format!("Memory importance: {}", memory.importance),
                    timestamp: current_time,
                });
                self.current_storage -= 1;
                self.stats.total_forgotten += 1;
            }
        }

        Ok(())
    }

    /// Consolidate memories
    pub fn consolidate(&mut self) -> Result<()> {
        while let Some(memory_id) = self.consolidation_queue.pop_front() {
            if let Some(memory) = self.long_term.get_mut(&memory_id) {
                // Strengthen encoding
                memory.encoding_strength = (memory.encoding_strength + 0.05).min(1.0);
                memory.forgetting_threshold *= 0.95; // Harder to forget
            }
        }

        self.stats.consolidations_performed += 1;
        Ok(())
    }

    /// Add semantic concept
    pub fn add_concept(&mut self, concept: SemanticConcept) {
        self.semantic.insert(concept.name.clone(), concept);
    }

    /// Retrieve semantic concept
    pub fn get_concept(&self, name: &str) -> Option<&SemanticConcept> {
        self.semantic.get(name)
    }

    /// Add procedural memory
    pub fn add_procedure(&mut self, procedure: ProceduralMemory) {
        self.procedural.push(procedure);
    }

    /// Find procedure
    pub fn find_procedure(&self, name: &str) -> Option<&ProceduralMemory> {
        self.procedural.iter().find(|p| p.name == name)
    }

    /// Record experience
    pub fn record_experience(&mut self, episodic: EpisodicMemory) {
        self.episodic.push(episodic.clone());
        self.stats.total_experiences += 1;

        // Check if consolidation needed
        if self.stats.total_experiences % 10 == 0 {
            self.consolidation_queue.push_back(self.episodic.last().unwrap().event.clone());
        }
    }

    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        self.stats.clone()
    }

    /// Apply forgetting
    pub fn apply_decay(&mut self) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        for memory in self.long_term.values_mut() {
            memory.retrieval_strength = memory.calculate_decay(current_time);
        }
    }
}

impl Default for MemorySystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Total memories stored
    pub total_memories: u64,
    /// Total experiences recorded
    pub total_experiences: u64,
    /// Total forgotten
    pub total_forgotten: u64,
    /// Consolidations performed
    pub consolidations_performed: u64,
    /// Average access count
    pub average_access_count: f64,
    /// Memory utilization
    pub utilization: f64,
}

impl Default for MemoryStats {
    fn default() -> Self {
        MemoryStats {
            total_memories: 0,
            total_experiences: 0,
            total_forgotten: 0,
            consolidations_performed: 0,
            average_access_count: 0.0,
            utilization: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_creation() {
        let memory = Memory::new(
            "mem1",
            MemoryType::Semantic,
            MemoryContent::Text("Test content".to_string()),
        );

        assert_eq!(memory.id, "mem1");
        assert_eq!(memory.memory_type, MemoryType::Semantic);
        assert_eq!(memory.encoding_strength, 1.0);
    }

    #[test]
    fn test_memory_access() {
        let mut memory = Memory::new(
            "mem1",
            MemoryType::Episodic,
            MemoryContent::Text("Test".to_string()),
        );

        memory.access();
        assert_eq!(memory.access_count, 1);
    }

    #[test]
    fn test_working_memory_buffer() {
        let mut buffer = WorkingMemoryBuffer::new("test", 3);
        buffer.add_item(WorkingMemoryItem {
            id: "item1".to_string(),
            content: "Test".to_string(),
            importance: 0.8,
            activation: 0.9,
            source: MemoryType::Working,
        });

        assert_eq!(buffer.items.len(), 1);
    }

    #[test]
    fn test_semantic_concept() {
        let mut concept = SemanticConcept::new("dog", "A domesticated carnivorous mammal");
        concept.add_attribute("has_tail", "true");
        concept.add_attribute("is_mammal", "true");

        assert_eq!(concept.attributes.len(), 2);
    }

    #[test]
    fn test_procedural_memory() {
        let mut procedure = ProceduralMemory::new("Make coffee");
        procedure.add_step(ProcedureStep::new(1, "Boil water"));
        procedure.add_step(ProcedureStep::new(2, "Add coffee grounds"));

        procedure.record_execution(true, 120.0);
        assert_eq!(procedure.success_rate, 1.0);
        assert_eq!(procedure.times_performed, 1);
    }

    #[test]
    fn test_memory_index() {
        let mut index = MemoryIndex::new();
        let mut memory = Memory::new(
            "mem1",
            MemoryType::Semantic,
            MemoryContent::Text("Test".to_string()),
        );
        memory.metadata.tags.push("test".to_string());

        index.add_memory(&memory);
        let results = index.search_tag("test");

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_memory_system() {
        let mut system = MemorySystem::new();

        let memory = Memory::new(
            "mem1",
            MemoryType::LongTerm,
            MemoryContent::Text("Important fact".to_string()),
        );

        system.store(memory).unwrap();
        let retrieved = system.retrieve("mem1");

        assert!(retrieved.is_some());
    }

    #[test]
    fn test_forgetting_threshold() {
        let mut memory = Memory::new(
            "mem1",
            MemoryType::LongTerm,
            MemoryContent::Text("Test".to_string()),
        );
        memory.importance = 0.9;
        memory.forgetting_threshold = 0.1;

        // After long time
        let decay = memory.calculate_decay(memory.last_accessed + 1000000);
        assert!(decay >= 0.1);
    }

    #[test]
    fn test_consolidation_queue() {
        let mut system = MemorySystem::new();
        system.consolidation_queue.push_back("memory1".to_string());

        assert_eq!(system.consolidation_queue.len(), 1);
        system.consolidate().unwrap();
        assert_eq!(system.consolidation_queue.len(), 0);
    }
}
