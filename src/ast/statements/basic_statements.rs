// Basic statement types for CURSED language

use std::any::Any;
use crate::ast::traits::{Node, Statement, Expression};
use crate::error::SourceLocation;

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
    pub location: SourceLocation,
}

impl ExpressionStatement {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self {
            expression,
            location: SourceLocation::default(),
        }
    }
}

impl Node for ExpressionStatement {
    fn string(&self) -> String {
        format!("{};", self.expression.string())
    }
    
    fn token_literal(&self) -> String {
        self.expression.token_literal()
    }
}

impl Statement for ExpressionStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Option<Box<dyn Expression>>,
    pub location: SourceLocation,
}

impl ReturnStatement {
    pub fn new(value: Option<Box<dyn Expression>>) -> Self {
        Self {
            value,
            location: SourceLocation::default(),
        }
    }
}

impl Node for ReturnStatement {
    fn string(&self) -> String {
        match &self.value {
            Some(expr) => format!("return {};", expr.string()),
            None => "return;".to_string(),
        }
    }
    
    fn token_literal(&self) -> String {
        "return".to_string()
    }
}

impl Statement for ReturnStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct BreakStatement {
    pub location: SourceLocation,
}

impl BreakStatement {
    pub fn new() -> Self {
        Self {
            location: SourceLocation::default(),
        }
    }
}

impl Node for BreakStatement {
    fn string(&self) -> String {
        "break;".to_string()
    }
    
    fn token_literal(&self) -> String {
        "break".to_string()
    }
}

impl Statement for BreakStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct ContinueStatement {
    pub location: SourceLocation,
}

impl ContinueStatement {
    pub fn new() -> Self {
        Self {
            location: SourceLocation::default(),
        }
    }
}

impl Node for ContinueStatement {
    fn string(&self) -> String {
        "continue;".to_string()
    }
    
    fn token_literal(&self) -> String {
        "continue".to_string()
    }
}

impl Statement for ContinueStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct ThrowStatement {
    pub expression: Box<dyn Expression>,
    pub location: SourceLocation,
}

impl ThrowStatement {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self {
            expression,
            location: SourceLocation::default(),
        }
    }
}

impl Node for ThrowStatement {
    fn string(&self) -> String {
        format!("throw {};", self.expression.string())
    }
    
    fn token_literal(&self) -> String {
        "throw".to_string()
    }
}

impl Statement for ThrowStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct TryStatement {
    pub body: Vec<Box<dyn Statement>>,
    pub catch_clause: Option<CatchStatement>,
    pub finally_clause: Option<FinallyStatement>,
    pub location: SourceLocation,
}

impl TryStatement {
    pub fn new(body: Vec<Box<dyn Statement>>) -> Self {
        Self {
            body,
            catch_clause: None,
            finally_clause: None,
            location: SourceLocation::default(),
        }
    }
}

impl Node for TryStatement {
    fn string(&self) -> String {
        let mut result = "try {\n".to_string();
        for stmt in &self.body {
            result.push_str(&format!("  {}\n", stmt.string()));
        }
        result.push('}');
        
        if let Some(catch) = &self.catch_clause {
            result.push_str(&format!(" {}", catch.string()));
        }
        
        if let Some(finally) = &self.finally_clause {
            result.push_str(&format!(" {}", finally.string()));
        }
        
        result
    }
    
    fn token_literal(&self) -> String {
        "try".to_string()
    }
}

impl Statement for TryStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct CatchStatement {
    pub parameter: Option<String>,
    pub body: Vec<Box<dyn Statement>>,
    pub location: SourceLocation,
}

impl CatchStatement {
    pub fn new(parameter: Option<String>, body: Vec<Box<dyn Statement>>) -> Self {
        Self {
            parameter,
            body,
            location: SourceLocation::default(),
        }
    }
}

impl Node for CatchStatement {
    fn string(&self) -> String {
        let mut result = "catch".to_string();
        if let Some(param) = &self.parameter {
            result.push_str(&format!(" ({})", param));
        }
        result.push_str(" {\n");
        for stmt in &self.body {
            result.push_str(&format!("  {}\n", stmt.string()));
        }
        result.push('}');
        result
    }
    
    fn token_literal(&self) -> String {
        "catch".to_string()
    }
}

impl Statement for CatchStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct FinallyStatement {
    pub body: Vec<Box<dyn Statement>>,
    pub location: SourceLocation,
}

impl FinallyStatement {
    pub fn new(body: Vec<Box<dyn Statement>>) -> Self {
        Self {
            body,
            location: SourceLocation::default(),
        }
    }
}

impl Node for FinallyStatement {
    fn string(&self) -> String {
        let mut result = "finally {\n".to_string();
        for stmt in &self.body {
            result.push_str(&format!("  {}\n", stmt.string()));
        }
        result.push('}');
        result
    }
    
    fn token_literal(&self) -> String {
        "finally".to_string()
    }
}

impl Statement for FinallyStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct PrintStatement {
    pub expression: Box<dyn Expression>,
    pub location: SourceLocation,
}

impl PrintStatement {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self {
            expression,
            location: SourceLocation::default(),
        }
    }
}

impl Node for PrintStatement {
    fn string(&self) -> String {
        format!("print({});", self.expression.string())
    }
    
    fn token_literal(&self) -> String {
        "print".to_string()
    }
}

impl Statement for PrintStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
