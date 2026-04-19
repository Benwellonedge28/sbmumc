//! Knowledge Module - Knowledge Graph and Representation for SBMUMC
//!
//! This module handles knowledge representation, storage, and retrieval.
//! It implements a dynamic knowledge graph that stores concepts, relationships,
//! and entities in a way that mirrors human understanding.

use crate::core::{EntityId, PropertyValue, Relationship, RelationType, Metadata, SbmumcError, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::RwLock;
use parking_lot::RwLock as PLRwLock;
use tracing::{debug, info};

/// Knowledge Graph - Main knowledge representation structure
pub struct KnowledgeGraph {
    /// All concepts in the graph
    concepts: PLRwLock<HashMap<EntityId, Concept>>,

    /// Index by name for fast lookup
    name_index: PLRwLock<HashMap<String, EntityId>>,

    /// Index by type for categorization
    type_index: PLRwLock<HashMap<String, HashSet<EntityId>>>,

    /// Relationship graph (from -> to relationships)
    relationships: PLRwLock<HashMap<EntityId, Vec<Relationship>>>,

    /// Reverse relationships (to -> from)
    reverse_relationships: PLRwLock<HashMap<EntityId, Vec<(EntityId, RelationType)>>>,

    /// Statistics
    stats: PLRwLock<KnowledgeStats>,
}

/// Concept in the knowledge graph
#[derive(Debug, Clone)]
pub struct Concept {
    pub id: EntityId,
    pub name: String,
    pub description: String,
    pub concept_type: String,
    pub properties: HashMap<String, PropertyValue>,
    pub metadata: Metadata,
}

impl KnowledgeGraph {
    /// Create a new knowledge graph
    pub fn new() -> Self {
        info!("Creating new knowledge graph");
        Self {
            concepts: PLRwLock::new(HashMap::new()),
            name_index: PLRwLock::new(HashMap::new()),
            type_index: PLRwLock::new(HashMap::new()),
            relationships: PLRwLock::new(HashMap::new()),
            reverse_relationships: PLRwLock::new(HashMap::new()),
            stats: PLRwLock::new(KnowledgeStats::default()),
        }
    }

    /// Add a concept to the graph
    pub fn add_concept(&self, concept: Concept) -> Result<EntityId> {
        let id = concept.id;
        let name = concept.name.clone();
        let concept_type = concept.concept_type.clone();

        // Check for duplicate name
        {
            let name_idx = self.name_index.read();
            if name_idx.contains_key(&name) {
                return Err(SbmumcError::Knowledge(format!(
                    "Concept with name '{}' already exists",
                    name
                )));
            }
        }

        // Add to main storage
        {
            let mut concepts = self.concepts.write();
            concepts.insert(id, concept);
        }

        // Update indexes
        {
            let mut name_idx = self.name_index.write();
            name_idx.insert(name, id);
        }
        {
            let mut type_idx = self.type_index.write();
            type_idx.entry(concept_type.clone()).or_default().insert(id);
        }

        // Update stats
        {
            let mut stats = self.stats.write();
            stats.total_concepts += 1;
        }

        debug!("Added concept: {} ({})", name, id);
        Ok(id)
    }

    /// Get a concept by ID
    pub fn get_concept(&self, id: EntityId) -> Option<Concept> {
        self.concepts.read().get(&id).cloned()
    }

    /// Get a concept by name
    pub fn get_concept_by_name(&self, name: &str) -> Option<Concept> {
        let name_idx = self.name_index.read();
        name_idx.get(name).and_then(|id| self.get_concept(*id))
    }

    /// Get concepts by type
    pub fn get_concepts_by_type(&self, concept_type: &str) -> Vec<Concept> {
        let type_idx = self.type_index.read();
        let concepts = self.concepts.read();

        type_idx
            .get(concept_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| concepts.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Update a concept
    pub fn update_concept(&self, id: EntityId, update: ConceptUpdate) -> Result<()> {
        let mut concepts = self.concepts.write();
        let concept = concepts.get_mut(&id).ok_or_else(|| {
            SbmumcError::ConceptNotFound(id.to_string())
        })?;

        if let Some(name) = update.name {
            concept.name = name;
        }
        if let Some(description) = update.description {
            concept.description = description;
        }
        if let Some(properties) = update.properties {
            concept.properties.extend(properties);
        }

        concept.metadata.updated_at = crate::core::Timestamp::now();

        Ok(())
    }

    /// Delete a concept
    pub fn delete_concept(&self, id: EntityId) -> Result<()> {
        let mut concepts = self.concepts.write();

        // Get concept info before removal
        let concept = concepts.remove(&id).ok_or_else(|| {
            SbmumcError::ConceptNotFound(id.to_string())
        })?;

        // Remove from name index
        {
            let mut name_idx = self.name_index.write();
            name_idx.remove(&concept.name);
        }

        // Remove from type index
        {
            let mut type_idx = self.type_index.write();
            if let Some(ids) = type_idx.get_mut(&concept.concept_type) {
                ids.remove(&id);
            }
        }

        // Remove relationships
        {
            let mut rels = self.relationships.write();
            rels.remove(&id);
        }
        {
            let mut rev_rels = self.reverse_relationships.write();
            rev_rels.remove(&id);
        }

        // Update stats
        {
            let mut stats = self.stats.write();
            stats.total_concepts -= 1;
        }

        Ok(())
    }

    /// Add a relationship between concepts
    pub fn add_relationship(&self, source_id: EntityId, relationship: Relationship) -> Result<()> {
        // Verify both concepts exist
        {
            let concepts = self.concepts.read();
            if !concepts.contains_key(&source_id) {
                return Err(SbmumcError::ConceptNotFound(source_id.to_string()));
            }
            if !concepts.contains_key(&relationship.target_id) {
                return Err(SbmumcError::ConceptNotFound(relationship.target_id.to_string()));
            }
        }

        // Add forward relationship
        {
            let mut rels = self.relationships.write();
            rels.entry(source_id).or_default().push(relationship.clone());
        }

        // Add reverse relationship
        {
            let mut rev_rels = self.reverse_relationships.write();
            rev_rels.entry(relationship.target_id).or_default().push((source_id, relationship.relation_type));
        }

        // Update stats
        {
            let mut stats = self.stats.write();
            stats.total_relationships += 1;
        }

        debug!("Added relationship: {} -> {}", source_id, relationship.target_id);
        Ok(())
    }

    /// Get relationships from a concept
    pub fn get_relationships(&self, id: EntityId) -> Vec<Relationship> {
        self.relationships.read().get(&id).cloned().unwrap_or_default()
    }

    /// Get reverse relationships to a concept
    pub fn get_reverse_relationships(&self, id: EntityId) -> Vec<(EntityId, RelationType)> {
        self.reverse_relationships.read().get(&id).cloned().unwrap_or_default()
    }

    /// Find concepts by property value
    pub fn find_by_property(&self, property_key: &str, value: &PropertyValue) -> Vec<Concept> {
        let concepts = self.concepts.read();
        concepts
            .values()
            .filter(|c| c.properties.get(property_key) == Some(value))
            .cloned()
            .collect()
    }

    /// Perform breadth-first search from a concept
    pub fn bfs(&self, start_id: EntityId, max_depth: usize) -> Vec<(EntityId, usize)> {
        let mut results = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((start_id, 0));
        visited.insert(start_id);

        while let Some((current_id, depth)) = queue.pop_front() {
            results.push((current_id, depth));

            if depth < max_depth {
                let rels = self.get_relationships(current_id);
                for rel in rels {
                    if !visited.contains(&rel.target_id) {
                        visited.insert(rel.target_id);
                        queue.push_back((rel.target_id, depth + 1));
                    }
                }
            }
        }

        results
    }

    /// Find shortest path between two concepts
    pub fn find_path(&self, start_id: EntityId, end_id: EntityId) -> Option<Vec<EntityId>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent = HashMap::new();

        queue.push_back(start_id);
        visited.insert(start_id);

        while let Some(current) = queue.pop_front() {
            if current == end_id {
                // Reconstruct path
                let mut path = Vec::new();
                let mut node = end_id;
                while node != start_id {
                    path.push(node);
                    node = parent.get(&node)?;
                }
                path.push(start_id);
                path.reverse();
                return Some(path);
            }

            for rel in self.get_relationships(current) {
                if !visited.contains(&rel.target_id) {
                    visited.insert(rel.target_id);
                    parent.insert(rel.target_id, current);
                    queue.push_back(rel.target_id);
                }
            }
        }

        None
    }

    /// Get graph statistics
    pub fn get_stats(&self) -> KnowledgeStats {
        self.stats.read().clone()
    }

    /// Get total concept count
    pub fn concept_count(&self) -> usize {
        self.concepts.read().len()
    }

    /// Clear the knowledge graph
    pub fn clear(&self) {
        let mut concepts = self.concepts.write();
        let mut name_idx = self.name_index.write();
        let mut type_idx = self.type_index.write();
        let mut rels = self.relationships.write();
        let mut rev_rels = self.reverse_relationships.write();
        let mut stats = self.stats.write();

        concepts.clear();
        name_idx.clear();
        type_idx.clear();
        rels.clear();
        rev_rels.clear();
        *stats = KnowledgeStats::default();

        info!("Knowledge graph cleared");
    }

    /// Merge another knowledge graph into this one
    pub fn merge(&self, other: &KnowledgeGraph) -> Result<()> {
        let other_concepts = other.concepts.read();
        let other_relationships = other.relationships.read();

        // Add all concepts
        for concept in other_concepts.values() {
            if self.get_concept_by_name(&concept.name).is_some() {
                // Skip duplicates
                continue;
            }
            self.add_concept(concept.clone())?;
        }

        // Add all relationships
        for (source_id, rels) in other_relationships.iter() {
            for rel in rels {
                let _ = self.add_relationship(*source_id, rel.clone());
            }
        }

        Ok(())
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the knowledge graph
#[derive(Debug, Clone, Default)]
pub struct KnowledgeStats {
    pub total_concepts: usize,
    pub total_relationships: usize,
    pub last_updated: Option<crate::core::Timestamp>,
}

/// Update for a concept
#[derive(Debug, Clone, Default)]
pub struct ConceptUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub properties: Option<HashMap<String, PropertyValue>>,
}

/// Knowledge Node wrapper for external access
pub struct KnowledgeNode {
    pub id: EntityId,
    pub graph: std::sync::Arc<KnowledgeGraph>,
}

impl KnowledgeNode {
    pub fn new(graph: std::sync::Arc<KnowledgeGraph>) -> Self {
        Self { id: EntityId::new(), graph }
    }

    pub fn get_concept(&self) -> Option<Concept> {
        self.graph.get_concept(self.id)
    }

    pub fn add_relationship(&self, target_id: EntityId, relation_type: RelationType) -> Result<()> {
        let relationship = Relationship {
            target_id,
            relation_type,
            properties: HashMap::new(),
            confidence: 1.0,
        };
        self.graph.add_relationship(self.id, relationship)
    }

    pub fn get_neighbors(&self, max_depth: usize) -> Vec<(EntityId, usize)> {
        self.graph.bfs(self.id, max_depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_graph() {
        let graph = KnowledgeGraph::new();

        // Create a concept
        let concept = Concept {
            id: EntityId::new(),
            name: "AI".to_string(),
            description: "Artificial Intelligence".to_string(),
            concept_type: "field".to_string(),
            properties: HashMap::new(),
            metadata: Metadata::default(),
        };

        let id = graph.add_concept(concept).unwrap();
        assert_eq!(graph.concept_count(), 1);

        // Retrieve by name
        let retrieved = graph.get_concept_by_name("AI").unwrap();
        assert_eq!(retrieved.name, "AI");

        // Add another concept and relationship
        let concept2 = Concept {
            id: EntityId::new(),
            name: "Machine Learning".to_string(),
            description: "ML is a subset of AI".to_string(),
            concept_type: "field".to_string(),
            properties: HashMap::new(),
            metadata: Metadata::default(),
        };
        let id2 = graph.add_concept(concept2).unwrap();

        let rel = Relationship {
            target_id: id2,
            relation_type: RelationType::PartOf,
            properties: HashMap::new(),
            confidence: 1.0,
        };
        graph.add_relationship(id, rel).unwrap();

        // Find path
        let path = graph.find_path(id, id2);
        assert!(path.is_some());

        // Get stats
        let stats = graph.get_stats();
        assert_eq!(stats.total_concepts, 2);
        assert_eq!(stats.total_relationships, 1);
    }
}
