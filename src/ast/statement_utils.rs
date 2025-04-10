//! Utility functions for working with statements

use crate::ast::{Statement, Expression};
use crate::ast::statements::ExpressionStatement;

pub trait StatementExtensions {
    /// Get the expression from a statement if it's an expression statement
    fn expression(&self) -> Option<&Box<dyn Expression>>;
}

impl StatementExtensions for Box<dyn Statement> {
    fn expression(&self) -> Option<&Box<dyn Expression>> {
        if let Some(expr_stmt) = self.as_any().downcast_ref::<ExpressionStatement>() {
            expr_stmt.expression.as_ref()
        } else {
            None
        }
    }
}