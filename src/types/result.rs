//! Enhanced Result and Option types for CURSED language

use crate::error_types::{Error, CursedError};
use std::fmt;

/// Enhanced Result type for CURSED language with additional functionality
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Result<T, E = Error> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub fn is_ok(&self) -> bool {
        matches!(*self, Result::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn ok(self) -> Option<T> {
        match self {
            Result::Ok(x) => Option::Some(x),
            Result::Err(_) => Option::None,
        }
    }

    pub fn err(self) -> Option<E> {
        match self {
            Result::Ok(_) => Option::None,
            Result::Err(x) => Option::Some(x),
        }
    }

    pub fn unwrap(self) -> T 
    where 
        E: fmt::Debug,
    {
        match self {
            Result::Ok(t) => t,
            Result::Err(e) => panic!("called `Result::unwrap()` on an `Err` value: {:?}", e),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Result::Ok(t) => t,
            Result::Err(_) => default,
        }
    }

    pub fn unwrap_or_else<F>(self, op: F) -> T
    where
        F: FnOnce(E) -> T,
    {
        match self {
            Result::Ok(t) => t,
            Result::Err(e) => op(e),
        }
    }

    pub fn map<U, F>(self, op: F) -> Result<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Result::Ok(t) => Result::Ok(op(t)),
            Result::Err(e) => Result::Err(e),
        }
    }

    pub fn map_err<F, O>(self, op: O) -> Result<T, F>
    where
        O: FnOnce(E) -> F,
    {
        match self {
            Result::Ok(t) => Result::Ok(t),
            Result::Err(e) => Result::Err(op(e)),
        }
    }

    pub fn and_then<U, F>(self, op: F) -> Result<U, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        match self {
            Result::Ok(t) => op(t),
            Result::Err(e) => Result::Err(e),
        }
    }
}

impl<T, E> From<std::result::Result<T, E>> for Result<T, E> {
    fn from(result: std::result::Result<T, E>) -> Self {
        match result {
            std::result::Result::Ok(t) => Result::Ok(t),
            std::result::Result::Err(e) => Result::Err(e),
        }
    }
}

impl<T, E> Into<std::result::Result<T, E>> for Result<T, E> {
    fn into(self) -> std::result::Result<T, E> {
        match self {
            Result::Ok(t) => std::result::Result::Ok(t),
            Result::Err(e) => std::result::Result::Err(e),
        }
    }
}

/// Enhanced Option type for CURSED language
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    pub fn is_some(&self) -> bool {
        matches!(*self, Option::Some(_))
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Option::Some(x) => x,
            Option::None => default,
        }
    }

    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Option::Some(x) => x,
            Option::None => f(),
        }
    }

    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Option::Some(x) => Option::Some(f(x)),
            Option::None => Option::None,
        }
    }

    pub fn and_then<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> Option<U>,
    {
        match self {
            Option::Some(x) => f(x),
            Option::None => Option::None,
        }
    }

    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Option::Some(v) => Result::Ok(v),
            Option::None => Result::Err(err),
        }
    }

    pub fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Option::Some(v) => Result::Ok(v),
            Option::None => Result::Err(err()),
        }
    }
}

impl<T> From<std::option::Option<T>> for Option<T> {
    fn from(option: std::option::Option<T>) -> Self {
        match option {
            std::option::Option::Some(t) => Option::Some(t),
            std::option::Option::None => Option::None,
        }
    }
}

impl<T> Into<std::option::Option<T>> for Option<T> {
    fn into(self) -> std::option::Option<T> {
        match self {
            Option::Some(t) => std::option::Option::Some(t),
            Option::None => std::option::Option::None,
        }
    }
}

/// Utility functions for result handling
pub mod result_utils {
    use super::{Result, Option, Error};

    pub fn flatten_result<T>(result: Result<Result<T, Error>, Error>) -> Result<T, Error> {
        match result {
            Result::Ok(inner) => inner,
            Result::Err(e) => Result::Err(e),
        }
    }

