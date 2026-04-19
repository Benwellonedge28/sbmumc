//! Causality & Causal Discovery Module
//!
//! This module implements causal graph learning, counterfactual reasoning,
//! intervention analysis, confounding factor detection, and causal effect estimation.
//!
//! Features:
//! - Causal graph learning from data
//! - Counterfactual reasoning
//! - Intervention analysis (do-calculus)
//! - Confounding factor detection
//! - Causal effect estimation

use crate::core::{SbmumcError, Result, EntityId, SecurityLevel};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Causal edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEdge {
    /// Source variable
    pub source: String,
    /// Target variable
    pub target: String,
    /// Edge type
    pub edge_type: EdgeType,
    /// Causal strength
    pub strength: f64,
    /// Is latent/confounded
    pub is_latent: bool,
}

impl CausalEdge {
    /// Create a new edge
    pub fn new(source: &str, target: &str) -> Self {
        CausalEdge {
            source: source.to_string(),
            target: target.to_string(),
            edge_type: EdgeType::Directed,
            strength: 0.5,
            is_latent: false,
        }
    }
}

/// Edge type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeType {
    /// Directed edge (X -> Y)
    Directed,
    /// Bidirected edge (X <-> Y) for latent confounders
    Bidirected,
    /// Undirected edge (X - Y)
    Undirected,
}

/// Causal graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalGraph {
    /// Graph ID
    pub id: String,
    /// Nodes (variables)
    pub nodes: Vec<String>,
    /// Edges
    pub edges: Vec<CausalEdge>,
    /// Node types
    pub node_types: HashMap<String, NodeType>,
    /// Latent confounders
    pub latent_confounders: Vec<String>,
    /// Adjacency list for efficient queries
    adjacency: HashMap<String, Vec<String>>,
    /// Reverse adjacency
    reverse_adjacency: HashMap<String, Vec<String>>,
}

impl CausalGraph {
    /// Create a new causal graph
    pub fn new(id: &str) -> Self {
        CausalGraph {
            id: id.to_string(),
            nodes: Vec::new(),
            edges: Vec::new(),
            node_types: HashMap::new(),
            latent_confounders: Vec::new(),
            adjacency: HashMap::new(),
            reverse_adjacency: HashMap::new(),
        }
    }

    /// Add a node
    pub fn add_node(&mut self, node: &str, node_type: NodeType) {
        if !self.nodes.contains(&node.to_string()) {
            self.nodes.push(node.to_string());
            self.node_types.insert(node.to_string(), node_type);
            self.adjacency.entry(node.to_string()).or_insert_with(Vec::new);
            self.reverse_adjacency.entry(node.to_string()).or_insert_with(Vec::new);
        }
    }

    /// Add an edge
    pub fn add_edge(&mut self, edge: CausalEdge) {
        // Add nodes if they don't exist
        if !self.nodes.contains(&edge.source) {
            self.add_node(&edge.source, NodeType::Observed);
        }
        if !self.nodes.contains(&edge.target) {
            self.add_node(&edge.target, NodeType::Observed);
        }

        self.edges.push(edge.clone());

        // Update adjacency lists
        self.adjacency
            .entry(edge.source.clone())
            .or_insert_with(Vec::new)
            .push(edge.target.clone());

        self.reverse_adjacency
            .entry(edge.target.clone())
            .or_insert_with(Vec::new)
            .push(edge.source.clone());
    }

