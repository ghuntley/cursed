/// Core AST node type for the CURSED programming language

use crate::ast::traits::{Node, Statement, Expression};
use crate::error::SourceLocation;
use std::any::Any;

/// Unified AST node type that can represent any node in the syntax tree
#[derive(Debug, Clone)]
pub enum ASTNode {
    /// Statement nodes
    Statement(Box<dyn Statement>),
    /// Expression nodes
    Expression(Box<dyn Expression>),
    /// Program root
    Program(Program),
    /// Block of statements
    Block(Block),
    /// Import statement
    Import(ImportNode),
    /// Comment node
    Comment(CommentNode),
}

/// Program root node
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<ASTNode>,
    pub location: Option<SourceLocation>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            location: None,
        }
    }

    pub fn with_statements(statements: Vec<ASTNode>) -> Self {
        Self {
            statements,
            location: None,
        }
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
    }

    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            String::new()
        } else {
            self.statements[0].token_literal()
        }
    }
}

/// Block of statements
#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<ASTNode>,
    pub location: Option<SourceLocation>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            location: None,
        }
    }

    pub fn with_statements(statements: Vec<ASTNode>) -> Self {
        Self {
            statements,
            location: None,
        }
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
    }

    fn token_literal(&self) -> String {
        "{"  .to_string()
    }
}

/// Import statement node
#[derive(Debug, Clone)]
pub struct ImportNode {
    pub path: String,
    pub alias: Option<String>,
    pub items: Vec<String>,
    pub location: Option<SourceLocation>,
}

impl ImportNode {
    pub fn new(path: String) -> Self {
        Self {
            path,
            alias: None,
            items: Vec::new(),
            location: None,
        }
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
    pub text: String,
    pub is_multiline: bool,
    pub location: Option<SourceLocation>,
}

impl CommentNode {
    pub fn new(text: String, is_multiline: bool) -> Self {
        Self {
            text,
            is_multiline,
            location: None,
        }
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
    }

    /// Create an expression node
    pub fn expression(expr: Box<dyn Expression>) -> Self {
        Self::Expression(expr)
    }

    /// Create a program node
    pub fn program(statements: Vec<ASTNode>) -> Self {
        Self::Program(Program::with_statements(statements))
    }

    /// Create a block node
    pub fn block(statements: Vec<ASTNode>) -> Self {
        Self::Block(Block::with_statements(statements))
    }

    /// Create an import node
    pub fn import(path: String) -> Self {
        Self::Import(ImportNode::new(path))
    }

    /// Create a comment node
    pub fn comment(text: String, is_multiline: bool) -> Self {
        Self::Comment(CommentNode::new(text, is_multiline))
    }

    /// Get the type name of this AST node
    pub fn type_name(&self) -> &'static str {
        match self {
            ASTNode::Statement(_) => "Statement",
            ASTNode::Expression(_) => "Expression",
            ASTNode::Program(_) => "Program",
            ASTNode::Block(_) => "Block",
            ASTNode::Import(_) => "Import",
            ASTNode::Comment(_) => "Comment",
        }
    }

    /// Check if this node is a statement
    pub fn is_statement(&self) -> bool {
        matches!(self, ASTNode::Statement(_))
    }

    /// Check if this node is an expression
    pub fn is_expression(&self) -> bool {
        matches!(self, ASTNode::Expression(_))
    }

    /// Get source location if available
    pub fn location(&self) -> Option<&SourceLocation> {
        match self {
            ASTNode::Program(p) => p.location.as_ref(),
            ASTNode::Block(b) => b.location.as_ref(),
            ASTNode::Import(i) => i.location.as_ref(),
            ASTNode::Comment(c) => c.location.as_ref(),
            _ => None,
        }
    }
}

impl Node for ASTNode {
    fn string(&self) -> String {
        match self {
            ASTNode::Statement(stmt) => stmt.string(),
            ASTNode::Expression(expr) => expr.string(),
            ASTNode::Program(program) => program.string(),
            ASTNode::Block(block) => block.string(),
            ASTNode::Import(import) => import.string(),
            ASTNode::Comment(comment) => comment.string(),
        }
    }

    fn token_literal(&self) -> String {
        match self {
            ASTNode::Statement(stmt) => stmt.token_literal(),
            ASTNode::Expression(expr) => expr.token_literal(),
            ASTNode::Program(program) => program.token_literal(),
            ASTNode::Block(block) => block.token_literal(),
            ASTNode::Import(import) => import.token_literal(),
            ASTNode::Comment(comment) => comment.token_literal(),
        }
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
}

/// Mutable visitor pattern for modifying AST nodes
pub trait ASTMutVisitor {
    type Result;

    fn visit_node_mut(&mut self, node: &mut ASTNode) -> Self::Result;
    fn visit_program_mut(&mut self, program: &mut Program) -> Self::Result;
    fn visit_block_mut(&mut self, block: &mut Block) -> Self::Result;
    fn visit_import_mut(&mut self, import: &mut ImportNode) -> Self::Result;
    fn visit_comment_mut(&mut self, comment: &mut CommentNode) -> Self::Result;
}

/// Walk through all nodes in an AST
pub fn walk_ast<V: ASTVisitor>(visitor: &mut V, node: &ASTNode) -> V::Result {
    match node {
        ASTNode::Program(program) => {
            for stmt in &program.statements {
                walk_ast(visitor, stmt);
            }
            visitor.visit_program(program)
        },
        ASTNode::Block(block) => {
            for stmt in &block.statements {
                walk_ast(visitor, stmt);
            }
            visitor.visit_block(block)
        },
        _ => visitor.visit_node(node),
    }
}