    pub fn transpose_result_option<T>(result: Result<Option<T>, Error>) -> Option<Result<T, Error>> {
        match result {
            Result::Ok(Option::Some(x)) => Option::Some(Result::Ok(x)),
            Result::Ok(Option::None) => Option::None,
            Result::Err(e) => Option::Some(Result::Err(e)),
        }
    }

    pub fn collect_results<T>(results: Vec<Result<T, Error>>) -> Result<Vec<T>, Error> {
        let mut values = Vec::new();
        for result in results {
            match result {
                Result::Ok(val) => values.push(val),
                Result::Err(e) => return Result::Err(e),
            }
        }
        Result::Ok(values)
    }

    pub fn partition_results<T>(results: Vec<Result<T, Error>>) -> (Vec<T>, Vec<Error>) {
        let mut oks = Vec::new();
        let mut errs = Vec::new();
        
        for result in results {
            match result {
                Result::Ok(val) => oks.push(val),
                Result::Err(err) => errs.push(err),
            }
        }
        
        (oks, errs)
    }

    pub fn try_with_context<T, F>(f: F, context: &str) -> Result<T, Error>
    where
        F: FnOnce() -> Result<T, Error>,
    {
        match f() {
            Result::Ok(val) => Result::Ok(val),
            Result::Err(Error::Parse(msg)) => Result::Err(Error::Parse(format!("{}: {}", context, msg))),
            Result::Err(Error::TypeCheck(msg)) => Result::Err(Error::TypeCheck(format!("{}: {}", context, msg))),
            Result::Err(Error::Compile(msg)) => Result::Err(Error::Compile(format!("{}: {}", context, msg))),
            Result::Err(Error::Runtime(msg)) => Result::Err(Error::Runtime(format!("{}: {}", context, msg))),
            Result::Err(other) => Result::Err(other),
        }
    }
}

/// Common error pattern matching utilities
pub mod error_patterns {
    use super::{Result, Error};

    pub fn is_parse_error(result: &Result<(), Error>) -> bool {
        matches!(result, Result::Err(Error::Parse(_)))
    }

    pub fn is_type_error(result: &Result<(), Error>) -> bool {
        matches!(result, Result::Err(Error::TypeCheck(_)))
    }

    pub fn is_compile_error(result: &Result<(), Error>) -> bool {
        matches!(result, Result::Err(Error::Compile(_)))
    }

    pub fn is_runtime_error(result: &Result<(), Error>) -> bool {
        matches!(result, Result::Err(Error::Runtime(_)))
    }

    pub fn is_io_error(result: &Result<(), Error>) -> bool {
        matches!(result, Result::Err(Error::Io(_)))
    }

