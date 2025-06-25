pub mod debug_context;
pub mod types;

pub use types::{SourceLocation, ErrorContext, ParseError, RuntimeError, IoError, ErrorManager, ErrorManagerConfig, ErrorCategory, ErrorSeverity, CursedErrorTrait};
pub use debug_context::{DebugContext, DebugContextBuilder, DebugResult, IntoDebugContext};

// Main CursedError type
#[derive(Debug, Clone)]
pub enum CursedError {
    Parse(String),
    Runtime(String),
    Io(String),
    Compilation(String),
}

impl std::fmt::Display for CursedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursedError::Parse(msg) => write!(f, "Parse error: {}", msg),
            CursedError::Runtime(msg) => write!(f, "Runtime error: {}", msg),
            CursedError::Io(msg) => write!(f, "IO error: {}", msg),
            CursedError::Compilation(msg) => write!(f, "Compilation error: {}", msg),
        }
    }
}

impl std::error::Error for CursedError {}

// Add conversion traits for database errors
// impl From<crate::stdlib::database::error::DatabaseError> for crate::error::CursedError {
//     fn from(err: crate::stdlib::database::error::DatabaseError) -> Self {
//         crate::CursedError::Runtime(format!("Database error: {}", err))
//     }
// }
