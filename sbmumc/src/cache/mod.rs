//!
//! # SBMUMC Module 1574: Cache Management System
//!
//! Multi-level caching with distributed support, eviction policies,
//! TTL management, and cache warming capabilities.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Cache entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub value: Vec<u8>,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub access_count: u64,
    pub last_access: u64,
    pub metadata: HashMap<String, String>,
    pub tags: Vec<String>,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub name: String,
    pub max_size_bytes: u64,
    pub max_entries: Option<usize>,
    pub default_ttl_secs: Option<u64>,
    pub eviction_policy: EvictionPolicy,
    pub compression: bool,
    pub serialization: SerializationType,
}

/// Eviction policies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EvictionPolicy {
    LRU,        // Least Recently Used
    LFU,        // Least Frequently Used
    FIFO,       // First In First Out
    TTL,        // Time To Live based
    Random,
    MRU,        // Most Recently Used
    Adaptive,
}

/// Serialization types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SerializationType {
    JSON,
    Binary,
    MessagePack,
    Protobuf,
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub name: String,
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub size_bytes: u64,
    pub entry_count: usize,
    pub hit_rate: f64,
    pub avg_ttl_secs: f64,
}

/// Cache event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEvent {
    pub event_type: CacheEventType,
    pub key: String,
    pub timestamp: u64,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheEventType {
    Hit,
    Miss,
    Eviction,
    Expiration,
    Update,
    Delete,
}

/// Cache store
pub struct CacheStore {
    config: CacheConfig,
    entries: Arc<RwLock<HashMap<String, CacheEntry>>>,
    access_order: Arc<RwLock<Vec<String>>>,  // For LRU tracking
    stats: Arc<RwLock<CacheStats>>,
    event_listeners: Arc<RwLock<Vec<fn(CacheEvent)>>>,
}

impl CacheStore {
    pub fn new(config: CacheConfig) -> Self {
        let name = config.name.clone();
        Self {
            config,
            entries: Arc::new(RwLock::new(HashMap::new())),
            access_order: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(CacheStats {
                name,
                hits: 0,
                misses: 0,
                evictions: 0,
                size_bytes: 0,
                entry_count: 0,
                hit_rate: 0.0,
                avg_ttl_secs: 0.0,
            })),
            event_listeners: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get value from cache
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let mut entries = self.entries.write().unwrap();

        if let Some(entry) = entries.get_mut(key) {
            // Check expiration
            if let Some(expires) = entry.expires_at {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;

                if now > expires {
                    entries.remove(key);
                    drop(entries);
                    self.record_event(CacheEvent {
                        event_type: CacheEventType::Expiration,
                        key: key.to_string(),
                        timestamp: now,
                        details: None,
                    });
                    return None;
                }
            }

            // Update access tracking
            entry.access_count += 1;
            entry.last_access = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            self.update_access_order(key);

            // Record hit
            self.record_hit();

            Some(entry.value.clone())
        } else {
            drop(entries);
            self.record_miss();
            None
        }
    }

    /// Set value in cache
    pub fn set(&self, key: String, value: Vec<u8>, ttl_secs: Option<u64>) -> Result<(), CacheError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let expires_at = ttl_secs.map(|secs| now + (secs * 1000));

        let entry = CacheEntry {
            key: key.clone(),
            value: value.clone(),
            created_at: now,
            expires_at,
            access_count: 1,
            last_access: now,
            metadata: HashMap::new(),
            tags: vec![],
        };

        let size = entry.value.len() as u64;

        // Check if we need to evict
        self.check_capacity(size)?;

        let mut entries = self.entries.write().unwrap();

        // Remove old entry if exists
        let old_size = entries.get(&key).map(|e| e.value.len() as u64).unwrap_or(0);

        entries.insert(key.clone(), entry);

        // Update stats
        drop(entries);
        self.update_size(size, old_size);
        self.update_access_order(&key);

        self.record_event(CacheEvent {
            event_type: CacheEventType::Update,
            key,
            timestamp: now,
            details: None,
        });

