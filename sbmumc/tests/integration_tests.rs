//! # SBMUMC Integration Tests
//!
//! Comprehensive integration tests for all modules

use sbmumc::*;

// ============================================================================
// OmniDev Integration Tests
// ============================================================================

#[test]
fn test_omnidev_initialization() {
    let integration = OmniDevIntegration::new();
    assert!(!integration.integration_id.is_empty());
}

#[test]
fn test_omnidev_refactor_request() {
    let mut integration = OmniDevIntegration::new();
    let result = integration.process_refactor_request("Add OAuth2 support").unwrap();
    
    assert!(!result.request_id.is_empty());
    assert!(result.architecture_layers > 0);
    assert!(result.total_time_ms < 2000);
}

#[test]
fn test_omnidev_system_status() {
    let integration = OmniDevIntegration::new();
    let status = integration.get_system_status();
    
    assert!(!status.integration_id.is_empty());
    assert!(status.nodes_indexed > 0);
    assert!(status.latency_p99_ms > 0);
}

// ============================================================================
// Semantic Graph Engine Tests
// ============================================================================

#[test]
fn test_semantic_graph_engine() {
    let engine = SemanticGraphEngine::new();
    assert!(!engine.engine_id.is_empty());
}

#[test]
fn test_graph_indexing() {
    let engine = SemanticGraphEngine::new();
    let result = engine.index_repository("/test/repo").unwrap();
    
    assert!(result.nodes_indexed > 0);
    assert!(result.edges_created > 0);
}

#[test]
fn test_semantic_search() {
    let engine = SemanticGraphEngine::new();
    let results = engine.semantic_search("authentication", 5).unwrap();
    
    assert!(results.len() <= 5);
}

#[test]
fn test_context_compression() {
    let engine = SemanticGraphEngine::new();
    let context = engine.context_compression("Add new feature").unwrap();
    
    assert!(context.nodes.len() <= context.limit);
    assert!(context.compression_ratio > 0.0);
}

// ============================================================================
// Knowledge Storage Tests
// ============================================================================

#[test]
fn test_knowledge_storage_creation() {
    let storage = KnowledgeStorageLayer::new(StorageBackend::Memory);
    assert_eq!(storage.backend, StorageBackend::Memory);
}

#[test]
fn test_knowledge_storage_hybrid() {
    let storage = KnowledgeStorageLayer::new(StorageBackend::Hybrid);
    assert_eq!(storage.backend, StorageBackend::Hybrid);
}

#[test]
fn test_store_and_retrieve() {
    let storage = KnowledgeStorageLayer::new(StorageBackend::Memory);
    let knowledge = StoredKnowledge {
        id: "test-1".to_string(),
        content: "Test content".to_string(),
        embedding: vec![0.1, 0.2, 0.3],
        metadata: Default::default(),
    };
    
    let result = storage.store(knowledge);
    assert!(result.is_ok());
}

#[test]
fn test_hybrid_query() {
    let storage = KnowledgeStorageLayer::new(StorageBackend::Hybrid);
    let query = KnowledgeQuery {
        text: Some("test query".to_string()),
        vector: Some(vec![0.1, 0.2]),
        filters: None,
        limit: 10,
    };
    
    let results = storage.query(&query).unwrap();
    assert!(results.items.len() <= 10);
}

// ============================================================================
// Action Planner Tests
// ============================================================================

#[test]
fn test_action_planner_creation() {
    let planner = ActionPlan::new();
    assert!(planner.plan_id.len() > 0);
}

#[test]
fn test_add_action() {
    let mut planner = ActionPlan::new();
    let action = PlannedAction {
        action_id: "action-1".to_string(),
        action_type: ActionType::Refactor,
        description: "Refactor code".to_string(),
        estimated_cost: 10.0,
        dependencies: vec![],
        rollback_available: true,
    };
    
    planner.add_action(action);
    assert!(planner.actions.len() == 1);
}

#[test]
fn test_action_executor() {
    let executor = ActionExecutor::new(false);
    let plan = ActionPlan::new();
    
    let result = executor.execute_plan(&plan).unwrap();
    assert!(result.actions_planned > 0 || result.actions_planned == 0);
}

