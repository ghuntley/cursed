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
/// - **CursedError Messages**: Accurate source location tracking requires proper AST nodes
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
// Remove duplicate import - ASTNode imported below

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
use crate::error::CursedError;

// Re-export commonly used types
pub use traits::{Node, Expression, Statement, TypeNode, GenericNode, Visitable, Visitor, Mutable, MutVisitor, Locatable, StatementExtensions, TypeParameter};
pub use expressions::*;
// Use explicit imports to avoid E0659 conflicts between statements and conditionals
pub use statements::{ExpressionStatement, ReturnStatement, BreakStatement, ContinueStatement, ThrowStatement, TryStatement, CatchStatement, FinallyStatement, ImportStatement, PackageStatement, MutStatement, ConstStatement, AssignmentStatement, ChannelReceiveStatement, ChannelSendStatement, ChannelCloseStatement, LetStatement, FactsStatement, EnumStatement, ConstantStatement, TypeAliasStatement, ModuleStatement, ForInStatement, DoWhileStatement};
pub use declarations::{FunctionStatement, FunctionDeclaration, SquadStatement, CollabStatement, GenericConstraint, AsyncFunctionStatement, AsyncFunctionDeclaration, StructDeclaration, InterfaceDeclaration};
pub use literals::*;
pub use operators::*;
// Import conditionals explicitly to avoid conflicts with statements
pub use conditionals::{IfStatement as ConditionalIfStatement, WhileStatement as ConditionalWhileStatement, ForStatement as ConditionalForStatement, SwitchStatement as ConditionalSwitchStatement, ElseStatement, ElseIfStatement, CaseStatement, DefaultStatement};
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
    ModuleDeclaration
// };
// Specific imports to avoid conflicts
pub use parser_support::{
// };

/// Root AST node representing a complete CURSED program
#[derive(Debug, Clone)]
pub struct Program {
impl Program {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn with_package(package_name: String) -> Self {
        Self {
        }
    }
    
    pub fn add_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    pub fn add_import(&mut self, import: ImportStatement) {
        self.imports.push(import);
    }
}

impl Node for Program {
    fn string(&self) -> String {
        let mut result = String::new();
        
        if let Some(package) = &self.package_name {
            result.push_str(&format!("vibe {}\n\n", package));
        for import in &self.imports {
            result.push_str(&format!("{}\n", import.string()));
        if !self.imports.is_empty() {
            result.push('\n');
        for statement in &self.statements {
            result.push_str(&format!("{}\n", statement.string()));
        result
    fn token_literal(&self) -> String {
        if let Some(first_stmt) = self.statements.first() {
            first_stmt.token_literal()
        } else {
            String::new()
        }
    }
impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}



/// Enum representing different types of AST nodes
#[derive(Debug, Clone)]
pub enum AstNodeType {
    /// Program root node
    /// Block statement
    /// Expression statement
    /// Function declaration
    /// Struct declaration  
    /// Interface declaration
    /// Enum declaration
    /// Variable declaration
    /// Constant declaration
    /// Type alias declaration
    /// Import statement
    /// Import declaration
    /// Module declaration
    /// If statement
    /// While statement
    /// For statement
    /// For-in statement
    /// Do-while statement
    /// Switch statement
    /// Try statement
    /// Return statement
/// Unified AST node wrapper
#[derive(Debug, Clone)]
pub struct AstNode {
    /// The specific node type and its data
    /// Source location information
    /// Additional metadata
impl AstNode {
    /// Create a new AST node
    pub fn new(node_type: AstNodeType) -> Self {
        Self {
        }
    }
    
    /// Create a new AST node with location
    pub fn with_location(node_type: AstNodeType, location: SourceLocation) -> Self {
        Self {
        }
    }

    /// Create a new statement node from a boxed statement
    pub fn new_statement(statement: Box<dyn Statement>) -> Self {
        Self::new(AstNodeType::ExpressionStatement(statement))
    /// Create a new program node
    pub fn new_program(program: Program) -> Self {
        Self::new(AstNodeType::Program(program))
    /// Get the string representation of this node
    pub fn string(&self) -> String {
        match &self.node_type {
        }
    }
}
