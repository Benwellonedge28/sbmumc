//! Commonsense Reasoning Module
//!
//! This module implements commonsense knowledge base, physical intuition engine,
//! social convention modeling, default reasoning, and analogy understanding.
//!
//! Features:
//! - Commonsense knowledge base
//! - Physical intuition engine
//! - Social convention modeling
//! - Default reasoning with exceptions
//! - Analogy and metaphor understanding

use crate::core::{SbmumcError, Result, EntityId, SecurityLevel};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Knowledge type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeType {
    /// Physical property
    Physical,
    /// Social norm
    Social,
    /// Causal
    Causal,
    /// Temporal
    Temporal,
    /// Spatial
    Spatial,
    /// Mental state
    Mental,
    /// Action precondition
    Action,
    /// Goal-oriented
    Goal,
}

/// Commonsense knowledge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonsenseKnowledge {
    /// Knowledge ID
    pub id: String,
    /// Knowledge type
    pub knowledge_type: KnowledgeType,
    /// Content
    pub content: String,
    /// Confidence
    pub confidence: f64,
    /// Exceptions
    pub exceptions: Vec<String>,
    /// Sources
    pub sources: Vec<String>,
    /// Is default (typically true)
    pub is_default: bool,
    /// Generalization level
    pub generalization_level: GeneralizationLevel,
}

impl CommonsenseKnowledge {
    /// Create new knowledge
    pub fn new(id: &str, knowledge_type: KnowledgeType, content: &str) -> Self {
        CommonsenseKnowledge {
            id: id.to_string(),
            knowledge_type,
            content: content.to_string(),
            confidence: 0.8,
            exceptions: Vec::new(),
            sources: vec!["SBMUMC_Commonsense".to_string()],
            is_default: true,
            generalization_level: GeneralizationLevel::Medium,
        }
    }

    /// Check if applies to situation
    pub fn applies_to(&self, situation: &str) -> bool {
        let situation_lower = situation.to_lowercase();
        let content_lower = self.content.to_lowercase();

        // Check if content keywords are in situation
        content_lower.split_whitespace()
            .any(|word| situation_lower.contains(word))
    }
}

/// Generalization level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeneralizationLevel {
    /// Very specific
    Specific,
    /// Medium generality
    Medium,
    /// Very general
    General,
    /// Universal
    Universal,
}

/// Physical property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalProperty {
    /// Property name
    pub name: String,
    /// Value
    pub value: PropertyValue,
    /// Unit (if applicable)
    pub unit: Option<String>,
    /// Typical range
    pub typical_range: Option<(f64, f64)>,
}

impl PhysicalProperty {
    /// Create new property
    pub fn new(name: &str) -> Self {
        PhysicalProperty {
            name: name.to_string(),
            value: PropertyValue::Unknown,
            unit: None,
            typical_range: None,
        }
    }
}

/// Property value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    /// Unknown
    Unknown,
    /// Boolean value
    Boolean(bool),
    /// Numeric value
    Numeric(f64),
    /// Categorical
    Categorical(String),
    /// Ordinal
    Ordinal { value: f64, max: f64 },
}

impl Default for PropertyValue {
    fn default() -> Self {
        PropertyValue::Unknown
    }
}

/// Physical law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalLaw {
    /// Law name
    pub name: String,
    /// Description
    pub description: String,
    /// Formula (if applicable)
    pub formula: Option<String>,
    /// Applicability conditions
    pub conditions: Vec<String>,
    /// Accuracy
    pub accuracy: f64,
}

impl PhysicalLaw {
    /// Create new law
    pub fn new(name: &str, description: &str) -> Self {
        PhysicalLaw {
            name: name.to_string(),
            description: description.to_string(),
            formula: None,
            conditions: Vec::new(),
            accuracy: 0.95,
        }
    }
}

/// Social norm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNorm {
    /// Norm ID
    pub id: String,
    /// Norm description
    pub description: String,
    /// Culture context
    pub culture: String,
    /// Severity
    pub severity: NormSeverity,
    /// Exceptions
    pub exceptions: Vec<String>,
    /// Is explicit rule
    pub is_explicit: bool,
}

impl SocialNorm {
    /// Create new norm
    pub fn new(id: &str, description: &str) -> Self {
        SocialNorm {
            id: id.to_string(),
            description: description.to_string(),
            culture: "Universal".to_string(),
            severity: NormSeverity::Moderate,
            exceptions: Vec::new(),
            is_explicit: false,
        }
    }
}

