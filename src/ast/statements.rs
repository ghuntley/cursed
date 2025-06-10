/// Statement AST nodes for the CURSED programming language
/// 
/// This module contains all statement types that represent actions
/// rather than values, using Gen Z slang keywords.

use crate::ast::traits::{Node, Statement, Expression};
use crate::ast::identifiers::Identifier;
use crate::lexer::Token;
use std::any::Any;

/// Variable declaration statement (sus x = 5)
#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: String,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
    pub type_annotation: Option<Box<dyn Expression>>,
}

impl LetStatement {
    pub fn new(token: String, name: Identifier, value: Option<Box<dyn Expression>>) -> Self {
        Self {
            token,
            name,
            value,
            type_annotation: None,
        }
    }
    
    pub fn with_type(
        token: String,
        name: Identifier,
        type_annotation: Box<dyn Expression>,
        value: Option<Box<dyn Expression>>,
    ) -> Self {
        Self {
            token,
            name,
            value,
            type_annotation: Some(type_annotation),
        }
    }
}

impl Node for LetStatement {
    fn string(&self) -> String {
        let mut result = format!("sus {}", self.name.string());
        
        if let Some(type_ann) = &self.type_annotation {
            result.push_str(&format!(" {}", type_ann.string()));
        }
        
        if let Some(value) = &self.value {
            result.push_str(&format!(" = {}", value.string()));
        }
        
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for LetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(LetStatement {
            token: self.token.clone(),
            name: self.name.clone(),
            value: self.value.as_ref().map(|v| v.clone_box()),
            type_annotation: self.type_annotation.as_ref().map(|t| t.clone_box()),
        })
    }
}

/// Constant declaration statement (facts PI = 3.14159)
#[derive(Debug, Clone)]
pub struct FactsStatement {
    pub token: String,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
    pub type_annotation: Option<Box<dyn Expression>>,
}

impl FactsStatement {
    pub fn new(token: String, name: Identifier, value: Box<dyn Expression>) -> Self {
        Self {
            token,
            name,
            value,
            type_annotation: None,
        }
    }
    
    pub fn with_type(
        token: String,
        name: Identifier,
        type_annotation: Box<dyn Expression>,
        value: Box<dyn Expression>,
    ) -> Self {
        Self {
            token,
            name,
            value,
            type_annotation: Some(type_annotation),
        }
    }
}


impl Node for FactsStatement {
    fn string(&self) -> String {
        let mut result = format!("facts {}", self.name.string());
        
        if let Some(type_ann) = &self.type_annotation {
            result.push_str(&format!(" {}", type_ann.string()));
        }
        
        result.push_str(&format!(" = {}", self.value.string()));
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}


impl Statement for FactsStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(FactsStatement {
            token: self.token.clone(),
            name: self.name.clone(),
            value: self.value.clone_box(),
            type_annotation: self.type_annotation.as_ref().map(|t| t.clone_box()),
        })
    }
}

/// Return statement (yolo expression)
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: String,
    pub return_value: Option<Box<dyn Expression>>,
}

impl ReturnStatement {
    pub fn new(token: String, return_value: Option<Box<dyn Expression>>) -> Self {
        Self { token, return_value }
    }
    
    pub fn empty() -> Self {
        Self {
            token: "yolo".to_string(),
            return_value: None,
        }
    }
    
    pub fn with_value(value: Box<dyn Expression>) -> Self {
        Self {
            token: "yolo".to_string(),
            return_value: Some(value),
        }
    }
}


impl Node for ReturnStatement {
    fn string(&self) -> String {
        if let Some(rv) = &self.return_value {
            format!("yolo {}", rv.string())
        } else {
            "yolo".to_string()
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}


impl Statement for ReturnStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ReturnStatement {
            token: self.token.clone(),
            return_value: self.return_value.as_ref().map(|v| v.clone_box()),
        })
    }
}

/// Expression statement (standalone expressions)
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: String,
    pub expression: Box<dyn Expression>,
}

impl ExpressionStatement {
    pub fn new(token: String, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
    
    pub fn from_expr(expression: Box<dyn Expression>) -> Self {
        let token = expression.token_literal();
        Self { token, expression }
    }
}


impl Node for ExpressionStatement {
    fn string(&self) -> String {
        self.expression.string()
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}


impl Statement for ExpressionStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ExpressionStatement {
            token: self.token.clone(),
            expression: self.expression.clone_box(),
        })
    }
}

/// Break statement (ghosted)
#[derive(Debug, Clone)]
pub struct BreakStatement {
    pub token: String,
    pub label: Option<String>,
}

impl BreakStatement {
    pub fn new() -> Self {
        Self {
            token: "ghosted".to_string(),
            label: None,
        }
    }
    
    pub fn with_label(label: String) -> Self {
        Self {
            token: "ghosted".to_string(),
            label: Some(label),
        }
    }
}


