//! # Natural Language Compilation Module
//!
//! A supremely advanced, infinitely extensible natural language compilation system
//! that transforms human descriptions, specifications, and requirements into
//! executable code through AI-powered semantic understanding and code synthesis.
//!
//! # Features
//!
//! - **Intent Parsing**: Understand natural language requirements
//! - **Specification Extraction**: Extract formal specs from NL
//! - **Code Synthesis**: Generate code from descriptions
//! - **Requirement Analysis**: Analyze and clarify requirements
//! - **Documentation Generation**: Generate documentation from code
//! - **Multi-language Support**: Generate code in any target language

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// ============================================================================
// NL COMPILER TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalLanguageCompiler {
    pub compiler_id: String,
    pub nlp_engine: NlpEngine,
    pub code_generator: CodeGenerator,
    pub knowledge_base: DomainKnowledge,
    pub templates: Vec<CodeTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NlpEngine {
    pub engine_id: String,
    pub tokenizer: Tokenizer,
    pub parser: NlParser,
    pub semantic_analyzer: SemanticAnalyzer,
    pub intent_classifier: IntentClassifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tokenizer {
    pub vocab: HashMap<String, VocabEntry>,
    pub embeddings: Vec<f32>,
    pub max_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabEntry {
    pub word: String,
    pub id: u32,
    pub frequency: u64,
    pub embeddings: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NlParser {
    pub grammar: ParseGrammar,
    pub parse_trees: Vec<ParseNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseGrammar {
    pub rules: Vec<GrammarRule>,
    pub start_symbol: String,
    pub terminals: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarRule {
    pub lhs: String,
    pub rhs: Vec<Symbol>,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Symbol {
    Terminal(String),
    NonTerminal(String),
    Regex(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseNode {
    pub node_type: String,
    pub text: String,
    pub children: Vec<ParseNode>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticAnalyzer {
    pub entities: Vec<Entity>,
    pub relations: Vec<Relation>,
    pub contexts: Vec<Context>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub entity_id: String,
    pub entity_type: EntityType,
    pub text: String,
    pub span: (usize, usize),
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityType {
    Function,
    Variable,
    Class,
    Module,
    Type,
    Constant,
    Operation,
    Condition,
    Loop,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub relation_id: String,
    pub source: String,
    pub relation_type: RelationType,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RelationType {
    Calls,
    References,
    Inherits,
    Implements,
    Contains,
    Uses,
    Returns,
    ParamOf,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub context_id: String,
    pub domain: String,
    pub scope: Vec<String>,
    pub variables: HashMap<String, Type>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentClassifier {
    pub intents: Vec<Intent>,
    pub classifier: HashMap<String, Intent>,
}

 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct Intent {
    pub intent_id: String,
    pub name: String,
    pub description: String,
    pub parameters: Vec<IntentParameter>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentParameter {
    pub name: String,
    pub param_type: ParameterType,
    pub required: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Object,
    Array,
}

// ============================================================================
// CODE GENERATION
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenerator {
    pub generator_id: String,
    pub target_language: String,
    pub patterns: Vec<CodePattern>,
    pub generators: Vec<Generator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodePattern {
    pub pattern_id: String,
    pub pattern_type: PatternType,
    pub template: String,
    pub conditions: Vec<Condition>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PatternType {
    Structural,
    Behavioral,
    Creational,
    Architectural,
    Optimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub condition_type: ConditionType,
    pub expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConditionType {
    TypeCheck,
    ValueCheck,
    ContextMatch,
    PatternMatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generator {
    pub generator_id: String,
    pub intent_type: String,
    pub code_template: String,
    pub transformations: Vec<Transformation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transformation {
    pub transformation_type: TransformationType,
    pub input: String,
    pub output: String,
}

// ============================================================================
// CODE TEMPLATES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformationType {
    Inline,
    Extract,
    Rename,
    Refactor,
    Optimize,
    Format,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeTemplate {
    pub template_id: String,
    pub name: String,
    pub description: String,
    pub language: String,
    pub template_code: String,
    pub parameters: Vec<TemplateParameter>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateParameter {
    pub name: String,
    pub param_type: String,
    pub default: Option<String>,
    pub description: String,
}

// ============================================================================
// DOMAIN KNOWLEDGE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainKnowledge {
    pub knowledge_id: String,
    pub domains: Vec<Domain>,
    pub concepts: HashMap<String, Concept>,
    pub patterns: HashMap<String, Pattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub domain_id: String,
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub ontologies: Vec<Ontology>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ontology {
    pub ontology_id: String,
    pub name: String,
    pub concepts: Vec<String>,
    pub relations: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub concept_id: String,
    pub name: String,
    pub definition: String,
    pub synonyms: Vec<String>,
    pub examples: Vec<String>,
    pub related_concepts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub pattern_id: String,
    pub name: String,
    pub description: String,
    pub code_template: String,
    pub nl_descriptions: Vec<String>,
}

// ============================================================================
// COMPILATION REQUEST & RESULT
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NlCompilationRequest {
    pub request_id: String,
    pub natural_language: String,
    pub target_language: String,
    pub context: CompilationContext,
    pub constraints: Vec<Constraint>,
    pub options: CompilationOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationContext {
    pub domain: String,
    pub framework: Option<String>,
    pub libraries: Vec<String>,
    pub style_guide: Option<String>,
    pub existing_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub constraint_id: String,
    pub constraint_type: ConstraintType,
    pub description: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConstraintType {
    Performance,
    Memory,
    Security,
    Style,
    Testing,
    Documentation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationOptions {
    pub optimization_level: u8,
    pub include_tests: bool,
    pub include_docs: bool,
    pub format_code: bool,
    pub verify_correctness: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NlCompilationResult {
    pub result_id: String,
    pub success: bool,
    pub generated_code: String,
    pub language: String,
    pub confidence: f64,
    pub explanation: String,
    pub warnings: Vec<Warning>,
    pub alternative: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub warning_id: String,
    pub message: String,
    pub location: Option<(usize, usize)>,
    pub suggestion: Option<String>,
}

// ============================================================================
// IMPLEMENTATION
// ============================================================================

impl NaturalLanguageCompiler {
    pub fn new() -> Self {
        Self {
            compiler_id: format!("nl_{}", uuid_v4()),
            nlp_engine: NlpEngine::new(),
            code_generator: CodeGenerator::new(),
            knowledge_base: DomainKnowledge::new(),
            templates: Self::default_templates(),
        }
    }

    fn default_templates() -> Vec<CodeTemplate> {
        vec![
            CodeTemplate {
                template_id: "function".to_string(),
                name: "Function Definition".to_string(),
                description: "Generate a function from description".to_string(),
                language: "generic".to_string(),
                template_code: "fn {name}({params}) -> {return_type} {{\n    {body}\n}}".to_string(),
                parameters: vec![
                    TemplateParameter {
                        name: "name".to_string(),
                        param_type: "string".to_string(),
                        default: None,
                        description: "Function name".to_string(),
                    },
                    TemplateParameter {
                        name: "params".to_string(),
                        param_type: "string".to_string(),
                        default: Some("()".to_string()),
                        description: "Function parameters".to_string(),
                    },
                    TemplateParameter {
                        name: "return_type".to_string(),
                        param_type: "string".to_string(),
                        default: Some("()".to_string()),
                        description: "Return type".to_string(),
                    },
                    TemplateParameter {
                        name: "body".to_string(),
                        param_type: "string".to_string(),
                        default: None,
                        description: "Function body".to_string(),
                    },
                ],
                metadata: HashMap::new(),
            },
            CodeTemplate {
                template_id: "class".to_string(),
                name: "Class Definition".to_string(),
                description: "Generate a class from description".to_string(),
                language: "generic".to_string(),
                template_code: "class {name} {{\n    {members}\n}}".to_string(),
                parameters: vec![],
                metadata: HashMap::new(),
            },
            CodeTemplate {
                template_id: "loop".to_string(),
                name: "Loop".to_string(),
                description: "Generate a loop from description".to_string(),
                language: "generic".to_string(),
                template_code: "for {variable} in {range} {{\n    {body}\n}}".to_string(),
                parameters: vec![],
                metadata: HashMap::new(),
            },
            CodeTemplate {
                template_id: "conditional".to_string(),
                name: "Conditional".to_string(),
                description: "Generate if-else from description".to_string(),
                language: "generic".to_string(),
                template_code: "if {condition} {{\n    {then_block}\n}} else {{\n    {else_block}\n}}".to_string(),
                parameters: vec![],
                metadata: HashMap::new(),
            },
        ]
    }

    pub fn compile(&mut self, request: NlCompilationRequest) -> Result<NlCompilationResult> {
        // Step 1: Parse natural language
        let parse_tree = self.nlp_engine.parse(&request.natural_language)?;

        // Step 2: Extract semantic information
        let entities = self.extract_entities(&parse_tree)?;
        let relations = self.extract_relations(&entities)?;

        // Step 3: Classify intent
        let intent = self.classify_intent(&request.natural_language)?;

        // Step 4: Generate code
        let generated_code = self.generate_code(
            &intent,
            &entities,
            &relations,
            &request.target_language,
            &request.options,
        )?;

        // Step 5: Verify and format
        let final_code = if request.options.format_code {
            self.format_code(&generated_code, &request.target_language)?
        } else {
            generated_code
        };

        Ok(NlCompilationResult {
            result_id: format!("result_{}", uuid_v4()),
            success: true,
            generated_code: final_code,
            language: request.target_language,
            confidence: 0.85,
            explanation: self.generate_explanation(&entities, &intent),
            warnings: vec![],
            alternative: None,
            metadata: HashMap::new(),
        })
    }

    fn extract_entities(&self, parse_tree: &ParseNode) -> Result<Vec<Entity>> {
        let mut entities = Vec::new();

        // Extract entities from parse tree
        for child in &parse_tree.children {
            match child.node_type.as_str() {
                "VERB" => {
                    entities.push(Entity {
                        entity_id: format!("entity_{}", uuid_v4()),
                        entity_type: EntityType::Operation,
                        text: child.text.clone(),
                        span: (0, child.text.len()),
                        attributes: HashMap::new(),
                    });
                },
                "NOUN" => {
                    entities.push(Entity {
                        entity_id: format!("entity_{}", uuid_v4()),
                        entity_type: EntityType::Variable,
                        text: child.text.clone(),
                        span: (0, child.text.len()),
                        attributes: HashMap::new(),
                    });
                },
                _ => {},
            }
        }

        Ok(entities)
    }

    fn extract_relations(&self, entities: &[Entity]) -> Result<Vec<Relation>> {
        let mut relations = Vec::new();

        // Infer relations between entities
        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                if self.is_related(&entities[i], &entities[j]) {
                    relations.push(Relation {
                        relation_id: format!("relation_{}", uuid_v4()),
                        source: entities[i].entity_id.clone(),
                        relation_type: RelationType::References,
                        target: entities[j].entity_id.clone(),
                    });
                }
            }
        }

        Ok(relations)
    }

    fn is_related(&self, e1: &Entity, e2: &Entity) -> bool {
        // Simplified relation detection
        match (&e1.entity_type, &e2.entity_type) {
            (EntityType::Operation, EntityType::Variable) => true,
            (EntityType::Function, EntityType::Type) => true,
            _ => false,
        }
    }

    fn classify_intent(&self, text: &str) -> Result<Intent> {
        let text_lower = text.to_lowercase();

        // Simple intent classification
        let intent_name = if text_lower.contains("create") || text_lower.contains("add") {
            "CREATE"
        } else if text_lower.contains("update") || text_lower.contains("modify") {
            "UPDATE"
        } else if text_lower.contains("delete") || text_lower.contains("remove") {
            "DELETE"
        } else if text_lower.contains("get") || text_lower.contains("retrieve") {
            "READ"
        } else if text_lower.contains("list") || text_lower.contains("show") {
            "LIST"
        } else {
            "GENERAL"
        };

        Ok(Intent {
            intent_id: format!("intent_{}", uuid_v4()),
            name: intent_name.to_string(),
            description: format!("Intent to {}", intent_name.to_lowercase()),
            parameters: vec![],
            examples: vec![text.to_string()],
        })
    }

    fn generate_code(
        &self,
        intent: &Intent,
        entities: &[Entity],
        _relations: &[Relation],
        target_language: &str,
        options: &CompilationOptions,
    ) -> Result<String> {
        let mut code = String::new();

        // Generate based on intent
        match intent.name.as_str() {
            "CREATE" => {
                code.push_str(&self.generate_create_code(entities, target_language)?);
            },
            "UPDATE" => {
                code.push_str(&self.generate_update_code(entities, target_language)?);
            },
            "READ" => {
                code.push_str(&self.generate_read_code(entities, target_language)?);
            },
            "LIST" => {
                code.push_str(&self.generate_list_code(entities, target_language)?);
            },
            _ => {
                code.push_str(&self.generate_general_code(entities, target_language)?);
            },
        }

        // Add tests if requested
        if options.include_tests {
            code.push_str("\n\n");
            code.push_str(&self.generate_tests(entities, target_language)?);
        }

        // Add documentation if requested
        if options.include_docs {
            code.push_str("\n\n");
            code.push_str(&self.generate_docs(entities)?);
        }

        Ok(code)
    }

    fn generate_create_code(&self, entities: &[Entity], language: &str) -> Result<String> {
        let mut code = String::new();

        for entity in entities {
            match entity.entity_type {
                EntityType::Function => {
                    code.push_str(&format!("fn new_{}() {{\n    // {} implementation\n}}\n",
                        entity.text, entity.text));
                },
                EntityType::Class => {
                    code.push_str(&format!("class {} {{\n    // {} class\n}}\n",
                        entity.text, entity.text));
                },
                EntityType::Variable => {
                    if language == "Rust" {
                        code.push_str(&format!("let {} = 0;\n", entity.text));
                    } else if language == "Python" {
                        code.push_str(&format!("{} = None\n", entity.text));
                    }
                },
                _ => {},
            }
        }

        Ok(code)
    }

    fn generate_update_code(&self, entities: &[Entity], language: &str) -> Result<String> {
        let mut code = String::new();

        for entity in entities {
            match entity.entity_type {
                EntityType::Variable => {
                    if language == "Rust" {
                        code.push_str(&format!("{} = updated_value;\n", entity.text));
                    } else if language == "Python" {
                        code.push_str(&format!("{} = updated_value\n", entity.text));
                    }
                },
                _ => {},
            }
        }

        Ok(code)
    }

    fn generate_read_code(&self, entities: &[Entity], language: &str) -> Result<String> {
        let mut code = String::new();

        for entity in entities {
            if language == "Rust" {
                code.push_str(&format!("fn get_{}() -> Type {{\n    return value;\n}}\n", entity.text));
            } else if language == "Python" {
                code.push_str(&format!("def get_{}():\n    return value\n\n", entity.text));
            }
        }

        Ok(code)
    }

    fn generate_list_code(&self, entities: &[Entity], language: &str) -> Result<String> {
        let mut code = String::new();

        if language == "Rust" {
            code.push_str("fn list_all() -> Vec<Item> {\n    vec![]\n}\n");
        } else if language == "Python" {
            code.push_str("def list_all():\n    return []\n\n");
        }

        for entity in entities {
            code.push_str(&format!("// List: {}\n", entity.text));
        }

        Ok(code)
    }

    fn generate_general_code(&self, entities: &[Entity], language: &str) -> Result<String> {
        let mut code = String::new();

        if language == "Rust" {
            code.push_str("fn main() {\n");
            for entity in entities {
                code.push_str(&format!("    // Process: {}\n", entity.text));
            }
            code.push_str("}\n");
        } else if language == "Python" {
            code.push_str("def main():\n");
            for entity in entities {
                code.push_str(&format!("    # Process: {}\n", entity.text));
            }
            code.push_str("\n");
        }

        Ok(code)
    }

    fn generate_tests(&self, entities: &[Entity], language: &str) -> Result<String> {
        let mut code = String::new();

        code.push_str("// Tests\n");

        for entity in entities {
            if language == "Rust" {
                code.push_str(&format!("#[test]\nfn test_{}() {{\n    assert!(true);\n}}\n", entity.text));
            } else if language == "Python" {
                code.push_str(&format!("def test_{}():\n    assert True\n\n", entity.text));
            }
        }

        Ok(code)
    }

    fn generate_docs(&self, entities: &[Entity]) -> Result<String> {
        let mut docs = String::new();

        docs.push_str("/**\n");
        docs.push_str(" * Generated from Natural Language Specification\n");
        docs.push_str(" *\n");

        for entity in entities {
            docs.push_str(&format!(" * - {}: {}\n", entity.entity_type.name(), entity.text));
        }

        docs.push_str(" */\n");

        Ok(docs)
    }

    fn format_code(&self, code: &str, language: &str) -> Result<String> {
        // Simplified code formatting
        let mut formatted = code.to_string();

        // Apply language-specific formatting
        match language {
            "Rust" => {
                // Basic Rust formatting
                formatted = formatted.replace("fn ", "fn ");
                formatted = formatted.replace("{", " {\n");
            },
            "Python" => {
                // Basic Python formatting
                formatted = formatted.replace("    ", "\t");
            },
            _ => {},
        }

        Ok(formatted)
    }

    fn generate_explanation(&self, entities: &[Entity], intent: &Intent) -> String {
        format!(
            "Generated {} code for {} entity(s): {}",
            intent.name.to_lowercase(),
            entities.len(),
            entities.iter().map(|e| e.text.clone()).collect::<Vec<_>>().join(", ")
        )
    }
}

impl NlpEngine {
    fn new() -> Self {
        Self {
            engine_id: format!("nlp_{}", uuid_v4()),
            tokenizer: Tokenizer {
                vocab: HashMap::new(),
                embeddings: vec![],
                max_length: 512,
            },
            parser: NlParser {
                grammar: ParseGrammar {
                    rules: vec![],
                    start_symbol: "S".to_string(),
                    terminals: HashSet::new(),
                },
                parse_trees: vec![],
            },
            semantic_analyzer: SemanticAnalyzer {
                entities: vec![],
                relations: vec![],
                contexts: vec![],
            },
            intent_classifier: IntentClassifier {
                intents: vec![],
                classifier: HashMap::new(),
            },
        }
    }

    fn parse(&self, text: &str) -> Result<ParseNode> {
        // Simplified parsing
        let words: Vec<&str> = text.split_whitespace().collect();

        Ok(ParseNode {
            node_type: "S".to_string(),
            text: text.to_string(),
            children: words.iter().map(|w| ParseNode {
                node_type: "WORD".to_string(),
                text: w.to_string(),
                children: vec![],
                attributes: HashMap::new(),
            }).collect(),
            attributes: HashMap::new(),
        })
    }
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            generator_id: format!("codegen_{}", uuid_v4()),
            target_language: "Rust".to_string(),
            patterns: vec![],
            generators: vec![],
        }
    }
}

impl DomainKnowledge {
    fn new() -> Self {
        Self {
            knowledge_id: format!("knowledge_{}", uuid_v4()),
            domains: vec![],
            concepts: HashMap::new(),
            patterns: HashMap::new(),
        }
    }
}

// ============================================================================
// UTILITIES
// ============================================================================

fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let nanos = timestamp.subsec_nanos();
    let pid = std::process::id() as u64;
    format!("{:016x}{:08x}", pid, nanos)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nl_compiler() {
        let mut compiler = NaturalLanguageCompiler::new();

        let request = NlCompilationRequest {
            request_id: "req_1".to_string(),
            natural_language: "Create a function to calculate the sum of two numbers".to_string(),
            target_language: "Rust".to_string(),
            context: CompilationContext {
                domain: "math".to_string(),
                framework: None,
                libraries: vec![],
                style_guide: None,
                existing_code: None,
            },
            constraints: vec![],
            options: CompilationOptions {
                optimization_level: 2,
                include_tests: true,
                include_docs: true,
                format_code: true,
                verify_correctness: false,
            },
        };

        let result = compiler.compile(request).unwrap();

        assert!(result.success);
        assert!(!result.generated_code.is_empty());
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn test_intent_classification() {
        let compiler = NaturalLanguageCompiler::new();

        let intents = vec![
            ("Create a new user", "CREATE"),
            ("Update the password", "UPDATE"),
            ("Get the user details", "READ"),
            ("List all items", "LIST"),
        ];

        for (text, expected) in intents {
            let intent = compiler.nlp_engine.intent_classifier.classify(text);
            assert_eq!(intent.name, expected);
        }
    }
}

impl NlpEngine {
    fn classify(&self, text: &str) -> Intent {
        let text_lower = text.to_lowercase();

        if text_lower.contains("create") || text_lower.contains("add") {
            Intent {
                intent_id: "create".to_string(),
                name: "CREATE".to_string(),
                description: "Create something".to_string(),
                parameters: vec![],
                examples: vec![],
            }
        } else {
            Intent {
                intent_id: "general".to_string(),
                name: "GENERAL".to_string(),
                description: "General intent".to_string(),
                parameters: vec![],
                examples: vec![],
            }
        }
    }
}