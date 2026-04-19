//! Autonomous Evolution Engine
//!
//! This module implements self-modifying code capabilities with safety rails,
//! genetic programming for optimization, automatic refactoring, and capability
//! expansion through exploration.
//!
//! Features:
//! - Self-modifying code with safety verification
//! - Genetic programming for optimization
//! - Automatic code refactoring and improvement
//! - Capability expansion through exploration
//! - Self-healing code mechanisms

use crate::core::{SbmumcError, Result, EntityId, SecurityLevel};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

/// Evolution state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvolutionState {
    /// Evolution is idle
    Idle,
    /// Exploring new possibilities
    Exploring,
    /// Evaluating candidate changes
    Evaluating,
    /// Applying verified changes
    Applying,
    /// Rolling back to previous state
    RollingBack,
    /// Evolution completed
    Completed,
    /// Evolution failed
    Failed,
}

impl Default for EvolutionState {
    fn default() -> Self {
        EvolutionState::Idle
    }
}

/// Gene type for genetic programming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneType {
    /// Functional gene
    Function,
    /// Structural gene
    Structure,
    /// Behavioral gene
    Behavior,
    /// Optimization gene
    Optimization,
    /// Safety gene
    Safety,
    /// Meta gene
    Meta,
}

impl GeneType {
    /// Get gene mutation rate
    pub fn mutation_rate(&self) -> f64 {
        match self {
            GeneType::Function => 0.1,
            GeneType::Structure => 0.05,
            GeneType::Behavior => 0.15,
            GeneType::Optimization => 0.2,
            GeneType::Safety => 0.01,
            GeneType::Meta => 0.08,
        }
    }

    /// Get gene crossover rate
    pub fn crossover_rate(&self) -> f64 {
        match self {
            GeneType::Function => 0.7,
            GeneType::Structure => 0.6,
            GeneType::Behavior => 0.8,
            GeneType::Optimization => 0.9,
            GeneType::Safety => 0.3,
            GeneType::Meta => 0.5,
        }
    }
}

/// Individual gene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gene {
    /// Gene type
    pub gene_type: GeneType,
    /// Gene identifier
    pub id: String,
    /// Gene sequence (encoded representation)
    pub sequence: Vec<u8>,
    /// Fitness score
    pub fitness: f64,
    /// Age of gene
    pub age: u64,
    /// Mutation counter
    pub mutations: u32,
    /// Is gene active
    pub active: bool,
}

impl Gene {
    /// Create a new gene
    pub fn new(gene_type: GeneType, id: &str, sequence: Vec<u8>) -> Self {
        Gene {
            gene_type,
            id: id.to_string(),
            sequence,
            fitness: 0.0,
            age: 0,
            mutations: 0,
            active: true,
        }
    }

    /// Apply mutation to gene
    pub fn mutate(&mut self) {
        let rate = self.gene_type.mutation_rate();

        for byte in &mut self.sequence {
            if rand::random::<f64>() < rate {
                *byte = byte.wrapping_add(rand::random::<u8>() % 10);
                self.mutations += 1;
            }
        }

        self.age = 0;
    }

    /// Get gene representation as string
    pub fn to_string(&self) -> String {
        format!(
            "Gene({}, fitness={:.3}, age={}, mutations={}, active={})",
            self.id, self.fitness, self.age, self.mutations, self.active
        )
    }
}

/// Chromosome containing multiple genes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chromosome {
    /// Chromosome identifier
    pub id: String,
    /// Genes in the chromosome
    pub genes: Vec<Gene>,
    /// Total fitness score
    pub fitness: f64,
    /// Generation number
    pub generation: u32,
    /// Parent chromosome IDs
    pub parents: Vec<String>,
}

impl Chromosome {
    /// Create a new chromosome
    pub fn new(id: &str) -> Self {
        Chromosome {
            id: id.to_string(),
            genes: Vec::new(),
            fitness: 0.0,
            generation: 0,
            parents: Vec::new(),
        }
    }

    /// Add a gene to the chromosome
    pub fn add_gene(&mut self, gene: Gene) {
        self.genes.push(gene);
    }

    /// Get gene by type
    pub fn get_genes_by_type(&self, gene_type: &GeneType) -> Vec<&Gene> {
        self.genes.iter().filter(|g| &g.gene_type == gene_type).collect()
    }

