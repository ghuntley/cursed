/// Enhanced error context with rich debug information
///
/// Provides rich error messages with source locations, stack traces,
/// code snippet extraction, and integration with panic system and question mark operator.

use crate::error::Error as CursedError;
use crate::error::{Error, SourceLocation};
use crate::runtime::debug_info::{EnhancedStackTrace, EnhancedStackFrame, DebugInfo};
use crate::runtime::debug_manager::DebugManager;
use crate::debug::enhanced_debug::{EnhancedDebugInfo, DebugInfoRegistry, SymbolMetadata};
use crate::runtime::debug_runtime::{RuntimeDebugger, VariableInspection};

use std::sync::Arc;
use std::path::{Path, PathBuf};
use std::fmt;
use std::collections::HashMap;

/// Rich error context with debug information
#[derive(Debug)]
pub struct DebugContext {
    /// Original error
    pub error: CursedError,
    /// Enhanced stack trace
    pub stack_trace: Option<EnhancedStackTrace>,
    /// Source code snippet around error location
    pub source_snippet: Option<String>,
    /// Additional debug annotations
    pub annotations: HashMap<String, String>,
    /// Error chain (if this error wraps others)
    pub error_chain: Vec<CursedError>,
    /// Goroutine context (if applicable)
    pub goroutine_id: Option<u64>,
    /// Debug manager for resolving symbols
    debug_manager: Option<Arc<DebugManager>>,
    /// Enhanced debug info registry
    debug_registry: Option<Arc<DebugInfoRegistry>>,
    /// Runtime debugger for variable inspection
    runtime_debugger: Option<Arc<RuntimeDebugger>>,
    /// Symbol metadata for current context
    symbol_metadata: Option<SymbolMetadata>,
    /// Variable inspections at error point
    variable_inspections: HashMap<String, VariableInspection>,
}

impl DebugContext {
    /// Create a new debug context
    pub fn new(error: CursedError) -> Self {
        DebugContext {
            error,
            stack_trace: None,
            source_snippet: None,
            annotations: HashMap::new(),
            error_chain: Vec::new(),
            goroutine_id: None,
            debug_manager: None,
            debug_registry: None,
            runtime_debugger: None,
            symbol_metadata: None,
            variable_inspections: HashMap::new(),
        }
    }

    /// Create debug context with stack trace
    pub fn with_stack_trace(mut self, stack_trace: EnhancedStackTrace) -> Self {
        self.stack_trace = Some(stack_trace);
        self
    }

    /// Create debug context with source snippet
    pub fn with_source_snippet(mut self, snippet: String) -> Self {
        self.source_snippet = Some(snippet);
        self
    }

    /// Add an annotation
    pub fn with_annotation(mut self, key: String, value: String) -> Self {
        self.annotations.insert(key, value);
        self
    }

    /// Add error to the chain
    pub fn with_error_chain(mut self, error: CursedError) -> Self {
        self.error_chain.push(error);
        self
    }

    /// Set goroutine context
    pub fn with_goroutine(mut self, goroutine_id: u64) -> Self {
        self.goroutine_id = Some(goroutine_id);
        self
    }

    /// Set debug manager
    pub fn with_debug_manager(mut self, debug_manager: Arc<DebugManager>) -> Self {
        self.debug_manager = Some(debug_manager);
        self
    }

    /// Set enhanced debug registry
    pub fn with_debug_registry(mut self, debug_registry: Arc<DebugInfoRegistry>) -> Self {
        self.debug_registry = Some(debug_registry);
        self
    }

    /// Set runtime debugger
    pub fn with_runtime_debugger(mut self, runtime_debugger: Arc<RuntimeDebugger>) -> Self {
        self.runtime_debugger = Some(runtime_debugger);
        self
    }

    /// Set symbol metadata
    pub fn with_symbol_metadata(mut self, symbol_metadata: SymbolMetadata) -> Self {
        self.symbol_metadata = Some(symbol_metadata);
        self
    }

    /// Add variable inspection
    pub fn with_variable_inspection(mut self, name: String, inspection: VariableInspection) -> Self {
        self.variable_inspections.insert(name, inspection);
        self
    }

