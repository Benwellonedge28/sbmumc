//! Creativity & Imagination Engine Module
//!
//! This module implements creative problem solving, artistic generation,
//! novel concept synthesis, what-if scenario imagination, and innovation
//! through combination.
//!
//! Features:
//! - Creative problem solving
//! - Artistic generation (music, art, literature)
//! - Novel concept synthesis
//! - What-if scenario imagination
//! - Innovation through combination

use crate::core::{SbmumcError, Result, EntityId, SecurityLevel};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Creativity type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreativityType {
    /// Divergent thinking
    Divergent,
    /// Convergent thinking
    Convergent,
    /// Lateral thinking
    Lateral,
    /// Analogical thinking
    Analogical,
    /// Transformational
    Transformational,
    /// Combinational
    Combinational,
    /// Exploratory
    Exploratory,
}

/// Creative domain
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreativeDomain {
    /// Visual art
    VisualArt,
    /// Music
    Music,
    /// Literature
    Literature,
    /// Science
    Science,
    /// Engineering
    Engineering,
    /// Mathematics
    Mathematics,
    /// ProblemSolving,
    ProblemSolving,
    /// General
    General,
}

/// Creative idea
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeIdea {
    /// Idea identifier
    pub id: String,
    /// Idea description
    pub description: String,
    /// Domain
    pub domain: CreativeDomain,
    /// Originality score
    pub originality: f64,
    /// Utility score
    pub utility: f64,
    /// Feasibility score
    pub feasibility: f64,
    /// Surprise score
    pub surprise: f64,
    /// Combined elements
    pub elements: Vec<String>,
    /// Inspiration sources
    pub inspiration: Vec<String>,
    /// Refinement iterations
    pub refinements: u32,
    /// Creation timestamp
    pub timestamp: u64,
}

impl CreativeIdea {
    /// Create a new creative idea
    pub fn new(id: &str, description: &str, domain: CreativeDomain) -> Self {
        CreativeIdea {
            id: id.to_string(),
            description: description.to_string(),
            domain,
            originality: 0.5,
            utility: 0.5,
            feasibility: 0.5,
            surprise: 0.5,
            elements: Vec::new(),
            inspiration: Vec::new(),
            refinements: 0,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Calculate overall creativity score
    pub fn creativity_score(&self) -> f64 {
        // SCAMPER-based creativity scoring
        let sum = self.originality + self.utility + self.feasibility + self.surprise;
        sum / 4.0
    }

    /// Add element
    pub fn add_element(&mut self, element: &str) {
        self.elements.push(element.to_string());
    }

    /// Add inspiration
    pub fn add_inspiration(&mut self, source: &str) {
        self.inspiration.push(source.to_string());
    }

    /// Refine the idea
    pub fn refine(&mut self, improvements: &str) {
        self.refinements += 1;
        // In real implementation, would apply improvements
        self.originality = (self.originality + 0.1).min(1.0);
        self.utility = (self.utility + 0.1).min(1.0);
    }
}

/// Concept combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptCombination {
    /// Concept A
    pub concept_a: ConceptRepresentation,
    /// Concept B
    pub concept_b: ConceptRepresentation,
    /// Novel properties
    pub novel_properties: Vec<NovelProperty>,
    /// Combined description
    pub combined_description: String,
    /// Innovation score
    pub innovation_score: f64,
}

impl ConceptCombination {
    /// Create a new combination
    pub fn new(concept_a: ConceptRepresentation, concept_b: ConceptRepresentation) -> Self {
        ConceptCombination {
            concept_a,
            concept_b,
            novel_properties: Vec::new(),
            combined_description: String::new(),
            innovation_score: 0.5,
        }
    }
}

/// Concept representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptRepresentation {
    /// Concept name
    pub name: String,
    /// Attributes
    pub attributes: Vec<String>,
    /// Relationships
    pub relationships: Vec<String>,
    /// Category
    pub category: String,
}

impl ConceptRepresentation {
    /// Create a new concept
    pub fn new(name: &str) -> Self {
        ConceptRepresentation {
            name: name.to_string(),
            attributes: Vec::new(),
            relationships: Vec::new(),
            category: "General".to_string(),
        }
    }

    /// Add attribute
    pub fn add_attribute(&mut self, attr: &str) {
        self.attributes.push(attr.to_string());
    }
}

