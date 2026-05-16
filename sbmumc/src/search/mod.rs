//!
//! # SBMUMC Module 1580: Search and Discovery Engine
//!
//! Powerful full-text search with fuzzy matching, semantic search,
//! faceted filtering, and relevance ranking.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Search index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndex {
    pub id: String,
    pub name: String,
    pub index_type: IndexType,
    pub schema: IndexSchema,
    pub settings: IndexSettings,
    pub document_count: u64,
    pub size_bytes: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Index types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IndexType {
    Inverted,
    Vector,
    Graph,
    Hybrid,
}

/// Index schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSchema {
    pub fields: Vec<SchemaField>,
    pub primary_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub field_type: FieldType,
    pub indexed: bool,
    pub stored: bool,
    pub analyzer: Option<String>,
    pub boost: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldType {
    Text,
    Keyword,
    Integer,
    Float,
    Boolean,
    Date,
    GeoPoint,
    Nested,
}

/// Index settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSettings {
    pub shards: u32,
    pub replicas: u32,
    pub refresh_interval_ms: u64,
    pub max_result_window: usize,
}

/// Indexed document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocument {
    pub id: String,
    pub index_id: String,
    pub content: HashMap<String, serde_json::Value>,
    pub vector_embedding: Option<Vec<f32>>,
    pub highlighted: Option<HashMap<String, String>>,
    pub score: Option<f32>,
    pub indexed_at: u64,
}

/// Search query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub index_id: String,
    pub query_string: Option<String>,
    pub filters: Vec<QueryFilter>,
    pub sort: Vec<SortSpec>,
    pub pagination: Pagination,
    pub highlight: Option<HighlightConfig>,
    pub aggregations: Vec<AggregationSpec>,
}

/// Query filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    Contains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
    Exists,
    Range,
}

/// Sort specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortSpec {
    pub field: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

/// Pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub offset: usize,
    pub limit: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { offset: 0, limit: 20 }
    }
}

/// Highlight configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightConfig {
    pub fields: Vec<String>,
    pub pre_tag: String,
    pub post_tag: String,
    pub fragment_size: usize,
    pub number_of_fragments: usize,
}

/// Aggregation specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationSpec {
    pub name: String,
    pub agg_type: AggregationType,
    pub field: String,
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AggregationType {
    Terms,
    Histogram,
    Range,
    Avg,
    Sum,
    Min,
    Max,
    Cardinality,
    Nested,
}

/// Search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub query_id: String,
    pub total_hits: u64,
    pub hits: Vec<SearchDocument>,
    pub aggregations: HashMap<String, AggregationResult>,
    pub took_ms: u64,
    pub max_score: f32,
}

/// Aggregation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationResult {
    pub name: String,
    pub result_type: String,
    pub buckets: Option<Vec<Bucket>>,
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
    pub key: String,
    pub doc_count: u64,
    pub aggregations: HashMap<String, AggregationResult>,
}

/// Autocomplete suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub text: String,
    pub score: f32,
    pub completion_type: CompletionType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompletionType {
    Prefix,
    Fuzzy,
    Exact,
    Phrase,
}

