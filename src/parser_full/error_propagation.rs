// CursedError propagation AST nodes and parsing
//
// This module defines the AST structures for enhanced error propagation
// expressions including question mark operators, type assertions, and recovery.

use crate::ast::traits::Expression;
use std::fmt;
use crate::error::CursedError;

/// Enhanced question mark expression with optional error recovery
#[derive(Debug, Clone)]
pub struct EnhancedQuestionMarkExpression {
impl fmt::Display for EnhancedQuestionMarkExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}?", self.expression)?;
        if let Some(recovery) = &self.error_recovery {
            write!(f, " or {}", recovery)?;
        }
        Ok(())
    }
}

impl crate::ast::traits::Node for EnhancedQuestionMarkExpression {
    fn string(&self) -> String {
        format!("{}", self)
    fn token_literal(&self) -> String {
        "?".to_string()
    }
}

impl Expression for EnhancedQuestionMarkExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Typed error propagation with specific error type expectations
#[derive(Debug, Clone)]
pub struct TypedErrorPropagation {
// impl fmt::Display for TypedErrorPropagation {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}?<{}>", self.expression, self.expected_error_type)
//     }
// }

impl crate::ast::traits::Node for TypedErrorPropagation {
    fn string(&self) -> String {
        format!("{}", self)
    fn token_literal(&self) -> String {
        "?".to_string()
    }
}

impl Expression for TypedErrorPropagation {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Unwrap-or expression for providing default values
#[derive(Debug, Clone)]
pub struct UnwrapOrExpression {
impl fmt::Display for UnwrapOrExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}({})", self.base, self.method_name, self.default_value)
    }
}

impl crate::ast::traits::Node for UnwrapOrExpression {
    fn string(&self) -> String {
        format!("{}.unwrap_or({})", self.expression.string(), self.fallback.string())
    fn token_literal(&self) -> String {
        "unwrap_or".to_string()
    }
}

impl Expression for UnwrapOrExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Try-catch expression with optional finally block
#[derive(Debug, Clone)]
pub struct TryExpression {
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

impl crate::ast::traits::Node for TryExpression {
    fn string(&self) -> String {
        let mut result = format!("try {}", self.try_block.string());
        for catch in &self.catch_blocks {
            result.push_str(&format!(" catch({}) {}", catch.error_type, catch.handler.string()));
        }
        if let Some(ref finally) = self.finally_block {
            result.push_str(&format!(" finally {}", finally.string()));
        }
        result
    fn token_literal(&self) -> String {
        "try".to_string()
    }
}

impl Expression for TryExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Field access expression with optional safe access
#[derive(Debug, Clone)]
pub struct FieldAccessExpression {
impl fmt::Display for FieldAccessExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.safe_access {
            write!(f, "{}?.{}", self.base, self.field_name)
        } else {
            write!(f, "{}.{}", self.base, self.field_name)
        }
    }
impl crate::ast::traits::Node for FieldAccessExpression {
    fn string(&self) -> String {
        let operator = if self.safe_access { "?." } else { "." };
        format!("{}{}{}", self.object.string(), operator, self.field_name)
    fn token_literal(&self) -> String {
        if self.safe_access { "?." } else { "." }.to_string()
    }
}

impl Expression for FieldAccessExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Method call expression with type arguments
#[derive(Debug, Clone)]
pub struct MethodCallExpression {
impl fmt::Display for MethodCallExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.receiver, self.method_name)?;
        
        if !self.type_arguments.is_empty() {
            write!(f, "<{}>", self.type_arguments.join(", "))?;
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

impl crate::ast::traits::Node for MethodCallExpression {
    fn string(&self) -> String {
        let mut result = format!("{}.{}", self.object.string(), self.method_name);
        if !self.type_arguments.is_empty() {
            result.push_str("<");
            for (i, arg) in self.type_arguments.iter().enumerate() {
                if i > 0 { result.push_str(", "); }
                result.push_str(arg);
            }
            result.push_str(">");
        }
        result.push_str("(");
        for (i, arg) in self.arguments.iter().enumerate() {
            if i > 0 { result.push_str(", "); }
            result.push_str(&arg.string());
        }
        result.push_str(")");
        result
    fn token_literal(&self) -> String {
        self.method_name.clone()
    }
}

impl Expression for MethodCallExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// CursedError propagation parser functions
pub struct ErrorPropagationParser;

impl ErrorPropagationParser {
    /// Parse enhanced question mark expression
    pub fn parse_enhanced_question_mark(
    ) -> EnhancedQuestionMarkExpression {
        EnhancedQuestionMarkExpression {
        }
    }

    /// Parse typed error propagation
    pub fn parse_typed_error_propagation(
    ) -> TypedErrorPropagation {
        TypedErrorPropagation {
        }
    }

    /// Parse unwrap-or expression
    pub fn parse_unwrap_or(
    ) -> UnwrapOrExpression {
        UnwrapOrExpression {
        }
    }

    /// Parse try expression
    pub fn parse_try(
    ) -> TryExpression {
        TryExpression {
        }
    }

    /// Parse field access expression
    pub fn parse_field_access(
    ) -> FieldAccessExpression {
        FieldAccessExpression {
        }
    }

    /// Parse method call expression
    pub fn parse_method_call(
    ) -> MethodCallExpression {
        MethodCallExpression {
        }
    }