/// Novel property from combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelProperty {
    /// Property name
    pub name: String,
    /// Description
    pub description: String,
    /// Emergence type
    pub emergence_type: EmergenceType,
    /// Unexpectedness score
    pub unexpectedness: f64,
}

impl NovelProperty {
    /// Create a new property
    pub fn new(name: &str, description: &str) -> Self {
        NovelProperty {
            name: name.to_string(),
            description: description.to_string(),
            emergence_type: EmergenceType::Combinational,
            unexpectedness: 0.5,
        }
    }
}

/// Emergence type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmergenceType {
    /// Combinational - combines existing elements
    Combinational,
    /// Exploratory - explores a conceptual space
    Exploratory,
    /// Transformational - changes constraints
    Transformational,
}

/// What-if scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatIfScenario {
    /// Scenario identifier
    pub id: String,
    /// Premise
    pub premise: String,
    /// Modifications
    pub modifications: Vec<String>,
    /// Predicted consequences
    pub consequences: Vec<Consequence>,
    /// Probability
    pub probability: f64,
    /// Impact score
    pub impact_score: f64,
    /// Creativity required
    pub creativity_required: f64,
}

impl WhatIfScenario {
    /// Create a new scenario
    pub fn new(id: &str, premise: &str) -> Self {
        WhatIfScenario {
            id: id.to_string(),
            premise: premise.to_string(),
            modifications: Vec::new(),
            consequences: Vec::new(),
            probability: 0.5,
            impact_score: 0.5,
            creativity_required: 0.5,
        }
    }

    /// Add modification
    pub fn add_modification(&mut self, modif: &str) {
        self.modifications.push(modif.to_string());
    }

    /// Add consequence
    pub fn add_consequence(&mut self, consequence: Consequence) {
        self.consequences.push(consequence);
    }
}

/// Consequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consequence {
    /// Consequence description
    pub description: String,
    /// Type
    pub consequence_type: ConsequenceType,
    /// Likelihood
    pub likelihood: f64,
    /// Significance
    pub significance: f64,
    /// Chain of events
    pub chain: Vec<String>,
}

impl Consequence {
    /// Create a new consequence
    pub fn new(description: &str, consequence_type: ConsequenceType) -> Self {
        Consequence {
            description: description.to_string(),
            consequence_type,
            likelihood: 0.5,
            significance: 0.5,
            chain: Vec::new(),
        }
    }
}

/// Consequence type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsequenceType {
    /// Direct effect
    Direct,
    /// Ripple effect
    Ripple,
    /// Cascading
    Cascading,
    /// Unintended
    Unintended,
    /// Long-term
    LongTerm,
    /// Social
    Social,
    /// Environmental
    Environmental,
}

/// Artistic element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtisticElement {
    /// Element type
    pub element_type: ArtisticElementType,
    /// Content
    pub content: String,
    /// Style
    pub style: String,
    /// Emotional impact
    pub emotional_impact: f64,
    /// Technical score
    pub technical_score: f64,
}

impl ArtisticElement {
    /// Create a new element
    pub fn new(element_type: ArtisticElementType, content: &str) -> Self {
        ArtisticElement {
            element_type,
            content: content.to_string(),
            style: "neutral".to_string(),
            emotional_impact: 0.5,
            technical_score: 0.5,
        }
    }
}

/// Artistic element type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtisticElementType {
    /// Color palette
    ColorPalette,
    /// Composition
    Composition,
    /// Rhythm
    Rhythm,
    /// Melody
    Melody,
    /// Harmony
    Harmony,
    /// Narrative,
    Narrative,
    /// Character,
    Character,
    /// Setting,
    Setting,
    /// Theme,
    Theme,
    /// Metaphor,
    Metaphor,
    /// Symbol,
    Symbol,
}

/// Problem representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemRepresentation {
    /// Problem name
    pub name: String,
    /// Initial state
    pub initial_state: Vec<String>,
    /// Goal state
    pub goal_state: Vec<String>,
    /// Constraints
    pub constraints: Vec<String>,
    /// Operators
    pub operators: Vec<Operator>,
    /// Current dead ends
    pub dead_ends: Vec<Vec<String>>,
}

impl ProblemRepresentation {
    /// Create a new problem
    pub fn new(name: &str) -> Self {
        ProblemRepresentation {
            name: name.to_string(),
            initial_state: Vec::new(),
            goal_state: Vec::new(),
            constraints: Vec::new(),
            operators: Vec::new(),
            dead_ends: Vec::new(),
        }
    }

