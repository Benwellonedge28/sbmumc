//! Samuel Benwellonedge Mukandara Universal Meta-Compiler (SBMUMC)
//!
//! A comprehensive AGI system and universal compiler framework designed to:
//! - Compile everything from grammar files to programming languages
//! - Provide meta-compiler capabilities
//! - Support sovereign OS development
//! - Ensure AI safety and control
//! - Enable human-AI collaboration
//!
//! # Architecture
//!
//! The system is organized into the following core modules:
//!
//! ## Core System
//! - [`cortex`] - Central processing unit for information handling
//! - [`core`] - Essential system components and utilities
//!
//! ## Knowledge & Reasoning
//! - [`knowledge`] - Knowledge representation and graph management
//! - [`reasoning`] - Reasoning, planning, and decision making
//!
//! ## Learning Systems
//! - [`learning`] - Meta-learning, active learning, and self-supervised learning
//!
//! ## Input/Output
//! - [`io`] - Multi-modal input/output handling
//! - [`language`] - Natural language processing
//!
//! ## Security & Control
//! - [`security`] - Layered security and intrusion detection
//! - [`admin`] - Administration interface
//!
//! ## AGI Capabilities
//! - [`agi`] - Self-awareness, theory of mind, emotional intelligence
//!
//! ## Ethics & Values
//! - [`ethics`] - Ethical frameworks, human values alignment, cultural adaptation
//!
//! ## Compilation
//! - [`compiler`] - Universal meta-compiler framework
//!
//! # Design Principles
//!
//! 1. **Program-Once-Compile-Run-Everywhere**: Cross-platform compilation targets
//! 2. **Sovereign AGI**: Self-controlled, explainable AI behavior
//! 3. **Human Safety First**: Prioritizing human well-being in all decisions
//! 4. **Cultural Adaptation**: Region-aware behavior and responses
//! 5. **Zero-Knowledge Initialization**: Starting from minimal pre-programmed knowledge

// ============================================================================
// CORE MODULES
// ============================================================================

pub mod cortex;
pub mod core;

// ============================================================================
// KNOWLEDGE & REASONING
// ============================================================================

pub mod knowledge;
pub mod reasoning;

// ============================================================================
// LEARNING SYSTEMS
// ============================================================================

pub mod learning;

// ============================================================================
// INPUT/OUTPUT
// ============================================================================

pub mod io;
pub mod language;

// ============================================================================
// SECURITY & ADMINISTRATION
// ============================================================================

pub mod security;
pub mod admin;

// ============================================================================
// COMPILER FRAMEWORK
// ============================================================================

pub mod compiler;

// ============================================================================
// AGI CAPABILITIES
// ============================================================================

pub mod agi;

// ============================================================================
// ETHICS & VALUES
// ============================================================================

pub mod ethics;

// ============================================================================
// RE-EXPORTS FOR CONVENIENCE
// ============================================================================

pub use cortex::Cortex;
pub use core::{Sbmumc, SbmumcConfig};
pub use knowledge::{KnowledgeGraph, KnowledgeNode};
pub use reasoning::{Planner, Reasoner};
pub use learning::{MetaLearner, SelfSupervisedLearner};
pub use security::{SecurityLayer, IntrusionDetector};
pub use io::{InputHandler, OutputHandler};
pub use language::{NlpEngine, Translator};
pub use admin::{AdminInterface, DocumentCompiler};
pub use compiler::{MetaCompiler, GrammarCompiler, LanguageCompiler};
pub use agi::{AgiEngine, SelfAwareness, TheoryOfMind, EmotionalIntelligence, CommonSenseKB, ImaginationEngine};
pub use ethics::{EthicalFramework, HumanValuesAlignment, CulturalAdaptation, SafetyConstraints};

// ============================================================================
// RE-EXPORTS FOR COMMON TYPES
// ============================================================================

pub use core::error::{SbmumcError, Result};
pub use core::types::*;

/// SBMUMC library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// System name
pub const SYSTEM_NAME: &str = "Samuel Benwellonedge Mukandara Universal Meta-Compiler";

/// Initialize the tracing/logging system
pub fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();
}
