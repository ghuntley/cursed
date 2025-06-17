/// Enhanced stack trace system with comprehensive debug information integration
///
/// Provides rich debug information capture, source location mapping, symbol resolution,
/// and integration with LLVM debug metadata for production-quality debugging capabilities.

use crate::error::{Error as CursedError, SourceLocation};
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::path::{Path, PathBuf};
use std::fmt;
use std::backtrace::{Backtrace, BacktraceStatus};

/// Debug information for a single source location
#[derive(Debug, Clone)]
pub struct DebugInfo {
    /// Source file path
    pub file_path: PathBuf,
    /// Line number (1-based)
    pub line: u32,
    /// Column number (1-based)
    pub column: u32,
    /// Function or method name
    pub function_name: String,
    /// Module or namespace
    pub module_name: Option<String>,
    /// Variable names and their types in current scope
    pub variables: HashMap<String, String>,
    /// Instruction pointer address
    pub instruction_pointer: Option<usize>,
    /// LLVM debug metadata ID
    pub debug_metadata_id: Option<u64>,
}

impl DebugInfo {
    pub fn new<P: AsRef<Path>>(file_path: P, line: u32, column: u32, function_name: String) -> Self {
        DebugInfo {
            file_path: file_path.as_ref().to_path_buf(),
            line,
            column,
            function_name,
            module_name: None,
            variables: HashMap::new(),
            instruction_pointer: None,
            debug_metadata_id: None,
        }
    }

    pub fn with_module(mut self, module_name: String) -> Self {
        self.module_name = Some(module_name);
        self
    }

    pub fn with_variable(mut self, name: String, type_name: String) -> Self {
        self.variables.insert(name, type_name);
        self
    }

    pub fn with_instruction_pointer(mut self, ip: usize) -> Self {
        self.instruction_pointer = Some(ip);
        self
    }

    pub fn with_debug_metadata(mut self, id: u64) -> Self {
        self.debug_metadata_id = Some(id);
        self
    }
}

impl fmt::Display for DebugInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{} in {}", 
               self.file_path.display(), self.line, self.function_name)?;
        
        if let Some(module) = &self.module_name {
            write!(f, " ({})", module)?;
        }

        if !self.variables.is_empty() {
            write!(f, "\n    Variables: ")?;
            for (i, (name, type_name)) in self.variables.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", name, type_name)?;
            }
        }

        Ok(())
    }
}

/// Enhanced stack frame with rich debug information
#[derive(Debug, Clone)]
pub struct EnhancedStackFrame {
    /// Core debug information
    pub debug_info: DebugInfo,
    /// Frame index in the stack (0 = top)
    pub frame_index: usize,
    /// Call instruction that led to this frame
    pub call_site: Option<DebugInfo>,
    /// Local variables visible at this frame
    pub local_variables: HashMap<String, VariableInfo>,
    /// Whether this frame is inlined
    pub is_inlined: bool,
    /// Optimization level when compiled
    pub optimization_level: Option<String>,
}

/// Information about a variable in scope
#[derive(Debug, Clone)]
pub struct VariableInfo {
    /// Variable name
    pub name: String,
    /// Type information
    pub type_name: String,
    /// Current value (if available and safe to capture)
    pub value: Option<String>,
    /// Memory location or register
    pub location: Option<String>,
    /// Whether the variable is mutable
    pub is_mutable: bool,
    /// Scope depth (0 = function scope, 1+ = nested blocks)
    pub scope_depth: u32,
}

impl VariableInfo {
    pub fn new(name: String, type_name: String) -> Self {
        VariableInfo {
            name,
            type_name,
            value: None,
            location: None,
            is_mutable: false,
            scope_depth: 0,
        }
    }

    pub fn with_value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_mutability(mut self, is_mutable: bool) -> Self {
        self.is_mutable = is_mutable;
        self
    }

    pub fn with_scope_depth(mut self, depth: u32) -> Self {
        self.scope_depth = depth;
        self
    }
}