    /// Add initial state element
    pub fn add_initial(&mut self, state: &str) {
        self.initial_state.push(state.to_string());
    }

    /// Add goal element
    pub fn add_goal(&mut self, goal: &str) {
        self.goal_state.push(goal.to_string());
    }

    /// Add constraint
    pub fn add_constraint(&mut self, constraint: &str) {
        self.constraints.push(constraint.to_string());
    }
}

/// Operator for problem solving
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operator {
    /// Operator name
    pub name: String,
    /// Preconditions
    pub preconditions: Vec<String>,
    /// Effects
    pub effects: Vec<String>,
    /// Cost
    pub cost: f64,
}

impl Operator {
    /// Create a new operator
    pub fn new(name: &str) -> Self {
        Operator {
            name: name.to_string(),
            preconditions: Vec::new(),
            effects: Vec::new(),
            cost: 1.0,
        }
    }
}

/// Brainstorming session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainstormingSession {
    /// Session identifier
    pub id: String,
    /// Topic
    pub topic: String,
    /// Ideas generated
    pub ideas: Vec<CreativeIdea>,
    /// Techniques used
    pub techniques: Vec<BrainstormingTechnique>,
    /// Duration
    pub duration_seconds: u64,
    /// Participants
    pub participants: u32,
    /// Quality score
    pub quality_score: f64,
    /// Quantity score
    pub quantity_score: f64,
}

impl BrainstormingSession {
    /// Create a new session
    pub fn new(id: &str, topic: &str) -> Self {
        BrainstormingSession {
            id: id.to_string(),
            topic: topic.to_string(),
            ideas: Vec::new(),
            techniques: Vec::new(),
            duration_seconds: 0,
            participants: 1,
            quality_score: 0.0,
            quantity_score: 0.0,
        }
    }

    /// Add idea
    pub fn add_idea(&mut self, idea: CreativeIdea) {
        self.ideas.push(idea);
        self.quantity_score = self.ideas.len() as f64;
    }

    /// Calculate quality
    pub fn calculate_quality(&mut self) {
        if self.ideas.is_empty() {
            self.quality_score = 0.0;
            return;
        }

        let sum: f64 = self.ideas.iter()
            .map(|i| i.creativity_score())
            .sum();

        self.quality_score = sum / self.ideas.len() as f64;
    }
}

/// Brainstorming technique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrainstormingTechnique {
    /// Classical brainstorming
    Classical,
    /// Reverse brainstorming
    Reverse,
    /// SCAMPER
    SCAMPER,
    /// Mind mapping
    MindMap,
    /// Six thinking hats
    SixThinkingHats,
    /// Random input
    RandomInput,
    /// Analogy,
    Analogy,
    /// Association,
    Association,
}

/// SCAMPER technique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCAMPER {
    /// Substitute elements
    pub substitute: Vec<String>,
    /// Combine elements
    pub combine: Vec<ConceptCombination>,
    /// Adapt elements
    pub adapt: Vec<String>,
    /// Modify elements
    pub modify: Vec<String>,
    /// Put to other use
    pub put_to_other_use: Vec<String>,
    /// Eliminate elements
    pub eliminate: Vec<String>,
    /// Rearrange elements
    pub rearrange: Vec<String>,
}

impl SCAMPER {
    /// Create new SCAMPER
    pub fn new() -> Self {
        SCAMPER {
            substitute: Vec::new(),
            combine: Vec::new(),
            adapt: Vec::new(),
            modify: Vec::new(),
            put_to_other_use: Vec::new(),
            eliminate: Vec::new(),
            rearrange: Vec::new(),
        }
    }
}

impl Default for SCAMPER {
    fn default() -> Self {
        Self::new()
    }
}

/// Lateral thinking move
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LateralThinkingMove {
    /// Move type
    pub move_type: LateralMoveType,
    /// Description
    pub description: String,
    /// Target dead end
    pub targets_dead_end: Option<String>,
    /// Effect on problem
    pub effect: String,
}

impl LateralThinkingMove {
    /// Create a new move
    pub fn new(move_type: LateralMoveType, description: &str) -> Self {
        LateralThinkingMove {
            move_type,
            description: description.to_string(),
            targets_dead_end: None,
            effect: String::new(),
        }
    }
}