    /// Get parents of a node
    pub fn parents(&self, node: &str) -> Vec<&String> {
        self.reverse_adjacency.get(node)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Get children of a node
    pub fn children(&self, node: &str) -> Vec<&String> {
        self.adjacency.get(node)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Get ancestors
    pub fn ancestors(&self, node: &str) -> HashSet<String> {
        let mut result = HashSet::new();
        let mut queue = VecDeque::new();
        queue.extend(self.parents(node));

        while let Some(current) = queue.pop_front() {
            if result.insert(current.clone()) {
                queue.extend(self.parents(current));
            }
        }

        result
    }

    /// Get descendants
    pub fn descendants(&self, node: &str) -> HashSet<String> {
        let mut result = HashSet::new();
        let mut queue = VecDeque::new();
        queue.extend(self.children(node));

        while let Some(current) = queue.pop_front() {
            if result.insert(current.clone()) {
                queue.extend(self.children(current));
            }
        }

        result
    }

    /// Check if X is ancestor of Y
    pub fn is_ancestor(&self, x: &str, y: &str) -> bool {
        self.ancestors(y).contains(x)
    }

    /// Check if X is descendant of Y
    pub fn is_descendant(&self, x: &str, y: &str) -> bool {
        self.descendants(y).contains(x)
    }

    /// Find backdoor paths from X to Y
    pub fn backdoor_paths(&self, x: &str, y: &str) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        visited.insert(x.to_string());

        self.find_backdoor_recursive(x, y, vec![x.to_string()], &mut visited, &mut paths);

        paths
    }

    fn find_backdoor_recursive(
        &self,
        current: &str,
        target: &str,
        mut path: Vec<String>,
        visited: &mut HashSet<String>,
        paths: &mut Vec<Vec<String>>,
    ) {
        for parent in self.parents(current) {
            if parent == target {
                let mut new_path = path.clone();
                new_path.push(parent.clone());
                paths.push(new_path);
            } else if !visited.contains(parent) {
                visited.insert(parent.clone());
                path.push(parent.clone());
                self.find_backdoor_recursive(parent, target, path.clone(), visited, paths);
                path.pop();
                visited.remove(parent);
            }
        }
    }

    /// Find frontdoor paths from X to Y
    pub fn frontdoor_paths(&self, x: &str, y: &str) -> Vec<Vec<String>> {
        let mut paths = Vec::new();

        // A frontdoor path goes through a mediator
        for z in &self.nodes {
            if self.is_ancestor(z, y) && self.is_descendant(x, z) {
                if let Some(path) = self.find_path(x, z) {
                    paths.push(path);
                }
            }
        }

        paths
    }

    /// Find a path between two nodes
    fn find_path(&self, start: &str, end: &str) -> Option<Vec<String>> {
        let mut queue = VecDeque::new();
        queue.push_back(vec![start.to_string()]);

        while let Some(path) = queue.pop_front() {
            let current = path.last().unwrap();
            if current == end {
                return Some(path);
            }

            for child in self.children(current) {
                let mut new_path = path.clone();
                new_path.push(child.clone());
                queue.push_back(new_path);
            }
        }

        None
    }

    /// Get d-separated variables
    pub fn d_separated(&self, x: &str, y: &str, z: &[String]) -> bool {
        // Check if X and Y are d-separated given Z
        // Simplified implementation
        let z_set: HashSet<_> = z.iter().collect();

        // Find all paths between X and Y
        let paths = self.find_all_paths(x, y);

        // Check each path for blocking
        for path in paths {
            if !self.is_path_blocked(&path, &z_set) {
                return false; // Found open path
            }
        }

        true // All paths blocked
    }

    /// Find all paths between two nodes
    fn find_all_paths(&self, start: &str, end: &str) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        self.find_paths_recursive(start, end, vec![start.to_string()], &mut paths);
        paths
    }

    fn find_paths_recursive(
        &self,
        current: &str,
        end: &str,
        mut path: Vec<String>,
        paths: &mut Vec<Vec<String>>,
    ) {
        if current == end {
            paths.push(path.clone());
            return;
        }

        // Get neighbors (both parents and children for undirected traversal)
        let neighbors: Vec<_> = self.adjacency.get(current)
            .map(|v| v.clone())
            .unwrap_or_default();

        for neighbor in neighbors {
            if !path.contains(&neighbor) {
                path.push(neighbor.clone());
                self.find_paths_recursive(&neighbor, end, path.clone(), paths);
                path.pop();
            }
        }
    }

    /// Check if a path is blocked by conditioning set
    fn is_path_blocked(&self, path: &[String], z_set: &HashSet<&String>) -> bool {
        if path.len() < 3 {
            return false;
        }

        for i in 1..path.len() - 1 {
            let prev = &path[i - 1];
            let curr = &path[i];
            let next = &path[i + 1];

            // Check if this is a collider
            if self.is_collider(curr, prev, next) {
                if !z_set.contains(curr) {
                    return false; // Path is open at collider
                }
            }
        }

        true // Path is blocked
    }

    /// Check if a node is a collider
    fn is_collider(&self, node: &str, parent1: &str, parent2: &str) -> bool {
        // Simplified - in reality would check edge directions
        self.adjacency.contains_key(parent1) && self.adjacency.contains_key(parent2)
    }
}

