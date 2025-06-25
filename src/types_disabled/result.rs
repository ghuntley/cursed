// Result and Option types for CURSED error handling
//
// This module provides Result<T, E> and Option<T> types that integrate with
// CURSED's panic/recovery system and question mark operator.

use crate::ast::traits::{Node, Expression, TypeNode};
use crate::error::{CursedError, SourceLocation};

use std::any::Any;
use std::fmt;

/// Result<T, E> type for error handling
/// Represents either success (Ok) or failure (Err)
#[derive(Debug, Clone)]
pub enum Result<T, E> {
    /// Success case containing a value
    Ok(T),
    /// CursedError case containing an error
    Err(E),
}

impl<T, E> Result<T, E> {
    /// Returns true if the result is Ok
    pub fn is_ok(&self) -> bool {
        matches!(self, Result::Ok(_))
    }

    /// Returns true if the result is Err
    pub fn is_err(&self) -> bool {
        matches!(self, Result::Err(_))
    }

    /// Unwraps the Ok value, panicking if Err
    pub fn unwrap(self) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Result::Ok(val) => val,
            Result::Err(err) => panic!("called `Result::unwrap()` on an `Err` value: {:?}", err),
        }
    }

    /// Unwraps the Ok value or returns a default
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Result::Ok(val) => val,
            Result::Err(_) => default,
        }
    }

    /// Unwraps the Err value, panicking if Ok
    pub fn unwrap_err(self) -> E
    where
        T: fmt::Debug,
    {
        match self {
            Result::Ok(val) => panic!("called `Result::unwrap_err()` on an `Ok` value: {:?}", val),
            Result::Err(err) => err,
        }
    }

    /// Unwraps the Ok value or computes it from a closure
    pub fn unwrap_or_else<F>(self, op: F) -> T
    where
        F: FnOnce(E) -> T,
    {
        match self {
            Result::Ok(val) => val,
            Result::Err(err) => op(err),
        }
    }

    /// Maps the Ok value to another type
    pub fn map<U, F>(self, op: F) -> Result<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Result::Ok(val) => Result::Ok(op(val)),
            Result::Err(err) => Result::Err(err),
        }
    }

    /// Maps the Err value to another type
    pub fn map_err<F, O>(self, op: O) -> Result<T, F>
    where
        O: FnOnce(E) -> F,
    {
        match self {
            Result::Ok(val) => Result::Ok(val),
            Result::Err(err) => Result::Err(op(err)),
        }
    }

    /// Applies a function to the contained value if Ok
    pub fn and_then<U, F>(self, op: F) -> Result<U, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        match self {
            Result::Ok(val) => op(val),
            Result::Err(err) => Result::Err(err),
        }
    }

    /// Returns the result if Ok, otherwise returns the other result
    pub fn or_else<F, O>(self, op: O) -> Result<T, F>
    where
        O: FnOnce(E) -> Result<T, F>,
    {
        match self {
            Result::Ok(val) => Result::Ok(val),
            Result::Err(err) => op(err),
        }
    }

    /// Converts to Option<T>, discarding the error
    pub fn ok(self) -> Option<T> {
        match self {
            Result::Ok(val) => Option::Some(val),
            Result::Err(_) => Option::None,
        }
    }

    /// Converts to Option<E>, discarding the ok value
    pub fn err(self) -> Option<E> {
        match self {
            Result::Ok(_) => Option::None,
            Result::Err(err) => Option::Some(err),
        }
    }

    /// Returns the Ok value as a reference
    pub fn as_ref(&self) -> Result<&T, &E> {
        match self {
            Result::Ok(val) => Result::Ok(val),
            Result::Err(err) => Result::Err(err),
        }
    }

    /// Returns the Ok value as a mutable reference
    pub fn as_mut(&mut self) -> Result<&mut T, &mut E> {
        match self {
            Result::Ok(val) => Result::Ok(val),
            Result::Err(err) => Result::Err(err),
        }
    }
}