        Ok(())
    }

    /// Delete key from cache
    pub fn delete(&self, key: &str) -> bool {
        let mut entries = self.entries.write().unwrap();

        if let Some(entry) = entries.remove(key) {
            let size = entry.value.len() as u64;
            drop(entries);

            self.update_size(0, size);
            self.remove_from_access_order(key);

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            self.record_event(CacheEvent {
                event_type: CacheEventType::Delete,
                key: key.to_string(),
                timestamp: now,
                details: None,
            });

            true
        } else {
            false
        }
    }

    /// Check if key exists
    pub fn exists(&self, key: &str) -> bool {
        let entries = self.entries.read().unwrap();
        entries.contains_key(key)
    }

    /// Clear all entries
    pub fn clear(&self) {
        let mut entries = self.entries.write().unwrap();
        entries.clear();

        let mut access = self.access_order.write().unwrap();
        access.clear();

        let mut stats = self.stats.write().unwrap();
        stats.size_bytes = 0;
        stats.entry_count = 0;

        self.record_event(CacheEvent {
            event_type: CacheEventType::Eviction,
            key: "*".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            details: Some("Cache cleared".to_string()),
        });
    }

    /// Get entry metadata
    pub fn get_metadata(&self, key: &str) -> Option<CacheEntry> {
        let entries = self.entries.read().unwrap();
        entries.get(key).cloned()
    }

    /// Update entry tags
    pub fn tag(&self, key: &str, tags: Vec<String>) -> Result<(), CacheError> {
        let mut entries = self.entries.write().unwrap();

        if let Some(entry) = entries.get_mut(key) {
            entry.tags = tags;
            Ok(())
        } else {
            Err(CacheError::KeyNotFound)
        }
    }

    /// Get keys by tag
    pub fn get_by_tag(&self, tag: &str) -> Vec<String> {
        let entries = self.entries.read().unwrap();
        entries
            .iter()
            .filter(|(_, e)| e.tags.contains(&tag.to_string()))
            .map(|(k, _)| k.clone())
            .collect()
    }

    /// Delete by tag
    pub fn delete_by_tag(&self, tag: &str) -> usize {
        let keys: Vec<String> = self.get_by_tag(tag);
        let mut count = 0;

        for key in keys {
            if self.delete(&key) {
                count += 1;
            }
        }

        count
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let stats = self.stats.read().unwrap();
        stats.clone()
    }

    /// Get all keys
    pub fn keys(&self) -> Vec<String> {
        let entries = self.entries.read().unwrap();
        entries.keys().cloned().collect()
    }

    /// Get entry count
    pub fn len(&self) -> usize {
        let entries = self.entries.read().unwrap();
        entries.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Register event listener
    pub fn on_event(&self, listener: fn(CacheEvent)) {
        let mut listeners = self.event_listeners.write().unwrap();
        listeners.push(listener);
    }

    /// Invalidate expired entries
    pub fn invalidate_expired(&self) -> usize {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let mut entries = self.entries.write().unwrap();
        let mut to_remove = Vec::new();

        for (key, entry) in entries.iter() {
            if let Some(expires) = entry.expires_at {
                if now > expires {
                    to_remove.push(key.clone());
                }
            }
        }

        let mut removed = 0;
        for key in &to_remove {
            if let Some(entry) = entries.remove(key) {
                self.update_size(0, entry.value.len() as u64);
                self.remove_from_access_order(key);
                removed += 1;

                self.record_event(CacheEvent {
                    event_type: CacheEventType::Expiration,
                    key: key.clone(),
                    timestamp: now,
                    details: None,
                });
            }
        }

        removed
    }

    /// Warm cache with data
    pub fn warm(&self, data: Vec<(String, Vec<u8>)>) -> Result<(), CacheError> {
        for (key, value) in data {
            self.set(key, value, None)?;
        }
        Ok(())
    }

    fn check_capacity(&self, new_size: u64) -> Result<(), CacheError> {
        let mut entries = self.entries.write().unwrap();

        // Check max size
        let stats = self.stats.read().unwrap();
        if stats.size_bytes + new_size > self.config.max_size_bytes {
            drop(stats);
            drop(entries);
            self.evict(new_size)?;
            entries = self.entries.write().unwrap();
        }

        // Check max entries
        if let Some(max) = self.config.max_entries {
            while entries.len() >= max {
                drop(entries);
                self.evict(0)?;
                entries = self.entries.write().unwrap();
            }
        }

        Ok(())
    }

    fn evict(&self, needed: u64) -> Result<(), CacheError> {
        let mut entries = self.entries.write().unwrap();
        let mut stats = self.stats.write().unwrap();

        let mut freed = 0u64;

        match self.config.eviction_policy {
            EvictionPolicy::LRU => {
                // Remove least recently used
                let mut access = self.access_order.write().unwrap();
                while freed < needed && !access.is_empty() {
                    if let Some(key) = access.pop() {
                        if let Some(entry) = entries.remove(&key) {
                            freed += entry.value.len() as u64;
                            stats.evictions += 1;

                            let now = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_millis() as u64;

                            drop(stats);
                            drop(entries);
                            drop(access);

                            self.record_event(CacheEvent {
                                event_type: CacheEventType::Eviction,
                                key,
                                timestamp: now,
                                details: Some("LRU eviction".to_string()),
                            });

                            entries = self.entries.write().unwrap();
                            stats = self.stats.write().unwrap();
                        }
                    }
                }
            }
            EvictionPolicy::LFU => {
                // Remove least frequently used
                while freed < needed {
                    let lfu_key = entries.iter()
                        .min_by_key(|(_, e)| e.access_count)
                        .map(|(k, _)| k.clone());

                    if let Some(key) = lfu_key {
                        if let Some(entry) = entries.remove(&key) {
                            freed += entry.value.len() as u64;
                            stats.evictions += 1;
                        }
                    } else {
                        break;
                    }
                }
            }
            EvictionPolicy::TTL => {
                // Remove entries with shortest TTL
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;

                while freed < needed {
                    let shortest_ttl = entries.iter()
                        .filter(|(_, e)| e.expires_at.is_some())
                        .min_by_key(|(_, e)| e.expires_at.unwrap_or(u64::MAX))
                        .map(|(k, _)| k.clone());

                    if let Some(key) = shortest_ttl {
                        if let Some(entry) = entries.remove(&key) {
                            freed += entry.value.len() as u64;
                            stats.evictions += 1;

                            drop(stats);
                            drop(entries);

                            self.record_event(CacheEvent {
                                event_type: CacheEventType::Eviction,
                                key,
                                timestamp: now,
                                details: Some("TTL eviction".to_string()),
                            });

                            entries = self.entries.write().unwrap();
                            stats = self.stats.write().unwrap();
                        }
                    } else {
                        break;
                    }
                }
            }
            _ => {
                // Random eviction
                while freed < needed && !entries.is_empty() {
                    let keys: Vec<String> = entries.keys().cloned();
                    if let Some(key) = keys.first().cloned() {
                        if let Some(entry) = entries.remove(&key) {
                            freed += entry.value.len() as u64;
                            stats.evictions += 1;
                        }
                    }
                }
            }
        }

        stats.size_bytes -= freed;
        stats.entry_count = entries.len();

        Ok(())
    }

    fn update_access_order(&self, key: &str) {
        let mut access = self.access_order.write().unwrap();

        // Remove if exists
        access.retain(|k| k != key);

        // Add to front
        access.insert(0, key.to_string());
    }

    fn remove_from_access_order(&self, key: &str) {
        let mut access = self.access_order.write().unwrap();
        access.retain(|k| k != key);
    }

    fn update_size(&self, new_size: u64, old_size: u64) {
        let mut stats = self.stats.write().unwrap();

        if old_size > 0 {
            stats.size_bytes = stats.size_bytes.saturating_sub(old_size);
        }
        stats.size_bytes += new_size;
        stats.entry_count = self.entries.read().unwrap().len();
    }

    fn record_hit(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.hits += 1;
        let total = stats.hits + stats.misses;
        stats.hit_rate = if total > 0 { stats.hits as f64 / total as f64 } else { 0.0 };
    }

    fn record_miss(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.misses += 1;
        let total = stats.hits + stats.misses;
        stats.hit_rate = if total > 0 { stats.hits as f64 / total as f64 } else { 0.0 };
    }

    fn record_event(&self, event: CacheEvent) {
        let listeners = self.event_listeners.read().unwrap();
        for listener in listeners.iter() {
            listener(event.clone());
        }
    }
}

