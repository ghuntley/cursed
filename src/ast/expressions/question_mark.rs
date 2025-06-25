use crate::ast::traits::Expression;
use std::fmt;
use std::any::Any;

/// AST node representing a question mark expression for error propagation
/// 
/// In CURSED, the `?` operator provides automatic error propagation similar to Rust.
/// It evaluates an expression and:
/// - If the result is an error, propagates it by returning early
/// - If the result is successful, unwraps and returns the inner value
/// 
/// Example: `let value = risky_operation()?;`
#[derive(Debug)]
pub struct QuestionMarkExpression {
    /// The expression to evaluate and potentially propagate errors from
    
    /// Source location information for error reporting
impl QuestionMarkExpression {
    /// Create a new question mark expression
    pub fn new(expression: Box<dyn Expression>, line: usize, column: usize) -> Self {
        Self {
        }
    }
    
    /// Get the inner expression being evaluated
    pub fn inner_expression(&self) -> &dyn Expression {
        &*self.expression
    /// Get source location for error reporting
    pub fn location(&self) -> (usize, usize) {
        (self.line, self.column)
    /// Convert to string representation for debugging
    pub fn to_string(&self) -> String {
        format!("{}?", self.expression.string())
    }
}

impl Clone for QuestionMarkExpression {
    fn clone(&self) -> Self {
        Self {
        }
    }
impl fmt::Display for QuestionMarkExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}?", self.expression.string())
    }
}

impl crate::ast::traits::Node for QuestionMarkExpression {
    fn string(&self) -> String {
        format!("{}?", self.expression.string())
    fn token_literal(&self) -> String {
        "?".to_string()
    }
}

impl Expression for QuestionMarkExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