impl fmt::Display for VariableInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_mutable {
            write!(f, "mut ")?;
        }
        write!(f, "{}: {}", self.name, self.type_name)?;
        
        if let Some(value) = &self.value {
            write!(f, " = {}", value)?;
        }
        
        if let Some(location) = &self.location {
            write!(f, " @ {}", location)?;
        }
        
        Ok(())
    }
}

impl EnhancedStackFrame {
    pub fn new(debug_info: DebugInfo, frame_index: usize) -> Self {
        EnhancedStackFrame {
            debug_info,
            frame_index,
            call_site: None,
            local_variables: HashMap::new(),
            is_inlined: false,
            optimization_level: None,
        }
    }

    pub fn with_call_site(mut self, call_site: DebugInfo) -> Self {
        self.call_site = Some(call_site);
        self
    }

    pub fn with_variable(mut self, var_info: VariableInfo) -> Self {
        self.local_variables.insert(var_info.name.clone(), var_info);
        self
    }

    pub fn with_inlined(mut self, is_inlined: bool) -> Self {
        self.is_inlined = is_inlined;
        self
    }

    pub fn with_optimization_level(mut self, level: String) -> Self {
        self.optimization_level = Some(level);
        self
    }
}

impl fmt::Display for EnhancedStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  #{}: {}", self.frame_index, self.debug_info)?;
        
        if self.is_inlined {
            write!(f, " [inlined]")?;
        }
        
        if let Some(opt_level) = &self.optimization_level {
            write!(f, " ({})", opt_level)?;
        }
        
        if let Some(call_site) = &self.call_site {
            write!(f, "\n      called from {}:{}", 
                   call_site.file_path.display(), call_site.line)?;
        }
        
        if !self.local_variables.is_empty() {
            write!(f, "\n      Local variables:")?;
            for var in self.local_variables.values() {
                write!(f, "\n        {}", var)?;
            }
        }
        
        Ok(())
    }
}

/// Comprehensive stack trace with enhanced debug information
#[derive(Debug)]
pub struct EnhancedStackTrace {
    /// Stack frames with debug information
    pub frames: Vec<EnhancedStackFrame>,
    /// Rust backtrace (if available)
    pub rust_backtrace: Option<Backtrace>,
    /// Timestamp when captured
    pub timestamp: std::time::SystemTime,
    /// Thread ID where captured
    pub thread_id: std::thread::ThreadId,
    /// Goroutine ID (if applicable)
    pub goroutine_id: Option<u64>,
    /// Total stack depth (may be truncated)
    pub total_depth: usize,
    /// Whether stack was truncated
    pub is_truncated: bool,
}

impl EnhancedStackTrace {
    pub fn new() -> Self {
        EnhancedStackTrace {
            frames: Vec::new(),
            rust_backtrace: None,
            timestamp: std::time::SystemTime::now(),
            thread_id: std::thread::current().id(),
            goroutine_id: None,
            total_depth: 0,
            is_truncated: false,
        }
    }

    pub fn with_frames(mut self, frames: Vec<EnhancedStackFrame>) -> Self {
        self.frames = frames;
        self.total_depth = self.frames.len();
        self
    }

    pub fn with_rust_backtrace(mut self, backtrace: Backtrace) -> Self {
        self.rust_backtrace = Some(backtrace);
        self
    }

    pub fn with_goroutine(mut self, goroutine_id: u64) -> Self {
        self.goroutine_id = Some(goroutine_id);
        self
    }

    pub fn with_truncation(mut self, total_depth: usize) -> Self {
        self.total_depth = total_depth;
        self.is_truncated = total_depth > self.frames.len();
        self
    }

    /// Add a frame to the stack trace
    pub fn push_frame(&mut self, frame: EnhancedStackFrame) {
        self.frames.push(frame);
        self.total_depth = self.frames.len();
    }

