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
        let mut result = format!("sus {}", self.to_string().string());
        
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
            name: self.to_string().clone(),
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
        let mut result = format!("facts {}", self.to_string().string());
        
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
            name: self.to_string().clone(),
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
        format!("{} = {}", self.to_string().string(), self.value.string())
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
            name: self.to_string().clone_box(),
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

/// Throw statement for error throwing (yeet expression)
#[derive(Debug, Clone)]
pub struct ThrowStatement {
    pub token: String,
    pub exception: Box<dyn Expression>,
}

impl ThrowStatement {
    pub fn new(token: String, exception: Box<dyn Expression>) -> Self {
        Self { token, exception }
    }
    
    pub fn throw_expr(exception: Box<dyn Expression>) -> Self {
        Self {
            token: "yeet".to_string(),
            exception,
        }
    }
}

impl Node for ThrowStatement {
    fn string(&self) -> String {
        format!("yeet {}", self.exception.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ThrowStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ThrowStatement {
            token: self.token.clone(),
            exception: self.exception.clone_box(),
        })
    }
}

/// Try statement for error handling (bet { ... })
#[derive(Debug, Clone)]
pub struct TryStatement {
    pub token: String,
    pub body: Box<dyn Statement>,
    pub catch_clauses: Vec<CatchStatement>,
    pub finally_clause: Option<FinallyStatement>,
}

impl TryStatement {
    pub fn new(token: String, body: Box<dyn Statement>) -> Self {
        Self {
            token,
            body,
            catch_clauses: Vec::new(),
            finally_clause: None,
        }
    }
    
    pub fn try_block(body: Box<dyn Statement>) -> Self {
        Self {
            token: "bet".to_string(),
            body,
            catch_clauses: Vec::new(),
            finally_clause: None,
        }
    }
    
    pub fn with_catch(mut self, catch_clause: CatchStatement) -> Self {
        self.catch_clauses.push(catch_clause);
        self
    }
    
    pub fn with_finally(mut self, finally_clause: FinallyStatement) -> Self {
        self.finally_clause = Some(finally_clause);
        self
    }
}

impl Node for TryStatement {
    fn string(&self) -> String {
        let mut result = format!("bet {{ {} }}", self.body.string());
        
        for catch in &self.catch_clauses {
            result.push_str(&format!(" {}", catch.string()));
        }
        
        if let Some(finally) = &self.finally_clause {
            result.push_str(&format!(" {}", finally.string()));
        }
        
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for TryStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(TryStatement {
            token: self.token.clone(),
            body: self.body.clone_box(),
            catch_clauses: self.catch_clauses.clone(),
            finally_clause: self.finally_clause.clone(),
        })
    }
}

/// Catch statement for error handling (sus error_var { ... })
#[derive(Debug, Clone)]
pub struct CatchStatement {
    pub token: String,
    pub error_variable: Option<Identifier>,
    pub error_type: Option<Box<dyn Expression>>,
    pub body: Box<dyn Statement>,
}

impl CatchStatement {
    pub fn new(token: String, body: Box<dyn Statement>) -> Self {
        Self {
            token,
            error_variable: None,
            error_type: None,
            body,
        }
    }
    
    pub fn catch_block(body: Box<dyn Statement>) -> Self {
        Self {
            token: "sus".to_string(),
            error_variable: None,
            error_type: None,
            body,
        }
    }
    
    pub fn with_error_var(mut self, error_variable: Identifier) -> Self {
        self.error_variable = Some(error_variable);
        self
    }
    
    pub fn with_error_type(mut self, error_type: Box<dyn Expression>) -> Self {
        self.error_type = Some(error_type);
        self
    }
}

impl Node for CatchStatement {
    fn string(&self) -> String {
        let mut result = "sus".to_string();
        
        if let Some(var) = &self.error_variable {
            result.push_str(&format!(" {}", var.string()));
        }
        
        if let Some(error_type) = &self.error_type {
            result.push_str(&format!(" {}", error_type.string()));
        }
        
        result.push_str(&format!(" {{ {} }}", self.body.string()));
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for CatchStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(CatchStatement {
            token: self.token.clone(),
            error_variable: self.error_variable.clone(),
            error_type: self.error_type.as_ref().map(|t| t.clone_box()),
            body: self.body.clone_box(),
        })
    }
}