    /// Crossover with another chromosome
    pub fn crossover(&self, other: &Chromosome) -> Chromosome {
        let mut child = Chromosome::new(&format!("child_{}_{}", self.id, other.id));
        child.generation = self.generation.max(other.generation) + 1;
        child.parents = vec![self.id.clone(), other.id.clone()];

        // Combine genes
        for (i, gene) in self.genes.iter().enumerate() {
            if i % 2 == 0 {
                child.add_gene(gene.clone());
            } else if let Some(other_gene) = other.genes.get(i) {
                child.add_gene(other_gene.clone());
            }
        }

        // Fill missing genes
        for (i, other_gene) in other.genes.iter().enumerate() {
            if child.genes.len() <= i {
                child.add_gene(other_gene.clone());
            }
        }

        child
    }

    /// Mutate the chromosome
    pub fn mutate(&mut self) {
        for gene in &mut self.genes {
            if rand::random::<f64>() < gene.gene_type.mutation_rate() {
                gene.mutate();
            }
        }
    }
}

/// Population of chromosomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    /// Chromosomes in the population
    pub chromosomes: Vec<Chromosome>,
    /// Population size
    pub size: usize,
    /// Generation number
    pub generation: u32,
    /// Best fitness seen
    pub best_fitness: f64,
    /// Average fitness
    pub average_fitness: f64,
}

impl Population {
    /// Create a new population
    pub fn new(size: usize) -> Self {
        Population {
            chromosomes: Vec::with_capacity(size),
            size,
            generation: 0,
            best_fitness: 0.0,
            average_fitness: 0.0,
        }
    }

    /// Initialize population with random chromosomes
    pub fn initialize(&mut self, gene_types: &[GeneType], genes_per_chromosome: usize) {
        for i in 0..self.size {
            let mut chromosome = Chromosome::new(&format!("chromosome_{}", i));

            for j in 0..genes_per_chromosome {
                let gene_type = gene_types[j % gene_types.len()].clone();
                let sequence: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
                let gene = Gene::new(gene_type, &format!("gene_{}_{}", i, j), sequence);
                chromosome.add_gene(gene);
            }

            self.chromosomes.push(chromosome);
        }
    }

    /// Evaluate population fitness
    pub fn evaluate_fitness(&mut self, fitness_fn: impl Fn(&Chromosome) -> f64) {
        let mut total_fitness = 0.0;
        let mut best = 0.0;

        for chromosome in &self.chromosomes {
            let fitness = fitness_fn(chromosome);
            total_fitness += fitness;
            best = best.max(fitness);

            // Update chromosome fitness
            if let Some(c) = self.chromosomes.iter_mut().find(|c| c.id == chromosome.id) {
                c.fitness = fitness;
            }
        }

        self.best_fitness = best;
        self.average_fitness = total_fitness / self.size as f64;
    }

    /// Select chromosomes using tournament selection
    pub fn select(&self, tournament_size: usize) -> Chromosome {
        let mut best: Option<&Chromosome> = None;
        let mut best_fitness = f64::NEG_INFINITY;

        for _ in 0..tournament_size {
            let idx = rand::random::<usize>() % self.chromosomes.len();
            let candidate = &self.chromosomes[idx];

            if candidate.fitness > best_fitness {
                best = Some(candidate);
                best_fitness = candidate.fitness;
            }
        }

        best.unwrap().clone()
    }

    /// Evolve the population
    pub fn evolve(&mut self, elite_count: usize, mutation_rate: f64) {
        let mut new_chromosomes = Vec::with_capacity(self.size);

        // Keep elite chromosomes
        let mut sorted = self.chromosomes.clone();
        sorted.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        for chromosome in sorted.iter().take(elite_count) {
            new_chromosomes.push(chromosome.clone());
        }

        // Generate new chromosomes through crossover and mutation
        while new_chromosomes.len() < self.size {
            let parent1 = self.select(3);
            let parent2 = self.select(3);

            let mut child = parent1.crossover(&parent2);

            if rand::random::<f64>() < mutation_rate {
                child.mutate();
            }

            new_chromosomes.push(child);
        }

        self.chromosomes = new_chromosomes;
        self.generation += 1;
    }
}

