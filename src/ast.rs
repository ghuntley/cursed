//! Abstract Syntax Tree for CURSED language

use crate::error::CursedError;

/// Root program node
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub imports: Vec<ImportStatement>,
    pub package: Option<PackageDeclaration>,
}

/// Import statement
#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub path: String,
    pub alias: Option<String>,
    pub items: Vec<String>,
}

/// Package declaration
#[derive(Debug, Clone)]
pub struct PackageDeclaration {
    pub name: String,
    pub version: Option<String>,
}

/// Statement types
#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Let(LetStatement),
    Return(ReturnStatement),
    If(IfStatement),
    Function(FunctionStatement),
    While(WhileStatement),
    For(ForStatement),
    Goroutine(GoroutineStatement),
    Channel(ChannelStatement),
}

/// Expression types
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
    String(String),
    Boolean(bool),
    Binary(BinaryExpression),
    Call(CallExpression),
    Literal(Literal),
    Unary(UnaryExpression),
    Array(Vec<Expression>),
    Map(Vec<(Expression, Expression)>),
}

/// Binary expression
#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

/// Function call expression
#[derive(Debug, Clone)]
pub struct CallExpression {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

/// Let statement
#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: String,
    pub value: Expression,
}

/// Return statement
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

/// If statement
#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Vec<Statement>,
    pub else_branch: Option<Vec<Statement>>,
}

/// Function statement
#[derive(Debug, Clone)]
pub struct FunctionStatement {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<Statement>,
}

/// While statement
#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

/// For statement
#[derive(Debug, Clone)]
pub struct ForStatement {
    pub init: Option<Box<Statement>>,
    pub condition: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Vec<Statement>,
}

/// Goroutine statement
#[derive(Debug, Clone)]
pub struct GoroutineStatement {
    pub expression: Expression,
}

/// Channel statement
#[derive(Debug, Clone)]
pub struct ChannelStatement {
    pub name: String,
    pub buffer_size: Option<Expression>,
}

/// Unary expression
#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not,
    Minus,
    Plus,
}

/// Literal values
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Nil,
}

/// Type annotations
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Float,
    String,
    Boolean,
    Void,
    Function(Vec<Type>, Box<Type>),
    Array(Box<Type>),
    Custom(String),
}

/// AST visitor trait for traversing the AST
pub trait AstVisitor<T> {
    fn visit_program(&mut self, program: &Program) -> T;
    fn visit_statement(&mut self, statement: &Statement) -> T;
    fn visit_expression(&mut self, expression: &Expression) -> T;
}

impl Default for Program {
    fn default() -> Self {
        Self {
            statements: vec![],
            imports: vec![],
            package: None,
        }
    }
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Parse a program from source code (simplified implementation)
pub fn parse_program(source: &str) -> Result<Program, CursedError> {
    // This is a simplified implementation for compatibility
    // Real parsing would use the lexer and parser modules
    let lexer = crate::lexer::Lexer::new(source.to_string());
    let mut parser = crate::parser::Parser::new(lexer)?;
    parser.parse_program()
}