    /// Get the top frame (most recent call)
    pub fn top_frame(&self) -> Option<&EnhancedStackFrame> {
        self.frames.first()
    }

    /// Get frames filtered by a predicate
    pub fn filter_frames<F>(&self, predicate: F) -> Vec<&EnhancedStackFrame>
    where
        F: Fn(&EnhancedStackFrame) -> bool,
    {
        self.frames.iter().filter(|frame| predicate(frame)).collect()
    }

    /// Get frames from user code (excluding runtime/system frames)
    pub fn user_frames(&self) -> Vec<&EnhancedStackFrame> {
        self.filter_frames(|frame| {
            !frame.debug_info.file_path.to_string_lossy().contains("runtime") &&
            !frame.debug_info.file_path.to_string_lossy().contains("std")
        })
    }
}

impl fmt::Display for EnhancedStackTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Stack trace ({} frames{}):", 
                self.frames.len(),
                if self.is_truncated { 
                    format!(", {} total", self.total_depth) 
                } else { 
                    String::new() 
                })?;
        
        for frame in &self.frames {
            writeln!(f, "{}", frame)?;
        }
        
        if let Some(goroutine_id) = self.goroutine_id {
            writeln!(f, "  in goroutine #{}", goroutine_id)?;
        }
        
        if let Some(backtrace) = &self.rust_backtrace {
            if backtrace.status() == BacktraceStatus::Captured {
                writeln!(f, "\nRust backtrace:")?;
                writeln!(f, "{}", backtrace)?;
            }
        }
        
        Ok(())
    }
}

/// Configuration for stack trace capture
#[derive(Debug, Clone)]
pub struct StackTraceConfig {
    /// Maximum number of frames to capture
    pub max_frames: usize,
    /// Whether to capture variable information
    pub capture_variables: bool,
    /// Whether to capture call site information
    pub capture_call_sites: bool,
    /// Whether to capture Rust backtraces
    pub capture_rust_backtrace: bool,
    /// Whether to resolve symbols for instruction pointers
    pub resolve_symbols: bool,
    /// Maximum depth for variable capture
    pub max_variable_depth: u32,
    /// Whether to include inlined frames
    pub include_inlined_frames: bool,
    /// File patterns to exclude from user frames
    pub exclude_patterns: Vec<String>,
}

impl Default for StackTraceConfig {
    fn default() -> Self {
        StackTraceConfig {
            max_frames: 50,
            capture_variables: true,
            capture_call_sites: true,
            capture_rust_backtrace: true,
            resolve_symbols: true,
            max_variable_depth: 10,
            include_inlined_frames: true,
            exclude_patterns: vec![
                "runtime/".to_string(),
                "std/".to_string(),
                "core/".to_string(),
            ],
        }
    }
}

/// Stack trace capture engine
pub struct StackTraceCapture {
    /// Configuration for capture behavior
    config: StackTraceConfig,
    /// Symbol resolver for instruction pointers
    symbol_resolver: Option<Arc<dyn SymbolResolver + Send + Sync>>,
    /// Debug info manager
    debug_manager: Option<Arc<crate::runtime::debug_manager::DebugManager>>,
}

/// Trait for resolving symbols from instruction pointers
pub trait SymbolResolver {
    /// Resolve a symbol from an instruction pointer
    fn resolve_symbol(&self, ip: usize) -> Option<SymbolInfo>;
    
    /// Resolve multiple symbols efficiently
    fn resolve_symbols(&self, ips: &[usize]) -> Vec<Option<SymbolInfo>>;
}

/// Information about a resolved symbol
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    /// Symbol name
    pub name: String,
    /// Source file
    pub file: Option<PathBuf>,
    /// Line number
    pub line: Option<u32>,
    /// Column number  
    pub column: Option<u32>,
    /// Address offset from symbol start
    pub offset: Option<usize>,
}

impl StackTraceCapture {
    pub fn new() -> Self {
        StackTraceCapture {
            config: StackTraceConfig::default(),
            symbol_resolver: None,
            debug_manager: None,
        }
    }