impl<T: fmt::Display, E: fmt::Display> fmt::Display for Result<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Result::Ok(val) => write!(f, "Ok({})", val),
            Result::Err(err) => write!(f, "Err({})", err),
        }
    }
}

/// Option<T> type for optional values
/// Represents either some value (Some) or nothing (None)
#[derive(Debug, Clone)]
pub enum Option<T> {
    /// Some value
    Some(T),
    /// No value
    None,
}

impl<T> Option<T> {
    /// Returns true if the option is Some
    pub fn is_some(&self) -> bool {
        matches!(self, Option::Some(_))
    }

    /// Returns true if the option is None
    pub fn is_none(&self) -> bool {
        matches!(self, Option::None)
    }

    /// Unwraps the Some value, panicking if None
    pub fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }

    /// Unwraps the Some value or returns a default
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => default,
        }
    }

    /// Unwraps the Some value or computes it from a closure
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Option::Some(val) => val,
            Option::None => f(),
        }
    }

    /// Maps the Some value to another type
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Option::Some(val) => Option::Some(f(val)),
            Option::None => Option::None,
        }
    }

    /// Applies a function to the contained value if Some
    pub fn and_then<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> Option<U>,
    {
        match self {
            Option::Some(val) => f(val),
            Option::None => Option::None,
        }
    }

    /// Returns the option if Some, otherwise returns the other option
    pub fn or(self, optb: Option<T>) -> Option<T> {
        match self {
            Option::Some(_) => self,
            Option::None => optb,
        }
    }

    /// Returns the option if Some, otherwise calls a function
    pub fn or_else<F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> Option<T>,
    {
        match self {
            Option::Some(_) => self,
            Option::None => f(),
        }
    }

    /// Converts to Result<T, E> with provided error
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Option::Some(val) => Result::Ok(val),
            Option::None => Result::Err(err),
        }
    }

    /// Converts to Result<T, E> with error from closure
    pub fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Option::Some(val) => Result::Ok(val),
            Option::None => Result::Err(err()),
        }
    }

    /// Returns the Some value as a reference
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Option::Some(val) => Option::Some(val),
            Option::None => Option::None,
        }
    }

    /// Returns the Some value as a mutable reference
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            Option::Some(val) => Option::Some(val),
            Option::None => Option::None,
        }
    }

    /// Takes the value out of the option, leaving None in its place
    pub fn take(&mut self) -> Option<T> {
        std::mem::replace(self, Option::None)
    }

    /// Filters the option based on a predicate
    pub fn filter<P>(self, predicate: P) -> Option<T>
    where
        P: FnOnce(&T) -> bool,
    {
        match self {
            Option::Some(val) if predicate(&val) => Option::Some(val),
            _ => Option::None,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Option<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Option::Some(val) => write!(f, "Some({})", val),
            Option::None => write!(f, "None"),
        }
    }
}

/// Result type expression for CURSED AST
#[derive(Debug, Clone)]
pub struct ResultTypeExpression {
    pub token: String,
    pub ok_type: Box<dyn Expression>,
    pub err_type: Box<dyn Expression>,
}

impl ResultTypeExpression {
    pub fn new(
        token: String,
        ok_type: Box<dyn Expression>,
        err_type: Box<dyn Expression>,
    ) -> Self {
        Self {
            token,
            ok_type,
            err_type,
        }
    }

    pub fn get_ok_type(&self) -> &Box<dyn Expression> {
        &self.ok_type
    }

    pub fn get_err_type(&self) -> &Box<dyn Expression> {
        &self.err_type
    }
}

impl Node for ResultTypeExpression {
    fn string(&self) -> String {
        format!("Result<{}, {}>", self.ok_type.string(), self.err_type.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for ResultTypeExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ResultTypeExpression {
            token: self.token.clone(),
            ok_type: self.ok_type.clone_box(),
            err_type: self.err_type.clone_box(),
        })
    }
}