impl Node for BreakStatement {
    fn string(&self) -> String {
        if let Some(label) = &self.label {
            format!("ghosted {}", label)
        } else {
            "ghosted".to_string()
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
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


impl Default for BreakStatement {
    fn default() -> Self {
        Self::new()
    }
}

/// Continue statement (simp)
#[derive(Debug, Clone)]
pub struct ContinueStatement {
    pub token: String,
    pub label: Option<String>,
}

impl ContinueStatement {
    pub fn new() -> Self {
        Self {
            token: "simp".to_string(),
            label: None,
        }
    }
    
    pub fn with_label(label: String) -> Self {
        Self {
            token: "simp".to_string(),
            label: Some(label),
        }
    }
}

impl Node for ContinueStatement {
    fn string(&self) -> String {
        if let Some(label) = &self.label {
            format!("simp {}", label)
        } else {
            "simp".to_string()
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
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


impl Default for ContinueStatement {
    fn default() -> Self {
        Self::new()
    }
}

/// Defer statement (later expression)
#[derive(Debug, Clone)]
pub struct DeferStatement {
    pub token: String,
    pub call: Box<dyn Expression>,
}

impl DeferStatement {
    pub fn new(token: String, call: Box<dyn Expression>) -> Self {
        Self { token, call }
    }
    
    pub fn defer_call(call: Box<dyn Expression>) -> Self {
        Self {
            token: "later".to_string(),
            call,
        }
    }
}

impl Node for DeferStatement {
    fn string(&self) -> String {
        format!("later {}", self.call.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}


impl Statement for DeferStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(DeferStatement {
            token: self.token.clone(),
            call: self.call.clone_box(),
        })
    }
}

/// Print statement for debugging and output
#[derive(Debug, Clone)]
pub struct PrintStatement {
    pub token: String,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl PrintStatement {
    pub fn new(token: String, arguments: Vec<Box<dyn Expression>>) -> Self {
        Self { token, arguments }
    }
    
    pub fn print_expr(expr: Box<dyn Expression>) -> Self {
        Self {
            token: "print".to_string(),
            arguments: vec![expr],
        }
    }
}

impl Node for PrintStatement {
    fn string(&self) -> String {
        let args: Vec<String> = self.arguments.iter()
            .map(|arg| arg.string())
            .collect();
        format!("print({})", args.join(", "))
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}


impl Statement for PrintStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(PrintStatement {
            token: self.token.clone(),
            arguments: self.arguments.iter().map(|arg| arg.clone_box()).collect(),
        })
    }
}

/// Assignment statement as a statement (not expression)
#[derive(Debug, Clone)]
pub struct AssignmentStatement {
    pub token: String,
    pub name: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
}

impl AssignmentStatement {
    pub fn new(token: String, name: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        Self { token, name, value }
    }
}

impl Node for AssignmentStatement {
    fn string(&self) -> String {
        format!("{} = {}", self.name.string(), self.value.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}


impl Statement for AssignmentStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(AssignmentStatement {
            token: self.token.clone(),
            name: self.name.clone_box(),
            value: self.value.clone_box(),
        })
    }
}

/// Panic statement for error throwing (yeet_error message)
#[derive(Debug, Clone)]
pub struct PanicStatement {
    pub token: String,
    pub message: Box<dyn Expression>,
}

impl PanicStatement {
    pub fn new(token: String, message: Box<dyn Expression>) -> Self {
        Self { token, message }
    }
}

impl Node for PanicStatement {
    fn string(&self) -> String {
        format!("yeet_error {}", self.message.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}


impl Statement for PanicStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(PanicStatement {
            token: self.token.clone(),
            message: self.message.clone_box(),
        })
    }
}

/// Recovery statement for error catching (catch { ... })
#[derive(Debug, Clone)]
pub struct RecoveryStatement {
    pub token: String,
    pub protected_block: Box<dyn Statement>,
    pub recovery_block: Option<Box<dyn Statement>>,
    pub error_variable: Option<Identifier>,
}

impl RecoveryStatement {
    pub fn new(token: String, protected_block: Box<dyn Statement>) -> Self {
        Self {
            token,
            protected_block,
            recovery_block: None,
            error_variable: None,
        }
    }
    
    pub fn with_recovery(mut self, recovery_block: Box<dyn Statement>) -> Self {
        self.recovery_block = Some(recovery_block);
        self
    }
    
    pub fn with_error_var(mut self, error_variable: Identifier) -> Self {
        self.error_variable = Some(error_variable);
        self
    }
}


impl Node for RecoveryStatement {
    fn string(&self) -> String {
        let mut result = format!("catch {{ {} }}", self.protected_block.string());
        if let Some(recovery) = &self.recovery_block {
            result.push_str(&format!(" recover {{ {} }}", recovery.string()));
        }
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}


impl Statement for RecoveryStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(RecoveryStatement {
            token: self.token.clone(),
            protected_block: self.protected_block.clone_box(),
            recovery_block: self.recovery_block.as_ref().map(|r| r.clone_box()),
            error_variable: self.error_variable.clone(),
        })
    }
}

/// Helper functions for creating statement nodes
pub fn let_stmt(name: &str, value: Option<Box<dyn Expression>>) -> LetStatement {
    LetStatement::new(
        "sus".to_string(),
        crate::ast::identifiers::Identifier::from_name(name),
        value,
    )
}

pub fn return_stmt(value: Option<Box<dyn Expression>>) -> ReturnStatement {
    ReturnStatement::new("yolo".to_string(), value)
}

pub fn expr_stmt(expression: Box<dyn Expression>) -> ExpressionStatement {
    ExpressionStatement::from_expr(expression)
}

pub fn panic_stmt(message: Box<dyn Expression>) -> PanicStatement {
    PanicStatement::new("yeet_error".to_string(), message)
}

pub fn recovery_stmt(protected_block: Box<dyn Statement>) -> RecoveryStatement {
    RecoveryStatement::new("catch".to_string(), protected_block)
}

// Control flow statements module
pub mod control_flow;
