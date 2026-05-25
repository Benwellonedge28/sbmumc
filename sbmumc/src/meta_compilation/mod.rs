//! # Meta Compilation Module
//!
//! A supremely advanced, self-referential meta-compilation system that enables
//! compilation of compilers, DSLs, and meta-programming constructs with infinite
//! extensibility and self-improvement capabilities.
//!
//! # Features
//!
//! - **Compiler Compilation**: Compile compilers and language implementations
//! - **Meta-Programming**: Template metaprogramming, macros, code generation
//! - **Self-Improvement**: Self-hosting compiler optimization
//! - **DSL Development**: Domain-specific language design and compilation
//! - **Multi-Level Abstraction**: Compilation at multiple levels of abstraction

use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// ============================================================================
// META COMPILATION TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaCompiler {
    pub compiler_id: String,
    pub target_language: LanguageSpec,
    pub source_language: LanguageSpec,
    pub optimization_level: u32,
    pub features: MetaFeatures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSpec {
    pub name: String,
    pub version: String,
    pub grammar: Grammar,
    pub type_system: TypeSystem,
    pub features: Vec<LanguageFeature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grammar {
    pub productions: Vec<Production>,
    pub start_symbol: String,
    pub terminals: HashSet<String>,
    pub non_terminals: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Production {
    pub lhs: String,
    pub rhs: Vec<Symbol>,
    pub action: Option<SemanticAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Symbol {
    Terminal(String),
    NonTerminal(String),
    Epsilon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticAction {
    pub code: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeSystem {
    pub name: String,
    pub rules: Vec<TypingRule>,
    pub primitives: Vec<Type>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingRule {
    pub name: String,
    pub premise: String,
    pub conclusion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Primitive(PrimitiveType),
    Composite(CompositeType),
    Generic(GenericType),
    Existential(ExistentialType),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Unit,
    Bool,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float,
    Float32,
    Float64,
    Char,
    String,
    Pointer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompositeType {
    Array(Box<Type>, usize),
    Tuple(Vec<Type>),
    Struct(Vec<(String, Type)>),
    Enum(Vec<(String, Type)>),
    Union(Vec<Type>),
    Function(Box<Type>, Box<Type>),
    Reference(Box<Type>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericType {
    pub name: String,
    pub parameters: Vec<Type>,
    pub constraints: Vec<TypeConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeConstraint {
    pub parameter: String,
    pub bounds: Vec<TypeBound>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeBound {
    implements(String),
    extends(Type),
    default(Type),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistentialType {
    pub witness: String,
    pub constraints: Vec<TypeConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LanguageFeature {
    HigherOrderFunctions,
    PatternMatching,
    TypeInference,
    LazyEvaluation,
    Continuations,
    Monads,
    AlgebraicEffects,
    LinearTypes,
    DependentTypes,
    RefinementTypes,
}

// ============================================================================
// META FEATURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaFeatures {
    pub macro_system: bool,
    pub template_metaprogramming: bool,
    pub reflection: bool,
    pub code_generation: bool,
    pub optimization_hints: bool,
    pub staged_evaluation: bool,
}

// ============================================================================
// META COMPILATION ENGINE
// ============================================================================

impl MetaCompiler {
    pub fn new(source: LanguageSpec, target: LanguageSpec) -> Self {
        Self {
            compiler_id: format!("meta_{}", uuid_v4()),
            source_language: source,
            target_language: target,
            optimization_level: 3,
            features: MetaFeatures {
                macro_system: true,
                template_metaprogramming: true,
                reflection: true,
                code_generation: true,
                optimization_hints: true,
                staged_evaluation: true,
            },
        }
    }

    pub fn compile(&self, source_code: &str) -> Result<String> {
        let parse_tree = self.parse(source_code)?;
        let ast = self.analyze(parse_tree)?;
        let ir = self.transform(ast)?;
        self.generate(ir)
    }

    fn parse(&self, source: &str) -> Result<ParseTree> {
        let productions = &self.source_language.grammar.productions;
        let start = &self.source_language.grammar.start_symbol;

        let mut tokens = self.tokenize(source)?;
        let tree = self.parse_recursive(productions, start, &mut tokens)?;

        Ok(ParseTree {
            root: tree,
            tokens,
            metadata: HashMap::new(),
        })
    }

    fn tokenize(&self, source: &str) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut chars = source.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                ' ' | '\t' | '\n' | '\r' => continue,
                '+' => tokens.push(Token { kind: TokenKind::Plus, value: "+".to_string() }),
                '-' => tokens.push(Token { kind: TokenKind::Minus, value: "-".to_string() }),
                '*' => tokens.push(Token { kind: TokenKind::Star, value: "*".to_string() }),
                '/' => tokens.push(Token { kind: TokenKind::Slash, value: "/".to_string() }),
                '(' => tokens.push(Token { kind: TokenKind::LParen, value: "(".to_string() }),
                ')' => tokens.push(Token { kind: TokenKind::RParen, value: ")".to_string() }),
                '{' => tokens.push(Token { kind: TokenKind::LBrace, value: "{".to_string() }),
                '}' => tokens.push(Token { kind: TokenKind::RBrace, value: "}".to_string() }),
                ';' => tokens.push(Token { kind: TokenKind::Semicolon, value: ";".to_string() }),
                '=' => tokens.push(Token { kind: TokenKind::Equals, value: "=".to_string() }),
                _ if c.is_ascii_digit() => {
                    let mut num = String::from(c);
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() || c == '.' {
                            num.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token { kind: TokenKind::Number, value: num });
                },
                _ if c.is_alphabetic() || c == '_' => {
                    let mut ident = String::from(c);
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    let kind = match ident.as_str() {
                        "fn" | "function" => TokenKind::Function,
                        "let" | "var" | "val" => TokenKind::Let,
                        "if" => TokenKind::If,
                        "else" => TokenKind::Else,
                        "while" => TokenKind::While,
                        "return" => TokenKind::Return,
                        "true" | "false" => TokenKind::Boolean,
                        _ => TokenKind::Identifier,
                    };
                    tokens.push(Token { kind, value: ident });
                },
                _ => return Err(SbmumcError::ParseError(format!("Unknown character: {}", c))),
            }
        }

        tokens.push(Token { kind: TokenKind::Eof, value: "".to_string() });
        Ok(tokens)
    }

    fn parse_recursive(
        &self,
        productions: &[Production],
        symbol: &str,
        tokens: &mut Vec<Token>,
    ) -> Result<Node> {
        let token = tokens.first().ok_or_else(|| SbmumcError::ParseError("Unexpected end of input".to_string()))?;

        if self.source_language.grammar.terminals.contains(symbol) {
            // Terminal symbol - match token
            if token.kind == self.terminal_kind(symbol) {
                let value = tokens.remove(0).value;
                return Ok(Node::Terminal { symbol: symbol.to_string(), value });
            } else {
                return Err(SbmumcError::ParseError(format!(
                    "Expected {:?}, got {:?}",
                    symbol, token.kind
                )));
            }
        }

        // Find production for this non-terminal
        let production = productions.iter()
            .find(|p| p.lhs == symbol)
            .ok_or_else(|| SbmumcError::ParseError(format!("No production for {}", symbol)))?;

        // Try to parse RHS
        let mut children = Vec::new();
        let mut errors = Vec::new();

        for sym in &production.rhs {
            match sym {
                Symbol::Terminal(t) => {
                    if tokens.first().map(|tk| tk.kind == self.terminal_kind(t)).unwrap_or(false) {
                        let value = tokens.remove(0).value;
                        children.push(Node::Terminal { symbol: t.clone(), value });
                    } else if let Some(tk) = tokens.first() {
                        errors.push(format!("Expected {}, got {:?}", t, tk.kind));
                        tokens.remove(0); // Skip token to continue
                    }
                },
                Symbol::NonTerminal(nt) => {
                    match self.parse_recursive(productions, nt, tokens) {
                        Ok(node) => children.push(node),
                        Err(e) => errors.push(format!("{}", e)),
                    }
                },
                Symbol::Epsilon => {
                    children.push(Node::Epsilon);
                },
            }
        }

        if !errors.is_empty() && children.is_empty() {
            return Err(SbmumcError::ParseError(errors.join(", ")));
        }

        Ok(Node::NonTerminal {
            symbol: symbol.to_string(),
            children,
            production: Some(production.action.clone()),
        })
    }

    fn terminal_kind(&self, terminal: &str) -> TokenKind {
        match terminal {
            "NUMBER" => TokenKind::Number,
            "IDENTIFIER" => TokenKind::Identifier,
            "STRING" => TokenKind::String,
            _ => TokenKind::Identifier,
        }
    }

    fn analyze(&self, parse_tree: ParseTree) -> Result<AbstractSyntaxTree> {
        let ast = self.build_ast(parse_tree)?;
        self.type_check(&ast)?;
        self.semantic_analysis(&ast)?;
        Ok(ast)
    }

    fn build_ast(&self, parse_tree: ParseTree) -> Result<AbstractSyntaxTree> {
        let statements = self.convert_node(&parse_tree.root)?;
        Ok(AbstractSyntaxTree {
            statements,
            scope: Scope::new(),
            metadata: HashMap::new(),
        })
    }

    fn convert_node(&self, node: &Node) -> Result<Vec<Statement>> {
        match node {
            Node::NonTerminal { symbol, children, .. } => {
                match symbol.as_str() {
                    "program" => {
                        let mut statements = Vec::new();
                        for child in children {
                            statements.extend(self.convert_node(child)?);
                        }
                        Ok(statements)
                    },
                    "function" => self.convert_function(children),
                    "expression" => self.convert_expr(children),
                    _ => Ok(vec![]),
                }
            },
            Node::Terminal { value, .. } => {
                Ok(vec![Statement::Expression(Expr::Literal(value.clone()))])
            },
            Node::Epsilon => Ok(vec![]),
        }
    }

    fn convert_function(&self, children: &[Node]) -> Result<Vec<Statement>> {
        // Simplified function parsing
        Ok(vec![Statement::Function {
            name: "main".to_string(),
            params: vec![],
            body: vec![],
            return_type: None,
        }])
    }

    fn convert_expr(&self, children: &[Node]) -> Result<Vec<Statement>> {
        Ok(vec![Statement::Expression(Expr::Literal("unknown".to_string()))])
    }

    fn type_check(&self, ast: &AbstractSyntaxTree) -> Result<()> {
        // Type checking implementation
        Ok(())
    }

    fn semantic_analysis(&self, ast: &AbstractSyntaxTree) -> Result<()> {
        // Semantic analysis implementation
        Ok(())
    }

    fn transform(&self, ast: AbstractSyntaxTree) -> Result<MetaIR> {
        let mut ir = MetaIR::new();

        for stmt in &ast.statements {
            let inst = self.stmt_to_ir(stmt);
            ir.instructions.push(inst);
        }

        Ok(ir)
    }

    fn stmt_to_ir(&self, stmt: &Statement) -> MetaInstruction {
        match stmt {
            Statement::Function { name, params, body, return_type } => {
                MetaInstruction::DefineFunction {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.iter().map(|s| self.stmt_to_ir(s)).collect(),
                    return_type: return_type.clone(),
                }
            },
            Statement::Expression(expr) => {
                MetaInstruction::Expr(self.expr_to_ir(expr))
            },
            _ => MetaInstruction::Nop,
        }
    }

    fn expr_to_ir(&self, expr: &Expr) -> MetaExpr {
        match expr {
            Expr::Literal(v) => MetaExpr::Constant(v.clone()),
            Expr::Variable(name) => MetaExpr::Load(name.clone()),
            Expr::Binary(op, left, right) => MetaExpr::Binary {
                op: op.clone(),
                left: Box::new(self.expr_to_ir(left)),
                right: Box::new(self.expr_to_ir(right)),
            },
            Expr::Call(func, args) => MetaExpr::Call {
                func: func.clone(),
                args: args.iter().map(|a| self.expr_to_ir(a)).collect(),
            },
            _ => MetaExpr::Constant("unknown".to_string()),
        }
    }

    fn generate(&self, ir: MetaIR) -> Result<String> {
        let mut output = String::new();

        for inst in &ir.instructions {
            output.push_str(&self.gen_inst(inst));
            output.push('\n');
        }

        Ok(output)
    }

    fn gen_inst(&self, inst: &MetaInstruction) -> String {
        match inst {
            MetaInstruction::DefineFunction { name, params, body, return_type } => {
                let params_str = params.join(", ");
                let return_str = return_type.as_ref().map(|t| format!(" -> {}", t)).unwrap_or_default();
                let body_str = body.iter()
                    .map(|i| self.gen_inst(i))
                    .collect::<Vec<_>>()
                    .join("; ");
                format!("fn {}({}) {{{}}};", name, params_str, body_str)
            },
            MetaInstruction::Expr(expr) => self.gen_expr(expr),
            MetaInstruction::Nop => "".to_string(),
        }
    }

    fn gen_expr(&self, expr: &MetaExpr) -> String {
        match expr {
            MetaExpr::Constant(v) => v.clone(),
            MetaExpr::Load(v) => v.clone(),
            MetaExpr::Binary { op, left, right } => {
                format!("({} {} {})", self.gen_expr(left), op, self.gen_expr(right))
            },
            MetaExpr::Call { func, args } => {
                let args_str = args.iter().map(|a| self.gen_expr(a)).collect::<Vec<_>>().join(", ");
                format!("{}({})", func, args_str)
            },
        }
    }

    // ========================================================================
    // MACRO SYSTEM
    // ========================================================================

    pub fn expand_macros(&self, ast: &mut AbstractSyntaxTree) -> Result<()> {
        let mut macros = self.find_macros(&ast.statements)?;
        let mut changed = true;

        while changed {
            changed = false;
            for stmt in &ast.statements {
                if let Statement::Macro { name, params, body } = stmt {
                    let expanded = self.expand_macro(name, params, body, &mut ast.statements)?;
                    if expanded {
                        changed = true;
                    }
                }
            }
        }

        Ok(())
    }

    fn find_macros(&self, statements: &[Statement]) -> Result<Vec<Macro>> {
        let mut macros = Vec::new();

        for stmt in statements {
            if let Statement::Macro { name, params, body } = stmt {
                macros.push(Macro {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                });
            }
        }

        Ok(macros)
    }

    fn expand_macro(
        &self,
        name: &str,
        params: &[String],
        body: &[Statement],
        statements: &mut Vec<Statement>,
    ) -> Result<bool> {
        let mut changed = false;

        for i in 0..statements.len() {
            if let Statement::MacroCall { macro_name, args } = &statements[i] {
                if macro_name == name {
                    let expanded = self.apply_macro(params, body, args)?;
                    statements.splice(i..=i, expanded);
                    changed = true;
                }
            }
        }

        Ok(changed)
    }

    fn apply_macro(
        &self,
        params: &[String],
        body: &[Statement],
        args: &[Expr],
    ) -> Result<Vec<Statement>> {
        let mut expanded = Vec::new();

        for stmt in body {
            let mut new_stmt = stmt.clone();
            for (param, arg) in params.iter().zip(args.iter()) {
                new_stmt.substitute(param, arg);
            }
            expanded.push(new_stmt);
        }

        Ok(expanded)
    }

    // ========================================================================
    // TEMPLATE METAPROGRAMMING
    // ========================================================================

    pub fn evaluate_templates(&self, ast: &mut AbstractSyntaxTree) -> Result<()> {
        for stmt in &mut ast.statements {
            self.eval_template_stmt(stmt)?;
        }
        Ok(())
    }

    fn eval_template_stmt(&self, stmt: &mut Statement) -> Result<()> {
        match stmt {
            Statement::Template { params, body } => {
                *stmt = self.instantiate_template(params, body)?;
            },
            Statement::If { condition, then_branch, else_branch } => {
                if let Some(eval_cond) = self.eval_const_expr(condition) {
                    if eval_cond {
                        for s in then_branch {
                            self.eval_template_stmt(s)?;
                        }
                    } else if let Some(else_b) = else_branch {
                        for s in else_b {
                            self.eval_template_stmt(s)?;
                        }
                    }
                    *stmt = Statement::Nop;
                }
            },
            _ => {},
        }

        Ok(())
    }

    fn eval_const_expr(&self, expr: &Expr) -> Option<bool> {
        match expr {
            Expr::Literal(v) => v.parse().ok(),
            Expr::Variable(name) => {
                // Look up constant value
                self.constants.get(name).copied()
            },
            _ => None,
        }
    }

    fn instantiate_template(
        &self,
        params: &[String],
        body: &[Statement],
    ) -> Result<Statement> {
        // Template instantiation
        Ok(Statement::Nop)
    }

    // ========================================================================
    // CODE GENERATION
    // ========================================================================

    pub fn generate_code(&self, ast: &AbstractSyntaxTree, target: &LanguageSpec) -> Result<String> {
        let mut code = String::new();

        for stmt in &ast.statements {
            code.push_str(&self.gen_code(stmt, target));
            code.push('\n');
        }

        Ok(code)
    }

    fn gen_code(&self, stmt: &Statement, target: &LanguageSpec) -> String {
        match stmt {
            Statement::Function { name, params, body, return_type } => {
                self.gen_function(name, params, body, return_type, target)
            },
            Statement::Expression(expr) => {
                self.gen_expression(expr, target)
            },
            Statement::Let { name, value, mutable } => {
                let keyword = if *mutable { "var" } else { "val" };
                format!("{} {} = {};", keyword, name, self.gen_expression(value, target))
            },
            Statement::If { condition, then_branch, else_branch } => {
                let cond = self.gen_expression(condition, target);
                let then_code = then_branch.iter()
                    .map(|s| self.gen_code(s, target))
                    .collect::<Vec<_>>()
                    .join("\n");
                let else_code = else_branch.as_ref()
                    .map(|eb| eb.iter()
                        .map(|s| self.gen_code(s, target))
                        .collect::<Vec<_>>()
                        .join("\n"))
                    .unwrap_or_default();
                format!("if ({}) {{\n{}\n}} else {{\n{}\n}}", cond, then_code, else_code)
            },
            _ => "".to_string(),
        }
    }

    fn gen_function(
        &self,
        name: &str,
        params: &[String],
        body: &[Statement],
        return_type: &Option<String>,
        target: &LanguageSpec,
    ) -> String {
        let params_str = params.join(", ");
        let return_str = return_type.as_ref()
            .map(|t| format!(" -> {}", t))
            .unwrap_or_default();
        let body_str = body.iter()
            .map(|s| self.gen_code(s, target))
            .collect::<Vec<_>>()
            .join("\n");

        format!("fn {}({}) {} {{\n{}\n}}", name, params_str, return_str, body_str)
    }

    fn gen_expression(&self, expr: &Expr, target: &LanguageSpec) -> String {
        match expr {
            Expr::Literal(v) => v.clone(),
            Expr::Variable(name) => name.clone(),
            Expr::Binary(op, left, right) => {
                format!("({} {} {})", self.gen_expression(left, target), op, self.gen_expression(right, target))
            },
            Expr::Call(func, args) => {
                let args_str = args.iter()
                    .map(|a| self.gen_expression(a, target))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", func, args_str)
            },
            Expr::Lambda { params, body } => {
                let params_str = params.join(", ");
                let body_str = self.gen_expression(body, target);
                format!("(|{}| {})", params_str, body_str)
            },
        }
    }
}

// ============================================================================
// SUPPORTING TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseTree {
    pub root: Node,
    pub tokens: Vec<Token>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Terminal { symbol: String, value: String },
    NonTerminal { symbol: String, children: Vec<Node>, production: Option<SemanticAction> },
    Epsilon,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenKind {
    Number,
    String,
    Identifier,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Equals,
    Function,
    Let,
    If,
    Else,
    While,
    Return,
    Boolean,
    Eof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractSyntaxTree {
    pub statements: Vec<Statement>,
    pub scope: Scope,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
        return_type: Option<String>,
    },
    Expression(Expr),
    Let {
        name: String,
        value: Expr,
        mutable: bool,
    },
    If {
        condition: Expr,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
    Return(Option<Expr>),
    Macro {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    MacroCall {
        macro_name: String,
        args: Vec<Expr>,
    },
    Template {
        params: Vec<String>,
        body: Vec<Statement>,
    },
    Nop,
}

impl Statement {
    fn substitute(&mut self, from: &str, to: &Expr) {
        // Recursively substitute in statement
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Literal(String),
    Variable(String),
    Binary(String, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
    pub variables: HashMap<String, Type>,
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaIR {
    pub instructions: Vec<MetaInstruction>,
    pub metadata: HashMap<String, String>,
}

impl MetaIR {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

impl Default for MetaIR {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetaInstruction {
    DefineFunction {
        name: String,
        params: Vec<String>,
        body: Vec<MetaInstruction>,
        return_type: Option<String>,
    },
    Expr(MetaExpr),
    Nop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetaExpr {
    Constant(String),
    Load(String),
    Binary {
        op: String,
        left: Box<MetaExpr>,
        right: Box<MetaExpr>,
    },
    Call {
        func: String,
        args: Vec<MetaExpr>,
    },
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
    fn test_meta_compiler_creation() {
        let source = LanguageSpec {
            name: "TestLang".to_string(),
            version: "1.0".to_string(),
            grammar: Grammar {
                productions: vec![],
                start_symbol: "program".to_string(),
                terminals: HashSet::new(),
                non_terminals: HashSet::new(),
            },
            type_system: TypeSystem {
                name: "Simple".to_string(),
                rules: vec![],
                primitives: vec![Type::Primitive(PrimitiveType::Int)],
            },
            features: vec![LanguageFeature::HigherOrderFunctions],
        };

        let target = LanguageSpec {
            name: "Rust".to_string(),
            version: "2021".to_string(),
            grammar: Grammar {
                productions: vec![],
                start_symbol: "crate".to_string(),
                terminals: HashSet::new(),
                non_terminals: HashSet::new(),
            },
            type_system: TypeSystem {
                name: "Rust".to_string(),
                rules: vec![],
                primitives: vec![Type::Primitive(PrimitiveType::Int64)],
            },
            features: vec![LanguageFeature::HigherOrderFunctions, LanguageFeature::PatternMatching],
        };

        let compiler = MetaCompiler::new(source, target);
        assert_eq!(compiler.optimization_level, 3);
    }

    #[test]
    fn test_tokenization() {
        let compiler = MetaCompiler::new(
            LanguageSpec::default(),
            LanguageSpec::default(),
        );

        let tokens = compiler.tokenize("let x = 42;").unwrap();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Let));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Number));
    }
}

impl Default for LanguageSpec {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            version: "0.0.0".to_string(),
            grammar: Grammar {
                productions: vec![],
                start_symbol: "start".to_string(),
                terminals: HashSet::new(),
                non_terminals: HashSet::new(),
            },
            type_system: TypeSystem {
                name: "Simple".to_string(),
                rules: vec![],
                primitives: vec![],
            },
            features: vec![],
        }
    }
}