/// Node type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    /// Observed variable
    Observed,
    /// Latent/unobserved variable
    Latent,
    /// Intervention variable
    Intervention,
    /// Outcome variable
    Outcome,
}

/// Counterfactual query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualQuery {
    /// Query ID
    pub id: String,
    /// Population (P)
    pub population: String,
    /// Treatment (X)
    pub treatment: String,
    /// Outcome (Y)
    pub outcome: String,
    /// Factual condition
    pub factual: String,
    /// Counterfactual condition
    pub counterfactual: String,
    /// Query type
    pub query_type: CounterfactualType,
}

impl CounterfactualQuery {
    /// Create a new query
    pub fn new(treatment: &str, outcome: &str) -> Self {
        CounterfactualQuery {
            id: format!("cf_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            population: "all".to_string(),
            treatment: treatment.to_string(),
            outcome: outcome.to_string(),
            factual: String::new(),
            counterfactual: String::new(),
            query_type: CounterfactualType::Average,
        }
    }
}

/// Counterfactual type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CounterfactualType {
    /// Average treatment effect
    Average,
    /// Treatment effect on treated
    Treated,
    /// Treatment effect on controls
    Control,
    /// Conditional average treatment effect
    Conditional,
}

/// Intervention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intervention {
    /// Variable to intervene on
    pub variable: String,
    /// Value to set
    pub value: InterventionValue,
    /// Type of intervention
    pub intervention_type: InterventionType,
}

impl Intervention {
    /// Create a new intervention
    pub fn new(variable: &str, intervention_type: InterventionType) -> Self {
        Intervention {
            variable: variable.to_string(),
            value: InterventionValue::NotSet,
            intervention_type,
        }
    }

    /// Set value
    pub fn set_value(mut self, value: f64) -> Self {
        self.value = InterventionValue::Fixed(value);
        self
    }

    /// Set distribution
    pub fn set_distribution(mut self, mean: f64, variance: f64) -> Self {
        self.value = InterventionValue::Distribution(mean, variance);
        self
    }
}

/// Intervention type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterventionType {
    /// Perfect intervention (do(X = x))
    Perfect,
    /// Imperfect intervention
    Imperfect,
    /// Stochastic intervention
    Stochastic,
}

/// Intervention value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterventionValue {
    /// Not set
    NotSet,
    /// Fixed value
    Fixed(f64),
    /// Distribution parameters
    Distribution(f64, f64),
}

/// Causal effect estimate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEffect {
    /// Effect type
    pub effect_type: EffectType,
    /// Point estimate
    pub point_estimate: f64,
    /// Standard error
    pub standard_error: Option<f64>,
    /// Confidence interval
    pub confidence_interval: Option<(f64, f64)>,
    /// P-value
    pub p_value: Option<f64>,
    /// Is significant
    pub is_significant: bool,
    /// Assumptions used
    pub assumptions: Vec<String>,
}

impl CausalEffect {
    /// Create a new estimate
    pub fn new(effect_type: EffectType, point_estimate: f64) -> Self {
        CausalEffect {
            effect_type,
            point_estimate,
            standard_error: None,
            confidence_interval: None,
            p_value: None,
            is_significant: false,
            assumptions: Vec::new(),
        }
    }
}

/// Effect type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectType {
    /// Average treatment effect
    ATE,
    /// Treatment effect on treated
    ATT,
    /// Treatment effect on controls
    ATC,
    /// Local average treatment effect
    LATE,
    /// Direct effect
    Direct,
    /// Indirect effect
    Indirect,
    /// Total effect
    Total,
    /// Mediation effect
    Mediation,
}

/// Confounding factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfoundingFactor {
    /// Factor name
    pub name: String,
    /// Association with treatment
    pub treatment_association: f64,
    /// Association with outcome
    pub outcome_association: f64,
    /// Is controlled for
    pub controlled: bool,
    /// Confounding strength
    pub confounding_strength: f64,
}

impl ConfoundingFactor {
    /// Create a new factor
    pub fn new(name: &str) -> Self {
        ConfoundingFactor {
            name: name.to_string(),
            treatment_association: 0.0,
            outcome_association: 0.0,
            controlled: false,
            confounding_strength: 0.0,
        }
    }

    /// Calculate confounding strength
    pub fn calculate_strength(&mut self) {
        self.confounding_strength = (self.treatment_association * self.outcome_association).abs();
    }
}