/// Search engine
pub struct SearchEngine {
    indexes: Arc<RwLock<HashMap<String, SearchIndex>>>,
    documents: Arc<RwLock<HashMap<String, Vec<SearchDocument>>>>,
    synonyms: Arc<RwLock<HashMap<String, Vec<String>>>>,
    stopwords: Arc<RwLock<Vec<String>>>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            indexes: Arc::new(RwLock::new(HashMap::new())),
            documents: Arc::new(RwLock::new(HashMap::new())),
            synonyms: Arc::new(RwLock::new(HashMap::new())),
            stopwords: Arc::new(RwLock::new(vec![
                "the".to_string(), "a".to_string(), "an".to_string(),
                "and".to_string(), "or".to_string(), "but".to_string(),
                "in".to_string(), "on".to_string(), "at".to_string(),
            ])),
        }
    }

    /// Create index
    pub fn create_index(&self, index: SearchIndex) -> Result<String, SearchError> {
        let mut indexes = self.indexes.write().unwrap();

        if indexes.contains_key(&index.id) {
            return Err(SearchError::IndexAlreadyExists);
        }

        indexes.insert(index.id.clone(), index.clone());

        let mut docs = self.documents.write().unwrap();
        docs.insert(index.id.clone(), Vec::new());

        Ok(index.id)
    }

    /// Delete index
    pub fn delete_index(&self, index_id: &str) -> Result<(), SearchError> {
        let mut indexes = self.indexes.write().unwrap();
        let mut docs = self.documents.write().unwrap();

        indexes.remove(index_id);
        docs.remove(index_id);

        Ok(())
    }

    /// Index document
    pub fn index_document(&self, doc: SearchDocument) -> Result<(), SearchError> {
        let indexes = self.indexes.read().unwrap();

        if !indexes.contains_key(&doc.index_id) {
            return Err(SearchError::IndexNotFound);
        }

        drop(indexes);

        let mut docs = self.documents.write().unwrap();

        if let Some(docs_list) = docs.get_mut(&doc.index_id) {
            // Remove existing document with same ID
            docs_list.retain(|d| d.id != doc.id);
            docs_list.push(doc);
        }

        Ok(())
    }

    /// Bulk index documents
    pub fn bulk_index(&self, index_id: &str, docs: Vec<SearchDocument>) -> Result<usize, SearchError> {
        let indexes = self.indexes.read().unwrap();

        if !indexes.contains_key(index_id) {
            return Err(SearchError::IndexNotFound);
        }

        drop(indexes);

        let mut doc_store = self.documents.write().unwrap();

        if let Some(docs_list) = doc_store.get_mut(index_id) {
            for doc in docs {
                docs_list.retain(|d| d.id != doc.id);
                docs_list.push(doc);
            }
            Ok(docs.len())
        } else {
            Err(SearchError::IndexNotFound)
        }
    }

    /// Search
    pub async fn search(&self, query: SearchQuery) -> Result<SearchResults, SearchError> {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let docs = self.documents.read().unwrap();
        let docs_list = docs.get(&query.index_id)
            .ok_or(SearchError::IndexNotFound)?;

        // Apply filters
        let mut results: Vec<SearchDocument> = docs_list.clone();

        for filter in &query.filters {
            results = self.apply_filter(results, filter);
        }

        // Apply text search if query string provided
        if let Some(qs) = &query.query_string {
            results = self.text_search(&results, qs);
        }

        // Calculate scores
        for doc in &mut results {
            doc.score = Some(self.calculate_score(doc, &query));
        }

        // Sort
        if !query.sort.is_empty() {
            results.sort_by(|a, b| {
                for sort in &query.sort {
                    let a_val = a.content.get(&sort.field).and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let b_val = b.content.get(&sort.field).and_then(|v| v.as_f64()).unwrap_or(0.0);

                    let cmp = a_val.partial_cmp(&b_val).unwrap();
                    if cmp != std::cmp::Ordering::Equal {
                        return match sort.direction {
                            SortDirection::Asc => cmp,
                            SortDirection::Desc => cmp.reverse(),
                        };
                    }
                }
                std::cmp::Ordering::Equal
            });
        } else {
            // Default sort by score
            results.sort_by(|a, b| {
                let a_score = a.score.unwrap_or(0.0);
                let b_score = b.score.unwrap_or(0.0);
                b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
            });
        }

        // Apply pagination
        let total_hits = results.len() as u64;
        let offset = query.pagination.offset;
        let limit = query.pagination.limit;

        let paginated: Vec<SearchDocument> = results.into_iter()
            .skip(offset)
            .take(limit)
            .collect();

        // Compute aggregations
        let aggregations = self.compute_aggregations(&paginated, &query.aggregations);

        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Ok(SearchResults {
            query_id: Uuid::new_v4().to_string(),
            total_hits,
            hits: paginated,
            aggregations,
            took_ms: end - start,
            max_score: 1.0,
        })
    }

    fn apply_filter(&self, docs: Vec<SearchDocument>, filter: &QueryFilter) -> Vec<SearchDocument> {
        docs.into_iter().filter(|doc| {
            if let Some(value) = doc.content.get(&filter.field) {
                match filter.operator {
                    FilterOperator::Equals => value == &filter.value,
                    FilterOperator::Contains => {
                        if let (Some(v_str), Some(f_str)) = (value.as_str(), filter.value.as_str()) {
                            v_str.contains(f_str)
                        } else {
                            false
                        }
                    }
                    FilterOperator::GreaterThan => {
                        if let (Some(v), Some(f)) = (value.as_f64(), filter.value.as_f64()) {
                            v > f
                        } else {
                            false
                        }
                    }
                    FilterOperator::LessThan => {
                        if let (Some(v), Some(f)) = (value.as_f64(), filter.value.as_f64()) {
                            v < f
                        } else {
                            false
                        }
                    }
                    FilterOperator::In => {
                        if let Some(arr) = value.as_array() {
                            arr.iter().any(|v| v == &filter.value)
                        } else {
                            false
                        }
                    }
                    _ => true,
                }
            } else {
                false
            }
        }).collect()
    }

    fn text_search(&self, docs: &[SearchDocument], query: &str) -> Vec<SearchDocument> {
        let query_lower = query.to_lowercase();

        docs.iter()
            .filter(|doc| {
                doc.content.values().any(|v| {
                    v.to_string().to_lowercase().contains(&query_lower)
                })
            })
            .cloned()
            .collect()
    }

    fn calculate_score(&self, doc: &SearchDocument, query: &SearchQuery) -> f32 {
        let mut score = 1.0;

        // Text match scoring
        if let Some(qs) = &query.query_string {
            let query_lower = qs.to_lowercase();
            for (field, value) in &doc.content {
                if let Some(value_str) = value.as_str() {
                    if value_str.to_lowercase().contains(&query_lower) {
                        score += 1.0;
                    }
                }
            }
        }

        score
    }

    fn compute_aggregations(&self, docs: &[SearchDocument], aggs: &[AggregationSpec]) -> HashMap<String, AggregationResult> {
        let mut results = HashMap::new();

        for spec in aggs {
            match spec.agg_type {
                AggregationType::Terms => {
                    let mut counts: HashMap<String, u64> = HashMap::new();
                    for doc in docs {
                        if let Some(value) = doc.content.get(&spec.field) {
                            let key = value.to_string();
                            *counts.entry(key).or_insert(0) += 1;
                        }
                    }

                    let buckets: Vec<Bucket> = counts.into_iter()
                        .map(|(key, doc_count)| Bucket {
                            key,
                            doc_count,
                            aggregations: HashMap::new(),
                        })
                        .collect();

                    results.insert(spec.name.clone(), AggregationResult {
                        name: spec.name.clone(),
                        result_type: "terms".to_string(),
                        buckets: Some(buckets),
                        value: None,
                    });
                }
                AggregationType::Count => {
                    results.insert(spec.name.clone(), AggregationResult {
                        name: spec.name.clone(),
                        result_type: "count".to_string(),
                        buckets: None,
                        value: Some(docs.len() as f64),
                    });
                }
                _ => {}
            }
        }

        results
    }

    /// Autocomplete
    pub async fn autocomplete(&self, index_id: &str, prefix: &str, limit: usize) -> Result<Vec<Suggestion>, SearchError> {
        let docs = self.documents.read().unwrap();
        let docs_list = docs.get(index_id)
            .ok_or(SearchError::IndexNotFound)?;

        let prefix_lower = prefix.to_lowercase();

        let mut suggestions: Vec<Suggestion> = docs_list.iter()
            .filter_map(|doc| {
                for (field, value) in &doc.content {
                    if let Some(text) = value.as_str() {
                        if text.to_lowercase().starts_with(&prefix_lower) {
                            return Some(Suggestion {
                                text: text.to_string(),
                                score: 1.0,
                                completion_type: CompletionType::Prefix,
                            });
                        }
                    }
                }
                None
            })
            .collect();

        suggestions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        suggestions.truncate(limit);

        Ok(suggestions)
    }

    /// Fuzzy search
    pub async fn fuzzy_search(&self, index_id: &str, term: &str, fuzziness: u32) -> Result<SearchResults, SearchError> {
        let docs = self.documents.read().unwrap();
        let docs_list = docs.get(index_id)
            .ok_or(SearchError::IndexNotFound)?;

        let results: Vec<SearchDocument> = docs_list.iter()
            .filter(|doc| {
                for (_, value) in &doc.content {
                    if let Some(text) = value.as_str() {
                        if self.edit_distance(term, text) <= fuzziness as usize {
                            return true;
                        }
                    }
                }
                false
            })
            .cloned()
            .collect();

        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Ok(SearchResults {
            query_id: Uuid::new_v4().to_string(),
            total_hits: results.len() as u64,
            hits: results,
            aggregations: HashMap::new(),
            took_ms: end - start_ms(),
            max_score: 1.0,
        })
    }

    fn edit_distance(&self, s1: &str, s2: &str) -> usize {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let m = s1_chars.len();
        let n = s2_chars.len();

        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 0..=m { dp[i][0] = i; }
        for j in 0..=n { dp[0][j] = j; }

        for i in 1..=m {
            for j in 1..=n {
                if s1_chars[i - 1] == s2_chars[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
                }
            }
        }

        dp[m][n]
    }

    /// Add synonym
    pub fn add_synonym(&self, term: String, synonyms: Vec<String>) {
        let mut syns = self.synonyms.write().unwrap();
        syns.insert(term, synonyms);
    }

    /// Get index info
    pub fn get_index(&self, index_id: &str) -> Option<SearchIndex> {
        let indexes = self.indexes.read().unwrap();
        indexes.get(index_id).cloned()
    }

    /// List indexes
    pub fn list_indexes(&self) -> Vec<SearchIndex> {
        let indexes = self.indexes.read().unwrap();
        indexes.values().cloned().collect()
    }
}

fn start_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

/// Search error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchError {
    IndexNotFound,
    IndexAlreadyExists,
    DocumentNotFound,
    QueryError(String),
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchError::IndexNotFound => write!(f, "Index not found"),
            SearchError::IndexAlreadyExists => write!(f, "Index already exists"),
            SearchError::DocumentNotFound => write!(f, "Document not found"),
            SearchError::QueryError(msg) => write!(f, "Query error: {}", msg),
        }
    }
}

impl std::error::Error for SearchError {}

// Re-export types
pub use SearchIndex;
pub use SearchDocument;
pub use SearchQuery;
pub use SearchResults;
pub use Suggestion;
pub use SearchEngine;