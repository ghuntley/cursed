// Types module containing Result, Option, and error pattern utilities
pub mod result;

// Re-export public types for easier access
pub use result::{
    ResultTypeExpression, OptionTypeExpression,
    Result, Option, result_utils, error_patterns
};