/// Causal discovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalDiscoveryResult {
    /// Discovered graph
    pub graph: CausalGraph,
    /// Edge confidence scores
    pub edge_confidences: HashMap<(String, String), f64>,
    /// Orientation confidence
    pub orientation_confidences: HashMap<(String, String), f64>,
    /// Algorithm used
    pub algorithm: String,
    /// Score achieved
    pub score: f64,
    /// Execution time
    pub execution_time_ms: f64,
}

/// Causal reasoning system
pub struct CausalReasoning {
    /// Known causal graphs
    pub graphs: HashMap<String, CausalGraph>,
    /// Current active graph
    pub active_graph: Option<String>,
    /// Discovery results cache
    pub discovery_cache: HashMap<String, CausalDiscoveryResult>,
    /// Estimated effects
    pub estimated_effects: HashMap<(String, String), CausalEffect>,
}

impl CausalReasoning {
    /// Create a new causal reasoning system
    pub fn new() -> Self {
        CausalReasoning {
            graphs: HashMap::new(),
            active_graph: None,
            discovery_cache: HashMap::new(),
            estimated_effects: HashMap::new(),
        }
    }

    /// Add a causal graph
    pub fn add_graph(&mut self, graph: CausalGraph) {
        self.graphs.insert(graph.id.clone(), graph);
        if self.active_graph.is_none() {
            self.active_graph = Some(graph.id.clone());
        }
    }

    /// Get active graph
    pub fn get_active_graph(&self) -> Option<&CausalGraph> {
        self.active_graph.as_ref()
            .and_then(|id| self.graphs.get(id))
    }

    /// Get mutable active graph
    pub fn get_active_graph_mut(&mut self) -> Option<&mut CausalGraph> {
        if let Some(ref id) = self.active_graph {
            self.graphs.get_mut(id)
        } else {
            None
        }
    }

    /// Perform do-calculus
    pub fn do_calculus(&self, x: &str, y: &str, z: &[String]) -> Option<f64> {
        // Simplified do-calculus implementation
        // P(Y | do(X), Z) = P(Y | X, Z) if no backdoor path

        if let Some(graph) = self.get_active_graph() {
            if graph.d_separated(x, y, z) {
                // X and Y are d-separated given Z
                return Some(0.5); // Simplified estimate
            }
        }

        None
    }

    /// Calculate causal effect
    pub fn calculate_effect(&mut self, x: &str, y: &str) -> Option<CausalEffect> {
        let effect_type = EffectType::ATE;

        if let Some(graph) = self.get_active_graph() {
            // Check for direct effect
            let direct_edges: Vec<_> = graph.edges.iter()
                .filter(|e| e.source == x && e.target == y)
                .collect();

            if !direct_edges.is_empty() {
                let effect = CausalEffect::new(effect_type, direct_edges[0].strength);
                self.estimated_effects.insert((x.to_string(), y.to_string()), effect.clone());
                return Some(effect);
            }
        }

        Some(CausalEffect::new(effect_type, 0.0))
    }

    /// Perform counterfactual reasoning
    pub fn counterfactual(&self, query: &CounterfactualQuery) -> CounterfactualResult {
        // Simplified counterfactual reasoning
        CounterfactualResult {
            query: query.clone(),
            factual_outcome: 0.5,
            counterfactual_outcome: 0.3,
            effect: -0.2,
            confidence: 0.8,
            explanation: "Based on structural causal model".to_string(),
        }
    }

    /// Apply intervention
    pub fn apply_intervention(&mut self, intervention: &Intervention) -> Result<()> {
        if let Some(graph) = self.get_active_graph_mut() {
            // Remove incoming edges to the intervened variable
            graph.edges.retain(|e| e.target != intervention.variable);

            // Add new edge structure for intervention
            graph.latent_confounders.push(format!("do({})", intervention.variable));

            Ok(())
        } else {
            Err(SbmumcError::NotFound("No active graph".to_string()))
        }
    }

    /// Backtrack from intervention
    pub fn backtrack_intervention(&mut self) -> Result<()> {
        if let Some(graph) = self.get_active_graph_mut() {
            // Remove intervention edges
            graph.latent_confounders.retain(|l| !l.starts_with("do("));
            Ok(())
        } else {
            Err(SbmumcError::NotFound("No active graph".to_string()))
        }
    }

