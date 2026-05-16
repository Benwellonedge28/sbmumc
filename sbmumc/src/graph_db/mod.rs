//!
//! # SBMUMC Module 1576: Graph Database Engine
//!
//! High-performance graph database with traversal algorithms,
//! pattern matching, path finding, and social network analysis.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Graph node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub properties: HashMap<String, PropertyValue>,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Property value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<PropertyValue>),
    Map(HashMap<String, PropertyValue>),
}

/// Graph edge/relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub label: String,
    pub properties: HashMap<String, PropertyValue>,
    pub weight: Option<f64>,
    pub directed: bool,
}

/// Graph path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub total_weight: f64,
    pub length: usize,
}

/// Traversal result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalResult {
    pub nodes: Vec<Node>,
    pub paths: Vec<Path>,
    pub stats: TraversalStats,
}

/// Traversal statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalStats {
    pub nodes_visited: usize,
    pub edges_traversed: usize,
    pub duration_ms: u64,
    pub depth_reached: u32,
}

/// Graph pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphPattern {
    pub node_patterns: Vec<NodePattern>,
    pub edge_patterns: Vec<EdgePattern>,
    pub constraints: Vec<PatternConstraint>,
}

/// Node pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePattern {
    pub alias: Option<String>,
    pub label: Option<String>,
    pub properties: Option<HashMap<String, PropertyValue>>,
}

/// Edge pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgePattern {
    pub source: String,
    pub target: String,
    pub label: Option<String>,
    pub min_hops: Option<u32>,
    pub max_hops: Option<u32>,
    pub directed: bool,
}

/// Pattern constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternConstraint {
    pub node_alias: String,
    pub property: String,
    pub operator: String,
    pub value: PropertyValue,
}

/// Graph index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphIndex {
    pub name: String,
    pub indexed_label: Option<String>,
    pub indexed_property: String,
    pub unique: bool,
}

/// Graph database
pub struct GraphDatabase {
    nodes: Arc<RwLock<HashMap<String, Node>>>,
    edges: Arc<RwLock<HashMap<String, Edge>>>,
    adjacency: Arc<RwLock<HashMap<String, Vec<String>>>>,  // node_id -> [edge_ids]
    reverse_adjacency: Arc<RwLock<HashMap<String, Vec<String>>>>,
    indexes: Arc<RwLock<Vec<GraphIndex>>>,
    label_index: Arc<RwLock<HashMap<String, HashSet<String>>>>,  // label -> node_ids
}

impl GraphDatabase {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            edges: Arc::new(RwLock::new(HashMap::new())),
            adjacency: Arc::new(RwLock::new(HashMap::new())),
            reverse_adjacency: Arc::new(RwLock::new(HashMap::new())),
            indexes: Arc::new(RwLock::new(Vec::new())),
            label_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create node
    pub fn create_node(&self, label: String, properties: HashMap<String, PropertyValue>) -> String {
        let node_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let node = Node {
            id: node_id.clone(),
            label: label.clone(),
            properties,
            created_at: timestamp,
            updated_at: timestamp,
        };

        let mut nodes = self.nodes.write().unwrap();
        nodes.insert(node_id.clone(), node);

        // Update label index
        let mut label_idx = self.label_index.write().unwrap();
        label_idx
            .entry(label)
            .or_insert_with(HashSet::new)
            .insert(node_id.clone());

        node_id
    }

    /// Create edge
    pub fn create_edge(&self, source: String, target: String, label: String, properties: HashMap<String, PropertyValue>, directed: bool) -> Result<String, GraphError> {
        // Verify nodes exist
        {
            let nodes = self.nodes.read().unwrap();
            if !nodes.contains_key(&source) {
                return Err(GraphError::NodeNotFound(source));
            }
            if !nodes.contains_key(&target) {
                return Err(GraphError::NodeNotFound(target));
            }
        }

        let edge_id = Uuid::new_v4().to_string();

        let edge = Edge {
            id: edge_id.clone(),
            source: source.clone(),
            target: target.clone(),
            label: label.clone(),
            properties,
            weight: None,
            directed,
        };

        let mut edges = self.edges.write().unwrap();
        edges.insert(edge_id.clone(), edge);

        // Update adjacency
        let mut adj = self.adjacency.write().unwrap();
        adj.entry(source.clone())
            .or_insert_with(Vec::new)
            .push(edge_id.clone());

        if !directed {
            let mut rev = self.reverse_adjacency.write().unwrap();
            rev.entry(source.clone())
                .or_insert_with(Vec::new)
                .push(edge_id.clone());
        }

        Ok(edge_id)
    }

