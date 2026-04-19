//! Reasoning Module - Reasoning, Planning, and Decision Making
//!
//! This module handles all reasoning capabilities including logical inference,
//! planning, decision making, and problem solving.

use crate::core::{SbmumcError, Result, EntityId, ReasoningStep, PropertyValue};
use crate::knowledge::KnowledgeGraph;
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{debug, info};

/// Reasoner - Main reasoning engine
pub struct Reasoner {
    /// Knowledge graph reference
    knowledge: Arc<KnowledgeGraph>,

    /// Inference engine
    inference_engine: InferenceEngine,

    /// Planning engine
    planner: Planner,

    /// Decision maker
    decision_maker: DecisionMaker,

    /// Configuration
    config: ReasonerConfig,
}

/// Inference engine for logical reasoning
pub struct InferenceEngine {
    /// Inference rules
    rules: Vec<InferenceRule>,

    /// Working memory
    working_memory: Vec<Fact>,

    /// Maximum inference depth
    max_depth: usize,
}

/// Inference rule
#[derive(Debug, Clone)]
pub struct InferenceRule {
    pub name: String,
    pub premises: Vec<Fact>,
    pub conclusion: Fact,
    pub confidence: f64,
}

/// Fact representation
#[derive(Debug, Clone, PartialEq)]
pub struct Fact {
    pub predicate: String,
    pub subject: String,
    pub object: Option<String>,
    pub confidence: f64,
}

impl Fact {
    pub fn new(predicate: &str, subject: &str, object: Option<&str>) -> Self {
        Self {
            predicate: predicate.to_string(),
            subject: subject.to_string(),
            object: object.map(|s| s.to_string()),
            confidence: 1.0,
        }
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence;
        self
    }
}

/// Planner for goal-oriented behavior
pub struct Planner {
    /// Planning algorithm
    algorithm: PlanningAlgorithm,

    /// Maximum plan steps
    max_plan_steps: usize,

    /// Action library
    actions: HashMap<String, Action>,
}

/// Action representation
#[derive(Debug, Clone)]
pub struct Action {
    pub name: String,
    pub preconditions: Vec<Fact>,
    pub effects: Vec<Fact>,
    pub cost: f64,
}

/// Planning algorithms
#[derive(Debug, Clone, Copy)]
pub enum PlanningAlgorithm {
    AStar,
    GreedyBestFirst,
    DepthFirst,
    BreadthFirst,
}

/// Decision maker for making choices
pub struct DecisionMaker {
    /// Decision criteria
    criteria: Vec<DecisionCriterion>,

    /// Risk tolerance
    risk_tolerance: f64,

    /// Decision history
    history: Vec<Decision>,
}

/// Decision criterion
#[derive(Debug, Clone)]
pub struct DecisionCriterion {
    pub name: String,
    pub weight: f64,
    pub evaluate: fn(&DecisionContext) -> f64,
}

/// Decision context
#[derive(Debug, Clone)]
pub struct DecisionContext {
    pub options: Vec<DecisionOption>,
    pub constraints: HashMap<String, PropertyValue>,
    pub state: HashMap<String, PropertyValue>,
}

/// Decision option
#[derive(Debug, Clone)]
pub struct DecisionOption {
    pub id: String,
    pub name: String,
    pub scores: HashMap<String, f64>,
    pub risk: f64,
    pub expected_value: f64,
}

/// Decision record
#[derive(Debug, Clone)]
pub struct Decision {
    pub id: EntityId,
    pub context: DecisionContext,
    pub chosen_option: String,
    pub reasoning: Vec<String>,
    pub outcome: Option<f64>,
    pub timestamp: crate::core::Timestamp,
}

/// Reasoner configuration
#[derive(Debug, Clone)]
pub struct ReasonerConfig {
    pub max_inference_depth: usize,
    pub max_plan_steps: usize,
    pub enable_abductive_reasoning: bool,
    pub enable_probabilistic_reasoning: bool,
    pub enable_analogical_reasoning: bool,
    pub confidence_threshold: f64,
}

impl Default for ReasonerConfig {
    fn default() -> Self {
        Self {
            max_inference_depth: 10,
            max_plan_steps: 100,
            enable_abductive_reasoning: true,
            enable_probabilistic_reasoning: true,
            enable_analogical_reasoning: true,
            confidence_threshold: 0.7,
        }
    }
}

impl Reasoner {
    /// Create a new reasoner
    pub fn new(knowledge: Arc<KnowledgeGraph>) -> Result<Self> {
        info!("Initializing Reasoner");

        let config = ReasonerConfig::default();

        Ok(Self {
            knowledge,
            inference_engine: InferenceEngine::new(config.max_inference_depth),
            planner: Planner::new(config.max_plan_steps),
            decision_maker: DecisionMaker::default(),
            config,
        })
    }