    /// Extract source snippet from the error location
    pub fn extract_source_snippet(&mut self, context_lines: u32) -> Result<(), Error> {
        if self.source_snippet.is_some() {
            return Ok(()); // Already have snippet
        }

        // Try to get source location from the error
        let source_location = match &self.error {
            CursedError::ParseError { line, column, .. } => {
                line.and_then(|l| column.map(|c| SourceLocation {
                    line: l,
                    column: c,
                    file: Some("<unknown>".to_string()),
                }))
            }
            CursedError::Runtime { .. } => {
                // Try to get from stack trace
                self.stack_trace.as_ref()
                    .and_then(|trace| trace.top_frame())
                    .map(|frame| SourceLocation {
                        line: frame.debug_info.line as usize,
                        column: frame.debug_info.column as usize,
                        file: Some(frame.debug_info.file_path.to_string_lossy().to_string()),
                    })
            }
            CursedError::Panic { source_location: Some(loc), .. } => Some(loc.clone().into()),
            _ => None,
        };

        if let Some(location) = source_location {
            if let Some(file) = &location.file {
                if let Some(debug_manager) = &self.debug_manager {
                    match debug_manager.get_source_snippet(Path::new(file), location.line as u32, context_lines) {
                        Ok(snippet) => {
                            self.source_snippet = Some(snippet);
                        }
                        Err(_) => {
                            // Fallback: try to read file directly
                            self.source_snippet = self.fallback_source_snippet(file, location.line as u32, context_lines);
                        }
                    }
                } else {
                    // No debug manager, try direct file access
                    self.source_snippet = self.fallback_source_snippet(file, location.line as u32, context_lines);
                }
            }
        }

        Ok(())
    }

    /// Fallback source snippet extraction
    fn fallback_source_snippet(&self, file_path: &str, line: u32, context_lines: u32) -> Option<String> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(file_path).ok()?;
        let reader = BufReader::new(file);
        let lines: Result<(), Error> = reader.split("\n").collect();
        let lines = lines.ok()?;

        let target_line = line.saturating_sub(1) as usize;
        let start_line = target_line.saturating_sub(context_lines as usize);
        let end_line = std::cmp::min(target_line + context_lines as usize + 1, lines.len());

        if start_line >= lines.len() {
            return None;
        }

        let mut snippet = String::new();
        for (i, line_content) in lines[start_line..end_line].iter().enumerate() {
            let line_number = start_line + i + 1;
            let marker = if line_number == line as usize { ">" } else { " " };
            snippet.push_str(&format!("{} {:4} | {}\n", marker, line_number, line_content));
        }