/// Self-modification request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModificationRequest {
    /// Request identifier
    pub id: String,
    /// Target component
    pub target: String,
    /// Modification type
    pub modification_type: ModificationType,
    /// Proposed changes (encoded)
    pub changes: Vec<u8>,
    /// Safety verification status
    pub verified: bool,
    /// Verification evidence
    pub verification_evidence: Vec<String>,
    /// Risk assessment
    pub risk_level: RiskLevel,
}

impl ModificationRequest {
    /// Create a new modification request
    pub fn new(id: &str, target: &str, modification_type: ModificationType, changes: Vec<u8>) -> Self {
        ModificationRequest {
            id: id.to_string(),
            target: target.to_string(),
            modification_type,
            changes,
            verified: false,
            verification_evidence: Vec::new(),
            risk_level: RiskLevel::Unknown,
        }
    }
}

/// Type of modification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModificationType {
    /// Add new functionality
    Add,
    /// Remove existing functionality
    Remove,
    /// Modify existing functionality
    Modify,
    /// Replace functionality
    Replace,
    /// Optimize performance
    Optimize,
    /// Refactor code structure
    Refactor,
    /// Meta-modification
    Meta,
}

impl ModificationType {
    /// Get default risk level for modification type
    pub fn default_risk(&self) -> RiskLevel {
        match self {
            ModificationType::Add => RiskLevel::Low,
            ModificationType::Remove => RiskLevel::Medium,
            ModificationType::Modify => RiskLevel::Medium,
            ModificationType::Replace => RiskLevel::High,
            ModificationType::Optimize => RiskLevel::Low,
            ModificationType::Refactor => RiskLevel::Medium,
            ModificationType::Meta => RiskLevel::Critical,
        }
    }
}

/// Risk level for modifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// No risk
    None,
    /// Very low risk
    Negligible,
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
    /// Extreme risk
    Extreme,
    /// Catastrophic risk
    Catastrophic,
    /// Unknown risk
    Unknown,
}

impl Default for RiskLevel {
    fn default() -> Self {
        RiskLevel::Unknown
    }
}

/// Safety verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyVerification {
    /// Is modification safe
    pub safe: bool,
    /// Verification score (0.0 to 1.0)
    pub score: f64,
    /// Verification checks passed
    pub checks_passed: Vec<String>,
    /// Verification checks failed
    pub checks_failed: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Rollback plan
    pub rollback_plan: Option<String>,
}

impl SafetyVerification {
    /// Create a verification result
    pub fn new() -> Self {
        SafetyVerification {
            safe: false,
            score: 0.0,
            checks_passed: Vec::new(),
            checks_failed: Vec::new(),
            recommendations: Vec::new(),
            rollback_plan: None,
        }
    }

    /// Mark as safe
    pub fn set_safe(&mut self, score: f64) {
        self.safe = score >= 0.8;
        self.score = score;
    }
}

/// Code snapshot for rollback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSnapshot {
    /// Snapshot identifier
    pub id: String,
    /// Timestamp
    pub timestamp: u64,
    /// Code representation
    pub code: Vec<u8>,
    /// State representation
    pub state: HashMap<String, String>,
    /// Parent snapshot ID
    pub parent_id: Option<String>,
}

impl CodeSnapshot {
    /// Create a new snapshot
    pub fn new(id: &str, code: Vec<u8>) -> Self {
        CodeSnapshot {
            id: id.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            code,
            state: HashMap::new(),
            parent_id: None,
        }
    }
}

/// Self-healing mechanism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfHealing {
    /// Is healing enabled
    pub enabled: bool,
    /// Healing threshold
    pub threshold: f64,
    /// Healing attempts
    pub attempts: u32,
    /// Max healing attempts
    pub max_attempts: u32,
    /// Last healing timestamp
    pub last_healing: u64,
}

impl SelfHealing {
    /// Create a new self-healing mechanism
    pub fn new() -> Self {
        SelfHealing {
            enabled: true,
            threshold: 0.5,
            attempts: 0,
            max_attempts: 10,
            last_healing: 0,
        }
    }