/// Finally statement for cleanup (periodt { ... })
#[derive(Debug, Clone)]
pub struct FinallyStatement {
    pub token: String,
    pub body: Box<dyn Statement>,
}

impl FinallyStatement {
    pub fn new(token: String, body: Box<dyn Statement>) -> Self {
        Self { token, body }
    }
    
    pub fn finally_block(body: Box<dyn Statement>) -> Self {
        Self {
            token: "periodt".to_string(),
            body,
        }
    }
}

impl Node for FinallyStatement {
    fn string(&self) -> String {
        format!("periodt {{ {} }}", self.body.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for FinallyStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(FinallyStatement {
            token: self.token.clone(),
            body: self.body.clone_box(),
        })
    }
}

/// Import statement for module imports (slurp "module")
#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub token: String,
    pub module_path: String,
    pub alias: Option<String>,
    pub items: Option<Vec<String>>,
}

impl ImportStatement {
    pub fn new(token: String, module_path: String) -> Self {
        Self {
            token,
            module_path,
            alias: None,
            items: None,
        }
    }
    
    pub fn import_module(module_path: String) -> Self {
        Self {
            token: "slurp".to_string(),
            module_path,
            alias: None,
            items: None,
        }
    }
    
    pub fn with_alias(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
    }
    
    pub fn with_items(mut self, items: Vec<String>) -> Self {
        self.items = Some(items);
        self
    }
}

impl Node for ImportStatement {
    fn string(&self) -> String {
        let mut result = format!("slurp \"{}\"", self.module_path);
        
        if let Some(alias) = &self.alias {
            result.push_str(&format!(" as {}", alias));
        }
        
        if let Some(items) = &self.items {
            result.push_str(&format!(" {{ {} }}", items.join(", ")));
        }
        
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ImportStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ImportStatement {
            token: self.token.clone(),
            module_path: self.module_path.clone(),
            alias: self.alias.clone(),
            items: self.items.clone(),
        })
    }
}

/// Package statement for package declaration (package "name")
#[derive(Debug, Clone)]
pub struct PackageStatement {
    pub token: String,
    pub name: String,
}

impl PackageStatement {
    pub fn new(token: String, name: String) -> Self {
        Self { token, name }
    }
    
    pub fn package_decl(name: String) -> Self {
        Self {
            token: "package".to_string(),
            name,
        }
    }
}

impl Node for PackageStatement {
    fn string(&self) -> String {
        format!("package \"{}\"", self.to_string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for PackageStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(PackageStatement {
            token: self.token.clone(),
            name: self.to_string().clone(),
        })
    }
}

/// Mutable variable declaration statement (flex x = 5)
#[derive(Debug, Clone)]
pub struct MutStatement {
    pub token: String,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
    pub type_annotation: Option<Box<dyn Expression>>,
}

impl MutStatement {
    pub fn new(token: String, name: Identifier, value: Option<Box<dyn Expression>>) -> Self {
        Self {
            token,
            name,
            value,
            type_annotation: None,
        }
    }
    
