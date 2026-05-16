//!
//! # SBMUMC Module 1579: Documentation Generation System
//!
//! Automatic documentation generation from code, API documentation,
//! architecture diagrams, and knowledge base management.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Documentation target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocTarget {
    pub id: String,
    pub name: String,
    pub doc_type: DocType,
    pub source_path: String,
    pub output_format: DocFormat,
    pub config: DocConfig,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Documentation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocType {
    API,
    Code,
    Architecture,
    UserGuide,
    Tutorial,
    Reference,
    Changelog,
    README,
}

/// Output formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocFormat {
    Markdown,
    HTML,
    PDF,
    ReStructuredText,
    AsciiDoc,
    Confluence,
}

/// Documentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocConfig {
    pub include_private: bool,
    pub include_examples: bool,
    pub include_tests: bool,
    pub include_diagrams: bool,
    pub style: DocStyle,
    pub toc_depth: u32,
    pub syntax_highlighting: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocStyle {
    Standard,
    Docusaurus,
    ReadTheDocs,
    Swagger,
    OpenAPI,
    VuePress,
}

/// Documentation section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSection {
    pub id: String,
    pub title: String,
    pub level: u32,
    pub content: String,
    pub subsections: Vec<DocSection>,
    pub code_examples: Vec<CodeExample>,
    pub diagrams: Vec<Diagram>,
}

/// Code example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub id: String,
    pub language: String,
    pub code: String,
    pub description: String,
    pub annotations: Vec<Annotation>,
}

/// Annotation for code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub line: u32,
    pub text: String,
    pub annotation_type: AnnotationType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnnotationType {
    Note,
    Warning,
    Tip,
    Important,
    Example,
}

/// Diagram types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Diagram {
    SequenceDiagram { steps: Vec<SequenceStep> },
    Flowchart { nodes: Vec<FlowNode>, edges: Vec<FlowEdge> },
    ClassDiagram { classes: Vec<ClassDef> },
    ArchitectureDiagram { components: Vec<Component> },
}

/// Sequence diagram step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceStep {
    pub from: String,
    pub to: String,
    pub action: String,
    pub response: Option<String>,
}

/// Flowchart node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowNode {
    pub id: String,
    pub label: String,
    pub node_type: FlowNodeType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FlowNodeType {
    Start,
    End,
    Process,
    Decision,
    Input,
    Output,
}

/// Flowchart edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowEdge {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
}

/// Class definition for diagrams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDef {
    pub name: String,
    pub fields: Vec<FieldDef>,
    pub methods: Vec<MethodDef>,
    pub relations: Vec<ClassRelation>,
}

/// Field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDef {
    pub name: String,
    pub field_type: String,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Package,
}

/// Method definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodDef {
    pub name: String,
    pub parameters: Vec<ParamDef>,
    pub return_type: String,
    pub visibility: Visibility,
}

/// Parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamDef {
    pub name: String,
    pub param_type: String,
    pub optional: bool,
}

/// Class relation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassRelation {
    pub target: String,
    pub relation_type: RelationType,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationType {
    Inheritance,
    Composition,
    Aggregation,
    Association,
    Dependency,
}

/// Component for architecture diagrams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub component_type: ComponentType,
    pub description: String,
    pub connections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComponentType {
    Service,
    Database,
    Cache,
    Queue,
    Gateway,
    Client,
}

/// Generated documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedDoc {
    pub id: String,
    pub target_id: String,
    pub sections: Vec<DocSection>,
    pub table_of_contents: Vec<TocEntry>,
    pub generated_at: u64,
    pub version: String,
}

/// Table of contents entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocEntry {
    pub title: String,
    pub level: u32,
    pub anchor: String,
    pub children: Vec<TocEntry>,
}

/// API endpoint documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpointDoc {
    pub method: String,
    pub path: String,
    pub summary: String,
    pub description: String,
    pub parameters: Vec<ApiParameter>,
    pub request_body: Option<ApiBody>,
    pub responses: Vec<ApiResponse>,
    pub security: Vec<String>,
    pub examples: Vec<ApiExample>,
}

