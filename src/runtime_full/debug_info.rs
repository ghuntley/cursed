/// Enhanced stack trace system with comprehensive debug information integration
///
/// Provides rich debug information capture, source location mapping, symbol resolution,
/// and integration with LLVM debug metadata for production-quality debugging capabilities.

use crate::error::{CursedError, SourceLocation};

use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::path::{Path, PathBuf};
use std::fmt;
use std::backtrace::{Backtrace, BacktraceStatus};

/// Debug information for a single source location
#[derive(Debug, Clone)]
pub struct DebugInfo {
    /// Source file path
    /// Line number (1-based)
    /// Column number (1-based)
    /// Function or method name
    /// Module or namespace
    /// Variable names and their types in current scope
    /// Instruction pointer address
    /// LLVM debug metadata ID
impl DebugInfo {
    pub fn new<P: AsRef<Path>>(file_path: P, line: u32, column: u32, function_name: String) -> Self {
        DebugInfo {
        }
    }

    pub fn with_module(mut self, module_name: String) -> Self {
        self.module_name = Some(module_name);
        self
    pub fn with_variable(mut self, name: String, type_name: String) -> Self {
        self.variables.insert(name, type_name);
        self
    pub fn with_instruction_pointer(mut self, ip: usize) -> Self {
        self.instruction_pointer = Some(ip);
        self
    pub fn with_debug_metadata(mut self, id: u64) -> Self {
        self.debug_metadata_id = Some(id);
        self
    }
}

impl fmt::Display for DebugInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
               self.file_path.display(), self.line, self.function_name)?;
        
        if let Some(module) = &self.module_name {
            write!(f, " ({})", module)?;
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
    /// Frame index in the stack (0 = top)
    /// Call instruction that led to this frame
    /// Local variables visible at this frame
    /// Whether this frame is inlined
    /// Optimization level when compiled
/// Information about a variable in scope
#[derive(Debug, Clone)]
pub struct VariableInfo {
    /// Variable name
    /// Type information
    /// Current value (if available and safe to capture)
    /// Memory location or register
    /// Whether the variable is mutable
    /// Scope depth (0 = function scope, 1+ = nested blocks)
impl VariableInfo {
    pub fn new(name: String, type_name: String) -> Self {
        VariableInfo {
        }
    }

    pub fn with_value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    pub fn with_location(mut self, location: String) -> Self {
        self.location = Some(location);
        self
    pub fn with_mutability(mut self, is_mutable: bool) -> Self {
        self.is_mutable = is_mutable;
        self
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
        if let Some(location) = &self.location {
            write!(f, " @ {}", location)?;
        Ok(())
    }
}

impl EnhancedStackFrame {
    pub fn new(debug_info: DebugInfo, frame_index: usize) -> Self {
        EnhancedStackFrame {
        }
    }

    pub fn with_call_site(mut self, call_site: DebugInfo) -> Self {
        self.call_site = Some(call_site);
        self
    pub fn with_variable(mut self, var_info: VariableInfo) -> Self {
        self.local_variables.insert(var_info.name.clone(), var_info);
        self
    pub fn with_inlined(mut self, is_inlined: bool) -> Self {
        self.is_inlined = is_inlined;
        self
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
        if let Some(opt_level) = &self.optimization_level {
            write!(f, " ({})", opt_level)?;
        if let Some(call_site) = &self.call_site {
                   call_site.file_path.display(), call_site.line)?;
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
    /// Rust backtrace (if available)
    /// Timestamp when captured
    /// Thread ID where captured
    /// Goroutine ID (if applicable)
    /// Total stack depth (may be truncated)
    /// Whether stack was truncated
impl EnhancedStackTrace {
    pub fn new() -> Self {
        EnhancedStackTrace {
        }
    }

    pub fn with_frames(mut self, frames: Vec<EnhancedStackFrame>) -> Self {
        self.frames = frames;
        self.total_depth = self.frames.len();
        self
    pub fn with_rust_backtrace(mut self, backtrace: Backtrace) -> Self {
        self.rust_backtrace = Some(backtrace);
        self
    pub fn with_goroutine(mut self, goroutine_id: u64) -> Self {
        self.goroutine_id = Some(goroutine_id);
        self
    pub fn with_truncation(mut self, total_depth: usize) -> Self {
        self.total_depth = total_depth;
        self.is_truncated = total_depth > self.frames.len();
        self
    /// Add a frame to the stack trace
    pub fn push_frame(&mut self, frame: EnhancedStackFrame) {
        self.frames.push(frame);
        self.total_depth = self.frames.len();
    /// Get the top frame (most recent call)
    pub fn top_frame(&self) -> Option<&EnhancedStackFrame> {
        self.frames.first()
    /// Get frames filtered by a predicate
    pub fn filter_frames<F>(&self, predicate: F) -> Vec<&EnhancedStackFrame>
    where
    {
        self.frames.iter().filter(|frame| predicate(frame)).collect()
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
                if self.is_truncated { 
                    format!(", {} total", self.total_depth) 
                } else { 
                    String::new() 
                })?;
        
        for frame in &self.frames {
            writeln!(f, "{}", frame)?;
        if let Some(goroutine_id) = self.goroutine_id {
            writeln!(f, "  in goroutine #{}", goroutine_id)?;
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
    /// Whether to capture variable information
    /// Whether to capture call site information
    /// Whether to capture Rust backtraces
    /// Whether to resolve symbols for instruction pointers
    /// Maximum depth for variable capture
    /// Whether to include inlined frames
    /// File patterns to exclude from user frames
impl Default for StackTraceConfig {
    fn default() -> Self {
        StackTraceConfig {
            exclude_patterns: vec![
                "runtime/".to_string(),
                "std/".to_string(),
                "core/".to_string(),
        }
    }
/// Stack trace capture engine
pub struct StackTraceCapture {
    /// Configuration for capture behavior
    /// Symbol resolver for instruction pointers
    /// Debug info manager
/// Trait for resolving symbols from instruction pointers
pub trait SymbolResolver {
    /// Resolve a symbol from an instruction pointer
    fn resolve_symbol(&self, ip: usize) -> Option<SymbolInfo>;
    
    /// Resolve multiple symbols efficiently
    fn resolve_symbols(&self, ips: &[usize]) -> Vec<Option<SymbolInfo>>;
/// Information about a resolved symbol
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    /// Symbol name
    /// Source file
    /// Line number
    /// Column number  
    /// Address offset from symbol start
impl StackTraceCapture {
    pub fn new() -> Self {
        StackTraceCapture {
        }
    }

    pub fn with_config(mut self, config: StackTraceConfig) -> Self {
        self.config = config;
        self
    pub fn with_symbol_resolver<R>(mut self, resolver: R) -> Self 
    where
    {
        self.symbol_resolver = Some(Arc::new(resolver));
        self
    pub fn with_debug_manager(mut self, manager: Arc<crate::runtime::debug_manager::DebugManager>) -> Self {
        self.debug_manager = Some(manager);
        self
    /// Capture a stack trace at the current location
    pub fn capture(&self) -> crate::error::Result<()> {
        self.capture_with_context(None)
    /// Capture a stack trace with optional goroutine context
    pub fn capture_with_context(&self, goroutine_id: Option<u64>) -> crate::error::Result<()> {
        let mut trace = EnhancedStackTrace::new();
        
        if let Some(gid) = goroutine_id {
            trace = trace.with_goroutine(gid);
        // Capture Rust backtrace if configured
        if self.config.capture_rust_backtrace {
            let backtrace = Backtrace::capture();
            trace = trace.with_rust_backtrace(backtrace);
        // Build enhanced frames
        let frames = self.build_enhanced_frames()?;
        trace = trace.with_frames(frames);

        Ok(trace)
    /// Build enhanced stack frames with debug information
    fn build_enhanced_frames(&self) -> crate::error::Result<()> {
        let mut frames = Vec::new();
        
        // For now, create sample frames - in a real implementation,
        // this would walk the actual call stack using platform-specific APIs
        
        // Placeholder frame representing current location
        let debug_info = DebugInfo::new(
            "src/main.csd",
        ).with_module("main".to_string());

        let mut frame = EnhancedStackFrame::new(debug_info, 0);
        
        if self.config.capture_variables {
            let var = VariableInfo::new(
            ).with_value("42".to_string())
             .with_mutability(true);
            
            frame = frame.with_variable(var);
        frames.push(frame);

        // Add more sample frames to demonstrate the structure
        if frames.len() < self.config.max_frames {
            let caller_debug = DebugInfo::new(
                "src/lib.csd",
            ).with_module("mylib".to_string());

            let caller_frame = EnhancedStackFrame::new(caller_debug, 1);
            frames.push(caller_frame);
        Ok(frames)
    /// Extract source code snippet around a location
    pub fn extract_source_snippet(&self, file_path: &Path, line: u32, context_lines: u32) -> crate::error::Result<()> {
        use std::fs;
        use std::io::{BufRead, BufReader};

        let file = fs::File::open(file_path)
            .map_err(|e| CursedError::Runtime(format!("Failed to open source file {}: {}", file_path.display(), e)))?;

        let reader = BufReader::new(file);
        let lines: Result<Vec<_>, _> = reader.lines().collect();
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
impl MockSymbolResolver {
    pub fn new() -> Self {
        MockSymbolResolver {
        }
    }

    pub fn add_symbol(&mut self, ip: usize, symbol: SymbolInfo) {
        self.symbols.insert(ip, symbol);
    }
}

impl SymbolResolver for MockSymbolResolver {
    fn resolve_symbol(&self, ip: usize) -> Option<SymbolInfo> {
        self.symbols.get(&ip).cloned()
    fn resolve_symbols(&self, ips: &[usize]) -> Vec<Option<SymbolInfo>> {
        ips.iter().map(|&ip| self.resolve_symbol(ip)).collect()
    }
}