    pub fn mut_var(name: Identifier, value: Option<Box<dyn Expression>>) -> Self {
        Self {
            token: "flex".to_string(),
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

impl Node for MutStatement {
    fn string(&self) -> String {
        let mut result = format!("flex {}", self.to_string().string());
        
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

impl Statement for MutStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(MutStatement {
            token: self.token.clone(),
            name: self.to_string().clone(),
            value: self.value.as_ref().map(|v| v.clone_box()),
            type_annotation: self.type_annotation.as_ref().map(|t| t.clone_box()),
        })
    }
}

/// Constant declaration statement (no_cap NAME = value)
#[derive(Debug, Clone)]
pub struct ConstStatement {
    pub token: String,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
    pub type_annotation: Option<Box<dyn Expression>>,
}

impl ConstStatement {
    pub fn new(token: String, name: Identifier, value: Box<dyn Expression>) -> Self {
        Self {
            token,
            name,
            value,
            type_annotation: None,
        }
    }
    
    pub fn const_decl(name: Identifier, value: Box<dyn Expression>) -> Self {
        Self {
            token: "no_cap".to_string(),
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

impl Node for ConstStatement {
    fn string(&self) -> String {
        let mut result = format!("no_cap {}", self.to_string().string());
        
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

impl Statement for ConstStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ConstStatement {
            token: self.token.clone(),
            name: self.to_string().clone(),
            value: self.value.clone_box(),
            type_annotation: self.type_annotation.as_ref().map(|t| t.clone_box()),
        })
    }
}

/// Channel receive statement (chan <- value)
#[derive(Debug, Clone)]
pub struct ChannelReceiveStatement {
    pub token: String,
    pub channel: Box<dyn Expression>,
    pub variable: Option<Identifier>,
}

impl ChannelReceiveStatement {
    pub fn new(token: String, channel: Box<dyn Expression>) -> Self {
        Self {
            token,
            channel,
            variable: None,
        }
    }
    
    pub fn receive_from(channel: Box<dyn Expression>) -> Self {
        Self {
            token: "<-".to_string(),
            channel,
            variable: None,
        }
    }
    
    pub fn with_variable(mut self, variable: Identifier) -> Self {
        self.variable = Some(variable);
        self
    }
}

impl Node for ChannelReceiveStatement {
    fn string(&self) -> String {
        if let Some(var) = &self.variable {
            format!("{} <- {}", var.string(), self.channel.string())
        } else {
            format!("<-{}", self.channel.string())
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ChannelReceiveStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ChannelReceiveStatement {
            token: self.token.clone(),
            channel: self.channel.clone_box(),
            variable: self.variable.clone(),
        })
    }
}

/// Channel send statement (chan -> value)
#[derive(Debug, Clone)]
pub struct ChannelSendStatement {
    pub token: String,
    pub channel: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
}

impl ChannelSendStatement {
    pub fn new(token: String, channel: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        Self { token, channel, value }
    }
    
    pub fn send_to(channel: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        Self {
            token: "->".to_string(),
            channel,
            value,
        }
    }
}

impl Node for ChannelSendStatement {
    fn string(&self) -> String {
        format!("{} -> {}", self.channel.string(), self.value.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ChannelSendStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ChannelSendStatement {
            token: self.token.clone(),
            channel: self.channel.clone_box(),
            value: self.value.clone_box(),
        })
    }
}

/// Channel close statement (close chan)
#[derive(Debug, Clone)]
pub struct ChannelCloseStatement {
    pub token: String,
    pub channel: Box<dyn Expression>,
}

impl ChannelCloseStatement {
    pub fn new(token: String, channel: Box<dyn Expression>) -> Self {
        Self { token, channel }
    }
    
    pub fn close_channel(channel: Box<dyn Expression>) -> Self {
        Self {
            token: "close".to_string(),
            channel,
        }
    }
}

impl Node for ChannelCloseStatement {
    fn string(&self) -> String {
        format!("close {}", self.channel.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Statement for ChannelCloseStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ChannelCloseStatement {
            token: self.token.clone(),
            channel: self.channel.clone_box(),
        })
    }
}

/// Helper functions for creating statement nodes
pub fn throw_stmt(exception: Box<dyn Expression>) -> ThrowStatement {
    ThrowStatement::throw_expr(exception)
}

pub fn try_stmt(body: Box<dyn Statement>) -> TryStatement {
    TryStatement::try_block(body)
}

pub fn catch_stmt(body: Box<dyn Statement>) -> CatchStatement {
    CatchStatement::catch_block(body)
}

pub fn finally_stmt(body: Box<dyn Statement>) -> FinallyStatement {
    FinallyStatement::finally_block(body)
}

pub fn import_stmt(module_path: &str) -> ImportStatement {
    ImportStatement::import_module(module_path.to_string())
}

pub fn package_stmt(name: &str) -> PackageStatement {
    PackageStatement::package_decl(name.to_string())
}

pub fn mut_stmt(name: &str, value: Option<Box<dyn Expression>>) -> MutStatement {
    MutStatement::mut_var(
        crate::ast::identifiers::Identifier::from_name(name),
        value,
    )
}

pub fn const_stmt(name: &str, value: Box<dyn Expression>) -> ConstStatement {
    ConstStatement::const_decl(
        crate::ast::identifiers::Identifier::from_name(name),
        value,
    )
}

// Control flow statements module
pub mod control_flow;
pub mod variable;

// Re-export control flow statements for convenience
pub use control_flow::*;
pub use variable::*;
