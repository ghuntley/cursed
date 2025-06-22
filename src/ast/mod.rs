/// CURSED Language Abstract Syntax Tree
///
/// This module provides a comprehensive AST implementation for the CURSED programming language,
/// following the grammar specifications and implementing Gen Z slang keywords.
///
/// # Why AST Reliability is Critical
///
/// The Abstract Syntax Tree is the foundation of any compiler, serving as the intermediate
/// representation between source code parsing and code generation. AST reliability is
/// absolutely critical for several reasons:
///
/// ## 1. Compiler Correctness
/// - **Semantic Preservation**: The AST must accurately represent the meaning of source code
/// - **Type Safety**: Incorrect AST structure can lead to type system failures
/// - **Code Generation**: LLVM and other backends depend on correct AST structure
/// - **Optimization**: Many optimizations rely on AST transformations
///
/// ## 2. Developer Experience  
/// - **Error Messages**: Accurate source location tracking requires proper AST nodes
/// - **IDE Support**: Language servers need reliable AST for features like autocomplete
/// - **Debugging**: Debug information generation depends on AST fidelity
/// - **Refactoring**: Safe code transformations require trustworthy AST representation
///
/// ## 3. Language Evolution
/// - **Feature Addition**: New language features require extending the AST
/// - **Backward Compatibility**: AST changes must not break existing code
/// - **Performance**: Efficient AST representation affects compilation speed
/// - **Memory Usage**: AST structure impacts compiler memory consumption
///
/// ## 4. Testing Strategy
/// Each AST node type requires comprehensive testing to ensure:
/// - Correct construction and field access
/// - Proper trait implementations (Debug, Clone, Display)
/// - String representation matches expected output
/// - Expression/Statement classification is accurate
/// - Source location tracking works correctly
/// - Memory safety in dynamic dispatch scenarios
///
/// Testing AST nodes in isolation allows catching issues early before they propagate
/// to more complex compilation phases, making debugging significantly easier.

use std::fmt::{Debug, Display};
use std::any::Any;
use std::collections::HashMap;
use crate::lexer::Token;
use crate::error::SourceLocation;

// Re-export all sub-modules for easier access
pub mod traits;
pub mod expressions;
pub mod statements;
pub mod declarations;
pub mod literals;
pub mod operators;
pub mod conditionals;
pub mod types;
pub mod identifiers;
pub mod block;
pub mod calls;
pub mod struct_expr;
pub mod fields;
pub mod if_expression;
pub mod dot_expression;
pub mod ast_node;
pub mod pointer;
pub mod range_expression;
pub mod channel;
pub mod slice_literal;
pub mod concurrency;
pub mod channel_range;
pub mod type_switch;
pub mod select;
pub mod documentation;
pub mod collections;

// Additional modules for parser support
pub mod parser_support;

// Core types module
pub mod core_types;

// Re-export commonly used types
pub use traits::{Node, Expression, Statement, TypeNode, GenericNode, Visitable, Visitor, Mutable, MutVisitor, Locatable, StatementExtensions, TypeParameter};
pub use expressions::*;
pub use statements::*;
pub use declarations::{FunctionStatement, FunctionDeclaration, SquadStatement, CollabStatement, GenericConstraint, AsyncFunctionStatement, AsyncFunctionDeclaration, StructDeclaration, InterfaceDeclaration};
pub use literals::*;
pub use operators::*;
pub use conditionals::*;
pub use types::*;
pub use identifiers::*;
pub use block::*;
pub use calls::*;
pub use struct_expr::*;
pub use fields::{FieldStatement, FieldDefinition};
pub use if_expression::*;
pub use dot_expression::*;
pub use ast_node::{ASTNode, Program as ASTProgram, Block as ASTBlock, ImportNode, CommentNode, ASTVisitor, ASTMutVisitor, walk_ast};
pub use pointer::*;
pub use range_expression::*;
pub use channel::*;
pub use slice_literal::*;
pub use concurrency::*;
pub use channel_range::*;
pub use type_switch::*;
pub use select::*;
pub use documentation::*;
pub use collections::*;
pub use core_types::{
    AST, VariableDeclaration, ConstantDeclaration, ImportDeclaration, 
    PackageDeclaration, InterfaceMethod, StructField, FieldVisibility,
    ModuleDeclaration
};
// Specific imports to avoid conflicts
pub use parser_support::{
    VariableStatement,
    ExpressionStatement as ParserExpressionStatement,
    ArrayLiteral as ParserArrayLiteral, 
    HashLiteral as ParserHashLiteral,
    NilLiteral as ParserNilLiteral,
    IndexExpression as ParserIndexExpression,
    DotExpression as ParserDotExpression,
};

/// Root AST node representing a complete CURSED program
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
    pub package_name: Option<String>,
    pub imports: Vec<ImportStatement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            package_name: None,
            imports: Vec::new(),
        }
    }
    
    pub fn with_package(package_name: String) -> Self {
        Self {
            statements: Vec::new(),
            package_name: Some(package_name),
            imports: Vec::new(),
        }
    }
    
    pub fn add_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
    
    pub fn add_import(&mut self, import: ImportStatement) {
        self.imports.push(import);
    }
}

