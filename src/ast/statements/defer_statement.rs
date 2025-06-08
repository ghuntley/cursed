//! Defer statement for deferred execution.
//!
//! This module defines the AST node for a defer statement, which is used
//! to defer the execution of a statement until the function returns.
//! Deferred statements execute in LIFO (last-in-first-out) order.

use crate::ast::traits::{Node, Statement, Expression};
use crate::lexer::Token;
use std::any::Any;

/// A defer statement that defers execution until function return
pub struct DeferStatement {
    pub token: Token,
    pub statement: Box<dyn Statement>,
}

impl Node for DeferStatement {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
    
    fn string(&self) -> String {
        format!("later {};", self.statement.string())
    }
}

impl Statement for DeferStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::statements::ExpressionStatement;
    use crate::ast::expressions::Identifier;
    
    #[test]
    fn test_defer_statement_string() {
        let identifier = Identifier {
            token: Token::Identifier("test".to_string()),
            value: "test".to_string(),
        };
        
        let expr_stmt = ExpressionStatement {
            token: Token::Identifier("test".to_string()),
            expression: Some(Box::new(identifier)),
        };
        
        let defer_stmt = DeferStatement {
            token: Token::Later,
            statement: Box::new(expr_stmt),
        };
        
        assert_eq!(defer_stmt.string(), "later test;");
        assert_eq!(defer_stmt.token_literal(), "later");
    }
}