/// API parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiParameter {
    pub name: String,
    pub location: ParamLocation,
    pub param_type: String,
    pub required: bool,
    pub description: String,
    pub schema: Option<serde_json::Value>,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParamLocation {
    Path,
    Query,
    Header,
    Cookie,
}

/// API request/response body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiBody {
    pub content_type: String,
    pub schema: serde_json::Value,
    pub example: Option<serde_json::Value>,
}

/// API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status_code: u32,
    pub description: String,
    pub schema: Option<serde_json::Value>,
    pub example: Option<serde_json::Value>,
}

/// API example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiExample {
    pub title: String,
    pub request: ExampleRequest,
    pub response: ExampleResponse,
}

/// Example request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleRequest {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
}

/// Example response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleResponse {
    pub status_code: u32,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
}

/// Documentation service
pub struct DocumentationService {
    targets: Arc<RwLock<HashMap<String, DocTarget>>>,
    generated_docs: Arc<RwLock<HashMap<String, GeneratedDoc>>>,
    templates: Arc<RwLock<HashMap<String, DocTemplate>>>,
    knowledge_base: Arc<RwLock<KnowledgeBase>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocTemplate {
    pub id: String,
    pub name: String,
    pub template_type: DocType,
    pub template: String,
    pub variables: Vec<TemplateVar>,
}

/// Template variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVar {
    pub name: String,
    pub var_type: String,
    pub required: bool,
}