    pub fn match_error_pattern<T>(result: Result<T, Error>) -> ErrorPattern<T> {
        match result {
            Result::Ok(val) => ErrorPattern::Success(val),
            Result::Err(Error::Parse(msg)) => ErrorPattern::ParseError(msg),
            Result::Err(Error::TypeCheck(msg)) => ErrorPattern::TypeError(msg),
            Result::Err(Error::Compile(msg)) => ErrorPattern::CompileError(msg),
            Result::Err(Error::Runtime(msg)) => ErrorPattern::RuntimeError(msg),
            Result::Err(Error::Io(msg)) => ErrorPattern::IoError(msg),
            Result::Err(Error::Import(msg)) => ErrorPattern::ImportError(msg),
            Result::Err(Error::Package(msg)) => ErrorPattern::PackageError(msg),
            Result::Err(Error::Template(msg)) => ErrorPattern::TemplateError(msg),
            Result::Err(Error::Optimization(msg)) => ErrorPattern::OptimizationError(msg),
            Result::Err(Error::Memory(msg)) => ErrorPattern::MemoryError(msg),
            Result::Err(Error::Debug(msg)) => ErrorPattern::DebugError(msg),
            Result::Err(Error::InvalidOptimizationLevel(msg)) => ErrorPattern::InvalidOptimizationLevelError(msg),
            Result::Err(Error::Type(msg)) => ErrorPattern::TypeError(msg),
            Result::Err(Error::Lexer(msg)) => ErrorPattern::ParseError(msg),
            Result::Err(Error::TypeParameterMismatch { context, .. }) => ErrorPattern::TypeError(context),
            Result::Err(Error::ConstraintViolation(msg)) => ErrorPattern::TypeError(msg),
            Result::Err(Error::ConstraintResolutionError(msg)) => ErrorPattern::TypeError(msg),
            Result::Err(Error::BoundViolation { reason, .. }) => ErrorPattern::TypeError(reason),
            Result::Err(Error::RecursiveGenericInstantiation(msg)) => ErrorPattern::CompileError(msg),
            Result::Err(Error::UnknownGenericType(msg)) => ErrorPattern::TypeError(msg),
            Result::Err(Error::UnknownGenericFunction(msg)) => ErrorPattern::CompileError(msg),
            Result::Err(Error::UnknownGenericStruct(msg)) => ErrorPattern::TypeError(msg),
            Result::Err(Error::UnknownVariable(msg)) => ErrorPattern::CompileError(msg),
            Result::Err(Error::MonomorphisationError(msg)) => ErrorPattern::CompileError(msg),
            Result::Err(Error::GenericNotFound(msg)) => ErrorPattern::CompileError(msg),
            Result::Err(Error::UnboundTypeParameter(msg)) => ErrorPattern::TypeError(msg),
            Result::Err(Error::InterfaceNotFound(msg)) => ErrorPattern::TypeError(msg),
        }
    }

    #[derive(Debug)]
    pub enum ErrorPattern<T> {
        Success(T),
        ParseError(String),
        TypeError(String),
        CompileError(String),
        RuntimeError(String),
        IoError(String),
        ImportError(String),
        PackageError(String),
        TemplateError(String),
        OptimizationError(String),
        MemoryError(String),
        DebugError(String),
        InvalidOptimizationLevelError(String),
    }

    impl<T> ErrorPattern<T> {
        pub fn is_success(&self) -> bool {
            matches!(self, ErrorPattern::Success(_))
        }

        pub fn is_error(&self) -> bool {
            !self.is_success()
        }

        pub fn unwrap_success(self) -> T {
            match self {
                ErrorPattern::Success(val) => val,
                _ => panic!("called `unwrap_success()` on an error pattern"),
            }
        }

        pub fn to_result(self) -> Result<T, Error> {
            match self {
                ErrorPattern::Success(val) => Result::Ok(val),
                ErrorPattern::ParseError(msg) => Result::Err(Error::Parse(msg)),
                ErrorPattern::TypeError(msg) => Result::Err(Error::TypeCheck(msg)),
                ErrorPattern::CompileError(msg) => Result::Err(Error::Compile(msg)),
                ErrorPattern::RuntimeError(msg) => Result::Err(Error::Runtime(msg)),
                ErrorPattern::IoError(msg) => Result::Err(Error::Io(msg)),
                ErrorPattern::ImportError(msg) => Result::Err(Error::Import(msg)),
                ErrorPattern::PackageError(msg) => Result::Err(Error::Package(msg)),
                ErrorPattern::TemplateError(msg) => Result::Err(Error::Template(msg)),
                ErrorPattern::OptimizationError(msg) => Result::Err(Error::Optimization(msg)),
                ErrorPattern::MemoryError(msg) => Result::Err(Error::Memory(msg)),
                ErrorPattern::DebugError(msg) => Result::Err(Error::Debug(msg)),
                ErrorPattern::InvalidOptimizationLevelError(msg) => Result::Err(Error::InvalidOptimizationLevel(msg)),
            }
        }
    }
}

// Legacy compatibility
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Result::Ok("CURSED advanced features enabled".to_string())
}