/// Cache error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheError {
    KeyNotFound,
    SizeExceeded,
    SerializationFailed,
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheError::KeyNotFound => write!(f, "Key not found"),
            CacheError::SizeExceeded => write!(f, "Cache size exceeded"),
            CacheError::SerializationFailed => write!(f, "Serialization failed"),
        }
    }
}

impl std::error::Error for CacheError {}

/// Distributed cache manager
pub struct DistributedCache {
    local_caches: Arc<RwLock<HashMap<String, Arc<CacheStore>>>>,
    sync_enabled: bool,
}

impl DistributedCache {
    pub fn new() -> Self {
        Self {
            local_caches: Arc::new(RwLock::new(HashMap::new())),
            sync_enabled: false,
        }
    }

    /// Create named cache
    pub fn create_cache(&self, config: CacheConfig) -> String {
        let cache = Arc::new(CacheStore::new(config.clone()));

        let mut caches = self.local_caches.write().unwrap();
        caches.insert(config.name.clone(), cache);

        config.name
    }

    /// Get cache by name
    pub fn get_cache(&self, name: &str) -> Option<Arc<CacheStore>> {
        let caches = self.local_caches.read().unwrap();
        caches.get(name).cloned()
    }

    /// List all caches
    pub fn list_caches(&self) -> Vec<String> {
        let caches = self.local_caches.read().unwrap();
        caches.keys().cloned().collect()
    }

    /// Delete cache
    pub fn delete_cache(&self, name: &str) -> bool {
        let mut caches = self.local_caches.write().unwrap();
        caches.remove(name).is_some()
    }

    /// Get aggregated stats
    pub fn get_aggregated_stats(&self) -> Vec<CacheStats> {
        let caches = self.local_caches.read().unwrap();
        caches.values().map(|c| c.stats()).collect()
    }

    /// Clear all caches
    pub fn clear_all(&self) {
        let caches = self.local_caches.read().unwrap();
        for cache in caches.values() {
            cache.clear();
        }
    }
}

// Re-export types
pub use CacheEntry;
pub use CacheConfig;
pub use CacheStats;
pub use CacheStore;
pub use DistributedCache;