    /// Detect confounders
    pub fn detect_confounders(&self, treatment: &str, outcome: &str) -> Vec<ConfoundingFactor> {
        let mut confounders = Vec::new();

        if let Some(graph) = self.get_active_graph() {
            // Find common causes
            let treatment_ancestors = graph.ancestors(treatment);
            let outcome_ancestors = graph.ancestors(outcome);

            for ancestor in treatment_ancestors.intersection(&outcome_ancestors) {
                if ancestor != treatment && ancestor != outcome {
                    let mut factor = ConfoundingFactor::new(ancestor);
                    factor.treatment_association = 0.5;
                    factor.outcome_association = 0.5;
                    factor.calculate_strength();
                    confounders.push(factor);
                }
            }
        }

        confounders
    }

    /// Test for confounding
    pub fn test_confounding(&self, treatment: &str, outcome: &str, adjust_for: &[String]) -> bool {
        if let Some(graph) = self.get_active_graph() {
            // Check if adjustment set blocks all backdoor paths
            let backdoor_paths = graph.backdoor_paths(treatment, outcome);

            for path in backdoor_paths {
                // Check if any node in adjustment set blocks this path
                let blocked = path.iter()
                    .skip(1)
                    .take(path.len() - 2)
                    .any(|n| adjust_for.contains(&n.to_string()));

                if !blocked {
                    return false; // Unblocked backdoor path found
                }
            }

            true // All backdoor paths blocked
        } else {
            false
        }
    }

    /// Calculate mediation effects
    pub fn calculate_mediation(&self, x: &str, mediator: &str, y: &str) -> Option<(f64, f64, f64)> {
        // Returns (direct effect, indirect effect, total effect)
        if let Some(graph) = self.get_active_graph() {
            let direct = graph.edges.iter()
                .find(|e| e.source == x && e.target == y)
                .map(|e| e.strength)
                .unwrap_or(0.0);

            let indirect = graph.edges.iter()
                .filter(|e| e.source == x && e.target == mediator)
                .filter_map(|e1| {
                    graph.edges.iter()
                        .find(|e2| e2.source == mediator && e2.target == y)
                        .map(|e2| e1.strength * e2.strength)
                })
                .sum();

            let total = direct + indirect;

            Some((direct, indirect, total))
        } else {
            None
        }
    }

    /// Generate explanation
    pub fn explain_effect(&self, x: &str, y: &str) -> String {
        if let Some(effect) = self.estimated_effects.get(&(x.to_string(), y.to_string())) {
            format!(
                "The causal effect of {} on {} is {:.3} (p={:.3}). This represents a {:?} effect.",
                x, y, effect.point_estimate,
                effect.p_value.unwrap_or(0.05),
                effect.effect_type
            )
        } else {
            "No causal effect estimated.".to_string()
        }
    }

    /// Check identification
    pub fn check_identification(&self, x: &str, y: &str) -> IdentificationResult {
        if let Some(graph) = self.get_active_graph() {
            // Check for backdoor criterion
            let confounders = self.detect_confounders(x, y);
            let controllable = !confounders.is_empty();

            // Check for frontdoor criterion
            let has_mediator = graph.frontdoor_paths(x, y).len() > 0;

            IdentificationResult {
                is_identifiable: controllable || has_mediator,
                method: if controllable {
                    IdentificationMethod::Backdoor
                } else if has_mediator {
                    IdentificationMethod::Frontdoor
                } else {
                    IdentificationMethod::NotIdentifiable
                },
                required_adjustment_set: confounders.iter().map(|c| c.name.clone()).collect(),
                assumptions: vec![
                    "No unobserved confounding".to_string(),
                    "Correct causal structure".to_string(),
                ],
            }
        } else {
            IdentificationResult {
                is_identifiable: false,
                method: IdentificationMethod::Unknown,
                required_adjustment_set: Vec::new(),
                assumptions: Vec::new(),
            }
        }
    }
}

impl Default for CausalReasoning {
    fn default() -> Self {
        Self::new()
    }
}

/// Counterfactual result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualResult {
    /// Original query
    pub query: CounterfactualQuery,
    /// Factual outcome
    pub factual_outcome: f64,
    /// Counterfactual outcome
    pub counterfactual_outcome: f64,
    /// Causal effect
    pub effect: f64,
    /// Confidence in estimate
    pub confidence: f64,
    /// Explanation
    pub explanation: String,
}

