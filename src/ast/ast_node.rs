// Remove circular import - ASTNode is defined in this file
/// Core AST node type for the CURSED programming language

use crate::ast::traits::{Node, Statement, Expression};
use crate::error::SourceLocation;
use std::any::Any;

/// Unified AST node type that can represent any node in the syntax tree
#[derive(Debug, Clone)]
pub enum ASTNode {
    /// Statement nodes
    /// Expression nodes
    /// Program root
    /// Block of statements
    /// Import statement
    /// Comment node
/// Program root node
#[derive(Debug, Clone)]
pub struct Program {
impl Program {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn with_statements(statements: Vec<ASTNode>) -> Self {
        Self {
        }
    }
impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for Program {
    fn string(&self) -> String {
        self.statements
            .iter()
            .map(|stmt| stmt.string())
            .collect::<Vec<_>>()
            .join("\n")
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            String::new()
        } else {
            self.statements[0].token_literal()
        }
    }
/// Block of statements
#[derive(Debug, Clone)]
pub struct Block {
impl Block {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn with_statements(statements: Vec<ASTNode>) -> Self {
        Self {
        }
    }
impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for Block {
    fn string(&self) -> String {
        let statements_str = self.statements
            .iter()
            .map(|stmt| stmt.string())
            .collect::<Vec<_>>()
            .join("\n");
        format!("{{\n{}\n}}", statements_str)
    fn token_literal(&self) -> String {
        "{"  .to_string()
    }
}

/// Import statement node
#[derive(Debug, Clone)]
pub struct ImportNode {
impl ImportNode {
    pub fn new(path: String) -> Self {
        Self {
        }
    }
impl Node for ImportNode {
    fn string(&self) -> String {
        if self.items.is_empty() {
            if let Some(alias) = &self.alias {
                format!("import \"{}\" as {}", self.path, alias)
            } else {
                format!("import \"{}\"", self.path)
            }
        } else {
            format!("import \"{}\" {{ {} }}", self.path, self.items.join(", "))
        }
    }

    fn token_literal(&self) -> String {
        "import".to_string()
    }
}

/// Comment node
#[derive(Debug, Clone)]
pub struct CommentNode {
impl CommentNode {
    pub fn new(text: String, is_multiline: bool) -> Self {
        Self {
        }
    }
impl Node for CommentNode {
    fn string(&self) -> String {
        if self.is_multiline {
            format!("/* {} */", self.text)
        } else {
            format!("// {}", self.text)
        }
    }

    fn token_literal(&self) -> String {
        if self.is_multiline { "/*" } else { "//" }.to_string()
    }
}

impl ASTNode {
    /// Create a statement node
    pub fn statement(stmt: Box<dyn Statement>) -> Self {
        Self::Statement(stmt)
    /// Create an expression node
    pub fn expression(expr: Box<dyn Expression>) -> Self {
        Self::Expression(expr)
    /// Create a program node
    pub fn program(statements: Vec<ASTNode>) -> Self {
        Self::Program(Program::with_statements(statements))
    /// Create a block node
    pub fn block(statements: Vec<ASTNode>) -> Self {
        Self::Block(Block::with_statements(statements))
    /// Create an import node
    pub fn import(path: String) -> Self {
        Self::Import(ImportNode::new(path))
    /// Create a comment node
    pub fn comment(text: String, is_multiline: bool) -> Self {
        Self::Comment(CommentNode::new(text, is_multiline))
    /// Get the type name of this AST node
    pub fn type_name(&self) -> &'static str {
        match self {
        }
    }

    /// Check if this node is a statement
    pub fn is_statement(&self) -> bool {
        matches!(self, ASTNode::Statement(_))
    /// Check if this node is an expression
    pub fn is_expression(&self) -> bool {
        matches!(self, ASTNode::Expression(_))
    /// Get source location if available
    pub fn location(&self) -> Option<&SourceLocation> {
        match self {
        }
    }
impl Node for ASTNode {
    fn string(&self) -> String {
        match self {
        }
    }

    fn token_literal(&self) -> String {
        match self {
        }
    }
/// Visitor pattern for traversing AST nodes
pub trait ASTVisitor {
    type Result;

    fn visit_node(&mut self, node: &ASTNode) -> Self::Result;
    fn visit_program(&mut self, program: &Program) -> Self::Result;
    fn visit_block(&mut self, block: &Block) -> Self::Result;
    fn visit_import(&mut self, import: &ImportNode) -> Self::Result;
    fn visit_comment(&mut self, comment: &CommentNode) -> Self::Result;
/// Mutable visitor pattern for modifying AST nodes
pub trait ASTMutVisitor {
    type Result;

    fn visit_node_mut(&mut self, node: &mut ASTNode) -> Self::Result;
    fn visit_program_mut(&mut self, program: &mut Program) -> Self::Result;
    fn visit_block_mut(&mut self, block: &mut Block) -> Self::Result;
    fn visit_import_mut(&mut self, import: &mut ImportNode) -> Self::Result;
    fn visit_comment_mut(&mut self, comment: &mut CommentNode) -> Self::Result;
/// Walk through all nodes in an AST
pub fn walk_ast<V: ASTVisitor>(visitor: &mut V, node: &ASTNode) -> V::Result {
    match node {
        ASTNode::Program(program) => {
            for stmt in &program.statements {
                walk_ast(visitor, stmt);
            }
            visitor.visit_program(program)
        ASTNode::Block(block) => {
            for stmt in &block.statements {
                walk_ast(visitor, stmt);
            }
            visitor.visit_block(block)
    }
}
