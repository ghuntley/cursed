use crate::error::SourceLocation;

#[derive(Debug, Clone)]
pub struct DebugContext {
    pub message: String,
    pub location: Option<SourceLocation>,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone)]
pub enum ErrorSeverity {
    Info,
    Warning,
    CursedError,
    Critical,
}

pub struct DebugContextBuilder {
    context: DebugContext,
}

impl DebugContextBuilder {
    pub fn new(message: String) -> Self {
        Self {
            context: DebugContext {
                message,
                location: None,
                severity: ErrorSeverity::CursedError,
            },
        }
    }
    
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.context.location = Some(location);
        self
    }
    
    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.context.severity = severity;
        self
    }
    
    pub fn build(self) -> DebugContext {
        self.context
    }
}

pub type DebugResult<T> = crate::error::Result<T>;

pub trait IntoDebugContext {
    fn into_debug_context(self) -> DebugContext;
}

// impl IntoDebugContext for CursedError {
//     fn into_debug_context(self) -> DebugContext {
//         DebugContext {
//             message: self.to_string(),
//             location: None,
//             severity: ErrorSeverity::CursedError,
//         }
//     }
// }