    /// Get node by ID
    pub fn get_node(&self, node_id: &str) -> Option<Node> {
        let nodes = self.nodes.read().unwrap();
        nodes.get(node_id).cloned()
    }

    /// Get edge by ID
    pub fn get_edge(&self, edge_id: &str) -> Option<Edge> {
        let edges = self.edges.read().unwrap();
        edges.get(edge_id).cloned()
    }

    /// Find nodes by label
    pub fn find_by_label(&self, label: &str) -> Vec<Node> {
        let label_idx = self.label_index.read().unwrap();
        let nodes = self.nodes.read().unwrap();

        if let Some(node_ids) = label_idx.get(label) {
            node_ids
                .iter()
                .filter_map(|id| nodes.get(id).cloned())
                .collect()
        } else {
            vec![]
        }
    }

    /// Find nodes by property
    pub fn find_by_property(&self, label: &str, property: &str, value: &PropertyValue) -> Vec<Node> {
        self.find_by_label(label)
            .into_iter()
            .filter(|n| n.properties.get(property) == Some(value.clone()))
            .collect()
    }

    /// BFS traversal
    pub fn bfs(&self, start: &str, max_depth: u32) -> Vec<Node> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back((start.to_string(), 0));
        visited.insert(start.to_string());

        while let Some((node_id, depth)) = queue.pop_front() {
            if depth > max_depth {
                break;
            }

            if let Some(node) = self.get_node(&node_id) {
                result.push(node);
            }

            // Get neighbors
            let adj = self.adjacency.read().unwrap();
            let edges = self.edges.read().unwrap();

            if let Some(edge_ids) = adj.get(&node_id) {
                for edge_id in edge_ids {
                    if let Some(edge) = edges.get(edge_id) {
                        let neighbor = edge.target.clone();
                        if !visited.contains(&neighbor) {
                            visited.insert(neighbor.clone());
                            queue.push_back((neighbor, depth + 1));
                        }
                    }
                }
            }
        }