/// Norm severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormSeverity {
    /// Mild social disapproval
    Mild,
    /// Moderate social sanction
    Moderate,
    /// Serious social consequence
    Serious,
    /// Critical violation
    Critical,
}

/// Default reasoning rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultRule {
    /// Rule ID
    pub id: String,
    /// Prerequisite
    pub prerequisite: String,
    /// Conclusion
    pub conclusion: String,
    /// Exception conditions
    pub exceptions: Vec<String>,
    /// Strength
    pub strength: f64,
    /// Is active
    pub is_active: bool,
}

impl DefaultRule {
    /// Create new rule
    pub fn new(id: &str, prerequisite: &str, conclusion: &str) -> Self {
        DefaultRule {
            id: id.to_string(),
            prerequisite: prerequisite.to_string(),
            conclusion: conclusion.to_string(),
            exceptions: Vec::new(),
            strength: 0.8,
            is_active: true,
        }
    }

    /// Check if applicable
    pub fn is_applicable(&self, facts: &[String]) -> bool {
        facts.iter().any(|f| f.to_lowercase().contains(&self.prerequisite.to_lowercase()))
    }

    /// Check if has exception
    pub fn has_exception(&self, facts: &[String]) -> bool {
        self.exceptions.iter()
            .any(|exc| facts.iter().any(|f| f.to_lowercase().contains(&exc.to_lowercase())))
    }
}

/// Analogy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analogy {
    /// Source domain
    pub source_domain: String,
    /// Target domain
    pub target_domain: String,
    /// Mapping
    pub mappings: Vec<AnalogyMapping>,
    /// Similarity score
    pub similarity: f64,
    /// Inference strength
    pub inference_strength: f64,
}

impl Analogy {
    /// Create new analogy
    pub fn new(source: &str, target: &str) -> Self {
        Analogy {
            source_domain: source.to_string(),
            target_domain: target.to_string(),
            mappings: Vec::new(),
            similarity: 0.0,
            inference_strength: 0.0,
        }
    }
}

/// Analogy mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalogyMapping {
    /// Source element
    pub source: String,
    /// Target element
    pub target: String,
    /// Relation preserved
    pub relation_preserved: bool,
    /// Confidence
    pub confidence: f64,
}

/// Metaphor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metaphor {
    /// Source concept
    pub source: String,
    /// Target concept
    pub target: String,
    /// Mapping type
    pub mapping_type: MetaphorMappingType,
    /// Meaning
    pub meaning: String,
    /// Is conventional
    pub is_conventional: bool,
}

impl Metaphor {
    /// Create new metaphor
    pub fn new(source: &str, target: &str, meaning: &str) -> Self {
        Metaphor {
            source: source.to_string(),
            target: target.to_string(),
            mapping_type: MetaphorMappingType::Structural,
            meaning: meaning.to_string(),
            is_conventional: false,
        }
    }
}

/// Metaphor mapping type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetaphorMappingType {
    /// Orientational (e.g., up is good)
    Orientational,
    /// Ontological (entities as objects)
    Ontological,
    /// Structural (systematic mappings)
    Structural,
    /// Image-schematic
    ImageSchematic,
}

/// Reasoning chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningChain {
    /// Chain ID
    pub id: String,
    /// Steps
    pub steps: Vec<ReasoningStep>,
    /// Confidence
    pub confidence: f64,
    /// Is sound
    pub is_sound: bool,
}

impl ReasoningChain {
    /// Create new chain
    pub fn new(id: &str) -> Self {
        ReasoningChain {
            id: id.to_string(),
            steps: Vec::new(),
            confidence: 1.0,
            is_sound: true,
        }
    }

    /// Add step
    pub fn add_step(&mut self, step: ReasoningStep) {
        self.confidence *= step.confidence;
        if !step.is_valid {
            self.is_sound = false;
        }
        self.steps.push(step);
    }
}

/// Reasoning step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    /// Step number
    pub step_number: u32,
    /// Description
    pub description: String,
    /// Rule applied
    pub rule_applied: Option<String>,
    /// Confidence
    pub confidence: f64,
    /// Is valid
    pub is_valid: bool,
}

impl ReasoningStep {
    /// Create new step
    pub fn new(step_number: u32, description: &str) -> Self {
        ReasoningStep {
            step_number,
            description: description.to_string(),
            rule_applied: None,
            confidence: 1.0,
            is_valid: true,
        }
    }
}