/// Knowledge base entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub entries: Vec<KnowledgeEntry>,
    pub categories: Vec<String>,
    pub tags: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
    pub related_entries: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl DocumentationService {
    pub fn new() -> Self {
        let service = Self {
            targets: Arc::new(RwLock::new(HashMap::new())),
            generated_docs: Arc::new(RwLock::new(HashMap::new())),
            templates: Arc::new(RwLock::new(HashMap::new())),
            knowledge_base: Arc::new(RwLock::new(KnowledgeBase {
                entries: vec![],
                categories: vec![],
                tags: HashMap::new(),
            })),
        };

        service.init_default_templates();
        service
    }

    /// Create documentation target
    pub fn create_target(&self, target: DocTarget) -> String {
        let mut targets = self.targets.write().unwrap();
        targets.insert(target.id.clone(), target.clone());
        target.id
    }

    /// Generate documentation
    pub async fn generate(&self, target_id: &str) -> Result<GeneratedDoc, DocError> {
        let targets = self.targets.read().unwrap();
        let target = targets.get(target_id)
            .ok_or(DocError::TargetNotFound)?
            .clone();
        drop(targets);

        let sections = match target.doc_type {
            DocType::API => self.generate_api_docs(&target).await?,
            DocType::Code => self.generate_code_docs(&target).await?,
            DocType::Architecture => self.generate_arch_docs(&target).await?,
            DocType::UserGuide => self.generate_user_guide(&target).await?,
            _ => self.generate_generic_docs(&target).await?,
        };

        let toc = self.build_toc(&sections);

        let generated = GeneratedDoc {
            id: Uuid::new_v4().to_string(),
            target_id: target_id.to_string(),
            sections,
            table_of_contents: toc,
            generated_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            version: "1.0".to_string(),
        };

        let mut docs = self.generated_docs.write().unwrap();
        docs.insert(generated.id.clone(), generated.clone());

        Ok(generated)
    }

    async fn generate_api_docs(&self, target: &DocTarget) -> Result<Vec<DocSection>, DocError> {
        // Simulated API documentation generation
        let mut sections = Vec::new();

        sections.push(DocSection {
            id: Uuid::new_v4().to_string(),
            title: "API Overview".to_string(),
            level: 1,
            content: "This API provides endpoints for interacting with the system.".to_string(),
            subsections: vec![],
            code_examples: vec![],
            diagrams: vec![],
        });

        sections.push(DocSection {
            id: Uuid::new_v4().to_string(),
            title: "Authentication".to_string(),
            level: 1,
            content: "All API requests require authentication using Bearer tokens.".to_string(),
            subsections: vec![],
            code_examples: vec![
                CodeExample {
                    id: Uuid::new_v4().to_string(),
                    language: "bash".to_string(),
                    code: "curl -H 'Authorization: Bearer <token>' https://api.example.com".to_string(),
                    description: "Example API call with authentication".to_string(),
                    annotations: vec![],
                }
            ],
            diagrams: vec![],
        });

        Ok(sections)
    }

    async fn generate_code_docs(&self, target: &DocTarget) -> Result<Vec<DocSection>, DocError> {
        let mut sections = Vec::new();

        sections.push(DocSection {
            id: Uuid::new_v4().to_string(),
            title: "Code Structure".to_string(),
            level: 1,
            content: format!("Documentation for code in {}", target.source_path),
            subsections: vec![],
            code_examples: vec![],
            diagrams: vec![],
        });

        Ok(sections)
    }

    async fn generate_arch_docs(&self, target: &DocTarget) -> Result<Vec<DocSection>, DocError> {
        let mut sections = Vec::new();

        sections.push(DocSection {
            id: Uuid::new_v4().to_string(),
            title: "Architecture Overview".to_string(),
            level: 1,
            content: "System architecture documentation.".to_string(),
            subsections: vec![],
            code_examples: vec![],
            diagrams: vec![
                Diagram::ArchitectureDiagram {
                    components: vec![
                        Component {
                            id: "api".to_string(),
                            name: "API Gateway".to_string(),
                            component_type: ComponentType::Gateway,
                            description: "Entry point for all requests".to_string(),
                            connections: vec!["service1".to_string(), "service2".to_string()],
                        }
                    ],
                }
            ],
        });

        Ok(sections)
    }

    async fn generate_user_guide(&self, target: &DocTarget) -> Result<Vec<DocSection>, DocError> {
        let mut sections = Vec::new();

        sections.push(DocSection {
            id: Uuid::new_v4().to_string(),
            title: "Getting Started".to_string(),
            level: 1,
            content: "Welcome to the user guide!".to_string(),
            subsections: vec![],
            code_examples: vec![],
            diagrams: vec![],
        });

        sections.push(DocSection {
            id: Uuid::new_v4().to_string(),
            title: "Configuration".to_string(),
            level: 1,
            content: "How to configure the system.".to_string(),
            subsections: vec![],
            code_examples: vec![],
            diagrams: vec![],
        });

        Ok(sections)
    }

    async fn generate_generic_docs(&self, target: &DocTarget) -> Result<Vec<DocSection>, DocError> {
        Ok(vec![DocSection {
            id: Uuid::new_v4().to_string(),
            title: target.name.clone(),
            level: 1,
            content: "Documentation content".to_string(),
            subsections: vec![],
            code_examples: vec![],
            diagrams: vec![],
        }])
    }

    fn build_toc(&self, sections: &[DocSection]) -> Vec<TocEntry> {
        sections.iter().map(|s| TocEntry {
            title: s.title.clone(),
            level: s.level,
            anchor: s.title.to_lowercase().replace(' ', "-"),
            children: s.subsections.iter().map(|sub| TocEntry {
                title: sub.title.clone(),
                level: sub.level,
                anchor: sub.title.to_lowercase().replace(' ', "-"),
                children: vec![],
            }).collect(),
        }).collect()
    }

    /// Export documentation
    pub fn export(&self, doc_id: &str, format: DocFormat) -> Result<String, DocError> {
        let docs = self.generated_docs.read().unwrap();
        let doc = docs.get(doc_id)
            .ok_or(DocError::DocumentNotFound)?
            .clone();
        drop(docs);

        match format {
            DocFormat::Markdown => self.to_markdown(&doc),
            DocFormat::HTML => self.to_html(&doc),
            DocFormat::PDF => self.to_pdf(&doc),
            DocFormat::ReStructuredText => self.to_rst(&doc),
            DocFormat::AsciiDoc => self.to_asciidoc(&doc),
            DocFormat::Confluence => self.to_confluence(&doc),
        }
    }

    fn to_markdown(&self, doc: &GeneratedDoc) -> Result<String, DocError> {
        let mut output = String::new();

        output.push_str(&format!("# {}\n\n", doc.target_id));

        // Table of contents
        output.push_str("## Table of Contents\n\n");
        for entry in &doc.table_of_contents {
            output.push_str(&format!("- [{}](#{})\n", entry.title, entry.anchor));
        }
        output.push('\n');

        // Content
        for section in &doc.sections {
            output.push_str(&format!("\n## {}\n\n", section.title));
            output.push_str(&section.content);
            output.push('\n');
        }

        Ok(output)
    }

    fn to_html(&self, doc: &GeneratedDoc) -> Result<String, DocError> {
        Ok(format!("<html><body><h1>{}</h1></body></html>", doc.target_id))
    }

    fn to_pdf(&self, doc: &GeneratedDoc) -> Result<String, DocError> {
        Ok(format!("PDF output for {}", doc.target_id))
    }

    fn to_rst(&self, doc: &GeneratedDoc) -> Result<String, DocError> {
        Ok(format!("{}\n{}\n", doc.target_id, "=".repeat(doc.target_id.len())))
    }

    fn to_asciidoc(&self, doc: &GeneratedDoc) -> Result<String, DocError> {
        Ok(format!("= {}\n\n", doc.target_id))
    }

    fn to_confluence(&self, doc: &GeneratedDoc) -> Result<String, DocError> {
        Ok(format!("Confluence markup for {}", doc.target_id))
    }

    /// Add to knowledge base
    pub fn add_to_knowledge_base(&self, entry: KnowledgeEntry) {
        let mut kb = self.knowledge_base.write().unwrap();
        kb.entries.push(entry);
    }

    /// Search knowledge base
    pub fn search_knowledge_base(&self, query: &str) -> Vec<KnowledgeEntry> {
        let kb = self.knowledge_base.read().unwrap();
        let query_lower = query.to_lowercase();

        kb.entries.iter()
            .filter(|e| {
                e.title.to_lowercase().contains(&query_lower) ||
                e.content.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect()
    }

    /// Get generated document
    pub fn get_document(&self, doc_id: &str) -> Option<GeneratedDoc> {
        let docs = self.generated_docs.read().unwrap();
        docs.get(doc_id).cloned()
    }

    /// List targets
    pub fn list_targets(&self) -> Vec<DocTarget> {
        let targets = self.targets.read().unwrap();
        targets.values().cloned().collect()
    }

    fn init_default_templates(&self) {
        let mut templates = self.templates.write().unwrap();

        templates.insert("api_overview".to_string(), DocTemplate {
            id: "api_overview".to_string(),
            name: "API Overview Template".to_string(),
            template_type: DocType::API,
            template: "# {{title}}\n\n{{description}}".to_string(),
            variables: vec![
                TemplateVar { name: "title".to_string(), var_type: "string".to_string(), required: true },
                TemplateVar { name: "description".to_string(), var_type: "string".to_string(), required: false },
            ],
        });
    }
}

/// Documentation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocError {
    TargetNotFound,
    DocumentNotFound,
    GenerationFailed,
    ExportFailed,
}

impl std::fmt::Display for DocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocError::TargetNotFound => write!(f, "Documentation target not found"),
            DocError::DocumentNotFound => write!(f, "Document not found"),
            DocError::GenerationFailed => write!(f, "Documentation generation failed"),
            DocError::ExportFailed => write!(f, "Documentation export failed"),
        }
    }
}

impl std::error::Error for DocError {}

// Re-export types
pub use DocTarget;
pub use DocType;
pub use DocFormat;
pub use DocSection;
pub use CodeExample;
pub use Diagram;
pub use GeneratedDoc;
pub use ApiEndpointDoc;
pub use DocumentationService;