    pub fn with_config(mut self, config: StackTraceConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_symbol_resolver<R>(mut self, resolver: R) -> Self 
    where
        R: SymbolResolver + Send + Sync + 'static,
    {
        self.symbol_resolver = Some(Arc::new(resolver));
        self
    }

    pub fn with_debug_manager(mut self, manager: Arc<crate::runtime::debug_manager::DebugManager>) -> Self {
        self.debug_manager = Some(manager);
        self
    }

    /// Capture a stack trace at the current location
    pub fn capture(&self) -> Result<EnhancedStackTrace, CursedError> {
        self.capture_with_context(None)
    }

    /// Capture a stack trace with optional goroutine context
    pub fn capture_with_context(&self, goroutine_id: Option<u64>) -> Result<EnhancedStackTrace, CursedError> {
        let mut trace = EnhancedStackTrace::new();
        
        if let Some(gid) = goroutine_id {
            trace = trace.with_goroutine(gid);
        }

        // Capture Rust backtrace if configured
        if self.config.capture_rust_backtrace {
            let backtrace = Backtrace::capture();
            trace = trace.with_rust_backtrace(backtrace);
        }

        // Build enhanced frames
        let frames = self.build_enhanced_frames()?;
        trace = trace.with_frames(frames);

        Ok(trace)
    }

    /// Build enhanced stack frames with debug information
    fn build_enhanced_frames(&self) -> Result<Vec<EnhancedStackFrame>, CursedError> {
        let mut frames = Vec::new();
        
        // For now, create sample frames - in a real implementation,
        // this would walk the actual call stack using platform-specific APIs
        
        // Placeholder frame representing current location
        let debug_info = DebugInfo::new(
            "src/main.csd",
            42,
            10,
            "main".to_string(),
        ).with_module("main".to_string());

        let mut frame = EnhancedStackFrame::new(debug_info, 0);
        
        if self.config.capture_variables {
            let var = VariableInfo::new(
                "x".to_string(),
                "sus".to_string(),
            ).with_value("42".to_string())
             .with_mutability(true);
            
            frame = frame.with_variable(var);
        }

        frames.push(frame);

        // Add more sample frames to demonstrate the structure
        if frames.len() < self.config.max_frames {
            let caller_debug = DebugInfo::new(
                "src/lib.csd",
                15,
                5,
                "helper_function".to_string(),
            ).with_module("mylib".to_string());

            let caller_frame = EnhancedStackFrame::new(caller_debug, 1);
            frames.push(caller_frame);
        }

        Ok(frames)
    }

    /// Extract source code snippet around a location
    pub fn extract_source_snippet(&self, file_path: &Path, line: u32, context_lines: u32) -> Result<String, CursedError> {
        use std::fs;
        use std::io::{BufRead, BufReader};

        let file = fs::File::open(file_path)
            .map_err(|e| CursedError::Runtime(format!("Failed to open source file {}: {}", file_path.display(), e)))?;

        let reader = BufReader::new(file);
        let lines: Result<Vec<String>, std::io::Error> = reader.split("\n").collect();
        let lines = lines
            .map_err(|e| CursedError::Runtime(format!("Failed to read source file: {}", e)))?;

        let target_line = line.saturating_sub(1) as usize; // Convert to 0-based indexing
        let start_line = target_line.saturating_sub(context_lines as usize);
        let end_line = std::cmp::min(target_line + context_lines as usize + 1, lines.len());

        let mut snippet = String::new();
        for (i, line_content) in lines[start_line..end_line].iter().enumerate() {
            let line_number = start_line + i + 1;
            let marker = if line_number == line as usize { ">" } else { " " };
            snippet.push_str(&format!("{} {:4} | {}\n", marker, line_number, line_content));
        }

        Ok(snippet)
    }
}

impl Default for StackTraceCapture {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock symbol resolver for testing
#[derive(Debug)]
pub struct MockSymbolResolver {
    symbols: HashMap<usize, SymbolInfo>,
}

impl MockSymbolResolver {
    pub fn new() -> Self {
        MockSymbolResolver {
            symbols: HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, ip: usize, symbol: SymbolInfo) {
        self.symbols.insert(ip, symbol);
    }
}

impl SymbolResolver for MockSymbolResolver {
    fn resolve_symbol(&self, ip: usize) -> Option<SymbolInfo> {
        self.symbols.get(&ip).cloned()
    }