/// Physical intuition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalIntuition {
    /// Object type
    pub object_type: String,
    /// Properties
    pub properties: HashMap<String, PhysicalProperty>,
    /// Typical behaviors
    pub behaviors: Vec<String>,
    /// Constraints
    pub constraints: Vec<String>,
}

impl PhysicalIntuition {
    /// Create new intuition
    pub fn new(object_type: &str) -> Self {
        PhysicalIntuition {
            object_type: object_type.to_string(),
            properties: HashMap::new(),
            behaviors: Vec::new(),
            constraints: Vec::new(),
        }
    }

    /// Predict behavior
    pub fn predict_behavior(&self, action: &str) -> Vec<String> {
        let mut predictions = Vec::new();

        // Simplified behavior prediction
        for behavior in &self.behaviors {
            if action.contains("push") || action.contains("throw") {
                predictions.push(format!("Object will {}", behavior));
            }
        }

        predictions
    }
}

/// Commonsense reasoning system
pub struct CommonsenseReasoning {
    /// Knowledge base
    pub knowledge_base: Vec<CommonsenseKnowledge>,
    /// Physical laws
    pub physical_laws: Vec<PhysicalLaw>,
    /// Social norms
    pub social_norms: Vec<SocialNorm>,
    /// Default rules
    pub default_rules: Vec<DefaultRule>,
    /// Analogies
    pub analogies: Vec<Analogy>,
    /// Metaphors
    pub metaphors: Vec<Metaphor>,
    /// Physical intuitions
    pub physical_intuitions: HashMap<String, PhysicalIntuition>,
    /// Knowledge index
    knowledge_index: HashMap<KnowledgeType, Vec<usize>>,
}

impl CommonsenseReasoning {
    /// Create new system
    pub fn new() -> Self {
        let mut system = CommonsenseReasoning {
            knowledge_base: Vec::new(),
            physical_laws: Vec::new(),
            social_norms: Vec::new(),
            default_rules: Vec::new(),
            analogies: Vec::new(),
            metaphors: Vec::new(),
            physical_intuitions: HashMap::new(),
            knowledge_index: HashMap::new(),
        };

        system.initialize_knowledge();
        system
    }

    /// Initialize with common knowledge
    fn initialize_knowledge(&mut self) {
        // Physical knowledge
        self.add_knowledge(CommonsenseKnowledge::new(
            "phys_1",
            KnowledgeType::Physical,
            "Objects fall down when dropped",
        ));
        self.add_knowledge(CommonsenseKnowledge::new(
            "phys_2",
            KnowledgeType::Physical,
            "Water flows downhill",
        ));
        self.add_knowledge(CommonsenseKnowledge::new(
            "phys_3",
            KnowledgeType::Physical,
            "Fire requires oxygen to burn",
        ));
        self.add_knowledge(CommonsenseKnowledge::new(
            "phys_4",
            KnowledgeType::Physical,
            "Heavy objects are harder to move",
        ));

        // Social knowledge
        self.add_knowledge(CommonsenseKnowledge::new(
            "soc_1",
            KnowledgeType::Social,
            "People greet each other when meeting",
        ));
        self.add_knowledge(CommonsenseKnowledge::new(
            "soc_2",
            KnowledgeType::Social,
            "It's polite to say please and thank you",
        ));

        // Temporal knowledge
        self.add_knowledge(CommonsenseKnowledge::new(
            "temp_1",
            KnowledgeType::Temporal,
            "Night follows day",
        ));
        self.add_knowledge(CommonsenseKnowledge::new(
            "temp_2",
            KnowledgeType::Temporal,
            "Tasks take time to complete",
        ));

        // Causal knowledge
        self.add_knowledge(CommonsenseKnowledge::new(
            "caus_1",
            KnowledgeType::Causal,
            "Pushing something makes it move",
        ));
        self.add_knowledge(CommonsenseKnowledge::new(
            "caus_2",
            KnowledgeType::Causal,
            "Removing support causes objects to fall",
        ));

        // Physical laws
        self.add_physical_law(PhysicalLaw::new(
            "gravity",
            "Objects with mass attract each other",
        ));

        // Default rules
        self.add_default_rule(DefaultRule::new(
            "default_1",
            "bird",
            "can fly",
        ));
        self.add_default_rule(DefaultRule::new(
            "default_2",
            "fruit",
            "is edible",
        ));

        // Initialize physical intuitions
        self.initialize_physical_intuitions();

        // Initialize metaphors
        self.initialize_metaphors();
    }