impl TypeNode for ResultTypeExpression {
    fn type_name(&self) -> String {
        format!("Result<{}, {}>", self.ok_type.string(), self.err_type.string())
    }

    fn is_generic(&self) -> bool {
        true
    }
}

/// Option type expression for CURSED AST
#[derive(Debug, Clone)]
pub struct OptionTypeExpression {
    pub token: String,
    pub inner_type: Box<dyn Expression>,
}

impl OptionTypeExpression {
    pub fn new(token: String, inner_type: Box<dyn Expression>) -> Self {
        Self {
            token,
            inner_type,
        }
    }

    pub fn get_inner_type(&self) -> &Box<dyn Expression> {
        &self.inner_type
    }
}

impl Node for OptionTypeExpression {
    fn string(&self) -> String {
        format!("Option<{}>", self.inner_type.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for OptionTypeExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(OptionTypeExpression {
            token: self.token.clone(),
            inner_type: self.inner_type.clone_box(),
        })
    }
}

impl TypeNode for OptionTypeExpression {
    fn type_name(&self) -> String {
        format!("Option<{}>", self.inner_type.string())
    }

    fn is_generic(&self) -> bool {
        true
    }
}

/// Conversion traits for integration with CURSED error system
// impl<T> From<CursedError> for crate::error::Result<T> {
//     fn from(err: CursedError) -> Self {
//         Result::Err(err)
//     }
// }

impl<T> From<Option<T>> for crate::error::Result<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Option::Some(val) => Result::Ok(val),
            Option::None => Result::Err(CursedError::Runtime("Option was None".to_string())),
        }
    }
}

/// Utility functions for working with Results
pub mod result_utils {
    use super::*;

    /// Wrap a value in Ok
    pub fn ok<T, E>(value: T) -> Result<T, E> {
        Result::Ok(value)
    }

    /// Wrap a value in Err
    pub fn err<T, E>(error: E) -> Result<T, E> {
        Result::Err(error)
    }

    /// Wrap a value in Some
    pub fn some<T>(value: T) -> Option<T> {
        Option::Some(value)
    }

    /// Return None
    pub fn none<T>() -> Option<T> {
        Option::None
    }

    /// Convert a boolean to Option
    pub fn bool_to_option<T>(condition: bool, value: T) -> Option<T> {
        if condition {
            Option::Some(value)
        } else {
            Option::None
        }
    }

    /// Transpose a Result of Option to Option of Result
    pub fn transpose<T, E>(result: Result<Option<T>, E>) -> Option<Result<T, E>> {
        match result {
            Result::Ok(Option::Some(val)) => Option::Some(Result::Ok(val)),
            Result::Ok(Option::None) => Option::None,
            Result::Err(err) => Option::Some(Result::Err(err)),
        }
    }
}

/// CursedError patterns for common CURSED error scenarios
pub mod error_patterns {
    use super::*;

    /// Create a parse error result
    pub fn parse_error<T>(message: &str, line: usize, column: usize) -> crate::error::Result<T> {
        Result::Err(CursedError::Parse(format!("{}:{}: {}", line, column, message)))
    }

    /// Create a runtime error result
    pub fn runtime_error<T>(message: &str) -> crate::error::Result<T> {
        Result::Err(CursedError::Runtime(message.to_string()))
    }

    /// Create a type error result
    pub fn type_error<T>(message: &str) -> crate::error::Result<T> {
        Result::Err(CursedError::Type(message.to_string()))
    }

    /// Create a compilation error result
    pub fn compilation_error<T>(message: &str) -> crate::error::Result<T> {
        Result::Err(CursedError::Parse(message.to_string()))
    }

    /// Create an I/O error result
    pub fn io_error<T>(io_err: std::io::Error) -> crate::error::Result<T> {
        Result::Err(CursedError::Io(io_err.to_string()))
    }
}