        Some(snippet)
    }

    /// Generate comprehensive error report
    pub fn generate_error_report(&mut self) -> String {
        let mut report = String::new();

        // Error header with severity
        let severity = self.severity();
        report.push_str(&format!("[{}] Error: {}\n", severity, self.error));
        report.push_str(&format!("{}\n", "=".repeat(80)));

        // Enhanced source location with symbol info
        if let Some(location) = self.primary_location() {
            report.push_str(&format!("Location: {}\n", location));
            
            // Add symbol metadata if available
            if let Some(metadata) = &self.symbol_metadata {
                report.push_str(&format!("Symbol: {} ({})\n", 
                    metadata.symbol_type, metadata.visibility));
                
                if let Some(gen_z_keyword) = metadata.attributes.get("gen_z_keyword") {
                    report.push_str(&format!("Gen Z Keyword: {}\n", gen_z_keyword));
                }
            }
            report.push_str("\n");
        }

        // Source snippet
        if self.source_snippet.is_none() {
            let _ = self.extract_source_snippet(3);
        }

        if let Some(snippet) = &self.source_snippet {
            report.push_str("Source code:\n");
            report.push_str(snippet);
            report.push_str("\n");
        }

        // Enhanced stack trace with user/system distinction
        if let Some(stack_trace) = &self.stack_trace {
            report.push_str("Stack trace:\n");
            let user_frames = self.user_frames();
            if !user_frames.is_empty() {
                report.push_str("  User code:\n");
                for frame in &user_frames {
                    report.push_str(&format!("    at {} ({}:{}:{})\n",
                        frame.debug_info.function_name,
                        frame.debug_info.file_path.display(),
                        frame.debug_info.line,
                        frame.debug_info.column
                    ));
                }
            }
            
            // Show full trace if needed
            if stack_trace.frames.len() > user_frames.len() {
                report.push_str("  Full trace:\n");
                report.push_str(&format!("    {}\n", stack_trace));
            }
            report.push_str("\n");
        }

        // Variable inspections
        if !self.variable_inspections.is_empty() {
            report.push_str("Variables at error point:\n");
            for (name, inspection) in &self.variable_inspections {
                report.push_str(&format!("  {}: {} = {} (size: {} bytes)\n",
                    name,
                    inspection.type_info.type_name,
                    inspection.contents,
                    inspection.size_estimate
                ));
            }
            report.push_str("\n");
        }

        // Enhanced annotations with categorization
        if !self.annotations.is_empty() {
            report.push_str("Additional information:\n");
            
            // Group annotations by category
            let mut context_info = Vec::new();
            let mut debug_info = Vec::new();
            let mut other_info = Vec::new();
            
            for (key, value) in &self.annotations {
                if key.starts_with("context") {
                    context_info.push((key, value));
                } else if key.starts_with("debug") {
                    debug_info.push((key, value));
                } else {
                    other_info.push((key, value));
                }
            }
            
            if !context_info.is_empty() {
                report.push_str("  Context:\n");
                for (key, value) in context_info {
                    report.push_str(&format!("    {}: {}\n", key, value));
                }
            }
            
            if !debug_info.is_empty() {
                report.push_str("  Debug:\n");
                for (key, value) in debug_info {
                    report.push_str(&format!("    {}: {}\n", key, value));
                }
            }
            
            if !other_info.is_empty() {
                report.push_str("  Other:\n");
                for (key, value) in other_info {
                    report.push_str(&format!("    {}: {}\n", key, value));
                }
            }
            
            report.push_str("\n");
        }

        // Error chain with enhanced formatting
        if !self.error_chain.is_empty() {
            report.push_str("Error chain:\n");
            for (i, error) in self.error_chain.iter().enumerate() {
                let chain_severity = match error {
                    CursedError::Panic { recoverable: false, .. } => "FATAL",
                    CursedError::Panic { recoverable: true, .. } => "CRITICAL",
                    _ => "ERROR",
                };
                report.push_str(&format!("  {}: [{}] {}\n", i + 1, chain_severity, error));
            }
            report.push_str("\n");
        }

        // Goroutine context
        if let Some(goroutine_id) = self.goroutine_id {
            report.push_str(&format!("Goroutine: #{}\n", goroutine_id));
        }

        // Debug registry statistics
        if let Some(registry) = &self.debug_registry {
            if let Ok(stats) = registry.get_statistics() {
                report.push_str(&format!("Debug registry: {}\n", stats));
            }
        }

        report
    }

    /// Get the primary source location for this error
    pub fn primary_location(&self) -> Option<SourceLocation> {
        // Check the main error first
        match &self.error {
            CursedError::ParseError { line, column, .. } => {
                line.and_then(|l| column.map(|c| SourceLocation {
                    line: l,
                    column: c,
                    file: Some("<unknown>".to_string()),
                }))
            }
            CursedError::Panic { source_location: Some(loc), .. } => Some(loc.clone().into()),
            _ => {
                // Check stack trace
                self.stack_trace.as_ref()
                    .and_then(|trace| trace.top_frame())
                    .map(|frame| SourceLocation {
                        line: frame.debug_info.line as usize,
                        column: frame.debug_info.column as usize,
                        file: Some(frame.debug_info.file_path.to_string_lossy().to_string()),
                    })
            }
        }
    }

    /// Get user-visible stack frames (excluding runtime/system)
    pub fn user_frames(&self) -> Vec<&EnhancedStackFrame> {
        self.stack_trace.as_ref()
            .map(|trace| trace.user_frames())
            .unwrap_or_default()
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match &self.error {
            CursedError::Panic { recoverable: false, .. } => ErrorSeverity::Fatal,
            CursedError::Panic { recoverable: true, .. } => ErrorSeverity::Critical,
            CursedError::ParseError { .. } => ErrorSeverity::Error,
            CursedError::Runtime(_) => ErrorSeverity::Error,
            CursedError::Type(_) => ErrorSeverity::Error,
            CursedError::Io(_) => ErrorSeverity::Warning,
            _ => ErrorSeverity::Info,
        }
    }

    /// Create helpful error message with suggestions
    pub fn create_helpful_message(&self) -> String {
        let mut message = self.error.to_string();

        // Add suggestions based on error type
        match &self.error {
            CursedError::ParseError { message: err_msg, .. } => {
                if err_msg.contains("expected") {
                    message.push_str("\n\nSuggestion: Check for missing punctuation or keywords");
                }
                if err_msg.contains("unexpected token") {
                    message.push_str("\n\nSuggestion: Verify syntax around the highlighted location");
                }
            }
            CursedError::Type(type_msg) => {
                if type_msg.contains("type mismatch") {
                    message.push_str("\n\nSuggestion: Check variable types and function signatures");
                }
                if type_msg.contains("undefined") {
                    message.push_str("\n\nSuggestion: Verify that variables and functions are declared");
                }
            }
            CursedError::Runtime(runtime_msg) => {
                if runtime_msg.contains("null pointer") {
                    message.push_str("\n\nSuggestion: Check for uninitialized variables or nil values");
                }
                if runtime_msg.contains("index out of bounds") {
                    message.push_str("\n\nSuggestion: Verify array/slice indices are within valid range");
                }
            }
            _ => {}
        }

        // Add context from annotations
        if let Some(context) = self.annotations.get("context") {
            message.push_str(&format!("\n\nContext: {}", context));
        }

        message
    }
}