/// Lateral move type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LateralMoveType {
    /// Escape from local optimum
    Escape,
    /// Challenge assumptions
    ChallengeAssumptions,
    /// Divide and reassemble
    DivideAndReassemble,
    /// Use random input
    UseRandom,
    /// Analogy
    UseAnalogy,
    /// Chunk and connect
    ChunkAndConnect,
    /// Reversal
    Reversal,
}

/// Creative engine
pub struct CreativityEngine {
    /// Idea database
    pub ideas: Vec<CreativeIdea>,
    /// Concept database
    pub concepts: HashMap<String, ConceptRepresentation>,
    /// Problem history
    pub problems: Vec<ProblemRepresentation>,
    /// Brainstorming sessions
    pub sessions: Vec<BrainstormingSession>,
    /// Creativity parameters
    pub params: CreativityParams,
    /// Innovation metrics
    pub metrics: InnovationMetrics,
    /// Inspiration sources
    pub inspiration_sources: Vec<InspirationSource>,
}

impl CreativityEngine {
    /// Create a new creativity engine
    pub fn new() -> Self {
        let mut engine = CreativityEngine {
            ideas: Vec::new(),
            concepts: HashMap::new(),
            problems: Vec::new(),
            sessions: Vec::new(),
            params: CreativityParams::default(),
            metrics: InnovationMetrics::default(),
            inspiration_sources: Vec::new(),
        };

        engine.initialize_concepts();
        engine.initialize_inspiration();

        engine
    }

    /// Initialize basic concepts
    fn initialize_concepts(&mut self) {
        let concepts = vec![
            ("nature", vec!["growth", "cycles", "adaptation"], "environment"),
            ("technology", vec!["efficiency", "automation", "connectivity"], "innovation"),
            ("society", vec!["relationships", "culture", "institutions"], "social"),
            ("art", vec!["expression", "beauty", "emotion"], "creative"),
            ("science", vec!["knowledge", "method", "discovery"], "intellectual"),
        ];

        for (name, attrs, category) in concepts {
            let mut concept = ConceptRepresentation::new(name);
            for attr in attrs {
                concept.add_attribute(attr);
            }
            concept.category = category.to_string();
            self.concepts.insert(name.to_string(), concept);
        }
    }

    /// Initialize inspiration sources
    fn initialize_inspiration(&mut self) {
        self.inspiration_sources.push(InspirationSource {
            name: "Nature".to_string(),
            description: "Natural patterns and systems".to_string(),
            domain: CreativeDomain::Engineering,
            relevance: 0.8,
        });

        self.inspiration_sources.push(InspirationSource {
            name: "Art History".to_string(),
            description: "Historical artistic movements".to_string(),
            domain: CreativeDomain::VisualArt,
            relevance: 0.9,
        });

        self.inspiration_sources.push(InspirationSource {
            name: "Science Fiction".to_string(),
            description: "Futuristic concepts".to_string(),
            domain: CreativeDomain::General,
            relevance: 0.7,
        });
    }

    /// Generate random creative idea
    pub fn generate_random_idea(&mut self, domain: CreativeDomain) -> CreativeIdea {
        let id = format!("idea_{}", self.ideas.len());
        let descriptions = vec![
            "A novel approach using unexpected combinations",
            "An innovative solution combining technology and nature",
            "A creative interpretation of an existing concept",
            "An imaginative solution to a common problem",
            "A fresh perspective on traditional methods",
        ];

        let mut idea = CreativeIdea::new(
            &id,
            descriptions[rand::random::<usize>() % descriptions.len()],
            domain,
        );

        // Add random elements
        let concepts: Vec<_> = self.concepts.keys().collect();
        if let Some(&concept) = concepts.get(rand::random::<usize>() % concepts.len()) {
            idea.add_element(concept);
            idea.add_inspiration(concept);
        }

        // Generate random scores
        idea.originality = rand::random::<f64>();
        idea.utility = rand::random::<f64>();
        idea.feasibility = rand::random::<f64>();
        idea.surprise = rand::random::<f64>();

        self.ideas.push(idea.clone());
        idea
    }