impl Node for Program {
    fn string(&self) -> String {
        let mut result = String::new();
        
        if let Some(package) = &self.package_name {
            result.push_str(&format!("vibe {}\n\n", package));
        }
        
        for import in &self.imports {
            result.push_str(&format!("{}\n", import.string()));
        }
        
        if !self.imports.is_empty() {
            result.push('\n');
        }
        
        for statement in &self.statements {
            result.push_str(&format!("{}\n", statement.string()));
        }
        
        result
    }

    fn token_literal(&self) -> String {
        if let Some(first_stmt) = self.statements.first() {
            first_stmt.token_literal()
        } else {
            String::new()
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

/// Package declaration statement (vibe package_name)
#[derive(Debug, Clone)]
pub struct PackageStatement {
    pub token: Token,
    pub name: String,
}

impl Node for PackageStatement {
    fn string(&self) -> String {
        format!("vibe {}", self.name)
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for PackageStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Import statement (yeet "package_path")
#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub token: Token,
    pub path: String,
    pub alias: Option<String>,
}

impl ImportStatement {
    pub fn new(token: Token, path: String) -> Self {
        Self {
            token,
            path,
            alias: None,
        }
    }
    
    pub fn with_alias(token: Token, path: String, alias: String) -> Self {
        Self {
            token,
            path,
            alias: Some(alias),
        }
    }
}

impl Node for ImportStatement {
    fn string(&self) -> String {
        if let Some(alias) = &self.alias {
            format!("yeet {} \"{}\"", alias, self.path)
        } else {
            format!("yeet \"{}\"", self.path)
        }
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for ImportStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Enum representing different types of AST nodes
#[derive(Debug, Clone)]
pub enum AstNodeType {
    /// Program root node
    Program(Program),
    /// Block statement
    BlockStatement(BlockStatement),
    /// Expression statement
    ExpressionStatement(Box<dyn Expression>),
    /// Function declaration
    FunctionDeclaration(FunctionDeclaration),
    /// Struct declaration  
    StructDeclaration(SquadStatement),
    /// Interface declaration
    InterfaceDeclaration(CollabStatement),
    /// Enum declaration
    EnumDeclaration(EnumStatement),
    /// Variable declaration
    VariableDeclaration(VariableStatement),
    /// Constant declaration
    ConstantDeclaration(ConstantStatement),
    /// Type alias declaration
    TypeAliasDeclaration(TypeAliasStatement),
    /// Import statement
    ImportStatement(ImportStatement),
    /// Import declaration
    Import(ImportStatement),
    /// Module declaration
    ModuleDeclaration(ModuleStatement),
    /// If statement
    IfStatement(IfStatement),
    /// While statement
    WhileStatement(WhileStatement),
    /// For statement
    ForStatement(ForStatement),
    /// For-in statement
    ForInStatement(ForInStatement),
    /// Do-while statement
    DoWhileStatement(DoWhileStatement),
    /// Switch statement
    SwitchStatement(SwitchStatement),
    /// Try statement
    TryStatement(TryStatement),
    /// Return statement
    ReturnStatement(ReturnStatement),
}

/// Unified AST node wrapper
#[derive(Debug, Clone)]
pub struct AstNode {
    /// The specific node type and its data
    pub node_type: AstNodeType,
    /// Source location information
    pub location: Option<SourceLocation>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl AstNode {
    /// Create a new AST node
    pub fn new(node_type: AstNodeType) -> Self {
        Self {
            node_type,
            location: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Create a new AST node with location
    pub fn with_location(node_type: AstNodeType, location: SourceLocation) -> Self {
        Self {
            node_type,
            location: Some(location),
            metadata: HashMap::new(),
        }
    }

    /// Create a new statement node
    pub fn new_statement(statement: Statement) -> Self {
        Self::new(AstNodeType::Statement(statement))
    }

    /// Create a new program node
    pub fn new_program(program: ASTProgram) -> Self {
        Self::new(AstNodeType::Program(program))
    }
    
    /// Get the string representation of this node
    pub fn string(&self) -> String {
        match &self.node_type {
            AstNodeType::Program(program) => program.string(),
            AstNodeType::BlockStatement(block) => block.string(),
            AstNodeType::ExpressionStatement(expr) => expr.string(),
            AstNodeType::FunctionDeclaration(func) => func.string(),
            AstNodeType::StructDeclaration(struct_decl) => struct_decl.string(),
            AstNodeType::InterfaceDeclaration(interface) => interface.string(),
            AstNodeType::EnumDeclaration(enum_decl) => enum_decl.string(),
            AstNodeType::VariableDeclaration(var) => var.string(),
            AstNodeType::ConstantDeclaration(const_decl) => const_decl.string(),
            AstNodeType::TypeAliasDeclaration(type_alias) => type_alias.string(),
            AstNodeType::ImportStatement(import) => import.string(),
            AstNodeType::Import(import) => import.string(),
            AstNodeType::ModuleDeclaration(module) => module.string(),
            AstNodeType::IfStatement(if_stmt) => if_stmt.string(),
            AstNodeType::WhileStatement(while_stmt) => while_stmt.string(),
            AstNodeType::ForStatement(for_stmt) => for_stmt.string(),
            AstNodeType::ForInStatement(for_in) => for_in.string(),
            AstNodeType::DoWhileStatement(do_while) => do_while.string(),
            AstNodeType::SwitchStatement(switch) => switch.string(),
            AstNodeType::TryStatement(try_stmt) => try_stmt.string(),
            AstNodeType::ReturnStatement(ret) => ret.string(),
        }
    }
}