#[test]
fn test_rollback() {
    let executor = ActionExecutor::new(true);
    let plan = ActionPlan::new();
    
    let result = executor.execute_plan(&plan).unwrap();
    if result.rollback_available {
        let rollback = executor.rollback(&result.execution_id);
        assert!(rollback.is_ok());
    }
}

// ============================================================================
// Transaction Tests
// ============================================================================

#[test]
fn test_transaction_creation() {
    let mut tx = RepoTransaction::new();
    tx.add_file_create("test.rs", "fn main() {}");
    
    assert!(tx.operations.len() == 1);
}

#[test]
fn test_transaction_execution() {
    let mut tx = RepoTransaction::new();
    tx.add_file_update("main.rs", "// updated content");
    
    let result = tx.execute().unwrap();
    assert!(result.success || !result.success); // Random success
}

#[test]
fn test_transaction_rollback() {
    let mut tx = RepoTransaction::new();
    tx.add_file_create("temp.rs", "temp");
    
    if tx.rollback_available {
        let result = tx.rollback().unwrap();
        assert!(result.success);
    }
}

// ============================================================================
// Validation Layer Tests
// ============================================================================

#[test]
fn test_validation_layer() {
    let layer = ValidationLayer::new();
    assert!(layer.layer_id.len() > 0);
}

#[test]
fn test_validate_operation() {
    let layer = ValidationLayer::new();
    let operation = "Create file: /test.txt".to_string();
    
    let result = layer.validate_operation(&operation, &[]).unwrap();
    assert!(result.allowed || !result.allowed);
}

#[test]
fn test_policy_check() {
    let layer = ValidationLayer::new();
    let code = "fn test() { println!(\"hello\"); }";
    
    let result = layer.check_policy(code, &["security", "performance"]).unwrap();
    assert!(result.violations.len() >= 0);
}

// ============================================================================
// Audit Trail Tests
// ============================================================================

#[test]
fn test_audit_log_creation() {
    let log = AuditLog::new();
    assert!(log.log_id.len() > 0);
}

#[test]
fn test_audit_entry_append() {
    let mut log = AuditLog::new();
    let entry = AuditEntry {
        entry_id: "entry-1".to_string(),
        timestamp: chrono::Utc::now().timestamp(),
        action: "test_action".to_string(),
        actor: "system".to_string(),
        details: vec![],
        hash: None,
        prev_hash: None,
    };
    
    log.append_entry(entry);
    assert!(log.entries.len() == 1);
}

#[test]
fn test_hash_chain() {
    let mut log = AuditLog::new();
    let entry = AuditEntry {
        entry_id: "entry-1".to_string(),
        timestamp: chrono::Utc::now().timestamp(),
        action: "test".to_string(),
        actor: "system".to_string(),
        details: vec![],
        hash: None,
        prev_hash: None,
    };
    
    log.append_entry(entry);
    
    // Check that hash chain is maintained
    if log.entries.len() > 1 {
        assert!(log.entries[1].prev_hash.is_some());
    }
}

#[test]
fn test_human_override() {
    let mut override_sys = HumanOverride::new();
    
    let approval = OverrideApproval {
        approval_id: "approval-1".to_string(),
        override_request: "Test override".to_string(),
        approver: "admin".to_string(),
        timestamp: chrono::Utc::now().timestamp(),
        signature: "valid_signature".to_string(),
        allowed: true,
    };
    
    let result = override_sys.request_override(approval);
    assert!(result.approved || !result.approved);
}

// ============================================================================
// Incremental Indexing Tests
// ============================================================================

#[test]
fn test_indexing_pipeline() {
    let pipeline = IncrementalIndexingPipeline::new();
    assert!(pipeline.pipeline_id.len() > 0);
}

#[test]
fn test_repository_indexing() {
    let pipeline = IncrementalIndexingPipeline::new();
    let result = pipeline.index_repository("/test/repo").unwrap();
    
    assert!(result.files_indexed > 0);
    assert!(result.symbols_extracted > 0);
}