    /// Combine concepts
    pub fn combine_concepts(&mut self, concept_a: &str, concept_b: &str) -> Option<ConceptCombination> {
        let a = self.concepts.get(concept_a)?;
        let b = self.concepts.get(concept_b)?;

        let mut combination = ConceptCombination::new(a.clone(), b.clone());

        // Find novel properties
        let mut novel_props = Vec::new();

        // Combine attributes
        for attr_a in &a.attributes {
            for attr_b in &b.attributes {
                let novel = format!("{} + {}", attr_a, attr_b);
                novel_props.push(NovelProperty::new(&novel, &format!("Novel combination of {} and {}", attr_a, attr_b)));
            }
        }

        combination.novel_properties = novel_props;
        combination.combined_description = format!(
            "{} with {}: {}",
            a.name,
            b.name,
            combination.novel_properties.iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        );

        // Calculate innovation score
        combination.innovation_score = (combination.novel_properties.len() as f64 * 0.2).min(1.0);

        Some(combination)
    }

    /// Generate what-if scenarios
    pub fn generate_whatif(&self, base_concept: &str, modifications: &[String]) -> Vec<WhatIfScenario> {
        let mut scenarios = Vec::new();

        for modif in modifications {
            let id = format!("whatif_{}_{}", base_concept, self.scenarios_generated());
            let mut scenario = WhatIfScenario::new(&id, &format!("What if we {}?", modif));
            scenario.add_modification(modif);

            // Generate consequences
            let cons = Consequence::new(
                &format!("Primary effect of {}", modif),
                ConsequenceType::Direct,
            );
            scenario.add_consequence(cons);

            scenarios.push(scenario);
        }

        scenarios
    }

    fn scenarios_generated(&self) -> usize {
        // This would track scenarios - simplified
        0
    }

    /// Apply lateral thinking
    pub fn lateral_think(&self, problem: &ProblemRepresentation) -> Vec<LateralThinkingMove> {
        let mut moves = Vec::new();

        // Challenge assumptions
        for constraint in &problem.constraints {
            moves.push(LateralThinkingMove::new(
                LateralMoveType::ChallengeAssumptions,
                &format!("What if '{}' wasn't true?", constraint),
            ));
        }

        // Reversal
        moves.push(LateralThinkingMove::new(
            LateralMoveType::Reversal,
            &format!("How might we achieve the opposite of {}", problem.goal_state.join(" and ")),
        ));

        // Random input
        let concepts: Vec<_> = self.concepts.keys().collect();
        if let Some(random_concept) = concepts.get(rand::random::<usize>() % concepts.len()) {
            moves.push(LateralThinkingMove::new(
                LateralMoveType::UseRandom,
                &format!("How might '{}' help solve this?", random_concept),
            ));
        }

        moves
    }

    /// Apply SCAMPER technique
    pub fn apply_scampper(&self, target: &str) -> SCAMPER {
        let mut scamper = SCAMPER::new();

        // Substitute
        let concepts: Vec<_> = self.concepts.keys().collect();
        if let Some(&sub) = concepts.get(rand::random::<usize>() % concepts.len()) {
            scamper.substitute.push(format!("Substitute {} with {}", target, sub));
        }

        // Combine
        if let Some(&other) = concepts.get(rand::random::<usize>() % concepts.len()) {
            if other != target {
                scamper.combine.push(ConceptCombination::new(
                    ConceptRepresentation::new(target),
                    ConceptRepresentation::new(other),
                ));
            }
        }

        // Adapt
        scamper.adapt.push(format!("Adapt {} for new context", target));

        // Modify
        scamper.modify.push(format!("Modify scale/quantity of {}", target));

        // Put to other use
        scamper.put_to_other_use.push(format!("Find new use for {}", target));

        // Eliminate
        scamper.eliminate.push(format!("What if we removed {}?", target));

        // Rearrange
        scamper.rearrange.push(format!("Rearrange {} elements", target));

        scamper
    }

    /// Start brainstorming session
    pub fn start_brainstorming(&mut self, topic: &str) -> BrainstormingSession {
        let id = format!("session_{}", self.sessions.len());
        BrainstormingSession::new(&id, topic)
    }

    /// Generate multiple ideas
    pub fn brainstorm(&mut self, topic: &str, count: usize, domain: CreativeDomain) -> BrainstormingSession {
        let mut session = self.start_brainstorming(topic);

        // Apply techniques
        session.techniques.push(BrainstormingTechnique::Classical);
        session.techniques.push(BrainstormingTechnique::SCAMPER);
        session.techniques.push(BrainstormingTechnique::RandomInput);

        // Generate ideas
        for _ in 0..count {
            let idea = self.generate_random_idea(domain);
            session.add_idea(idea);
        }

        session.calculate_quality();
        self.sessions.push(session.clone());
        session
    }

