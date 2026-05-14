//! # SBMUMC Benchmark Suite
//!
//! Performance benchmarks for all core modules

use std::time::{Duration, Instant};

// ============================================================================
// Benchmark Utilities
// ============================================================================

pub struct Benchmark {
    name: String,
    start: Option<Instant>,
    iterations: usize,
}

impl Benchmark {
    pub fn new(name: &str) -> Self {
        println!("\nRunning benchmark: {}", name);
        Self {
            name: name.to_string(),
            start: Some(Instant::now()),
            iterations: 1,
        }
    }

    pub fn iterations(mut self, n: usize) -> Self {
        self.iterations = n;
        self
    }

    pub fn run<F, T>(&mut self, f: F) -> T
    where
        F: Fn() -> T,
    {
        f()
    }

    pub fn finish(&mut self) {
        if let Some(start) = self.start.take() {
            let elapsed = start.elapsed();
            let per_iter = elapsed / self.iterations as u32;
            println!(
                "  {} iterations in {:?} ({:?} per iteration)",
                self.iterations,
                elapsed,
                per_iter
            );
        }
    }
}

// ============================================================================
// OmniDev Benchmarks
// ============================================================================

pub fn bench_omnidev_integration() {
    use sbmumc::OmniDevIntegration;
    
    let mut b = Benchmark::new("OmniDev Integration Initialization");
    let result = b.run(|| {
        OmniDevIntegration::new()
    });
    b.finish();
    
    assert!(!result.integration_id.is_empty());

    let mut integration = OmniDevIntegration::new();
    
    let mut b = Benchmark::new("OmniDev Process Refactor Request");
    b.iterations(100);
    let _ = b.run(|| {
        integration.process_refactor_request("Add new feature")
    });
    b.finish();

    let mut b = Benchmark::new("OmniDev System Status");
    b.iterations(1000);
    let _ = b.run(|| {
        integration.get_system_status()
    });
    b.finish();
}

pub fn bench_semantic_graph() {
    use sbmumc::SemanticGraphEngine;
    
    let engine = SemanticGraphEngine::new();
    
    let mut b = Benchmark::new("Semantic Search (5 results)");
    b.iterations(100);
    let _ = b.run(|| {
        engine.semantic_search("authentication", 5)
    });
    b.finish();
    
    let mut b = Benchmark::new("Context Compression");
    b.iterations(100);
    let _ = b.run(|| {
        engine.context_compression("Add new functionality")
    });
    b.finish();
    
    let mut b = Benchmark::new("Repository Indexing");
    b.iterations(10);
    let _ = b.run(|| {
        engine.index_repository("/test/repo")
    });
    b.finish();
}

pub fn bench_knowledge_storage() {
    use sbmumc::{KnowledgeStorageLayer, StorageBackend, StoredKnowledge, KnowledgeQuery};
    
    let storage = KnowledgeStorageLayer::new(StorageBackend::Memory);
    
    let knowledge = StoredKnowledge {
        id: "bench-1".to_string(),
        content: "Benchmark content".to_string(),
        embedding: vec![0.1; 384],
        metadata: Default::default(),
    };
    
    let mut b = Benchmark::new("Knowledge Store");
    b.iterations(1000);
    let _ = b.run(|| {
        storage.store(knowledge.clone())
    });
    b.finish();
    
    let query = KnowledgeQuery {
        text: Some("test".to_string()),
        vector: Some(vec![0.1; 384]),
        filters: None,
        limit: 10,
    };
    
    let mut b = Benchmark::new("Knowledge Query");
    b.iterations(100);
    let _ = b.run(|| {
        storage.query(&query)
    });
    b.finish();
}

pub fn bench_transaction() {
    use sbmumc::RepoTransaction;
    
    let mut b = Benchmark::new("Transaction Creation");
    b.iterations(10000);
    let _ = b.run(|| {
        RepoTransaction::new()
    });
    b.finish();
    
    let mut tx = RepoTransaction::new();
    tx.add_file_create("test.rs", "fn main() {}");
    tx.add_file_update("main.rs", "updated");
    
    let mut b = Benchmark::new("Transaction Execution");
    b.iterations(1000);
    let _ = b.run(|| {
        let mut t = RepoTransaction::new();
        t.add_file_create("test.rs", "content");
        t.execute()
    });
    b.finish();
}