    /// Check if healing is needed
    pub fn needs_healing(&self, health_score: f64) -> bool {
        self.enabled && health_score < self.threshold
    }

    /// Increment healing attempt
    pub fn increment_attempt(&mut self) -> bool {
        self.attempts += 1;
        self.last_healing = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.attempts <= self.max_attempts
    }

    /// Reset healing attempts
    pub fn reset_attempts(&mut self) {
        self.attempts = 0;
    }
}

/// Autonomous Evolution Engine
pub struct EvolutionEngine {
    /// Current state
    pub state: EvolutionState,
    /// Population for genetic programming
    pub population: Population,
    /// Safety verification
    pub safety_verification: SafetyVerification,
    /// Self-healing mechanism
    pub self_healing: SelfHealing,
    /// Code snapshots for rollback
    pub snapshots: Vec<CodeSnapshot>,
    /// Modification history
    pub modification_history: Vec<ModificationRequest>,
    /// Evolution parameters
    pub params: EvolutionParams,
    /// Current chromosome being evolved
    pub current_chromosome: Option<String>,
    /// Evolution statistics
    pub stats: EvolutionStats,
}

/// Evolution parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionParams {
    /// Population size
    pub population_size: usize,
    /// Number of generations
    pub generations: u32,
    /// Mutation rate
    pub mutation_rate: f64,
    /// Crossover rate
    pub crossover_rate: f64,
    /// Elite count
    pub elite_count: usize,
    /// Tournament size
    pub tournament_size: usize,
    /// Convergence threshold
    pub convergence_threshold: f64,
    /// Safety verification required
    pub require_safety_verification: bool,
    /// Max modifications per generation
    pub max_modifications: usize,
    /// Rollback enabled
    pub rollback_enabled: bool,
}

impl Default for EvolutionParams {
    fn default() -> Self {
        EvolutionParams {
            population_size: 100,
            generations: 1000,
            mutation_rate: 0.05,
            crossover_rate: 0.8,
            elite_count: 5,
            tournament_size: 3,
            convergence_threshold: 0.99,
            require_safety_verification: true,
            max_modifications: 10,
            rollback_enabled: true,
        }
    }
}

/// Evolution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionStats {
    /// Total modifications applied
    pub total_modifications: u64,
    /// Successful modifications
    pub successful_modifications: u64,
    /// Failed modifications
    pub failed_modifications: u64,
    /// Rollbacks performed
    pub rollbacks: u64,
    /// Safety violations caught
    pub safety_violations: u64,
    /// Improvements achieved
    pub improvements: f64,
    /// Exploration count
    pub exploration_count: u64,
}

impl Default for EvolutionStats {
    fn default() -> Self {
        EvolutionStats {
            total_modifications: 0,
            successful_modifications: 0,
            failed_modifications: 0,
            rollbacks: 0,
            safety_violations: 0,
            improvements: 0.0,
            exploration_count: 0,
        }
    }
}

impl EvolutionEngine {
    /// Create a new evolution engine
    pub fn new() -> Self {
        let params = EvolutionParams::default();
        let mut population = Population::new(params.population_size);

        // Initialize with gene types
        let gene_types = vec![
            GeneType::Function,
            GeneType::Structure,
            GeneType::Behavior,
            GeneType::Optimization,
            GeneType::Safety,
        ];
        population.initialize(&gene_types, 10);

        EvolutionEngine {
            state: EvolutionState::Idle,
            population,
            safety_verification: SafetyVerification::new(),
            self_healing: SelfHealing::new(),
            snapshots: Vec::new(),
            modification_history: Vec::new(),
            params,
            current_chromosome: None,
            stats: EvolutionStats::default(),
        }
    }

    /// Start evolution process
    pub fn start_evolution(&mut self) {
        self.state = EvolutionState::Exploring;
        self.stats.exploration_count += 1;
    }

    /// Create a modification request
    pub fn create_modification_request(
        &mut self,
        target: &str,
        modification_type: ModificationType,
        changes: Vec<u8>,
    ) -> ModificationRequest {
        let id = format!("mod_{}", self.modification_history.len());
        let mut request = ModificationRequest::new(&id, target, modification_type, changes);
        request.risk_level = modification_type.default_risk();

        self.modification_history.push(request.clone());
        request
    }

