//! AST Node Support for Documentation Extractors
//! 
//! This module provides enhanced AST node structures that are needed
//! for comprehensive documentation extraction but may not be fully
//! implemented in the core AST.

use crate::error::SourceLocation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Enhanced function declaration with complete information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    /// Function name
    pub name: String,
    /// Function parameters
    pub parameters: Vec<Parameter>,
    /// Return type
    pub return_type: Option<Expression>,
    /// Function body
    pub body: crate::ast::AstNode,
    /// Generic parameters
    pub generic_params: Option<Vec<String>>,
    /// Generic constraints
    pub constraints: Option<Vec<GenericConstraint>>,
    /// Whether function is async
    pub is_async: bool,
    /// Whether function is public
    pub is_public: bool,
    /// Source location
    pub location: SourceLocation,
}

/// Enhanced struct declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDeclaration {
    /// Struct name
    pub name: String,
    /// Struct fields
    pub fields: Vec<Field>,
    /// Generic parameters
    pub generic_params: Option<Vec<String>>,
    /// Generic constraints
    pub constraints: Option<Vec<GenericConstraint>>,
    /// Whether struct is public
    pub is_public: bool,
    /// Source location
    pub location: SourceLocation,
}

/// Enhanced interface declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceDeclaration {
    /// Interface name
    pub name: String,
    /// Interface methods
    pub methods: Vec<FunctionDeclaration>,
    /// Generic parameters
    pub generic_params: Option<Vec<String>>,
    /// Generic constraints
    pub constraints: Option<Vec<GenericConstraint>>,
    /// Whether interface is public
    pub is_public: bool,
    /// Source location
    pub location: SourceLocation,
}

/// Enhanced enum declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDeclaration {
    /// Enum name
    pub name: String,
    /// Enum variants
    pub variants: Vec<EnumVariant>,
    /// Whether enum is public
    pub is_public: bool,
    /// Source location
    pub location: SourceLocation,
}

/// Enum variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    /// Variant name
    pub name: String,
    /// Variant fields
    pub fields: Vec<Field>,
    /// Variant discriminant value
    pub discriminant: Option<Expression>,
}

/// Type alias declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAliasDeclaration {
    /// Alias name
    pub name: String,
    /// Target type
    pub target_type: Expression,
    /// Whether alias is public
    pub is_public: bool,
    /// Source location
    pub location: SourceLocation,
}

/// Module declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDeclaration {
    /// Module name
    pub name: String,
    /// Module body
    pub body: Option<crate::ast::AstNode>,
    /// Whether module is public
    pub is_public: bool,
    /// Source location
    pub location: SourceLocation,
}

/// Import statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportStatement {
    /// Import source
    pub source: String,
    /// Import type
    pub import_type: ImportType,
    /// Import alias
    pub alias: Option<String>,
    /// Imported items
    pub items: Vec<String>,
    /// Source location
    pub location: SourceLocation,
}

/// Import types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportType {
    /// import "module"
    Module,
    /// import { item1, item2 } from "module"
    Named,
    /// import * as alias from "module"
    Wildcard,
    /// import alias from "module"
    Default,
}

/// Variable declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDeclaration {
    /// Variable name
    pub name: String,
    /// Variable type
    pub var_type: Option<Expression>,
    /// Initial value
    pub init: Option<Expression>,
    /// Whether variable is mutable
    pub is_mutable: bool,
    /// Whether variable is constant
    pub is_const: bool,
    /// Whether variable is public
    pub is_public: bool,
    /// Source location
    pub location: SourceLocation,
}

/// Constant declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantDeclaration {
    /// Constant name
    pub name: String,
    /// Constant type
    pub const_type: Option<Expression>,
    /// Constant value
    pub value: Expression,
    /// Whether constant is public
    pub is_public: bool,
    /// Source location
    pub location: SourceLocation,
}

/// Function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: Option<Expression>,
    /// Default value
    pub default_value: Option<Expression>,
    /// Whether parameter is variadic
    pub is_variadic: bool,
}

/// Struct/interface field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: Option<Expression>,
    /// Whether field is public
    pub is_public: bool,
    /// Whether field is optional
    pub is_optional: bool,
}

/// Generic constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericConstraint {
    /// Constraint type
    pub constraint_type: String,
    /// Target type
    pub target_type: String,
    /// Constraint expression
    pub expression: String,
}

/// Expression placeholder for documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expression {
    /// Expression type
    pub expr_type: ExpressionType,
}

impl crate::ast::traits::Node for Expression {
    fn string(&self) -> String {
        match &self.expr_type {
            ExpressionType::Identifier(id) => id.to_string().clone(),
            ExpressionType::Literal(lit) => lit.value.clone(),
            ExpressionType::FunctionCall(call) => {
                format!("{}({})", call.function.string(), 
                    call.arguments.iter()
                        .map(|arg| arg.string())
                        .collect::<Vec<_>>()
                        .join(", "))
            },
            ExpressionType::BinaryExpression(bin) => {
                format!("{} {} {}", bin.left.string(), bin.operator, bin.right.string())
            },
            ExpressionType::UnaryExpression(un) => {
                format!("{}{}", un.operator, un.operand.string())
            },
            _ => format!("{:?}", self.expr_type)
        }
    }
    