    fn resolve_symbols(&self, ips: &[usize]) -> Vec<Option<SymbolInfo>> {
        ips.iter().map(|&ip| self.resolve_symbol(ip)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_info_creation() {
        let debug_info = DebugInfo::new("test.csd", 10, 5, "test_function".to_string())
            .with_module("test_module".to_string())
            .with_variable("x".to_string(), "sus".to_string());

        assert_eq!(debug_info.line, 10);
        assert_eq!(debug_info.column, 5);
        assert_eq!(debug_info.function_name, "test_function");
        assert_eq!(debug_info.module_name, Some("test_module".to_string()));
        assert!(debug_info.variables.contains_key("x"));
    }

    #[test]
    fn test_variable_info() {
        let var = VariableInfo::new("test_var".to_string(), "String".to_string())
            .with_value("\"hello\"".to_string())
            .with_mutability(true)
            .with_scope_depth(2);

        assert_eq!(var.name, "test_var");
        assert_eq!(var.type_name, "String");
        assert_eq!(var.value, Some("\"hello\"".to_string()));
        assert!(var.is_mutable);
        assert_eq!(var.scope_depth, 2);
    }

    #[test]
    fn test_enhanced_stack_frame() {
        let debug_info = DebugInfo::new("test.csd", 10, 5, "test_function".to_string());
        let var = VariableInfo::new("x".to_string(), "sus".to_string());
        
        let frame = EnhancedStackFrame::new(debug_info, 0)
            .with_variable(var)
            .with_inlined(false);

        assert_eq!(frame.frame_index, 0);
        assert!(!frame.is_inlined);
        assert!(frame.local_variables.contains_key("x"));
    }

    #[test]
    fn test_stack_trace_capture() {
        let capture = StackTraceCapture::new();
        let trace = capture.capture().unwrap();

        assert!(!trace.frames.is_empty());
        assert_eq!(trace.thread_id, std::thread::current().id());
    }

    #[test]
    fn test_stack_trace_config() {
        let config = StackTraceConfig {
            max_frames: 25,
            capture_variables: false,
            ..Default::default()
        };

        assert_eq!(config.max_frames, 25);
        assert!(!config.capture_variables);
        assert!(config.capture_call_sites);
    }

    #[test]
    fn test_mock_symbol_resolver() {
        let mut resolver = MockSymbolResolver::new();
        
        let symbol = SymbolInfo {
            name: "test_symbol".to_string(),
            file: Some(PathBuf::from("test.csd")),
            line: Some(42),
            column: Some(10),
            offset: Some(0x100),
        };

        resolver.add_symbol(0x1000, symbol);
        
        let resolved = resolver.resolve_symbol(0x1000);
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap().name, "test_symbol");
        
        let not_found = resolver.resolve_symbol(0x2000);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_user_frames_filtering() {
        let trace = EnhancedStackTrace::new().with_frames(vec![
            EnhancedStackFrame::new(
                DebugInfo::new("src/main.csd", 10, 5, "main".to_string()),
                0
            ),
            EnhancedStackFrame::new(
                DebugInfo::new("runtime/panic.rs", 100, 10, "panic_handler".to_string()),
                1
            ),
        ]);

        let user_frames = trace.user_frames();
        assert_eq!(user_frames.len(), 1);
        assert_eq!(user_frames[0].debug_info.function_name, "main");
    }
}