    /// Perform deduction
    pub fn deduce(&self, goal: &str) -> Result<Vec<Fact>> {
        debug!("Deducting: {}", goal);
        self.inference_engine.forward_chain(&self.knowledge, goal)
    }

    /// Perform abduction
    pub fn abduce(&self, observation: &str) -> Result<Vec<Fact>> {
        if !self.config.enable_abductive_reasoning {
            return Err(SbmumcError::Reasoning("Abductive reasoning not enabled".to_string()));
        }

        debug!("Abducing for: {}", observation);
        self.inference_engine.abductive_reasoning(observation)
    }

    /// Create a plan to achieve a goal
    pub fn plan(&self, goal: &str, initial_state: Vec<Fact>) -> Result<Plan> {
        debug!("Planning for goal: {}", goal);
        self.planner.create_plan(&self.config, goal, initial_state, &self.knowledge)
    }

    /// Make a decision
    pub fn decide(&self, context: DecisionContext) -> Result<Decision> {
        debug!("Making decision from {} options", context.options.len());
        self.decision_maker.make_decision(context)
    }

    /// Solve a problem
    pub fn solve(&self, problem: &Problem) -> Result<Solution> {
        debug!("Solving problem: {}", problem.description);

        // Extract facts from problem
        let facts: Vec<Fact> = problem
            .facts
            .iter()
            .map(|(p, s, o)| Fact::new(p, s, o.as_deref()))
            .collect();

        // Try deduction first
        let mut reasoning_chain = Vec::new();
        if let Ok(results) = self.deduce(&problem.goal) {
            for (i, fact) in results.iter().enumerate() {
                reasoning_chain.push(ReasoningStep {
                    step_number: i + 1,
                    premise: format!("Given: {:?}", facts),
                    inference: format!("Derived: {:?}", fact),
                    confidence: fact.confidence,
                });
            }
        }

        // Try planning if deduction didn't fully solve it
        let plan = if reasoning_chain.is_empty() || reasoning_chain.len() < 3 {
            self.plan(&problem.goal, facts).ok()
        } else {
            None
        };

        Ok(Solution {
            problem: problem.description.clone(),
            reasoning_chain,
            plan,
            confidence: 0.8,
        })
    }

    /// Add an inference rule
    pub fn add_rule(&mut self, rule: InferenceRule) {
        self.inference_engine.add_rule(rule);
    }

    /// Add an action to the library
    pub fn add_action(&mut self, action: Action) {
        self.planner.add_action(action);
    }

    /// Get reasoning statistics
    pub fn get_stats(&self) -> ReasoningStats {
        ReasoningStats {
            rules_count: self.inference_engine.rules.len(),
            actions_count: self.planner.actions.len(),
            decisions_count: self.decision_maker.history.len(),
        }
    }
}

impl InferenceEngine {
    pub fn new(max_depth: usize) -> Self {
        Self {
            rules: Vec::new(),
            working_memory: Vec::new(),
            max_depth,
        }
    }

    /// Add an inference rule
    pub fn add_rule(&mut self, rule: InferenceRule) {
        self.rules.push(rule);
    }

    /// Forward chaining inference
    pub fn forward_chain(&self, knowledge: &KnowledgeGraph, goal: &str) -> Result<Vec<Fact>> {
        let mut results = Vec::new();
        let mut agenda = VecDeque::new();

        // Initialize agenda with facts from knowledge
        // In a full implementation, we would query the knowledge graph

        // Apply rules
        for rule in &self.rules {
            if rule.premises.iter().all(|p| self.matches(&agenda, p)) {
                let mut conclusion = rule.conclusion.clone();
                conclusion.confidence *= rule.confidence;
                if conclusion.predicate == goal {
                    results.push(conclusion);
                }
                agenda.push_back(conclusion);
            }
        }

        Ok(results)
    }

    /// Check if a fact matches agenda items
    fn matches(&self, agenda: &VecDeque<Fact>, pattern: &Fact) -> bool {
        agenda.iter().any(|f| {
            f.predicate == pattern.predicate && f.subject == pattern.subject
        })
    }

    /// Abductive reasoning - find explanations for observations
    pub fn abductive_reasoning(&self, observation: &str) -> Result<Vec<Fact>> {
        let mut explanations = Vec::new();

        // Find rules that could explain this observation
        for rule in &self.rules {
            if rule.conclusion.predicate == observation {
                for premise in &rule.premises {
                    let mut explanation = premise.clone();
                    explanation.confidence *= rule.confidence;
                    explanations.push(explanation);
                }
            }
        }

        Ok(explanations)
    }
}

impl Planner {
    pub fn new(max_plan_steps: usize) -> Self {
        Self {
            algorithm: PlanningAlgorithm::AStar,
            max_plan_steps,
            actions: HashMap::new(),
        }
    }

