// Enhanced debug information types for CURSED debugging

use std::collections::HashMap;
use std::time::Instant;

/// Enhanced debug information structure
#[derive(Debug, Clone)]
pub struct EnhancedDebugInfo {
    pub function_name: String,
    pub source_file: String,
    pub line_number: usize,
    pub column: usize,
    pub variables: HashMap<String, String>,
    pub call_stack: Vec<StackFrame>,
    pub timestamp: Instant,
}

impl EnhancedDebugInfo {
    pub fn new() -> Self {
        Self {
            function_name: String::new(),
            source_file: String::new(),
            line_number: 0,
            column: 0,
            variables: HashMap::new(),
            call_stack: Vec::new(),
            timestamp: Instant::now(),
        }
    }

    pub fn with_location(mut self, file: String, line: usize, column: usize) -> Self {
        self.source_file = file;
        self.line_number = line;
        self.column = column;
        self
    }

    pub fn with_function(mut self, name: String) -> Self {
        self.function_name = name;
        self
    }

    pub fn add_variable(mut self, name: String, value: String) -> Self {
        self.variables.insert(name, value);
        self
    }
}

impl Default for EnhancedDebugInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Stack frame information
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub source_file: String,
    pub line_number: usize,
    pub local_variables: HashMap<String, String>,
}

/// Configuration for enhanced stack traces
#[derive(Debug, Clone)]
pub struct EnhancedStackTraceConfig {
    pub max_frames: usize,
    pub include_locals: bool,
    pub include_source_lines: bool,
    pub simplify_paths: bool,
}

impl Default for EnhancedStackTraceConfig {
    fn default() -> Self {
        Self {
            max_frames: 50,
            include_locals: true,
            include_source_lines: false,
            simplify_paths: true,
        }
    }
}

/// Enhanced stack trace generator
pub struct EnhancedStackTrace {
    config: EnhancedStackTraceConfig,
    frames: Vec<StackFrame>,
}

impl EnhancedStackTrace {
    pub fn new(config: EnhancedStackTraceConfig) -> Self {
        Self {
            config,
            frames: Vec::new(),
        }
    }

    pub fn capture(&mut self) {
        // Stub implementation - would capture actual stack in real implementation
        self.frames.clear();
        self.frames.push(StackFrame {
            function_name: "main".to_string(),
            source_file: "main.csd".to_string(),
            line_number: 1,
            local_variables: HashMap::new(),
        });
    }

    pub fn frames(&self) -> &[StackFrame] {
        &self.frames
    }
}

impl Default for EnhancedStackTrace {
    fn default() -> Self {
        Self::new(EnhancedStackTraceConfig::default())
    }
}
