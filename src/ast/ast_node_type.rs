/// AST node type enumeration for ML feature extraction
use crate::ast::expressions::Expression;
use crate::ast::statements::Statement;
use crate::ast::declarations::{FunctionDeclaration, VariableDeclaration, StructDeclaration, InterfaceDeclaration};

#[derive(Debug, Clone, PartialEq)]
pub enum AstNodeType {
    FunctionDeclaration(Box<FunctionDeclaration>),
    Statement(Box<Statement>),
    Expression(Box<Expression>),
    VariableDeclaration(Box<VariableDeclaration>),
    StructDeclaration(Box<StructDeclaration>),
    InterfaceDeclaration(Box<InterfaceDeclaration>),
}
