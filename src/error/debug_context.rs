/// Debug context module for enhanced error reporting

#[derive(Debug, Clone)]
pub struct DebugContext {
    pub message: String,
    pub location: Option<SourceLocation>,
    pub severity: ErrorSeverity,
    pub stack_trace: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ErrorSeverity {
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct DebugContextBuilder {
    message: String,
    location: Option<SourceLocation>,
    severity: ErrorSeverity,
    stack_trace: Vec<String>,
}

impl DebugContextBuilder {
    pub fn new(message: String) -> Self {
        Self {
            message,
            location: None,
            severity: ErrorSeverity::Error,
            stack_trace: Vec::new(),
        }
    }
    
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
    
    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = severity;
        self
    }
    
    pub fn with_stack_trace(mut self, stack_trace: Vec<String>) -> Self {
        self.stack_trace = stack_trace;
        self
    }
    
    pub fn build(self) -> DebugContext {
        DebugContext {
            message: self.message,
            location: self.location,
            severity: self.severity,
            stack_trace: self.stack_trace,
        }
    }
}

pub type DebugResult<T> = Result<T, DebugContext>;

pub trait IntoDebugContext {
    fn into_debug_context(self) -> DebugContext;
}

impl IntoDebugContext for String {
    fn into_debug_context(self) -> DebugContext {
        DebugContextBuilder::new(self).build()
    }
}

impl IntoDebugContext for &str {
    fn into_debug_context(self) -> DebugContext {
        DebugContextBuilder::new(self.to_string()).build()
    }
}
