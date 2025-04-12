//! AST nodes for function call expressions in the CURSED language.
//!
//! This module defines the AST representations for function calls, including:
//! - Regular function calls: `function(arg1, arg2)`
//! - Generic function calls: `function[T1, T2](arg1, arg2)`
//!
//! These expressions represent the invocation of functions with arguments and
//! optional generic type parameters.

use crate::ast::{Expression, Node};
use crate::lexer::token::Token;
use std::any::Any;

/// Represents a function call expression in the AST.
///
/// A call expression consists of a function expression (typically an identifier
/// or dot expression) followed by argument expressions enclosed in parentheses.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// foo(1, "hello", true)
/// ```
///
/// The AST would have a `CallExpression` with:
/// - function: identifier "foo"
/// - arguments: [IntegerLiteral(1), StringLiteral("hello"), BooleanLiteral(true)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        let args: Vec<String> = self.arguments.iter().map(|arg| arg.string()).collect();
        format!(
            "{} {} {}",
            self.function.string(),
            self.token_literal(),
            args.join(", ")
        )
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_call_expression(&self) -> bool {
        true
    }

    fn as_call_expression(&self) -> Option<(&dyn Expression, Vec<&dyn Expression>)> {
        let args: Vec<&dyn Expression> = self
            .arguments
            .iter()
            .map(|arg| arg.as_ref() as &dyn Expression)
            .collect();
        Some((self.function.as_ref(), args))
    }
}

/// Represents a function call with generic type parameters in the AST.
///
/// A generic call expression extends the regular call expression by including
/// type arguments enclosed in square brackets before the function arguments.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// create[int, string](5, "value")
/// ```
///
/// The AST would have a `GenericCallExpression` with:
/// - function: identifier "create"
/// - type_arguments: [TypeLiteral(int), TypeLiteral(string)]
/// - arguments: [IntegerLiteral(5), StringLiteral("value")]
pub struct GenericCallExpression {
    pub token: Token,
    pub function: Box<dyn Expression>,
    pub type_arguments: Vec<Box<dyn Expression>>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Node for GenericCallExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        let type_args: Vec<String> = self.type_arguments.iter().map(|arg| arg.string()).collect();
        let args: Vec<String> = self.arguments.iter().map(|arg| arg.string()).collect();
        format!(
            "{} {} [{}] {}",
            self.function.string(),
            self.token_literal(),
            type_args.join(", "),
            args.join(", ")
        )
    }
}

impl Expression for GenericCallExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_call_expression(&self) -> bool {
        true
    }

    fn as_call_expression(&self) -> Option<(&dyn Expression, Vec<&dyn Expression>)> {
        let args: Vec<&dyn Expression> = self
            .arguments
            .iter()
            .map(|arg| arg.as_ref() as &dyn Expression)
            .collect();
        Some((self.function.as_ref(), args))
    }
}
