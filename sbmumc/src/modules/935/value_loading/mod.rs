//! # SBMUMC Module 935: Value Loading
//! 
//! Mechanisms for encoding and loading values into AGI systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueRepresentation {
    ExplicitRules,
    UtilityFunctions,
    NeuralEmbedding,
    Constitutional,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueVector {
    pub vector_id: String,
    pub representation: ValueRepresentation,
    pub dimensions: Vec<(String, f64)>,
    pub embedding: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSet {
    pub value_set_id: String,
    pub name: String,
    pub source: String,
    pub values: Vec<ValueVector>,
    pub consistency_score: f64,
}

impl ValueVector {
    pub fn new(representation: ValueRepresentation) -> Self {
        Self {
            vector_id: format!("vv_{}", uuid_simple()),
            representation,
            dimensions: Vec::new(),
            embedding: Vec::new(),
        }
    }

    pub fn add_dimension(&mut self, name: &str, value: f64) {
        self.dimensions.push((name.to_string(), value));
        self.embedding.push(value);
    }

    pub fn cosine_similarity(&self, other: &ValueVector) -> f64 {
        let dot = self.embedding.iter()
            .zip(other.embedding.iter())
            .map(|(a, b)| a * b)
            .sum::<f64>();
        let mag1 = (self.embedding.iter().map(|x| x * x).sum::<f64>()).sqrt();
        let mag2 = (other.embedding.iter().map(|x| x * x).sum::<f64>()).sqrt();
        if mag1 == 0.0 || mag2 == 0.0 { 0.0 } else { dot / (mag1 * mag2) }
    }
}

impl ValueSet {
    pub fn new(name: &str, source: &str) -> Self {
        Self {
            value_set_id: format!("vset_{}", uuid_simple()),
            name: name.to_string(),
            source: source.to_string(),
            values: Vec::new(),
            consistency_score: 0.0,
        }
    }

    pub fn add_value(&mut self, value: ValueVector) {
        self.values.push(value);
        self.compute_consistency();
    }

    pub fn compute_consistency(&mut self) {
        if self.values.len() < 2 {
            self.consistency_score = 1.0;
            return;
        }
        let mut total_sim = 0.0;
        let mut count = 0;
        for i in 0..self.values.len() {
            for j in (i + 1)..self.values.len() {
                total_sim += self.values[i].cosine_similarity(&self.values[j]);
                count += 1;
            }
        }
        self.consistency_score = if count > 0 { total_sim / count as f64 } else { 1.0 };
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{:x}{:x}", duration.as_secs(), duration.subsec_nanos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_vector() {
        let mut vec = ValueVector::new(ValueRepresentation::NeuralEmbedding);
        vec.add_dimension("benevolence", 0.9);
        vec.add_dimension("honesty", 0.85);
        assert!(vec.embedding.len() == 2);
    }

    #[test]
    fn test_value_set_consistency() {
        let mut set = ValueSet::new("Human Values", "Constitutional AI");
        set.add_value(ValueVector::new(ValueRepresentation::NeuralEmbedding));
        set.add_value(ValueVector::new(ValueRepresentation::NeuralEmbedding));
        assert!(set.consistency_score >= 0.0);
    }
}
