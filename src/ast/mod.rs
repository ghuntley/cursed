/// Abstract Syntax Tree module for CURSED
/// 
/// This module defines the AST nodes and structures for the CURSED language.

pub mod core_types;
pub mod parser_support;
pub mod statements;

use std::fmt;

// Basic AST node trait
pub trait Node {
    fn string(&self) -> String;
}

// Core AST structures
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
    
    pub fn add_statement(&mut self, stmt: Statement) {
        self.statements.push(stmt);
    }
}

impl Node for Program {
    fn string(&self) -> String {
        self.statements.iter()
            .map(|s| s.string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Assignment(String, Expression),
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
}

impl Node for Statement {
    fn string(&self) -> String {
        match self {
            Statement::Expression(expr) => expr.string(),
            Statement::Assignment(name, expr) => format!("{} = {}", name, expr.string()),
            Statement::FunctionDeclaration { name, params, body: _ } => {
                format!("func {}({})", name, params.join(", "))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(i64),
    String(String),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    Call {
        function: Box<Expression>,
        args: Vec<Expression>,
    },
}

impl Node for Expression {
    fn string(&self) -> String {
        match self {
            Expression::Number(n) => n.to_string(),
            Expression::String(s) => format!("\"{}\"", s),
            Expression::Identifier(name) => name.clone(),
            Expression::Binary { left, operator, right } => {
                format!("({} {} {})", left.string(), operator, right.string())
            }
            Expression::Call { function, args } => {
                let args_str = args.iter()
                    .map(|a| a.string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", function.string(), args_str)
            }
        }
    }
}

// Parser support functions
pub fn parse_program(source: &str) -> crate::error::Result<Program> {
    // Minimal parser implementation
    Ok(Program::new())
}

pub fn new_program(program: Program) -> Program {
    program
}