    /// Generate artistic content
    pub fn generate_artistic(&self, domain: CreativeDomain, theme: &str) -> Vec<ArtisticElement> {
        let mut elements = Vec::new();

        match domain {
            CreativeDomain::VisualArt => {
                elements.push(ArtisticElement::new(
                    ArtisticElementType::ColorPalette,
                    "Warm, vibrant colors",
                ));
                elements.push(ArtisticElement::new(
                    ArtisticElementType::Composition,
                    "Dynamic balance",
                ));
            },
            CreativeDomain::Music => {
                elements.push(ArtisticElement::new(
                    ArtisticElementType::Melody,
                    "Flowing melodic line",
                ));
                elements.push(ArtisticElement::new(
                    ArtisticElementType::Harmony,
                    "Rich harmonic structure",
                ));
            },
            CreativeDomain::Literature => {
                elements.push(ArtisticElement::new(
                    ArtisticElementType::Character,
                    "Complex protagonist",
                ));
                elements.push(ArtisticElement::new(
                    ArtisticElementType::Setting,
                    "Evocative atmosphere",
                ));
                elements.push(ArtisticElement::new(
                    ArtisticElementType::Theme,
                    theme,
                ));
            },
            _ => {},
        }

        elements
    }

    /// Get creative inspiration
    pub fn get_inspiration(&self, domain: CreativeDomain) -> Vec<&InspirationSource> {
        self.inspiration_sources.iter()
            .filter(|s| s.domain == domain || s.domain == CreativeDomain::General)
            .collect()
    }

    /// Refine idea
    pub fn refine_idea(&mut self, idea_id: &str, improvements: &str) -> Option<&CreativeIdea> {
        if let Some(idea) = self.ideas.iter_mut().find(|i| i.id == idea_id) {
            idea.refine(improvements);
            return Some(idea);
        }
        None
    }

    /// Evaluate creativity
    pub fn evaluate_creativity(&self, idea: &CreativeIdea) -> CreativityEvaluation {
        CreativityEvaluation {
            overall_score: idea.creativity_score(),
            originality_adequate: idea.originality > 0.5,
            utility_adequate: idea.utility > 0.5,
            feasibility_adequate: idea.feasibility > 0.5,
            surprise_adequate: idea.surprise > 0.5,
            recommendations: self.generate_recommendations(idea),
        }
    }

    /// Generate recommendations
    fn generate_recommendations(&self, idea: &CreativeIdea) -> Vec<String> {
        let mut recs = Vec::new();

        if idea.originality < 0.5 {
            recs.push("Increase originality by exploring less conventional approaches".to_string());
        }
        if idea.utility < 0.5 {
            recs.push("Strengthen practical value and applicability".to_string());
        }
        if idea.feasibility < 0.5 {
            recs.push("Address feasibility concerns or break down into smaller steps".to_string());
        }
        if idea.surprise < 0.5 {
            recs.push("Add unexpected elements to increase novelty".to_string());
        }

        recs
    }

    /// Get creativity statistics
    pub fn get_stats(&self) -> CreativityStats {
        let total = self.ideas.len();
        let avg_originality: f64 = if total > 0 {
            self.ideas.iter().map(|i| i.originality).sum::<f64>() / total as f64
        } else {
            0.0
        };

        CreativityStats {
            total_ideas: total,
            average_originality: avg_originality,
            sessions_completed: self.sessions.len(),
            concepts_combined: self.concepts.len(),
            inspiration_sources: self.inspiration_sources.len(),
        }
    }
}

impl Default for CreativityEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Creativity parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativityParams {
    /// Originality weight
    pub originality_weight: f64,
    /// Utility weight
    pub utility_weight: f64,
    /// Feasibility weight
    pub feasibility_weight: f64,
    /// Surprise weight
    pub surprise_weight: f64,
    /// Maximum iterations
    pub max_iterations: u32,
    /// Convergence threshold
    pub convergence_threshold: f64,
}

impl Default for CreativityParams {
    fn default() -> Self {
        CreativityParams {
            originality_weight: 0.3,
            utility_weight: 0.3,
            feasibility_weight: 0.2,
            surprise_weight: 0.2,
            max_iterations: 100,
            convergence_threshold: 0.8,
        }
    }
}

