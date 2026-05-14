// SBMUMC OmniDev Language Server Protocol (LSP)
// Common protocol definitions for all editor integrations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeParams {
    pub process_id: Option<u64>,
    pub root_uri: Option<String>,
    pub capabilities: ClientCapabilities,
    pub workspace_folders: Option<Vec<WorkspaceFolder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCapabilities {
    pub text_document: Option<TextDocumentClientCapabilities>,
    pub workspace: Option<WorkspaceClientCapabilities>,
    pub window: Option<WindowClientCapabilities>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentClientCapabilities {
    pub synchronization: Option<TextDocumentSyncClientCapabilities>,
    pub completion: Option<CompletionClientCapabilities>,
    pub hover: Option<HoverClientCapabilities>,
    pub signature_help: Option<SignatureHelpClientCapabilities>,
    pub goto_definition: Option<GotoDefinitionClientCapabilities>,
    pub references: Option<ReferencesClientCapabilities>,
    pub code_action: Option<CodeActionClientCapabilities>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceClientCapabilities {
    pub workspace_folders: bool,
    pub apply_edit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowClientCapabilities {
    pub work_done_progress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentSyncClientCapabilities {
    pub dynamic_registration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionClientCapabilities {
    pub dynamic_registration: bool,
    pub completion_item: Option<CompletionItemCapabilities>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItemCapabilities {
    pub snippet_support: bool,
    pub commit_characters_support: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverClientCapabilities {
    pub dynamic_registration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureHelpClientCapabilities {
    pub dynamic_registration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GotoDefinitionClientCapabilities {
    pub dynamic_registration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferencesClientCapabilities {
    pub dynamic_registration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeActionClientCapabilities {
    pub dynamic_registration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFolder {
    pub uri: String,
    pub name: String,
}

// ============================================================================
// SBMUMC OmniDev Extended Capabilities
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbmumcCapabilities {
    pub semantic_search: bool,
    pub semantic_index: bool,
    pub refactor_suggestions: bool,
    pub commit_generation: bool,
    pub test_generation: bool,
    pub formal_verification: bool,
    pub audit_trail: bool,
    pub inline_completion: bool,
    pub code_lens: bool,
}

impl Default for SbmumcCapabilities {
    fn default() -> Self {
        Self {
            semantic_search: true,
            semantic_index: true,
            refactor_suggestions: true,
            commit_generation: true,
            test_generation: true,
            formal_verification: true,
            audit_trail: true,
            inline_completion: true,
            code_lens: true,
        }
    }
}

// ============================================================================
// OmniDev Request Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmniDevParams {
    pub query: String,
    pub context_uri: Option<String>,
    pub language: Option<String>,
    pub position: Option<Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactorParams {
    pub text_document: TextDocumentIdentifier,
    pub range: Range,
    pub context: RefactorContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactorContext {
    pub selected_code: String,
    pub language: String,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchParams {
    pub query: String,
    pub max_results: Option<usize>,
    pub scope: Option<SearchScope>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchScope {
    pub files: Option<Vec<String>>,
    pub folders: Option<Vec<String>>,
    pub exclude_patterns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitParams {
    pub diff: String,
    pub conventional: Option<bool>,
    pub scope: Option<String>,
    pub breaking: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestGenerationParams {
    pub text_document: TextDocumentIdentifier,
    pub framework: Option<String>,
    pub style: Option<TestStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestStyle {
    Unit,
    Integration,
    Property,
    Benchmark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationParams {
    pub text_document: TextDocumentIdentifier,
    pub claims: Vec<String>,
    pub prover: Option<String>,
}

// ============================================================================
// Response Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmniDevResponse {
    pub response: String,
    pub actions: Vec<SuggestedAction>,
    pub context_nodes: Vec<ContextNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedAction {
    pub title: String,
    pub command: String,
    pub arguments: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextNode {
    pub id: String,
    pub label: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactorResponse {
    pub suggestions: Vec<RefactorSuggestion>,
    pub applicable_diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactorSuggestion {
    pub title: String,
    pub description: String,
    pub new_code: String,
    pub confidence: f64,
    pub kind: RefactorKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefactorKind {
    ExtractMethod,
    InlineVariable,
    Rename,
    MoveCode,
    Simplify,
    Optimize,
    Security,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total_count: usize,
    pub search_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub context: String,
    pub score: f64,
    pub match_type: MatchType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchType {
    Exact,
    Semantic,
    Pattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitResponse {
    pub message: String,
    pub title: String,
    pub body: Option<String>,
    pub breaking: bool,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestGenerationResponse {
    pub content: String,
    pub file_path: String,
    pub framework: String,
    pub tests_generated: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResponse {
    pub proofs: Vec<ProofResult>,
    pub total_proven: usize,
    pub total_claims: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofResult {
    pub claim: String,
    pub proven: bool,
    pub prover: String,
    pub time_ms: u64,
    pub error: Option<String>,
}

// ============================================================================
// Common LSP Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: DiagnosticSeverity,
    pub code: Option<String>,
    pub source: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAction {
    pub title: String,
    pub kind: Option<CodeActionKind>,
    pub command: Option<Command>,
    pub edit: Option<WorkspaceEdit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeActionKind {
    QuickFix,
    Refactor,
    Source,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub title: String,
    pub command: String,
    pub arguments: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceEdit {
    pub changes: Option<HashMap<String, Vec<TextEdit>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

// ============================================================================
// Notification Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDiagnosticsParams {
    pub uri: String,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowMessageParams {
    pub r#type: MessageType,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Error = 1,
    Warning = 2,
    Info = 3,
    Log = 4,
}

// ============================================================================
// SBMUMC Custom Methods
// ============================================================================

pub const SBMUMC_OMNIDEV_METHOD: &str = "sbmumc/omnidev";
pub const SBMUMC_REFACTOR_METHOD: &str = "sbmumc/refactor";
pub const SBMUMC_SEARCH_METHOD: &str = "sbmumc/search";
pub const SBMUMC_COMMIT_METHOD: &str = "sbmumc/commit";
pub const SBMUMC_TEST_METHOD: &str = "sbmumc/test";
pub const SBMUMC_VERIFY_METHOD: &str = "sbmumc/verify";
pub const SBMUMC_INDEX_METHOD: &str = "sbmumc/index";
pub const SBMUMC_AUDIT_METHOD: &str = "sbmumc/audit";
pub const SBMUMC_STATUS_METHOD: &str = "sbmumc/status";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capabilities_default() {
        let caps = SbmumcCapabilities::default();
        assert!(caps.semantic_search);
        assert!(caps.formal_verification);
        assert!(caps.audit_trail);
    }

    #[test]
    fn test_position_serialization() {
        let pos = Position { line: 10, character: 5 };
        let json = serde_json::to_string(&pos).unwrap();
        assert!(json.contains("10"));
        assert!(json.contains("5"));
    }
}