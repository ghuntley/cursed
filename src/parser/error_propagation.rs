//! Error propagation AST nodes and parsing
//!
//! This module defines the AST structures for enhanced error propagation
//! expressions including question mark operators, type assertions, and recovery.

use crate::ast::traits::Expression;
use std::fmt;

/// Enhanced question mark expression with optional error recovery
#[derive(Debug, Clone)]
pub struct EnhancedQuestionMarkExpression {
    pub expression: Box<dyn Expression>,
    pub error_recovery: Option<Box<dyn Expression>>,
    pub source_location: Option<String>,
}

impl fmt::Display for EnhancedQuestionMarkExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}?", self.expression)?;
        if let Some(recovery) = &self.error_recovery {
            write!(f, " or {}", recovery)?;
        }
        Ok(())
    }
}

impl Expression for EnhancedQuestionMarkExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Typed error propagation with specific error type expectations
#[derive(Debug, Clone)]
pub struct TypedErrorPropagation {
    pub expression: Box<dyn Expression>,
    pub expected_error_type: String,
    pub conversion_logic: Option<Box<dyn Expression>>,
}

impl fmt::Display for TypedErrorPropagation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}?<{}>", self.expression, self.expected_error_type)
    }
}

impl Expression for TypedErrorPropagation {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Unwrap-or expression for providing default values
#[derive(Debug, Clone)]
pub struct UnwrapOrExpression {
    pub base: Box<dyn Expression>,
    pub default_value: Box<dyn Expression>,
    pub method_name: String,
}

impl fmt::Display for UnwrapOrExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}({})", self.base, self.method_name, self.default_value)
    }
}

impl Expression for UnwrapOrExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Try-catch expression with optional finally block
#[derive(Debug, Clone)]
pub struct TryExpression {
    pub try_block: Box<dyn Expression>,
    pub catch_block: Option<Box<dyn Expression>>,
    pub finally_block: Option<Box<dyn Expression>>,
}

impl fmt::Display for TryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "try {}", self.try_block)?;
        if let Some(catch) = &self.catch_block {
            write!(f, " catch {}", catch)?;
        }
        if let Some(finally) = &self.finally_block {
            write!(f, " finally {}", finally)?;
        }
        Ok(())
    }
}

impl Expression for TryExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Field access expression with optional safe access
#[derive(Debug, Clone)]
pub struct FieldAccessExpression {
    pub base: Box<dyn Expression>,
    pub field_name: String,
    pub safe_access: bool,
}

impl fmt::Display for FieldAccessExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.safe_access {
            write!(f, "{}?.{}", self.base, self.field_name)
        } else {
            write!(f, "{}.{}", self.base, self.field_name)
        }
    }
}

impl Expression for FieldAccessExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Method call expression with type arguments
#[derive(Debug, Clone)]
pub struct MethodCallExpression {
    pub receiver: Box<dyn Expression>,
    pub method_name: String,
    pub arguments: Vec<Box<dyn Expression>>,
    pub type_arguments: Vec<String>,
}

impl fmt::Display for MethodCallExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.receiver, self.method_name)?;
        
        if !self.type_arguments.is_empty() {
            write!(f, "<{}>", self.type_arguments.join(", "))?;
        }
        
        write!(f, "(")?;
        for (i, arg) in self.arguments.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ")")
    }
}

impl Expression for MethodCallExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Error propagation parser functions
pub struct ErrorPropagationParser;

impl ErrorPropagationParser {
    /// Parse enhanced question mark expression
    pub fn parse_enhanced_question_mark(
        expression: Box<dyn Expression>,
        error_recovery: Option<Box<dyn Expression>>,
        source_location: Option<String>,
    ) -> EnhancedQuestionMarkExpression {
        EnhancedQuestionMarkExpression {
            expression,
            error_recovery,
            source_location,
        }
    }

    /// Parse typed error propagation
    pub fn parse_typed_error_propagation(
        expression: Box<dyn Expression>,
        expected_error_type: String,
        conversion_logic: Option<Box<dyn Expression>>,
    ) -> TypedErrorPropagation {
        TypedErrorPropagation {
            expression,
            expected_error_type,
            conversion_logic,
        }
    }