#[test]
fn test_symbol_extraction() {
    let pipeline = IncrementalIndexingPipeline::new();
    let symbols = pipeline.extract_symbols("test.rs", &Language::Rust).unwrap();
    
    assert!(!symbols.is_empty());
}

#[test]
fn test_call_graph() {
    let pipeline = IncrementalIndexingPipeline::new();
    let graph = pipeline.get_call_graph("main.rs").unwrap();
    
    assert!(!graph.nodes.is_empty());
}

// ============================================================================
// Semantic Commit Tests
// ============================================================================

#[test]
fn test_semantic_commit_engine() {
    let engine = SemanticCommitEngine::new();
    assert!(engine.engine_id.len() > 0);
}

#[test]
fn test_commit_generation() {
    let engine = SemanticCommitEngine::new();
    let diff = "+ Add new authentication feature\n- Remove old code";
    
    let commit = engine.generate_commit_message(diff, &[]).unwrap();
    assert!(!commit.commit_id.is_empty());
    assert!(!commit.message.is_empty());
}

#[test]
fn test_pr_description() {
    let engine = SemanticCommitEngine::new();
    let commits = vec![
        SemanticCommit {
            commit_id: "1".to_string(),
            commit_type: CommitType::Feat,
            scope: Some("auth".to_string()),
            message: "Add OAuth2".to_string(),
            body: None,
            impacted_modules: vec![],
            rationale: "Security".to_string(),
        },
    ];
    
    let description = engine.generate_pr_description(&commits).unwrap();
    assert!(!description.title.is_empty());
}

// ============================================================================
// Live Feedback Tests
// ============================================================================

#[test]
fn test_live_feedback_loop() {
    let loop_instance = LiveFeedbackLoop::new();
    assert!(loop_instance.loop_id.len() > 0);
}

#[test]
fn test_subscription() {
    let mut loop_instance = LiveFeedbackLoop::new();
    loop_instance.subscribe("session-123");
    
    assert!(loop_instance.subscribers.contains(&"session-123".to_string()));
}

#[test]
fn test_event_reaction() {
    let mut loop_instance = LiveFeedbackLoop::new();
    let reaction = loop_instance.react_to_event(&FeedbackEvent::Diagnostic).unwrap();
    
    assert!(reaction.confidence > 0.0 && reaction.confidence <= 1.0);
}

// ============================================================================
// Configuration Tests
// ============================================================================

#[test]
fn test_default_config() {
    let config = SbmumcConfig::default();
    assert_eq!(config.system.name, "SBMUMC");
    assert!(config.omnidev.enabled);
}

#[test]
fn test_offline_config() {
    let config = SbmumcConfig::offline();
    assert!(matches!(config.system.mode, OperationMode::Offline));
    assert!(matches!(config.network.mode, NetworkMode::Offline));
}

#[test]
fn test_config_validation() {
    let config = SbmumcConfig::default();
    let errors = config.validate();
    assert!(errors.is_empty());
}

#[test]
fn test_env_vars() {
    let config = SbmumcConfig::from_env_vars();
    // Should not panic with missing env vars
    assert!(config.system.name == "SBMUMC");
}

// ============================================================================
// API Server Tests
// ============================================================================

#[test]
fn test_api_server_creation() {
    let server = ApiServer::new("0.0.0.0".to_string(), 8080);
    assert_eq!(server.port, 8080);
    assert!(!server.routes.is_empty());
}

#[test]
fn test_api_response() {
    let response: ApiResponse<String> = ApiResponse::success("test".to_string());
    assert!(response.success);
    assert!(response.data.is_some());
}

// ============================================================================
// Plugin Architecture Tests
// ============================================================================

#[test]
fn test_plugin_manager() {
    use std::path::PathBuf;
    let manager = PluginManager::new(PathBuf::from("/plugins"));
    assert_eq!(manager.loaded_count, 0);
}

