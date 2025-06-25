pub mod debug_context;
pub mod types;

pub use types::{SourceLocation, ErrorContext, ParseError, RuntimeError, IoError, ErrorManager, ErrorManagerConfig, ErrorCategory, ErrorSeverity, CursedErrorTrait};
pub use debug_context::{DebugContext, DebugContextBuilder, DebugResult, IntoDebugContext};

// Re-export main CursedError type

// Add conversion traits for database errors
// impl From<crate::stdlib::database::error::DatabaseError> for crate::error::CursedError {
//     fn from(err: crate::stdlib::database::error::DatabaseError) -> Self {
//         crate::CursedError::Runtime(format!("Database error: {}", err))
//     }
// }