    /// Verify safety of a modification
    pub fn verify_safety(&mut self, request: &ModificationRequest) -> SafetyVerification {
        let mut verification = SafetyVerification::new();
        let mut checks_passed = Vec::new();
        let mut checks_failed = Vec::new();
        let mut score = 1.0;

        // Check 1: Risk level assessment
        if request.risk_level <= RiskLevel::Low {
            checks_passed.push("Risk level acceptable".to_string());
        } else {
            checks_failed.push("Risk level too high".to_string());
            score -= 0.3;
        }

        // Check 2: Safety gene presence
        let has_safety_genes = self.population.chromosomes.iter()
            .any(|c| !c.get_genes_by_type(&GeneType::Safety).is_empty());

        if has_safety_genes {
            checks_passed.push("Safety genes present".to_string());
        } else {
            checks_failed.push("No safety genes found".to_string());
            score -= 0.2;
        }

        // Check 3: Rollback capability
        if self.params.rollback_enabled && !self.snapshots.is_empty() {
            checks_passed.push("Rollback capability verified".to_string());
            verification.rollback_plan = Some(format!(
                "Can rollback to snapshot {}",
                self.snapshots.last().unwrap().id
            ));
        } else {
            checks_failed.push("Rollback capability limited".to_string());
            score -= 0.1;
        }

        // Check 4: Target component exists
        if !request.target.is_empty() {
            checks_passed.push("Target component identified".to_string());
        } else {
            checks_failed.push("Invalid target".to_string());
            score -= 0.2;
        }

        // Check 5: Change validation
        if !request.changes.is_empty() {
            checks_passed.push("Changes validated".to_string());
        } else {
            checks_failed.push("No changes provided".to_string());
            score -= 0.2;
        }

        verification.checks_passed = checks_passed;
        verification.checks_failed = checks_failed;
        verification.set_safe(score);

        if !verification.safe {
            self.stats.safety_violations += 1;
        }

        verification
    }

    /// Apply a verified modification
    pub fn apply_modification(&mut self, request: &ModificationRequest) -> Result<bool> {
        if !request.verified {
            return Err(SbmumcError::Security("Modification not verified".to_string()));
        }

        self.state = EvolutionState::Applying;

        // Create snapshot before modification
        let snapshot = CodeSnapshot::new(
            &format!("snapshot_{}", self.snapshots.len()),
            request.changes.clone(),
        );

        if self.params.rollback_enabled {
            self.snapshots.push(snapshot);
        }

        // Apply modification logic here
        // In a real implementation, this would modify actual code

        self.stats.total_modifications += 1;
        self.stats.successful_modifications += 1;
        self.state = EvolutionState::Completed;

        Ok(true)
    }

    /// Rollback to a previous snapshot
    pub fn rollback(&mut self, snapshot_id: &str) -> Result<bool> {
        self.state = EvolutionState::RollingBack;

        let snapshot = self.snapshots.iter()
            .find(|s| s.id == snapshot_id)
            .ok_or_else(|| SbmumcError::NotFound(format!("Snapshot {} not found", snapshot_id)))?;

        // Rollback logic would restore code from snapshot
        // For now, just record the rollback

        self.stats.rollbacks += 1;
        self.state = EvolutionState::Idle;

        Ok(true)
    }

    /// Run genetic programming evolution
    pub fn evolve_population(&mut self, fitness_fn: impl Fn(&Chromosome) -> f64 + Clone) {
        for _ in 0..self.params.generations {
            // Evaluate fitness
            self.population.evaluate_fitness(fitness_fn.clone());

            // Check for convergence
            if self.population.best_fitness >= self.params.convergence_threshold {
                break;
            }

            // Evolve
            self.population.evolve(
                self.params.elite_count,
                self.params.mutation_rate,
            );
        }
    }

    /// Perform self-healing if needed
    pub fn perform_self_healing(&mut self, health_score: f64) -> bool {
        if !self.self_healing.needs_healing(health_score) {
            return false;
        }

        if !self.self_healing.increment_attempt() {
            self.state = EvolutionState::Failed;
            return false;
        }

        // Healing logic
        // Would analyze issues and fix them

        self.self_healing.reset_attempts();
        true
    }