        result
    }

    /// DFS traversal
    pub fn dfs(&self, start: &str, max_depth: u32) -> Vec<Node> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();

        self.dfs_recursive(start, 0, max_depth, &mut visited, &mut result);

        result
    }

    fn dfs_recursive(&self, node_id: &str, depth: u32, max_depth: u32, visited: &mut HashSet<String>, result: &mut Vec<Node>) {
        if depth > max_depth || visited.contains(node_id) {
            return;
        }

        visited.insert(node_id.to_string());

        if let Some(node) = self.get_node(node_id) {
            result.push(node);
        }

        let adj = self.adjacency.read().unwrap();
        let edges = self.edges.read().unwrap();

        if let Some(edge_ids) = adj.get(node_id) {
            for edge_id in edge_ids {
                if let Some(edge) = edges.get(edge_id) {
                    self.dfs_recursive(&edge.target, depth + 1, max_depth, visited, result);
                }
            }
        }
    }

    /// Find shortest path (Dijkstra)
    pub fn shortest_path(&self, start: &str, end: &str) -> Option<Path> {
        let mut distances: HashMap<String, f64> = HashMap::new();
        let mut previous: HashMap<String, String> = HashMap::new();
        let mut visited = HashSet::new();

        distances.insert(start.to_string(), 0.0);

        let nodes = self.nodes.read().unwrap();

        while !visited.contains(end) {
            // Find unvisited node with minimum distance
            let min_node = distances
                .iter()
                .filter(|(n, _)| !visited.contains(*n))
                .min_by_key(|(_, d)| *d as i64)
                .map(|(n, _)| n.clone());

            if let Some(current) = min_node {
                visited.insert(current.clone());

                if current == end {
                    break;
                }

                // Update distances
                let adj = self.adjacency.read().unwrap();
                let edges = self.edges.read().unwrap();

                if let Some(edge_ids) = adj.get(&current) {
                    for edge_id in edge_ids {
                        if let Some(edge) = edges.get(edge_id) {
                            let weight = edge.weight.unwrap_or(1.0);
                            let new_dist = distances.get(&current).unwrap() + weight;

                            if new_dist < *distances.get(&edge.target).unwrap_or(&f64::MAX) {
                                distances.insert(edge.target.clone(), new_dist);
                                previous.insert(edge.target.clone(), current.clone());
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }

        // Reconstruct path
        if !distances.contains_key(end) {
            return None;
        }

        let mut path_nodes = Vec::new();
        let mut path_edges = Vec::new();
        let mut current = end.to_string();
        let mut total_weight = 0.0;

        while let Some(prev) = previous.get(&current) {
            let edges = self.edges.read().unwrap();

            // Find edge connecting prev and current
            let edge = edges.values()
                .find(|e| e.source == *prev && e.target == current)
                .cloned();

            if let Some(e) = edge {
                total_weight += e.weight.unwrap_or(1.0);
                path_edges.push(e);
            }

            if let Some(node) = nodes.get(&current) {
                path_nodes.push(node.clone());
            }

            current = prev.clone();
        }

        if let Some(node) = nodes.get(start) {
            path_nodes.push(node.clone());
        }

        path_nodes.reverse();
        path_edges.reverse();

        Some(Path {
            nodes: path_nodes,
            edges: path_edges,
            total_weight,
            length: path_edges.len(),
        })
    }

    /// Find all paths between two nodes
    pub fn find_all_paths(&self, start: &str, end: &str, max_hops: u32) -> Vec<Path> {
        let mut all_paths = Vec::new();
        let mut current_path = Vec::new();
        let mut visited = HashSet::new();

        self.find_paths_recursive(start, end, 0, max_hops, &mut current_path, &mut visited, &mut all_paths);

        all_paths
    }

    fn find_paths_recursive(
        &self,
        current: &str,
        end: &str,
        depth: u32,
        max_hops: u32,
        path: &mut Vec<String>,
        visited: &mut HashSet<String>,
        results: &mut Vec<Path>,
    ) {
        if depth >= max_hops {
            return;
        }

        if current == end {
            // Reconstruct path
            let nodes = self.nodes.read().unwrap();
            let edges = self.edges.read().unwrap();

            let path_nodes: Vec<Node> = path.iter().filter_map(|id| nodes.get(id).cloned()).collect();
            let path_edges: Vec<Edge> = path.windows(2).filter_map(|w| {
                edges.values().find(|e| e.source == w[0] && e.target == w[1]).cloned()
            }).collect();

            results.push(Path {
                nodes: path_nodes,
                edges: path_edges,
                total_weight: path_edges.len() as f64,
                length: path_edges.len(),
            });
            return;
        }

        visited.insert(current.to_string());
        path.push(current.to_string());

        let adj = self.adjacency.read().unwrap();
        let edges = self.edges.read().unwrap();

        if let Some(edge_ids) = adj.get(current) {
            for edge_id in edge_ids {
                if let Some(edge) = edges.get(edge_id) {
                    if !visited.contains(&edge.target) {
                        self.find_paths_recursive(&edge.target, end, depth + 1, max_hops, path, visited, results);
                    }
                }
            }
        }

        path.pop();
        visited.remove(current);
    }

    /// Pattern matching (simplified)
    pub fn match_pattern(&self, pattern: &GraphPattern) -> Vec<Path> {
        // Simplified pattern matching - in production would use more sophisticated algorithms
        let mut results = Vec::new();

        for node_pattern in &pattern.node_patterns {
            if let Some(label) = &node_pattern.label {
                let nodes = self.find_by_label(label);
                for node in nodes {
                    // Try to match edge patterns
                    let mut path = Path {
                        nodes: vec![node.clone()],
                        edges: vec![],
                        total_weight: 0.0,
                        length: 0,
                    };

                    // Simple single-hop matching
                    if !pattern.edge_patterns.is_empty() {
                        let adj = self.adjacency.read().unwrap();
                        let edges = self.edges.read().unwrap();

                        if let Some(edge_ids) = adj.get(&node.id) {
                            for edge_id in edge_ids {
                                if let Some(edge) = edges.get(edge_id) {
                                    if let Some(edge_label) = &pattern.edge_patterns[0].label {
                                        if edge.label == *edge_label {
                                            if let Some(target) = self.get_node(&edge.target) {
                                                path.nodes.push(target);
                                                path.edges.push(edge.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if path.nodes.len() > 1 {
                        results.push(path);
                    }
                }
            }
        }

        results
    }

    /// Calculate PageRank
    pub fn pagerank(&self, iterations: u32, damping: f64) -> HashMap<String, f64> {
        let nodes = self.nodes.read().unwrap();
        let node_ids: Vec<String> = nodes.keys().cloned().collect();
        let n = node_ids.len();

        let mut ranks: HashMap<String, f64> = node_ids.iter().map(|id| (id.clone(), 1.0 / n as f64)).collect();

        for _ in 0..iterations {
            let mut new_ranks = HashMap::new();

            for node_id in &node_ids {
                let mut sum = 0.0;

                // Find incoming edges
                let rev_adj = self.reverse_adjacency.read().unwrap();
                let edges = self.edges.read().unwrap();

                for edge in edges.values() {
                    if edge.target == *node_id {
                        let weight = edge.weight.unwrap_or(1.0);
                        sum += ranks.get(&edge.source).unwrap_or(&0.0) * weight;
                    }
                }

                new_ranks.insert(node_id.clone(), (1.0 - damping) / n as f64 + damping * sum);
            }

            ranks = new_ranks;
        }

        ranks
    }

    /// Delete node and connected edges
    pub fn delete_node(&self, node_id: &str) -> Result<(), GraphError> {
        let mut nodes = self.nodes.write().unwrap();

        if nodes.remove(node_id).is_none() {
            return Err(GraphError::NodeNotFound(node_id.to_string()));
        }

        // Remove connected edges
        let mut edges = self.edges.write().unwrap();
        let mut to_remove = Vec::new();

        for (edge_id, edge) in edges.iter() {
            if edge.source == node_id || edge.target == node_id {
                to_remove.push(edge_id.clone());
            }
        }

        for edge_id in &to_remove {
            edges.remove(edge_id);
        }

        // Update adjacency
        let mut adj = self.adjacency.write().unwrap();
        let mut rev_adj = self.reverse_adjacency.write().unwrap();
        adj.remove(node_id);
        rev_adj.remove(node_id);

        // Update label index
        let label_idx = self.label_index.read().unwrap();
        for (_, node_ids) in label_idx.iter() {
            if node_ids.contains(node_id) {
                // Would need to update, simplified here
            }
        }

        Ok(())
    }

    /// Get node degree
    pub fn degree(&self, node_id: &str) -> (usize, usize) {
        let adj = self.adjacency.read().unwrap();
        let rev_adj = self.reverse_adjacency.read().unwrap();

        let out = adj.get(node_id).map(|v| v.len()).unwrap_or(0);
        let inp = rev_adj.get(node_id).map(|v| v.len()).unwrap_or(0);

        (inp, out)
    }

    /// Get graph statistics
    pub fn stats(&self) -> GraphStats {
        let nodes = self.nodes.read().unwrap();
        let edges = self.edges.read().unwrap();
        let label_idx = self.label_index.read().unwrap();

        GraphStats {
            total_nodes: nodes.len(),
            total_edges: edges.len(),
            labels: label_idx.keys().cloned().collect(),
            avg_degree: if nodes.is_empty() {
                0.0
            } else {
                (edges.len() * 2) as f64 / nodes.len() as f64
            },
        }
    }
}

/// Graph statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStats {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub labels: Vec<String>,
    pub avg_degree: f64,
}

/// Graph error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphError {
    NodeNotFound(String),
    EdgeNotFound(String),
    ConstraintViolation(String),
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphError::NodeNotFound(id) => write!(f, "Node not found: {}", id),
            GraphError::EdgeNotFound(id) => write!(f, "Edge not found: {}", id),
            GraphError::ConstraintViolation(msg) => write!(f, "Constraint violation: {}", msg),
        }
    }
}

impl std::error::Error for GraphError {}

// Re-export types
pub use Node;
pub use Edge;
pub use Path;
pub use GraphPattern;
pub use GraphDatabase;