impl fmt::Display for DebugContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)?;
        
        if let Some(location) = self.primary_location() {
            write!(f, " at {}", location)?;
        }
        
        if let Some(goroutine_id) = self.goroutine_id {
            write!(f, " (goroutine #{})", goroutine_id)?;
        }
        
        Ok(())
    }
}

/// Error severity levels for categorization
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
    Fatal,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Info => write!(f, "INFO"),
            ErrorSeverity::Warning => write!(f, "WARNING"),
            ErrorSeverity::Error => write!(f, "ERROR"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
            ErrorSeverity::Fatal => write!(f, "FATAL"),
        }
    }
}

/// Enhanced error result with debug context
pub type DebugResult<T> = Result<T, DebugContext>;

/// Trait for converting errors to debug context
pub trait IntoDebugContext {
    fn into_debug_context(self) -> DebugContext;
    fn with_context(self, context: &str) -> DebugContext;
    fn with_stack_trace(self, stack_trace: EnhancedStackTrace) -> DebugContext;
}

impl IntoDebugContext for CursedError {
    fn into_debug_context(self) -> DebugContext {
        DebugContext::new(self)
    }

    fn with_context(self, context: &str) -> DebugContext {
        DebugContext::new(self)
            .with_annotation("context".to_string(), context.to_string())
    }

    fn with_stack_trace(self, stack_trace: EnhancedStackTrace) -> DebugContext {
        DebugContext::new(self)
            .with_stack_trace(stack_trace)
    }
}

/// Error context builder for convenient construction
pub struct DebugContextBuilder {
    context: DebugContext,
}

impl DebugContextBuilder {
    pub fn new(error: CursedError) -> Self {
        DebugContextBuilder {
            context: DebugContext::new(error),
        }
    }

    pub fn stack_trace(mut self, stack_trace: EnhancedStackTrace) -> Self {
        self.context = self.context.with_stack_trace(stack_trace);
        self
    }

    pub fn annotation(mut self, key: &str, value: &str) -> Self {
        self.context = self.context.with_annotation(key.to_string(), value.to_string());
        self
    }

    pub fn source_snippet(mut self, snippet: String) -> Self {
        self.context = self.context.with_source_snippet(snippet);
        self
    }

    pub fn goroutine(mut self, goroutine_id: u64) -> Self {
        self.context = self.context.with_goroutine(goroutine_id);
        self
    }

    pub fn debug_manager(mut self, debug_manager: Arc<DebugManager>) -> Self {
        self.context = self.context.with_debug_manager(debug_manager);
        self
    }

    pub fn build(self) -> DebugContext {
        self.context
    }
}

/// Macro for creating debug context with stack trace capture
#[macro_export]
macro_rules! debug_error {
    ($error:expr) => {{
        use $crate::runtime::debug_info::StackTraceCapture;
        use $crate::error::debug_context::IntoDebugContext;
        
        let capture = StackTraceCapture::new();
        let stack_trace = capture.capture().unwrap_or_else(|_| {
            use $crate::runtime::debug_info::EnhancedStackTrace;
            EnhancedStackTrace::new()
        });
        
        $error.with_stack_trace(stack_trace)
    }};
    
    ($error:expr, $context:expr) => {{
        use $crate::runtime::debug_info::StackTraceCapture;
        use $crate::error::debug_context::IntoDebugContext;
        
        let capture = StackTraceCapture::new();
        let stack_trace = capture.capture().unwrap_or_else(|_| {
            use $crate::runtime::debug_info::EnhancedStackTrace;
            EnhancedStackTrace::new()
        });
        
        $error.with_context($context).with_stack_trace(stack_trace)
    }};
}