    /// Parse unwrap-or expression
    pub fn parse_unwrap_or(
        base: Box<dyn Expression>,
        default_value: Box<dyn Expression>,
        method_name: String,
    ) -> UnwrapOrExpression {
        UnwrapOrExpression {
            base,
            default_value,
            method_name,
        }
    }

    /// Parse try expression
    pub fn parse_try(
        try_block: Box<dyn Expression>,
        catch_block: Option<Box<dyn Expression>>,
        finally_block: Option<Box<dyn Expression>>,
    ) -> TryExpression {
        TryExpression {
            try_block,
            catch_block,
            finally_block,
        }
    }

    /// Parse field access expression
    pub fn parse_field_access(
        base: Box<dyn Expression>,
        field_name: String,
        safe_access: bool,
    ) -> FieldAccessExpression {
        FieldAccessExpression {
            base,
            field_name,
            safe_access,
        }
    }

    /// Parse method call expression
    pub fn parse_method_call(
        receiver: Box<dyn Expression>,
        method_name: String,
        arguments: Vec<Box<dyn Expression>>,
        type_arguments: Vec<String>,
    ) -> MethodCallExpression {
        MethodCallExpression {
            receiver,
            method_name,
            arguments,
            type_arguments,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug, Clone)]
    struct MockExpr(String);
    
    impl fmt::Display for MockExpr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    
    impl Expression for MockExpr {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[test]
    fn test_enhanced_question_mark_display() {
        let expr = Box::new(MockExpr("test".to_string()));
        let recovery = Some(Box::new(MockExpr("default".to_string())) as Box<dyn Expression>);
        
        let question_mark = EnhancedQuestionMarkExpression {
            expression: expr,
            error_recovery: recovery,
            source_location: None,
        };
        
        assert_eq!(question_mark.to_string(), "test? or default");
    }

    #[test]
    fn test_typed_error_propagation_display() {
        let expr = Box::new(MockExpr("test".to_string()));
        
        let typed_prop = TypedErrorPropagation {
            expression: expr,
            expected_error_type: "MyError".to_string(),
            conversion_logic: None,
        };
        
        assert_eq!(typed_prop.to_string(), "test?<MyError>");
    }

    #[test]
    fn test_unwrap_or_display() {
        let base = Box::new(MockExpr("result".to_string()));
        let default = Box::new(MockExpr("42".to_string()));
        
        let unwrap_or = UnwrapOrExpression {
            base,
            default_value: default,
            method_name: "unwrap_or".to_string(),
        };
        
        assert_eq!(unwrap_or.to_string(), "result.unwrap_or(42)");
    }

    #[test]
    fn test_try_expression_display() {
        let try_block = Box::new(MockExpr("risky()".to_string()));
        let catch_block = Some(Box::new(MockExpr("handle()".to_string())) as Box<dyn Expression>);
        
        let try_expr = TryExpression {
            try_block,
            catch_block,
            finally_block: None,
        };
        
        assert_eq!(try_expr.to_string(), "try risky() catch handle()");
    }

    #[test]
    fn test_field_access_display() {
        let base = Box::new(MockExpr("obj".to_string()));
        
        let field_access = FieldAccessExpression {
            base,
            field_name: "value".to_string(),
            safe_access: false,
        };
        
        assert_eq!(field_access.to_string(), "obj.value");
    }

    #[test]
    fn test_safe_field_access_display() {
        let base = Box::new(MockExpr("obj".to_string()));
        
        let safe_field_access = FieldAccessExpression {
            base,
            field_name: "value".to_string(),
            safe_access: true,
        };
        
        assert_eq!(safe_field_access.to_string(), "obj?.value");
    }

    #[test]
    fn test_method_call_display() {
        let receiver = Box::new(MockExpr("obj".to_string()));
        let args = vec![
            Box::new(MockExpr("arg1".to_string())) as Box<dyn Expression>,
            Box::new(MockExpr("arg2".to_string())) as Box<dyn Expression>,
        ];
        
        let method_call = MethodCallExpression {
            receiver,
            method_name: "method".to_string(),
            arguments: args,
            type_arguments: vec!["String".to_string(), "i32".to_string()],
        };
        
        assert_eq!(method_call.to_string(), "obj.method<String, i32>(arg1, arg2)");
    }
}
