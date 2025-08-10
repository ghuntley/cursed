// Error types for CURSED language
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    Parse(String),
    Type(String),
    Runtime(String),
    Compile(String),
    Import(String),
    Lexer(String),
    Io(String),
    Memory(String),
    TypeCheck(String),
    Package(String),
    Template(String),
    Optimization(String),
    Debug(String),
    InvalidOptimizationLevel(String),
    // Generic instantiation errors
    TypeParameterMismatch { expected: usize, provided: usize, context: String },
    GenericNotFound(String),
    UnboundTypeParameter(String),
    InterfaceNotFound(String),
    ConstraintViolation(String),
    ConstraintResolutionError(String),
    BoundViolation { type_param: String, concrete_type: String, bound: String, reason: String },
    RecursiveGenericInstantiation(String),
    UnknownGenericType(String),
    UnknownGenericFunction(String),
    UnknownGenericStruct(String),
    UnknownVariable(String),
    MonomorphisationError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(msg) => write!(f, "Parse error: {}", msg),
            Error::Type(msg) => write!(f, "Type error: {}", msg),
            Error::Runtime(msg) => write!(f, "Runtime error: {}", msg),
            Error::Compile(msg) => write!(f, "Compile error: {}", msg),
            Error::Import(msg) => write!(f, "Import error: {}", msg),
            Error::Lexer(msg) => write!(f, "Lexer error: {}", msg),
            Error::Io(msg) => write!(f, "I/O error: {}", msg),
            Error::Memory(msg) => write!(f, "Memory error: {}", msg),
            Error::TypeCheck(msg) => write!(f, "Type check error: {}", msg),
            Error::Package(msg) => write!(f, "Package error: {}", msg),
            Error::Template(msg) => write!(f, "Template error: {}", msg),
            Error::Optimization(msg) => write!(f, "Optimization error: {}", msg),
            Error::Debug(msg) => write!(f, "Debug error: {}", msg),
            Error::InvalidOptimizationLevel(msg) => write!(f, "Invalid optimization level: {}", msg),
            Error::TypeParameterMismatch { expected, provided, context } => 
                write!(f, "Type parameter mismatch in {}: expected {} parameters, got {}", context, expected, provided),
            Error::ConstraintViolation(msg) => write!(f, "Constraint violation: {}", msg),
            Error::ConstraintResolutionError(msg) => write!(f, "Constraint resolution error: {}", msg),
            Error::BoundViolation { type_param, concrete_type, bound, reason } => 
                write!(f, "Bound violation for type parameter '{}': type '{}' does not satisfy bound '{}' - {}", 
                       type_param, concrete_type, bound, reason),
            Error::RecursiveGenericInstantiation(msg) => write!(f, "Recursive generic instantiation: {}", msg),
            Error::UnknownGenericType(msg) => write!(f, "Unknown generic type: {}", msg),
            Error::UnknownGenericFunction(msg) => write!(f, "Unknown generic function: {}", msg),
            Error::UnknownGenericStruct(msg) => write!(f, "Unknown generic struct: {}", msg),
            Error::UnknownVariable(msg) => write!(f, "Unknown variable: {}", msg),
            Error::MonomorphisationError(msg) => write!(f, "Monomorphisation error: {}", msg),
            Error::GenericNotFound(name) => write!(f, "Generic not found: {}", name),
            Error::UnboundTypeParameter(name) => write!(f, "Unbound type parameter: {}", name),
            Error::InterfaceNotFound(name) => write!(f, "Interface not found: {}", name),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

// Legacy type alias for compatibility
pub type CursedError = Error;