    /// Initialize physical intuitions
    fn initialize_physical_intuitions(&mut self) {
        let mut solid = PhysicalIntuition::new("solid_object");
        solid.behaviors.push("moves when pushed".to_string());
        solid.behaviors.push("falls when unsupported".to_string());
        solid.behaviors.push("bounces when dropped on hard surface".to_string());
        solid.constraints.push("has spatial extent".to_string());
        solid.constraints.push("occupies space".to_string());
        self.physical_intuitions.insert("solid".to_string(), solid);

        let mut liquid = PhysicalIntuition::new("liquid");
        liquid.behaviors.push("flows to lowest point".to_string());
        liquid.behaviors.push("takes shape of container".to_string());
        liquid.behaviors.push("spreads when spilled".to_string());
        self.physical_intuitions.insert("liquid".to_string(), liquid);
    }

    /// Initialize metaphors
    fn initialize_metaphors(&mut self) {
        self.metaphors.push(Metaphor::new(
            "time",
            "money",
            "Time is valuable like money, can be spent or wasted",
        ));
        self.metaphors.push(Metaphor::new(
            "argument",
            "warfare",
            "Arguments involve attacking and defending positions",
        ));
    }

    /// Add knowledge
    pub fn add_knowledge(&mut self, knowledge: CommonsenseKnowledge) {
        let idx = self.knowledge_base.len();
        self.knowledge_base.push(knowledge);

        // Update index
        let kb = &self.knowledge_base[idx];
        self.knowledge_index
            .entry(kb.knowledge_type)
            .or_insert_with(Vec::new)
            .push(idx);
    }

    /// Add physical law
    pub fn add_physical_law(&mut self, law: PhysicalLaw) {
        self.physical_laws.push(law);
    }

    /// Add social norm
    pub fn add_social_norm(&mut self, norm: SocialNorm) {
        self.social_norms.push(norm);
    }

    /// Add default rule
    pub fn add_default_rule(&mut self, rule: DefaultRule) {
        self.default_rules.push(rule);
    }

    /// Query knowledge
    pub fn query(&self, query_type: Option<KnowledgeType>, keywords: &[String]) -> Vec<&CommonsenseKnowledge> {
        self.knowledge_base.iter()
            .filter(|k| {
                // Filter by type if specified
                if let Some(t) = query_type {
                    if k.knowledge_type != t {
                        return false;
                    }
                }

                // Filter by keywords
                let content_lower = k.content.to_lowercase();
                keywords.is_empty() || keywords.iter()
                    .any(|kw| content_lower.contains(&kw.to_lowercase()))
            })
            .collect()
    }

    /// Default reasoning
    pub fn default_reasoning(&self, facts: &[String]) -> Vec<ReasoningChain> {
        let mut chains = Vec::new();
        let mut chain_id = 0;

        for rule in &self.default_rules {
            if rule.is_applicable(facts) && !rule.has_exception(facts) {
                let mut chain = ReasoningChain::new(&format!("chain_{}", chain_id));
                chain.add_step(ReasoningStep::new(
                    1,
                    &format!("Given: {}", rule.prerequisite),
                ));
                chain.add_step(ReasoningStep::new(
                    2,
                    &format!("Default rule applies: {}", rule.conclusion),
                ));
                chains.push(chain);
                chain_id += 1;
            }
        }

        chains
    }

    /// Physical reasoning
    pub fn physical_reasoning(&self, scenario: &str) -> Vec<String> {
        let mut conclusions = Vec::new();

        // Check physical intuitions
        for intuition in self.physical_intuitions.values() {
            for behavior in &intuition.behaviors {
                if scenario.to_lowercase().contains(&intuition.object_type) {
                    conclusions.push(format!("Intuition: {}", behavior));
                }
            }
        }

        // Check physical laws
        for law in &self.physical_laws {
            conclusions.push(format!("Law: {}", law.description));
        }

        conclusions
    }

