//! Core Module - Essential types, errors, and utilities for SBMUMC
//!
//! This module provides the foundational types and error handling used throughout
//! the SBMUMC system.

pub mod types;
pub mod error;
pub mod config;
pub mod utils;

pub use types::*;
pub use error::{SbmumcError, Result};
pub use config::SbmumcConfig;
pub use utils::*;

/// Main SBMUMC system struct
pub mod sbmumc_system {
    use super::*;
    use crate::{Cortex, knowledge::KnowledgeGraph, reasoning::Reasoner, learning::MetaLearner, security::SecurityLayer, io::InputHandler, io::OutputHandler, admin::AdminInterface, compiler::MetaCompiler};
    use std::sync::Arc;
    use parking_lot::RwLock;
    use tokio::sync::mpsc;

    /// The main SBMUMC system orchestrator
    pub struct Sbmumc {
        /// System configuration
        config: SbmumcConfig,

        /// Central processing unit
        cortex: Arc<Cortex>,

        /// Knowledge graph
        knowledge: Arc<RwLock<KnowledgeGraph>>,

        /// Reasoning engine
        reasoner: Arc<Reasoner>,

        /// Meta-learning system
        meta_learner: Arc<MetaLearner>,

        /// Security layer
        security: Arc<SecurityLayer>,

        /// Input handler
        input_handler: Arc<InputHandler>,

        /// Output handler
        output_handler: Arc<OutputHandler>,

        /// Admin interface
        admin: Arc<AdminInterface>,

        /// Meta-compiler
        compiler: Arc<MetaCompiler>,

        /// Shutdown signal sender
        shutdown_tx: Option<mpsc::Sender<()>>,

        /// Running state
        running: Arc<RwLock<bool>>,
    }

    impl Sbmumc {
        /// Create a new SBMUMC instance
        pub async fn new(config: SbmumcConfig) -> Result<Self> {
            info!("Initializing SBMUMC system");

            // Initialize core components
            let cortex = Arc::new(Cortex::new(&config)?);
            let knowledge = Arc::new(RwLock::new(KnowledgeGraph::new()));
            let reasoner = Arc::new(Reasoner::new(knowledge.clone())?);
            let meta_learner = Arc::new(MetaLearner::new()?);
            let security = Arc::new(SecurityLayer::new(&config)?);
            let input_handler = Arc::new(InputHandler::new()?);
            let output_handler = Arc::new(OutputHandler::new()?);
            let admin = Arc::new(AdminInterface::new(knowledge.clone(), security.clone()).await?);
            let compiler = Arc::new(MetaCompiler::new()?);

            // Create shutdown channel
            let (shutdown_tx, _) = mpsc::channel(1);

            Ok(Self {
                config,
                cortex,
                knowledge,
                reasoner,
                meta_learner,
                security,
                input_handler,
                output_handler,
                admin,
                compiler,
                shutdown_tx: Some(shutdown_tx),
                running: Arc::new(RwLock::new(false)),
            })
        }

        /// Start the SBMUMC system
        pub async fn start(&mut self) -> Result<()> {
            if *self.running.read() {
                return Err(SbmumcError::SystemAlreadyRunning);
            }

            info!("Starting SBMUMC components");

            // Start security monitoring
            self.security.start_monitoring().await?;

            // Start the cortex
            self.cortex.start().await?;

            // Mark as running
            *self.running.write() = true;

            info!("SBMUMC system started");
            Ok(())
        }

        /// Stop the SBMUMC system
        pub async fn stop(&mut self) -> Result<()> {
            if !*self.running.read() {
                return Err(SbmumcError::SystemNotRunning);
            }

            info!("Stopping SBMUMC system");

            // Stop security monitoring
            self.security.stop_monitoring().await?;

            // Stop the cortex
            self.cortex.stop().await?;

            // Mark as not running
            *self.running.write() = false;

            info!("SBMUMC system stopped");
            Ok(())
        }

        /// Compile a file or project
        pub async fn compile_file(&self, path: &std::path::Path, target: &str) -> Result<CompilationResult> {
            self.compiler.compile_file(path, target).await
        }

        /// Compile a grammar specification
        pub async fn compile_grammar(&self, grammar: &str) -> Result<GrammarResult> {
            self.compiler.compile_grammar(grammar).await
        }

        /// Compile a programming language definition
        pub async fn compile_language(&self, definition: &LanguageDefinition) -> Result<LanguageResult> {
            self.compiler.compile_language(definition).await
        }

        /// Start the admin interface
        pub async fn start_admin_interface(&self) -> Result<()> {
            self.admin.start().await
        }

        /// Wait for shutdown signal
        pub async fn wait_for_shutdown(&self) -> Result<()> {
            let _ = self.shutdown_tx.take();
            // In a real implementation, this would wait for a shutdown signal
            tokio::signal::ctrl_c().await.map_err(|_| SbmumcError::ShutdownFailed)?;
            Ok(())
        }

        /// Get system status
        pub fn get_status(&self) -> SystemStatus {
            SystemStatus {
                running: *self.running.read(),
                version: crate::VERSION.to_string(),
                components: vec![
                    ComponentStatus {
                        name: "cortex".to_string(),
                        active: *self.running.read(),
                    },
                    ComponentStatus {
                        name: "security".to_string(),
                        active: *self.running.read(),
                    },
                ],
            }
        }
    }

    /// Compilation result
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct CompilationResult {
        pub success: bool,
        pub output_path: Option<String>,
        pub errors: Vec<String>,
        pub warnings: Vec<String>,
    }

    /// Grammar compilation result
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct GrammarResult {
        pub success: bool,
        pub parser_code: Option<String>,
        pub lexer_code: Option<String>,
    }

    /// Language definition
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct LanguageDefinition {
        pub name: String,
        pub version: String,
        pub grammar: String,
        pub semantics: String,
    }

    /// Language compilation result
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct LanguageResult {
        pub success: bool,
        pub compiler_code: Option<String>,
        pub runtime_code: Option<String>,
    }

    /// System status
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct SystemStatus {
        pub running: bool,
        pub version: String,
        pub components: Vec<ComponentStatus>,
    }

    /// Component status
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct ComponentStatus {
        pub name: String,
        pub active: bool,
    }
}
