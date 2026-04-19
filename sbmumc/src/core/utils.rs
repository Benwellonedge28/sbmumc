//! Utility Functions for SBMUMC
//!
//! Common utility functions and helpers used throughout the system.

use super::{EntityId, Timestamp};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;

/// Generate a unique request ID
pub fn generate_request_id() -> EntityId {
    EntityId::new()
}

/// Get current timestamp
pub fn current_timestamp() -> Timestamp {
    Timestamp::now()
}

/// Calculate processing time in milliseconds
pub fn calculate_processing_time(start: std::time::Instant) -> u64 {
    start.elapsed().as_millis() as u64
}

/// Merge metadata maps
pub fn merge_metadata(
    base: &mut HashMap<String, super::PropertyValue>,
    updates: HashMap<String, super::PropertyValue>,
) {
    for (key, value) in updates {
        base.insert(key, value);
    }
}

/// Sanitize input string
pub fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || matches!(c, '.' | ',' | '!' | '?' | '-' | '_'))
        .collect()
}

/// Normalize text for processing
pub fn normalize_text(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| !c.is_control())
        .collect()
}

/// Calculate confidence score
pub fn calculate_confidence(evidence_count: usize, total_possible: usize) -> f64 {
    if total_possible == 0 {
        return 0.0;
    }
    (evidence_count as f64 / total_possible as f64).min(1.0).max(0.0)
}

/// Interpolate between two values
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Clamp a value between min and max
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

/// Format bytes to human readable string
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

/// Format duration to human readable string
pub fn format_duration(duration_secs: u64) -> String {
    let hours = duration_secs / 3600;
    let minutes = (duration_secs % 3600) / 60;
    let seconds = duration_secs % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Calculate edit distance (Levenshtein distance)
pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = matrix[i - 1][j].min(matrix[i][j - 1]).min(matrix[i - 1][j - 1]) + cost;
        }
    }

    matrix[len1][len2]
}

/// Calculate similarity between two strings (0.0 to 1.0)
pub fn string_similarity(s1: &str, s2: &str) -> f64 {
    if s1.is_empty() && s2.is_empty() {
        return 1.0;
    }
    let distance = edit_distance(s1, s2);
    let max_len = s1.len().max(s2.len());
    if max_len == 0 {
        return 1.0;
    }
    1.0 - (distance as f64 / max_len as f64)
}

/// Deep clone for Arc types
pub trait DeepClone {
    fn deep_clone(&self) -> Self
    where
        Self: Sized,
    {
        Self::clone(&self)
    }
}

impl<T: Clone> DeepClone for Arc<T> {
    fn deep_clone(&self) -> Arc<T> {
        Arc::clone(self)
    }
}

/// Thread-safe lazy initialization
pub struct Lazy<T> {
    cell: std::sync::OnceLock<T>,
    init: fn() -> T,
}

impl<T> Lazy<T> {
    pub fn new(init: fn() -> T) -> Self {
        Self {
            cell: std::sync::OnceLock::new(),
            init,
        }
    }

    pub fn get(&self) -> &T {
        self.cell.get_or_init(self.init)
    }
}

/// Batch processor for handling multiple items
pub struct BatchProcessor<T> {
    batch_size: usize,
    items: Vec<T>,
}

impl<T> BatchProcessor<T> {
    pub fn new(batch_size: usize) -> Self {
        Self {
            batch_size,
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn process<B, F>(&mut self, f: F) -> Vec<B>
    where
        F: Fn(&[T]) -> Vec<B>,
    {
        let results = f(&self.items);
        self.items.clear();
        results
    }

    pub fn into_inner(self) -> Vec<T> {
        self.items
    }
}

/// Rate limiter
pub struct RateLimiter {
    max_per_second: f64,
    tokens: f64,
    last_update: std::time::Instant,
}

impl RateLimiter {
    pub fn new(max_per_second: f64) -> Self {
        Self {
            max_per_second,
            tokens: max_per_second,
            last_update: std::time::Instant::now(),
        }
    }

    pub fn try_acquire(&mut self) -> bool {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        self.last_update = now;

        self.tokens = (self.tokens + elapsed * self.max_per_second).min(self.max_per_second);

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

/// Simple cache with TTL
pub struct TimedCache<K, V> {
    entries: HashMap<K, (V, std::time::Instant)>,
    ttl_secs: u64,
}

impl<K: std::hash::Hash + Eq, V> TimedCache<K, V> {
    pub fn new(ttl_secs: u64) -> Self {
        Self {
            entries: HashMap::new(),
            ttl_secs,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.entries.insert(key, (value, std::time::Instant::now()));
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some((value, time)) = self.entries.get(key) {
            if time.elapsed().as_secs() < self.ttl_secs {
                return Some(value);
            }
        }
        self.entries.remove(key);
        None
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

/// Circular buffer
pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    capacity: usize,
    head: usize,
    tail: usize,
    count: usize,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![Default::default(); capacity],
            capacity,
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.buffer[self.tail] = item;
        self.tail = (self.tail + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        } else {
            self.head = (self.head + 1) % self.capacity;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.buffer.iter().cycle().skip(self.head).take(self.count)
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("", ""), 0);
        assert_eq!(edit_distance("hello", "hello"), 0);
    }

    #[test]
    fn test_string_similarity() {
        assert!((string_similarity("hello", "hello") - 1.0).abs() < 0.001);
        assert!(string_similarity("hello", "hallo") > 0.7);
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(2.0);
        assert!(limiter.try_acquire());
        assert!(limiter.try_acquire());
        assert!(!limiter.try_acquire());
    }

    #[test]
    fn test_circular_buffer() {
        let mut buffer = CircularBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);

        let items: Vec<_> = buffer.iter().cloned().collect();
        assert_eq!(items, vec![4, 2, 3]);
    }
}