/// Innovation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnovationMetrics {
    /// Total breakthroughs
    pub breakthroughs: u32,
    /// Ideas implemented
    pub implemented: u32,
    /// Ideas patented
    pub patented: u32,
    /// Ideas published
    pub published: u32,
    /// Average time to insight
    pub avg_insight_time: f64,
}

impl Default for InnovationMetrics {
    fn default() -> Self {
        InnovationMetrics {
            breakthroughs: 0,
            implemented: 0,
            patented: 0,
            published: 0,
            avg_insight_time: 3600.0,
        }
    }
}

/// Inspiration source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspirationSource {
    /// Source name
    pub name: String,
    /// Description
    pub description: String,
    /// Domain
    pub domain: CreativeDomain,
    /// Relevance score
    pub relevance: f64,
}

/// Creativity evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativityEvaluation {
    /// Overall score
    pub overall_score: f64,
    /// Is originality adequate
    pub originality_adequate: bool,
    /// Is utility adequate
    pub utility_adequate: bool,
    /// Is feasibility adequate
    pub feasibility_adequate: bool,
    /// Is surprise adequate
    pub surprise_adequate: bool,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Creativity statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativityStats {
    /// Total ideas generated
    pub total_ideas: usize,
    /// Average originality
    pub average_originality: f64,
    /// Sessions completed
    pub sessions_completed: usize,
    /// Concepts combined
    pub concepts_combined: usize,
    /// Inspiration sources
    pub inspiration_sources: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creative_idea() {
        let mut idea = CreativeIdea::new("test_1", "A creative solution", CreativeDomain::ProblemSolving);
        idea.originality = 0.8;
        idea.utility = 0.9;
        idea.feasibility = 0.7;
        idea.surprise = 0.6;

        assert!(idea.creativity_score() > 0.7);
    }

    #[test]
    fn test_concept_combination() {
        let engine = CreativityEngine::new();
        let mut a = ConceptRepresentation::new("A");
        a.add_attribute("fast");

        let mut b = ConceptRepresentation::new("B");
        b.add_attribute("smart");

        let combination = ConceptCombination::new(a, b);
        assert!(!combination.combined_description.is_empty() || combination.novel_properties.is_empty());
    }

    #[test]
    fn test_lateral_thinking() {
        let engine = CreativityEngine::new();
        let mut problem = ProblemRepresentation::new("Test Problem");
        problem.add_constraint("Must be fast");
        problem.add_goal("Efficient solution");

        let moves = engine.lateral_think(&problem);
        assert!(!moves.is_empty());
    }

    #[test]
    fn test_scampper() {
        let engine = CreativityEngine::new();
        let scamper = engine.apply_scampper("current_concept");

        assert!(!scamper.substitute.is_empty());
        assert!(!scamper.combine.is_empty());
    }

    #[test]
    fn test_brainstorming() {
        let mut engine = CreativityEngine::new();
        let session = engine.brainstorm("Innovation topic", 5, CreativeDomain::General);

        assert_eq!(session.ideas.len(), 5);
        assert!(session.quality_score >= 0.0);
    }

    #[test]
    fn test_artistic_generation() {
        let engine = CreativityEngine::new();
        let elements = engine.generate_artistic(CreativeDomain::Music, "joy");

        assert!(!elements.is_empty());
    }

    #[test]
    fn test_creativity_stats() {
        let engine = CreativityEngine::new();
        let stats = engine.get_stats();

        assert_eq!(stats.total_ideas, 0);
    }

    #[test]
    fn test_whatif_scenario() {
        let engine = CreativityEngine::new();
        let scenarios = engine.generate_whatif("AI", &["becomes conscious".to_string(), "replaces jobs".to_string()]);

        assert_eq!(scenarios.len(), 2);
    }

    #[test]
    fn test_problem_representation() {
        let mut problem = ProblemRepresentation::new("Test");
        problem.add_initial("State A");
        problem.add_goal("State B");
        problem.add_constraint("Constraint 1");

        assert_eq!(problem.initial_state.len(), 1);
        assert_eq!(problem.goal_state.len(), 1);
        assert_eq!(problem.constraints.len(), 1);
    }

    #[test]
    fn test_creativity_engine() {
        let mut engine = CreativityEngine::new();
        let idea = engine.generate_random_idea(CreativeDomain::Science);

        assert!(!idea.id.is_empty());
        assert_eq!(idea.domain, CreativeDomain::Science);
    }
}