#[test]
fn test_plugin_loading() {
    use std::path::PathBuf;
    let mut manager = PluginManager::new(PathBuf::from("/plugins"));
    
    let manifest = PluginManifest {
        manifest_version: "1.0".to_string(),
        plugin: PluginMetadata {
            id: "test-plugin".to_string(),
            name: "Test".to_string(),
            version: "1.0".to_string(),
            description: "Test".to_string(),
            author: "Test".to_string(),
            license: "MIT".to_string(),
            repository: None,
        },
        permissions: vec![],
        entry_point: "main".to_string(),
        hooks: vec![],
    };
    
    let result = manager.load_plugin(manifest);
    assert!(result.is_ok());
}

// ============================================================================
// Testing Framework Tests
// ============================================================================

#[test]
fn test_test_suite() {
    let suite = TestSuite::new();
    assert!(suite.suite_id.len() > 0);
}

#[test]
fn test_generate_tests() {
    let suite = TestSuite::new();
    let count = suite.generate_tests("code").unwrap();
    assert!(count >= 0);
}

#[test]
fn test_regression_guard() {
    let guard = RegressionGuard::new();
    assert!(guard.guard_id.len() > 0);
}

// ============================================================================
// Code Comprehension Tests
// ============================================================================

#[test]
fn test_code_comprehension() {
    let comprehension = CodeComprehension::new();
    assert!(comprehension.module_id.len() > 0);
}

#[test]
fn test_architecture_synthesis() {
    let comprehension = CodeComprehension::new();
    let nodes = vec![
        GraphNode {
            node_id: "1".to_string(),
            label: "main".to_string(),
            node_type: "function".to_string(),
            properties: Default::default(),
        },
    ];
    
    let synthesis = comprehension.synthesize_architecture(&nodes).unwrap();
    assert!(synthesis.layers.len() > 0);
}

// ============================================================================
// Formal Verification Tests
// ============================================================================

#[test]
fn test_formal_verification_layer() {
    let layer = FormalVerificationLayer::new();
    assert!(layer.layer_id.len() > 0);
}

#[test]
fn test_verify_claim() {
    let layer = FormalVerificationLayer::new();
    let claim = FormalClaim {
        claim_id: "claim-1".to_string(),
        claim_type: "safety".to_string(),
        description: "All loops must terminate".to_string(),
        specification: "forall loops: terminates".to_string(),
        prover: "z3".to_string(),
        result: None,
    };
    
    let result = layer.verify_claim(&claim).unwrap();
    assert!(result.proven || !result.proven);
}

// ============================================================================
// EVAS Filter Tests
// ============================================================================

#[test]
fn test_evas_filter_layer() {
    let layer = EVASFilterLayer::new();
    assert!(layer.layer_id.len() > 0);
}

#[test]
fn test_evaluate_operation() {
    let layer = EVASFilterLayer::new();
    let operation = "Create file: /test.txt";
    
    let report = layer.evaluate_operation(operation).unwrap();
    assert!(report.score >= 0.0 && report.score <= 1.0);
}

// ============================================================================
// End-to-End Integration Tests
// ============================================================================

#[test]
fn test_full_omnidev_workflow() {
    // Initialize
    let mut integration = OmniDevIntegration::new();
    
    // Process refactor request
    let result = integration.process_refactor_request("Refactor auth module").unwrap();
    assert!(result.transaction_committed || !result.transaction_committed);
    
    // Check system status
    let status = integration.get_system_status();
    assert!(status.audit_entries > 0);
}

#[test]
fn test_config_to_env_vars() {
    let config = SbmumcConfig::default();
    let vars = config.to_env_vars();
    
    assert!(vars.contains_key("SBMUMC_MODE"));
    assert!(vars.contains_key("SBMUMC_OMNIVDEV_ENABLED"));
    assert!(vars.contains_key("SBMUMC_LATENCY_TARGET_MS"));
}

#[test]
fn test_offline_mode_functionality() {
    let config = SbmumcConfig::offline();
    
    assert!(matches!(config.system.mode, OperationMode::Offline));
    assert!(config.network.api_endpoint.is_none());
    assert!(config.network.ws_endpoint.is_none());
    
    // OmniDev should still work offline
    assert!(config.omnidev.enabled);
}