/// Integration with question mark operator
impl From<DebugContext> for CursedError {
    fn from(debug_context: DebugContext) -> Self {
        debug_context.error
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::debug_info::{EnhancedStackTrace, EnhancedStackFrame, DebugInfo};
    use std::path::PathBuf;

    #[test]
    fn test_debug_context_creation() {
        let error = CursedError::Runtime("Test error".to_string());
        let context = DebugContext::new(error);
        
        assert!(context.stack_trace.is_none());
        assert!(context.source_snippet.is_none());
        assert!(context.annotations.is_empty());
    }

    #[test]
    fn test_debug_context_with_stack_trace() {
        let error = CursedError::Runtime("Test error".to_string());
        let stack_trace = EnhancedStackTrace::new();
        
        let context = DebugContext::new(error)
            .with_stack_trace(stack_trace);
        
        assert!(context.stack_trace.is_some());
    }

    #[test]
    fn test_debug_context_annotations() {
        let error = CursedError::Runtime("Test error".to_string());
        let context = DebugContext::new(error)
            .with_annotation("test_key".to_string(), "test_value".to_string());
        
        assert_eq!(context.annotations.get("test_key"), Some(&"test_value".to_string()));
    }

    #[test]
    fn test_error_severity() {
        let runtime_error = CursedError::Runtime("test".to_string());
        let context = DebugContext::new(runtime_error);
        assert_eq!(context.severity(), ErrorSeverity::Error);
        
        let panic_error = CursedError::panic_error("test".to_string());
        let panic_context = DebugContext::new(panic_error);
        assert_eq!(panic_context.severity(), ErrorSeverity::Critical);
    }

    #[test]
    fn test_debug_context_builder() {
        let error = CursedError::Runtime("Test error".to_string());
        let stack_trace = EnhancedStackTrace::new();
        
        let context = DebugContextBuilder::new(error)
            .stack_trace(stack_trace)
            .annotation("key", "value")
            .goroutine(42)
            .build();
        
        assert!(context.stack_trace.is_some());
        assert_eq!(context.goroutine_id, Some(42));
        assert_eq!(context.annotations.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_into_debug_context() {
        let error = CursedError::Runtime("Test error".to_string());
        let context = error.into_debug_context();
        
        match context.error {
            CursedError::Runtime(msg) => assert_eq!(msg, "Test error"),
            _ => panic!("Expected Runtime error"),
        }
    }

    #[test]
    fn test_helpful_message_generation() {
        let error = CursedError::ParseError {
            message: "expected semicolon".to_string(),
            line: Some(1),
            column: Some(5),
        };
        
        let context = DebugContext::new(error);
        let helpful = context.create_helpful_message();
        
        assert!(helpful.contains("Suggestion"));
        assert!(helpful.contains("punctuation"));
    }

    #[test]
    fn test_error_report_generation() {
        let error = CursedError::Runtime("Test error".to_string());
        let debug_info = DebugInfo::new("test.csd", 10, 5, "test_function".to_string());
        let frame = EnhancedStackFrame::new(debug_info, 0);
        let stack_trace = EnhancedStackTrace::new().with_frames(vec![frame]);
        
        let mut context = DebugContext::new(error)
            .with_stack_trace(stack_trace)
            .with_annotation("context".to_string(), "test context".to_string());
        
        let report = context.generate_error_report();
        
        assert!(report.contains("Error:"));
        assert!(report.contains("Stack trace"));
        assert!(report.contains("Additional information"));
        assert!(report.contains("test context"));
    }

    #[test]
    fn test_user_frames_filtering() {
        let user_frame = EnhancedStackFrame::new(
            DebugInfo::new("src/main.csd", 10, 5, "main".to_string()),
            0
        );
        let runtime_frame = EnhancedStackFrame::new(
            DebugInfo::new("runtime/panic.rs", 100, 10, "panic_handler".to_string()),
            1
        );
        
        let stack_trace = EnhancedStackTrace::new().with_frames(vec![user_frame, runtime_frame]);
        let context = DebugContext::new(CursedError::Runtime("test".to_string()))
            .with_stack_trace(stack_trace);
        
        let user_frames = context.user_frames();
        assert_eq!(user_frames.len(), 1);
        assert_eq!(user_frames[0].debug_info.function_name, "main");
    }
}