    /// Social reasoning
    pub fn social_reasoning(&self, context: &str, action: &str) -> SocialReasoningResult {
        let mut violations = Vec::new();
        let mut applicable_norms = Vec::new();

        for norm in &self.social_norms {
            if context.to_lowercase().contains(&norm.culture.to_lowercase())
                || norm.culture == "Universal"
            {
                applicable_norms.push(norm.clone());

                // Check if action violates norm
                if action.to_lowercase().contains(&norm.description.to_lowercase()) {
                    violations.push(Violation {
                        norm: norm.clone(),
                        explanation: format!("Action '{}' may violate norm '{}'", action, norm.description),
                    });
                }
            }
        }

        SocialReasoningResult {
            applicable_norms,
            violations,
            recommendations: violations.iter()
                .map(|v| format!("Consider: {}", v.norm.description))
                .collect(),
        }
    }

    /// Create analogy
    pub fn create_analogy(&self, source: &str, target: &str) -> Option<Analogy> {
        // Find structural similarities
        let source_knowledge = self.query(None, &[source.to_string()]);
        let target_knowledge = self.query(None, &[target.to_string()]);

        let mut analogy = Analogy::new(source, target);

        // Create mappings based on shared properties
        for s_kb in &source_knowledge {
            for t_kb in &target_knowledge {
                if s_kb.knowledge_type == t_kb.knowledge_type {
                    analogy.mappings.push(AnalogyMapping {
                        source: s_kb.content.clone(),
                        target: t_kb.content.clone(),
                        relation_preserved: true,
                        confidence: 0.7,
                    });
                }
            }
        }

        if analogy.mappings.is_empty() {
            return None;
        }

        // Calculate similarity
        analogy.similarity = analogy.mappings.len() as f64 * 0.1;
        analogy.inference_strength = analogy.similarity * 0.8;

        self.analogies.push(analogy.clone());
        Some(analogy)
    }

    /// Interpret metaphor
    pub fn interpret_metaphor(&self, metaphor_text: &str) -> Vec<MetaphorInterpretation> {
        let mut interpretations = Vec::new();

        for metaphor in &self.metaphors {
            if metaphor_text.to_lowercase().contains(&metaphor.source)
                && metaphor_text.to_lowercase().contains(&metaphor.target)
            {
                interpretations.push(MetaphorInterpretation {
                    metaphor: metaphor.clone(),
                    literal_meaning: metaphor_text.to_string(),
                    metaphorical_meaning: metaphor.meaning.clone(),
                    mappings: self.extract_mappings(metaphor),
                });
            }
        }

        interpretations
    }

    /// Extract mappings from metaphor
    fn extract_mappings(&self, metaphor: &Metaphor) -> Vec<String> {
        vec![
            format!("{} is like {}", metaphor.source, metaphor.target),
            format!("{} attributes transfer to {}", metaphor.source, metaphor.target),
        ]
    }

    /// Check consistency
    pub fn check_consistency(&self, facts: &[String]) -> ConsistencyResult {
        let mut contradictions = Vec::new();

        for (i, fact1) in facts.iter().enumerate() {
            for fact2 in facts.iter().skip(i + 1) {
                // Check for direct contradictions
                if fact1.to_lowercase() != fact2.to_lowercase()
                    && (fact1.contains("not") != fact2.contains("not"))
                {
                    contradictions.push(Contradiction {
                        fact1: fact1.clone(),
                        fact2: fact2.clone(),
                        explanation: "Potential contradiction detected".to_string(),
                    });
                }
            }
        }

        ConsistencyResult {
            is_consistent: contradictions.is_empty(),
            contradictions,
            suggestions: if !contradictions.is_empty() {
                vec!["Consider additional context to resolve contradictions".to_string()]
            } else {
                Vec::new()
            },
        }
    }

    /// Predict outcomes
    pub fn predict_outcomes(&self, action: &str, object: &str) -> Vec<Prediction> {
        let mut predictions = Vec::new();

        // Check physical intuitions
        if let Some(intuition) = self.physical_intuitions.get(object) {
            for behavior in &intuition.behaviors {
                predictions.push(Prediction {
                    outcome: behavior.clone(),
                    confidence: 0.7,
                    reasoning: format!("Based on physical intuition about {}", object),
                });
            }
        }

        // Apply causal knowledge
        let causal_kb = self.query(Some(KnowledgeType::Causal), &[]);
        for kb in causal_kb {
            if action.to_lowercase().contains(&kb.content.split_whitespace().next().unwrap_or("")) {
                predictions.push(Prediction {
                    outcome: format!("May cause: {}", kb.content),
                    confidence: kb.confidence,
                    reasoning: "Causal knowledge".to_string(),
                });
            }
        }

        predictions
    }