    /// Add an action to the library
    pub fn add_action(&mut self, action: Action) {
        self.actions.insert(action.name.clone(), action);
    }

    /// Create a plan to achieve a goal
    pub fn create_plan(
        &self,
        config: &ReasonerConfig,
        goal: &str,
        initial_state: Vec<Fact>,
        knowledge: &KnowledgeGraph,
    ) -> Result<Plan> {
        let mut steps = Vec::new();
        let mut current_state: HashSet<String> = initial_state.iter().map(|f| f.to_string()).collect();
        let mut remaining_goals = vec![goal.to_string()];
        let mut depth = 0;

        while !remaining_goals.is_empty() && depth < self.max_plan_steps.min(config.max_plan_steps) {
            let current_goal = remaining_goals.remove(0);

            // Find applicable actions
            let applicable: Vec<&Action> = self
                .actions
                .values()
                .filter(|a| {
                    a.effects.iter().any(|e| e.predicate == current_goal)
                })
                .collect();

            if let Some(action) = applicable.first() {
                steps.push(PlanStep {
                    action: action.name.clone(),
                    reasoning: format!("Apply {} to achieve {}", action.name, current_goal),
                    cost: action.cost,
                });

                // Add preconditions as new goals
                for pre in &action.preconditions {
                    if !current_state.contains(&pre.to_string()) {
                        remaining_goals.push(pre.predicate.clone());
                    }
                }

                // Remove achieved goal from state
                current_state.insert(current_goal);
            }

            depth += 1;
        }

        Ok(Plan {
            steps,
            total_cost: steps.iter().map(|s| s.cost).sum(),
            achieved: remaining_goals.is_empty(),
        })
    }
}

impl DecisionMaker {
    pub fn make_decision(&mut self, context: DecisionContext) -> Result<Decision> {
        let mut best_option: Option<&DecisionOption> = None;
        let mut best_score = f64::MIN;
        let mut reasoning = Vec::new();

        for option in &context.options {
            let score = self.evaluate_option(option, &context);
            reasoning.push(format!("Evaluated {}: score = {:.2}", option.name, score));

            if score > best_score {
                best_score = score;
                best_option = Some(option);
            }
        }

        let chosen = best_option.ok_or_else(|| {
            SbmumcError::Reasoning("No valid decision options".to_string())
        })?;

        let decision = Decision {
            id: EntityId::new(),
            context,
            chosen_option: chosen.id.clone(),
            reasoning,
            outcome: None,
            timestamp: crate::core::Timestamp::now(),
        };

        self.history.push(decision.clone());
        Ok(decision)
    }

    fn evaluate_option(&self, option: &DecisionOption, context: &DecisionContext) -> f64 {
        let mut total_score = 0.0;
        let mut total_weight = 0.0;

        for criterion in &self.criteria {
            if let Some(score) = option.scores.get(&criterion.name) {
                let weight = criterion.weight;
                total_score += score * weight;
                total_weight += weight;
            }
        }

        if total_weight > 0.0 {
            total_score / total_weight
        } else {
            0.0
        }
    }
}

impl Default for DecisionMaker {
    fn default() -> Self {
        Self {
            criteria: vec![
                DecisionCriterion {
                    name: "quality".to_string(),
                    weight: 1.0,
                    evaluate: |_| 0.8,
                },
                DecisionCriterion {
                    name: "speed".to_string(),
                    weight: 0.8,
                    evaluate: |_| 0.7,
                },
            ],
            risk_tolerance: 0.5,
            history: Vec::new(),
        }
    }
}

/// Plan representation
#[derive(Debug, Clone)]
pub struct Plan {
    pub steps: Vec<PlanStep>,
    pub total_cost: f64,
    pub achieved: bool,
}

/// Single plan step
#[derive(Debug, Clone)]
pub struct PlanStep {
    pub action: String,
    pub reasoning: String,
    pub cost: f64,
}

/// Problem representation
#[derive(Debug, Clone)]
pub struct Problem {
    pub description: String,
    pub facts: Vec<(String, String, Option<String>)>,
    pub goal: String,
    pub constraints: Vec<String>,
}

/// Solution representation
#[derive(Debug, Clone)]
pub struct Solution {
    pub problem: String,
    pub reasoning_chain: Vec<ReasoningStep>,
    pub plan: Option<Plan>,
    pub confidence: f64,
}

/// Reasoning statistics
#[derive(Debug, Clone)]
pub struct ReasoningStats {
    pub rules_count: usize,
    pub actions_count: usize,
    pub decisions_count: usize,
}

impl std::fmt::Display for Fact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(obj) = &self.object {
            write!(f, "{}({}, {})", self.predicate, self.subject, obj)
        } else {
            write!(f, "{}({})", self.predicate, self.subject)
        }
    }
}