    fn token_literal(&self) -> String {
        match &self.expr_type {
            ExpressionType::Identifier(id) => id.to_string().clone(),
            ExpressionType::Literal(lit) => lit.value.clone(),
            _ => "expression".to_string()
        }
    }
}

impl crate::ast::traits::Expression for Expression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn crate::ast::traits::Expression> {
        Box::new(self.clone())
    }
}

/// Expression types for documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpressionType {
    /// Identifier expression
    Identifier(IdentifierExpression),
    /// Literal expression
    Literal(Literal),
    /// Function call expression
    FunctionCall(FunctionCallExpression),
    /// Array access expression
    ArrayAccess(ArrayAccessExpression),
    /// Member access expression
    MemberAccess(MemberAccessExpression),
    /// Binary expression
    BinaryExpression(BinaryExpression),
    /// Unary expression
    UnaryExpression(UnaryExpression),
    /// Conditional expression
    ConditionalExpression(ConditionalExpression),
    /// Assignment expression
    AssignmentExpression(AssignmentExpression),
    /// Update expression
    UpdateExpression(UpdateExpression),
    /// Await expression
    AwaitExpression(AwaitExpression),
    /// Yield expression
    YieldExpression(YieldExpression),
    /// New expression
    NewExpression(NewExpression),
    /// This expression
    ThisExpression,
    /// Super expression
    Super,
    /// Arrow function expression
    ArrowFunction(ArrowFunctionExpression),
    /// Type assertion expression
    TypeAssertion(TypeAssertionExpression),
    /// Type assertion with question mark
    TypeAssertionQuestion(TypeAssertionQuestionExpression),
    /// Parenthesized expression
    ParenthesizedExpression(ParenthesizedExpression),
}

/// Identifier expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifierExpression {
    /// Identifier name
    pub name: String,
}

/// Function call expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallExpression {
    /// Function being called
    pub function: Box<Expression>,
    /// Function arguments
    pub arguments: Vec<Expression>,
}

/// Array access expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayAccessExpression {
    /// Array being accessed
    pub array: Box<Expression>,
    /// Index expression
    pub index: Box<Expression>,
}

/// Member access expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberAccessExpression {
    /// Object being accessed
    pub object: Box<Expression>,
    /// Member name
    pub member: String,
}

/// Binary expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryExpression {
    /// Left operand
    pub left: Box<Expression>,
    /// Operator
    pub operator: String,
    /// Right operand
    pub right: Box<Expression>,
}

/// Unary expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryExpression {
    /// Operator
    pub operator: String,
    /// Operand
    pub operand: Box<Expression>,
}

/// Conditional expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalExpression {
    /// Condition
    pub condition: Box<Expression>,
    /// True expression
    pub true_expr: Box<Expression>,
    /// False expression
    pub false_expr: Box<Expression>,
}

/// Assignment expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentExpression {
    /// Left side
    pub left: Box<Expression>,
    /// Right side
    pub right: Box<Expression>,
}

/// Update expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExpression {
    /// Operator
    pub operator: String,
    /// Operand
    pub operand: Box<Expression>,
    /// Whether operator is prefix
    pub prefix: bool,
}

/// Await expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwaitExpression {
    /// Expression being awaited
    pub expression: Box<Expression>,
}

/// Yield expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldExpression {
    /// Yielded value
    pub argument: Option<Box<Expression>>,
}

/// New expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewExpression {
    /// Constructor being called
    pub callee: Box<Expression>,
    /// Constructor arguments
    pub arguments: Vec<Expression>,
}

/// Arrow function expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrowFunctionExpression {
    /// Function parameters
    pub parameters: Vec<Parameter>,
    /// Function body
    pub body: Box<Expression>,
}

/// Type assertion expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAssertionExpression {
    /// Expression being asserted
    pub expression: Box<Expression>,
    /// Type annotation
    pub type_annotation: Box<Expression>,
}

/// Type assertion with question mark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAssertionQuestionExpression {
    /// Expression being asserted
    pub expression: Box<Expression>,
    /// Type annotation
    pub type_annotation: Box<Expression>,
}

/// Parenthesized expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParenthesizedExpression {
    /// Inner expression
    pub expression: Box<Expression>,
}

/// Literal values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    /// String literal
    String(String),
    /// Number literal
    Number(String),
    /// Boolean literal
    Boolean(bool),
    /// Null literal
    Null,
    /// Array literal
    Array(Vec<Expression>),
    /// Object literal
    Object(HashMap<String, Expression>),
}

impl Default for Expression {
    fn default() -> Self {
        Self {
            expr_type: ExpressionType::Identifier(IdentifierExpression {
                name: "unknown".to_string(),
            }),
        }
    }
}