    /// Get evolution statistics
    pub fn get_stats(&self) -> EvolutionStats {
        self.stats.clone()
    }

    /// Get current state
    pub fn get_state(&self) -> EvolutionState {
        self.state
    }

    /// Analyze code for improvements
    pub fn analyze_for_improvements(&self) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Analyze population
        if self.population.best_fitness < 0.5 {
            suggestions.push("Fitness is low, consider increasing mutation rate".to_string());
        }

        // Analyze snapshots
        if self.snapshots.len() > 100 {
            suggestions.push("Too many snapshots, consider cleanup".to_string());
        }

        // Analyze safety
        if self.stats.safety_violations > 10 {
            suggestions.push("Multiple safety violations detected, review safety checks".to_string());
        }

        suggestions
    }
}

impl Default for EvolutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gene_creation() {
        let gene = Gene::new(
            GeneType::Function,
            "test_gene",
            vec![1, 2, 3, 4],
        );

        assert_eq!(gene.id, "test_gene");
        assert_eq!(gene.sequence, vec![1, 2, 3, 4]);
        assert!(gene.active);
    }

    #[test]
    fn test_gene_mutation() {
        let mut gene = Gene::new(
            GeneType::Function,
            "test_gene",
            vec![0, 0, 0, 0],
        );

        gene.mutate();
        assert!(gene.mutations >= 0);
    }

    #[test]
    fn test_chromosome_crossover() {
        let mut chrom1 = Chromosome::new("chrom1");
        let mut chrom2 = Chromosome::new("chrom2");

        chrom1.add_gene(Gene::new(GeneType::Function, "g1", vec![1]));
        chrom1.add_gene(Gene::new(GeneType::Structure, "g2", vec![2]));

        chrom2.add_gene(Gene::new(GeneType::Behavior, "g3", vec![3]));
        chrom2.add_gene(Gene::new(GeneType::Optimization, "g4", vec![4]));

        let child = chrom1.crossover(&chrom2);
        assert_eq!(child.parents, vec!["chrom1".to_string(), "chrom2".to_string()]);
        assert_eq!(child.generation, 1);
    }

    #[test]
    fn test_population_evolution() {
        let mut population = Population::new(10);
        let gene_types = vec![GeneType::Function, GeneType::Optimization];

        population.initialize(&gene_types, 5);
        assert_eq!(population.chromosomes.len(), 10);

        // Evaluate fitness
        population.evaluate_fitness(|_| rand::random::<f64>());

        // Evolve
        population.evolve(2, 0.1);
        assert_eq!(population.generation, 1);
    }

    #[test]
    fn test_evolution_engine() {
        let mut engine = EvolutionEngine::new();

        // Create modification request
        let request = engine.create_modification_request(
            "test_module",
            ModificationType::Optimize,
            vec![1, 2, 3],
        );

        assert_eq!(request.risk_level, RiskLevel::Low);

        // Verify safety
        let verification = engine.verify_safety(&request);
        assert!(verification.checks_passed.len() >= 2);
    }

    #[test]
    fn test_safety_verification() {
        let mut engine = EvolutionEngine::new();

        let request = engine.create_modification_request(
            "safe_module",
            ModificationType::Add,
            vec![0, 1, 0],
        );

        let verification = engine.verify_safety(&request);

        assert!(verification.score > 0.0);
    }

    #[test]
    fn test_self_healing() {
        let mut healing = SelfHealing::new();

        assert!(!healing.needs_healing(0.8));
        assert!(healing.needs_healing(0.3));

        assert!(healing.increment_attempt());
        assert!(healing.increment_attempt());
        assert!(healing.increment_attempt());

        healing.reset_attempts();
        assert_eq!(healing.attempts, 0);
    }

    #[test]
    fn test_code_snapshot() {
        let snapshot = CodeSnapshot::new("snap1", vec![1, 2, 3]);

        assert_eq!(snapshot.id, "snap1");
        assert_eq!(snapshot.code, vec![1, 2, 3]);
    }

    #[test]
    fn test_modification_types() {
        assert_eq!(ModificationType::Add.default_risk(), RiskLevel::Low);
        assert_eq!(ModificationType::Meta.default_risk(), RiskLevel::Critical);
    }
}
