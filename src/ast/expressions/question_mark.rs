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
    pub expression: Box<dyn Expression>,
    
    /// Source location information for error reporting
    pub line: usize,
    pub column: usize,
}

impl QuestionMarkExpression {
    /// Create a new question mark expression
    pub fn new(expression: Box<dyn Expression>, line: usize, column: usize) -> Self {
        Self {
            expression,
            line,
            column,
        }
    }
    
    /// Get the inner expression being evaluated
    pub fn inner_expression(&self) -> &dyn Expression {
        &*self.expression
    }
    
    /// Get source location for error reporting
    pub fn location(&self) -> (usize, usize) {
        (self.line, self.column)
    }
    
    /// Convert to string representation for debugging
    pub fn to_string(&self) -> String {
        format!("{}?", self.expression.string())
    }
}

impl Clone for QuestionMarkExpression {
    fn clone(&self) -> Self {
        Self {
            expression: self.expression.clone_box(),
            line: self.line,
            column: self.column,
        }
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
    }

    fn token_literal(&self) -> String {
        "?".to_string()
    }
}

impl Expression for QuestionMarkExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::identifiers::Identifier;
    use crate::ast::traits::{Node, Expression};

    #[test]
    fn test_question_mark_creation() {
        let var_expr = Identifier::new("test_var".to_string(), "test_var".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            1,
            5
        );
        
        assert_eq!(question_expr.line, 1);
        assert_eq!(question_expr.column, 5);
        assert_eq!(question_expr.location(), (1, 5));
    }
    
    #[test]
    fn test_question_mark_display() {
        let var_expr = Identifier::new("hello".to_string(), "hello".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            1,
            7
        );
        
        assert_eq!(question_expr.to_string(), "hello?");
        assert_eq!(format!("{}", question_expr), "hello?");
    }
    
    #[test]
    fn test_question_mark_nested() {
        let var_expr = Identifier::new("result".to_string(), "result".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            1,
            7
        );
        
        // Test nested question mark (though this would be unusual)
        let nested_question = QuestionMarkExpression::new(
            Box::new(question_expr),
            1,
            8
        );
        
        assert_eq!(nested_question.to_string(), "result??");
    }
    
    #[test]
    fn test_inner_expression_access() {
        let var_expr = Identifier::new("value".to_string(), "value".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            2,
            8
        );
        
        let inner = question_expr.inner_expression();
        assert_eq!(inner.string(), "value");
    }
    
    #[test]
    fn test_question_mark_expression_trait() {
        let var_expr = Identifier::new("test".to_string(), "test".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            1,
            5
        );
        
        // Test that it implements Expression trait
        let expr: Box<dyn Expression> = Box::new(question_expr);
        assert_eq!(expr.string(), "test?");
    }
    
    #[test]
    fn test_question_mark_cloning() {
        let var_expr = Identifier::new("test".to_string(), "test".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(var_expr),
            1,
            5
        );
        
        let cloned = question_expr.clone();
        assert_eq!(cloned.string(), question_expr.string());
        assert_eq!(cloned.location(), question_expr.location());
    }
}
