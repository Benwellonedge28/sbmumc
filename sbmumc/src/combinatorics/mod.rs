//! Combinatorics Module
//!
//! This module implements combinatorics, counting techniques,
//! and discrete structures for the SBMUMC system.

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Combinatorics system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Combinatorics {
    pub comb_id: String,
    pub enumeration: EnumerationTechniques,
    pub graph_theory: GraphTheory,
    pub design_theory: CombinatorialDesign,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumerationTechniques {
    pub counting_principles: Vec<CountingPrinciple>,
    pub permutations: PermutationTheory,
    pub combinations: CombinationTheory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountingPrinciple {
    pub principle_name: String,
    pub statement: String,
    pub formula: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermutationTheory {
    pub permutation_types: Vec<PermutationType>,
    pub formulas: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermutationType {
    pub type_name: String,
    pub description: String,
    pub formula: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinationTheory {
    pub combination_types: Vec<String>,
    pub binomial_identities: Vec<BinomialIdentity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinomialIdentity {
    pub identity_name: String,
    pub formula: String,
    pub proof_technique: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphTheory {
    pub graph_types: Vec<GraphType>,
    pub graph_properties: Vec<GraphProperty>,
    pub algorithms: Vec<GraphAlgorithm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphType {
    pub type_name: String,
    pub definition: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphProperty {
    pub property_name: String,
    pub description: String,
    pub characterization: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphAlgorithm {
    pub algorithm_name: String,
    pub purpose: String,
    pub complexity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinatorialDesign {
    pub block_designs: Vec<BlockDesign>,
    pub tournament_designs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDesign {
    pub design_name: String,
    pub parameters: HashMap<String, u32>,
    pub properties: Vec<String>,
}

impl Combinatorics {
    pub fn new() -> Self {
        Self {
            comb_id: String::from("combinatorics_v1"),
            enumeration: EnumerationTechniques {
                counting_principles: vec![
                    CountingPrinciple { principle_name: String::from("Multiplication Rule"), statement: String::from("If task A has m ways, task B has n ways, both can be done in m*n ways"), formula: String::from("m * n"), examples: vec![String::from("Password combinations")] },
                ],
                permutations: PermutationTheory {
                    permutation_types: vec![
                        PermutationType { type_name: String::from("Linear permutation"), description: String::from("Arrangements in a line"), formula: String::from("n!") },
                    ],
                    formulas: HashMap::from([(String::from("n!"), String::from("n*(n-1)*...*1"))]),
                },
                combinations: CombinationTheory {
                    combination_types: vec![String::from("Selection without order")],
                    binomial_identities: vec![
                        BinomialIdentity { identity_name: String::from("Pascal's Identity"), formula: String::from("C(n,k) = C(n-1,k) + C(n-1,k-1)"), proof_technique: String::from("Combinatorial argument") },
                    ],
                },
            },
            graph_theory: GraphTheory {
                graph_types: vec![
                    GraphType { type_name: String::from("Complete graph"), definition: String::from("Every pair of vertices connected"), examples: vec![String::from("K_n")] },
                ],
                graph_properties: vec![
                    GraphProperty { property_name: String::from("Eulerian"), description: String::from("Path using each edge exactly once"), characterization: String::from("Connected with 0 or 2 odd vertices") },
                ],
                algorithms: vec![
                    GraphAlgorithm { algorithm_name: String::from("Dijkstra's algorithm"), purpose: String::from("Shortest path"), complexity: String::from("O((V+E)log V)") },
                ],
            },
            design_theory: CombinatorialDesign {
                block_designs: vec![
                    BlockDesign { design_name: String::from("Balanced Incomplete Block Design (BIBD)"), parameters: HashMap::from([(String::from("v"), 7), (String::from("k"), 3), (String::from("r"), 3)]), properties: vec![String::from("Every pair appears in lambda blocks")] },
                ],
                tournament_designs: vec![String::from("Round-robin scheduling")],
            },
        }
    }

    pub fn count_permutations(&self, n: u64, r: Option<u64>) -> u64 {
        let r = r.unwrap_or(n);
        if r > n { return 0; }
        (r..=n).product()
    }

    pub fn count_combinations(&self, n: u64, r: u64) -> u64 {
        self.count_permutations(n, Some(r)) / self.count_permutations(r, None)
    }
}

impl Default for Combinatorics { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_creation() { let c = Combinatorics::new(); assert_eq!(c.comb_id, "combinatorics_v1"); } #[test] fn test_combinations() { let c = Combinatorics::new(); assert_eq!(c.count_combinations(5, 2), 10); } }
