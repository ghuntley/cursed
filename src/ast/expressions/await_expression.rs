/// Await expression AST node for the CURSED programming language
/// 
/// Represents await expressions with the syntax:
/// await expression

use crate::ast::traits::{Node, Expression};
use std::any::Any;

/// Await expression (await expression)
#[derive(Debug, Clone)]
pub struct AwaitExpression {
    pub token: String,
    pub expression: Box<dyn Expression>,
    pub source_location: Option<SourceLocation>,
}

/// Source location information for better error reporting
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

impl AwaitExpression {
    pub fn new(token: String, expression: Box<dyn Expression>) -> Self {
        Self {
            token,
            expression,
            source_location: None,
        }
    }

    pub fn with_location(
        token: String, 
        expression: Box<dyn Expression>,
        location: SourceLocation
    ) -> Self {
        Self {
            token,
            expression,
            source_location: Some(location),
        }
    }

    /// Check if the awaited expression is a function call
    pub fn is_function_call(&self) -> bool {
        // This would need to be implemented based on the actual Expression trait
        // For now, we check the string representation
        let expr_str = self.expression.string();
        expr_str.contains('(') && expr_str.contains(')')
    }

    /// Check if the awaited expression is an identifier
    pub fn is_identifier(&self) -> bool {
        // Check if the expression is just an identifier (no parentheses or operators)
        let expr_str = self.expression.string();
        !expr_str.contains('(') && !expr_str.contains('[') && !expr_str.contains('.')
    }

    /// Get the base expression type for type checking
    pub fn get_expression_type(&self) -> String {
        // This would be implemented with proper type checking
        // For now, return a placeholder
        "Promise<T>".to_string()
    }
}

impl Node for AwaitExpression {
    fn string(&self) -> String {
        format!("await {}", self.expression.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for AwaitExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(AwaitExpression {
            token: self.token.clone(),
            expression: self.expression.clone_box(),
            source_location: self.source_location.clone(),
        })
    }
}

/// Await expression in assignment context
#[derive(Debug, Clone)]
pub struct AwaitAssignment {
    pub token: String,
    pub variable_name: String,
    pub await_expression: AwaitExpression,
    pub variable_type: Option<String>,
}

impl AwaitAssignment {
    pub fn new(
        token: String,
        variable_name: String,
        await_expression: AwaitExpression,
        variable_type: Option<String>,
    ) -> Self {
        Self {
            token,
            variable_name,
            await_expression,
            variable_type,
        }
    }
}

impl Node for AwaitAssignment {
    fn string(&self) -> String {
        match &self.variable_type {
            Some(var_type) => format!("facts {} {} = {}", 
                self.variable_name, 
                var_type, 
                self.await_expression.string()
            ),
            None => format!("facts {} = {}", 
                self.variable_name, 
                self.await_expression.string()
            ),
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for AwaitAssignment {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(AwaitAssignment {
            token: self.token.clone(),
            variable_name: self.variable_name.clone(),
            await_expression: self.await_expression.clone(),
            variable_type: self.variable_type.clone(),
        })
    }
}

/// Async block expression for grouped async operations
#[derive(Debug, Clone)]
pub struct AsyncBlockExpression {
    pub token: String,
    pub statements: Vec<Box<dyn Expression>>,
    pub await_expressions: Vec<AwaitExpression>,
}

impl AsyncBlockExpression {
    pub fn new(token: String) -> Self {
        Self {
            token,
            statements: Vec::new(),
            await_expressions: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: Box<dyn Expression>) {
        self.statements.push(statement);
    }

    pub fn add_await(&mut self, await_expr: AwaitExpression) {
        self.await_expressions.push(await_expr);
    }

    /// Check if this block contains any await expressions
    pub fn has_awaits(&self) -> bool {
        !self.await_expressions.is_empty()
    }

    /// Get the number of await expressions in this block
    pub fn await_count(&self) -> usize {
        self.await_expressions.len()
    }
}

impl Node for AsyncBlockExpression {
    fn string(&self) -> String {
        let mut result = String::from("async {\n");
        
        for stmt in &self.statements {
            result.push_str(&format!("  {}\n", stmt.string()));
        }
        
        for await_expr in &self.await_expressions {
            result.push_str(&format!("  {}\n", await_expr.string()));
        }
        
        result.push('}');
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for AsyncBlockExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(AsyncBlockExpression {
            token: self.token.clone(),
            statements: self.statements.iter().map(|s| s.clone_box()).collect(),
            await_expressions: self.await_expressions.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::identifiers::Identifier;

    #[test]
    fn test_await_expression_creation() {
        let identifier = Box::new(Identifier::new("async_call".to_string(), "async_call".to_string()));
        let await_expr = AwaitExpression::new("await".to_string(), identifier);

        assert_eq!(await_expr.token, "await");
        assert!(await_expr.source_location.is_none());
    }

    #[test]
    fn test_await_expression_string_representation() {
        let identifier = Box::new(Identifier::new("fetch_data".to_string(), "fetch_data".to_string()));
        let await_expr = AwaitExpression::new("await".to_string(), identifier);

        let string_repr = await_expr.string();
        assert_eq!(string_repr, "await fetch_data");
    }

    #[test]
    fn test_await_with_location() {
        let identifier = Box::new(Identifier::new("test".to_string(), "test".to_string()));
        let location = SourceLocation {
            line: 10,
            column: 5,
            file: Some("test.csd".to_string()),
        };
        
        let await_expr = AwaitExpression::with_location("await".to_string(), identifier, location);
        
        assert!(await_expr.source_location.is_some());
        let loc = await_expr.source_location.unwrap();
        assert_eq!(loc.line, 10);
        assert_eq!(loc.column, 5);
    }

    #[test]
    fn test_await_assignment() {
        let identifier = Box::new(Identifier::new("api_call".to_string(), "api_call".to_string()));
        let await_expr = AwaitExpression::new("await".to_string(), identifier);
        
        let assignment = AwaitAssignment::new(
            "facts".to_string(),
            "result".to_string(),
            await_expr,
            Some("String".to_string()),
        );

        let string_repr = assignment.string();
        assert!(string_repr.contains("facts result String = await api_call"));
    }

    #[test]
    fn test_async_block_expression() {
        let mut async_block = AsyncBlockExpression::new("async".to_string());
        
        assert!(!async_block.has_awaits());
        assert_eq!(async_block.await_count(), 0);

        let identifier = Box::new(Identifier::new("test".to_string(), "test".to_string()));
        let await_expr = AwaitExpression::new("await".to_string(), identifier);
        async_block.add_await(await_expr);

        assert!(async_block.has_awaits());
        assert_eq!(async_block.await_count(), 1);
    }

    #[test]
    fn test_expression_type_detection() {
        let identifier = Box::new(Identifier::new("simple_var".to_string(), "simple_var".to_string()));
        let await_expr = AwaitExpression::new("await".to_string(), identifier);

        assert!(await_expr.is_identifier());
        assert!(!await_expr.is_function_call());
    }

    #[test]
    fn test_await_expression_cloning() {
        let identifier = Box::new(Identifier::new("clone_test".to_string(), "clone_test".to_string()));
        let await_expr = AwaitExpression::new("await".to_string(), identifier);

        let cloned = await_expr.clone();
        assert_eq!(await_expr.token, cloned.token);
        assert_eq!(await_expr.expression.string(), cloned.expression.string());
    }
}
