// AST Node Support for Documentation Extractors
// 
// This module provides enhanced AST node structures that are needed
// for comprehensive documentation extraction but may not be fully
// implemented in the core AST.

use crate::error::SourceLocation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Enhanced function declaration with complete information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    /// Function name
    /// Function parameters
    /// Return type
    /// Function body
    /// Generic parameters
    /// Generic constraints
    /// Whether function is async
    /// Whether function is public
    /// Source location
/// Enhanced struct declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDeclaration {
    /// Struct name
    /// Struct fields
    /// Generic parameters
    /// Generic constraints
    /// Whether struct is public
    /// Source location
/// Enhanced interface declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceDeclaration {
    /// Interface name
    /// Interface methods
    /// Generic parameters
    /// Generic constraints
    /// Whether interface is public
    /// Source location
/// Enhanced enum declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDeclaration {
    /// Enum name
    /// Enum variants
    /// Whether enum is public
    /// Source location
/// Enum variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    /// Variant name
    /// Variant fields
    /// Variant discriminant value
/// Type alias declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAliasDeclaration {
    /// Alias name
    /// Target type
    /// Whether alias is public
    /// Source location
/// Module declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDeclaration {
    /// Module name
    /// Module body
    /// Whether module is public
    /// Source location
/// Import statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportStatement {
    /// Import source
    /// Import type
    /// Import alias
    /// Imported items
    /// Source location
/// Import types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportType {
    /// import "module"
    /// import { item1, item2 } from "module"
    /// import * as alias from "module"
    /// import alias from "module"
/// Variable declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDeclaration {
    /// Variable name
    /// Variable type
    /// Initial value
    /// Whether variable is mutable
    /// Whether variable is constant
    /// Whether variable is public
    /// Source location
/// Constant declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantDeclaration {
    /// Constant name
    /// Constant type
    /// Constant value
    /// Whether constant is public
    /// Source location
/// Function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// Parameter name
    /// Parameter type
    /// Default value
    /// Whether parameter is variadic
/// Struct/interface field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    /// Field name
    /// Field type
    /// Whether field is public
    /// Whether field is optional
/// Generic constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericConstraint {
    /// Constraint type
    /// Target type
    /// Constraint expression
/// Expression placeholder for documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expression {
    /// Expression type
impl crate::ast::traits::Node for Expression {
    fn string(&self) -> String {
        match &self.expr_type {
            ExpressionType::FunctionCall(call) => {
                    call.arguments.iter()
                        .map(|arg| arg.string())
                        .collect::<Vec<_>>()
                        .join(", "))
            ExpressionType::BinaryExpression(bin) => {
                format!("{} {} {}", bin.left.string(), bin.operator, bin.right.string())
            ExpressionType::UnaryExpression(un) => {
                format!("{}{}", un.operator, un.operand.string())
            _ => format!("{:?}", self.expr_type)
        }
    }
    
    fn token_literal(&self) -> String {
        match &self.expr_type {
            _ => "expression".to_string()
        }
    }
impl crate::ast::traits::Expression for Expression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    fn clone_box(&self) -> Box<dyn crate::ast::traits::Expression> {
        Box::new(self.clone())
    }
}

/// Expression types for documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpressionType {
    /// Identifier expression
    /// Literal expression
    /// Function call expression
    /// Array access expression
    /// Member access expression
    /// Binary expression
    /// Unary expression
    /// Conditional expression
    /// Assignment expression
    /// Update expression
    /// Await expression
    /// Yield expression
    /// New expression
    /// This expression
    /// Super expression
    /// Arrow function expression
    /// Type assertion expression
    /// Type assertion with question mark
    /// Parenthesized expression
/// Identifier expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifierExpression {
    /// Identifier name
/// Function call expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallExpression {
    /// Function being called
    /// Function arguments
/// Array access expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayAccessExpression {
    /// Array being accessed
    /// Index expression
/// Member access expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberAccessExpression {
    /// Object being accessed
    /// Member name
/// Binary expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryExpression {
    /// Left operand
    /// Operator
    /// Right operand
/// Unary expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryExpression {
    /// Operator
    /// Operand
/// Conditional expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalExpression {
    /// Condition
    /// True expression
    /// False expression
/// Assignment expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentExpression {
    /// Left side
    /// Right side
/// Update expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExpression {
    /// Operator
    /// Operand
    /// Whether operator is prefix
/// Await expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwaitExpression {
    /// Expression being awaited
/// Yield expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldExpression {
    /// Yielded value
/// New expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewExpression {
    /// Constructor being called
    /// Constructor arguments
/// Arrow function expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrowFunctionExpression {
    /// Function parameters
    /// Function body
/// Type assertion expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAssertionExpression {
    /// Expression being asserted
    /// Type annotation
/// Type assertion with question mark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAssertionQuestionExpression {
    /// Expression being asserted
    /// Type annotation
/// Parenthesized expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParenthesizedExpression {
    /// Inner expression
/// Literal values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    /// String literal
    /// Number literal
    /// Boolean literal
    /// Null literal
    /// Array literal
    /// Object literal
impl Default for Expression {
    fn default() -> Self {
        Self {
            expr_type: ExpressionType::Identifier(IdentifierExpression {
        }
    }
}