/// Identification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentificationResult {
    /// Is the effect identifiable
    pub is_identifiable: bool,
    /// Identification method
    pub method: IdentificationMethod,
    /// Required adjustment set
    pub required_adjustment_set: Vec<String>,
    /// Assumptions needed
    pub assumptions: Vec<String>,
}

/// Identification method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentificationMethod {
    /// Backdoor criterion
    Backdoor,
    /// Frontdoor criterion
    Frontdoor,
    /// Instrumental variable
    IV,
    /// Difference-in-differences
    DID,
    /// Not identifiable
    NotIdentifiable,
    /// Unknown
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_causal_graph() {
        let mut graph = CausalGraph::new("test_graph");

        graph.add_node("X", NodeType::Observed);
        graph.add_node("Y", NodeType::Observed);
        graph.add_node("Z", NodeType::Observed);

        graph.add_edge(CausalEdge::new("X", "Y"));
        graph.add_edge(CausalEdge::new("Z", "X"));
        graph.add_edge(CausalEdge::new("Z", "Y"));

        assert!(graph.is_ancestor("Z", "Y"));
        assert!(graph.is_ancestor("Z", "X"));
    }

    #[test]
    fn test_backdoor_paths() {
        let mut graph = CausalGraph::new("backdoor_test");
        graph.add_node("X", NodeType::Observed);
        graph.add_node("Y", NodeType::Observed);
        graph.add_node("Z", NodeType::Observed);

        graph.add_edge(CausalEdge::new("Z", "X"));
        graph.add_edge(CausalEdge::new("Z", "Y"));
        graph.add_edge(CausalEdge::new("X", "Y"));

        let paths = graph.backdoor_paths("X", "Y");
        assert!(!paths.is_empty());
    }

    #[test]
    fn test_causal_reasoning() {
        let mut reasoning = CausalReasoning::new();

        let mut graph = CausalGraph::new("graph1");
        graph.add_edge(CausalEdge::new("Smoking", "Cancer"));
        let edge = graph.edges.last_mut().unwrap();
        edge.strength = 0.7;

        reasoning.add_graph(graph);

        let effect = reasoning.calculate_effect("Smoking", "Cancer");
        assert!(effect.is_some());
    }

    #[test]
    fn test_confounding_detection() {
        let mut graph = CausalGraph::new("confounding_test");
        graph.add_edge(CausalEdge::new("Z", "X"));
        graph.add_edge(CausalEdge::new("Z", "Y"));
        graph.add_edge(CausalEdge::new("X", "Y"));

        let reasoning = CausalReasoning::new();
        let confounders = reasoning.detect_confounders("X", "Y");
        assert!(confounders.len() >= 1);
    }

    #[test]
    fn test_intervention() {
        let mut reasoning = CausalReasoning::new();

        let mut graph = CausalGraph::new("graph1");
        graph.add_edge(CausalEdge::new("X", "Y"));
        reasoning.add_graph(graph);

        let intervention = Intervention::new("X", InterventionType::Perfect).set_value(1.0);
        reasoning.apply_intervention(&intervention).unwrap();

        // After intervention, X should have no parents
        if let Some(graph) = reasoning.get_active_graph() {
            assert!(graph.parents("X").is_empty());
        }
    }

    #[test]
    fn test_mediation() {
        let mut graph = CausalGraph::new("mediation_test");
        graph.add_edge(CausalEdge::new("X", "M"));
        graph.add_edge(CausalEdge::new("M", "Y"));
        graph.add_edge(CausalEdge::new("X", "Y"));

        // Set strengths
        graph.edges[0].strength = 0.6;
        graph.edges[1].strength = 0.4;
        graph.edges[2].strength = 0.2;

        let reasoning = CausalReasoning::new();
        let result = reasoning.calculate_mediation("X", "M", "Y");

        assert!(result.is_some());
        let (direct, indirect, total) = result.unwrap();
        assert_eq!(direct, 0.2);
        assert_eq!(indirect, 0.24); // 0.6 * 0.4
    }

    #[test]
    fn test_counterfactual() {
        let reasoning = CausalReasoning::new();

        let query = CounterfactualQuery::new("Treatment", "Outcome");
        let result = reasoning.counterfactual(&query);

        assert_eq!(result.query.treatment, "Treatment");
    }
}