    /// Explain phenomenon
    pub fn explain(&self, phenomenon: &str) -> Explanation {
        // Find relevant knowledge
        let relevant = self.query(None, &[phenomenon.to_string()]);

        let mut explanation = Explanation {
            phenomenon: phenomenon.to_string(),
            causes: Vec::new(),
            mechanisms: Vec::new(),
            confidence: 0.0,
        };

        for kb in relevant {
            match kb.knowledge_type {
                KnowledgeType::Causal => {
                    explanation.causes.push(kb.content.clone());
                },
                KnowledgeType::Physical => {
                    explanation.mechanisms.push(kb.content.clone());
                },
                _ => {}
            }
        }

        if !explanation.causes.is_empty() || !explanation.mechanisms.is_empty() {
            explanation.confidence = 0.8;
        }

        explanation
    }
}

impl Default for CommonsenseReasoning {
    fn default() -> Self {
        Self::new()
    }
}

/// Social reasoning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialReasoningResult {
    /// Applicable norms
    pub applicable_norms: Vec<SocialNorm>,
    /// Violations detected
    pub violations: Vec<Violation>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    /// Violated norm
    pub norm: SocialNorm,
    /// Explanation
    pub explanation: String,
}

/// Consistency result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyResult {
    /// Is consistent
    pub is_consistent: bool,
    /// Contradictions found
    pub contradictions: Vec<Contradiction>,
    /// Suggestions
    pub suggestions: Vec<String>,
}

/// Contradiction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contradiction {
    /// First fact
    pub fact1: String,
    /// Second fact
    pub fact2: String,
    /// Explanation
    pub explanation: String,
}

/// Prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    /// Predicted outcome
    pub outcome: String,
    /// Confidence
    pub confidence: f64,
    /// Reasoning
    pub reasoning: String,
}

/// Explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    /// Phenomenon to explain
    pub phenomenon: String,
    /// Possible causes
    pub causes: Vec<String>,
    /// Underlying mechanisms
    pub mechanisms: Vec<String>,
    /// Confidence
    pub confidence: f64,
}

/// Metaphor interpretation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphorInterpretation {
    /// Original metaphor
    pub metaphor: Metaphor,
    /// Literal meaning
    pub literal_meaning: String,
    /// Metaphorical meaning
    pub metaphorical_meaning: String,
    /// Conceptual mappings
    pub mappings: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_query() {
        let system = CommonsenseReasoning::new();
        let results = system.query(Some(KnowledgeType::Physical), &["fall".to_string()]);

        assert!(!results.is_empty());
    }

    #[test]
    fn test_default_reasoning() {
        let system = CommonsenseReasoning::new();
        let facts = vec!["I saw a bird".to_string()];

        let chains = system.default_reasoning(&facts);
        assert!(!chains.is_empty());
    }

    #[test]
    fn test_physical_reasoning() {
        let system = CommonsenseReasoning::new();
        let conclusions = system.physical_reasoning("The ball is on the table");

        assert!(!conclusions.is_empty());
    }

    #[test]
    fn test_social_reasoning() {
        let system = CommonsenseReasoning::new();
        let result = system.social_reasoning("In a formal meeting", "Being rude");

        assert!(!result.applicable_norms.is_empty() || !result.violations.is_empty());
    }

    #[test]
    fn test_analogy_creation() {
        let system = CommonsenseReasoning::new();
        let analogy = system.create_analogy("sun", "lightbulb");

        assert!(analogy.is_some());
    }

    #[test]
    fn test_metaphor_interpretation() {
        let system = CommonsenseReasoning::new();
        let interpretations = system.interpret_metaphor("Time is money");

        assert!(!interpretations.is_empty());
    }

    #[test]
    fn test_consistency_check() {
        let system = CommonsenseReasoning::new();
        let facts = vec![
            "The sky is blue".to_string(),
            "The sky is not blue".to_string(),
        ];

        let result = system.check_consistency(&facts);
        assert!(!result.is_consistent);
    }

    #[test]
    fn test_prediction() {
        let system = CommonsenseReasoning::new();
        let predictions = system.predict_outcomes("pushing", "solid");

        assert!(!predictions.is_empty());
    }

    #[test]
    fn test_explanation() {
        let system = CommonsenseReasoning::new();
        let explanation = system.explain("Why do objects fall");

        assert!(explanation.confidence > 0.0);
    }
}