pub fn bench_action_planner() {
    use sbmumc::{ActionPlan, ActionExecutor, PlannedAction, ActionType};
    
    let mut b = Benchmark::new("Action Plan Creation");
    b.iterations(10000);
    let _ = b.run(|| {
        ActionPlan::new()
    });
    b.finish();
    
    let mut plan = ActionPlan::new();
    for i in 0..100 {
        plan.add_action(PlannedAction {
            action_id: format!("action-{}", i),
            action_type: ActionType::Refactor,
            description: "Test".to_string(),
            estimated_cost: 1.0,
            dependencies: vec![],
            rollback_available: true,
        });
    }
    
    let executor = ActionExecutor::new(false);
    
    let mut b = Benchmark::new("Action Execution (100 actions)");
    b.iterations(10);
    let _ = b.run(|| {
        executor.execute_plan(&plan)
    });
    b.finish();
}

pub fn bench_indexing() {
    use sbmumc::{IncrementalIndexingPipeline, Language};
    
    let pipeline = IncrementalIndexingPipeline::new();
    
    let mut b = Benchmark::new("Repository Indexing");
    b.iterations(10);
    let _ = b.run(|| {
        pipeline.index_repository("/large/repo")
    });
    b.finish();
    
    let mut b = Benchmark::new("Symbol Extraction (Rust)");
    b.iterations(100);
    let _ = b.run(|| {
        pipeline.extract_symbols("src/main.rs", &Language::Rust)
    });
    b.finish();
    
    let mut b = Benchmark::new("Call Graph Generation");
    b.iterations(100);
    let _ = b.run(|| {
        pipeline.get_call_graph("src/main.rs")
    });
    b.finish();
}

pub fn bench_audit_trail() {
    use sbmumc::{AuditLog, AuditEntry};
    
    let mut log = AuditLog::new();
    
    let mut b = Benchmark::new("Audit Entry Append");
    b.iterations(10000);
    let _ = b.run(|| {
        let entry = AuditEntry {
            entry_id: format!("entry-{}", rand_simple() * 1000000.0 as u64),
            timestamp: chrono::Utc::now().timestamp(),
            action: "benchmark_action".to_string(),
            actor: "benchmark".to_string(),
            details: vec![],
            hash: None,
            prev_hash: None,
        };
        log.append_entry(entry);
    });
    b.finish();
    
    let mut b = Benchmark::new("Audit Log Export");
    b.iterations(100);
    let _ = b.run(|| {
        log.export_json()
    });
    b.finish();
}

pub fn bench_validation() {
    use sbmumc::ValidationLayer;
    
    let layer = ValidationLayer::new();
    let code = "fn test() { println!(\"hello\"); }";
    
    let mut b = Benchmark::new("Policy Check (single)");
    b.iterations(1000);
    let _ = b.run(|| {
        layer.check_policy(code, &["security"])
    });
    b.finish();
}

pub fn bench_configuration() {
    use sbmumc::SbmumcConfig;
    
    let mut b = Benchmark::new("Config Creation");
    b.iterations(10000);
    let _ = b.run(|| {
        SbmumcConfig::default()
    });
    b.finish();
    
    let config = SbmumcConfig::default();
    
    let mut b = Benchmark::new("Config Validation");
    b.iterations(10000);
    let _ = b.run(|| {
        config.validate()
    });
    b.finish();
    
    let mut b = Benchmark::new("Config to Env Vars");
    b.iterations(1000);
    let _ = b.run(|| {
        config.to_env_vars()
    });
    b.finish();
}

// ============================================================================
// Main Benchmark Runner
// ============================================================================

pub fn run_all_benchmarks() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║              SBMUMC PERFORMANCE BENCHMARKS                   ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    bench_omnidev_integration();
    bench_semantic_graph();
    bench_knowledge_storage();
    bench_transaction();
    bench_action_planner();
    bench_indexing();
    bench_audit_trail();
    bench_validation();
    bench_configuration();

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    BENCHMARKS COMPLETE                       ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}
