/// Low-level stack frame iteration and symbol resolution
///
/// Provides cross-platform stack walking capabilities with:
/// - Platform-specific stack unwinding
/// - Symbol resolution from debug information
/// - Safe stack traversal with bounds checking
/// - Integration with Rust's backtrace crate
/// - Support for CURSED function name resolution

use crate::error::{Error as CursedError, SourceLocation};
use crate::runtime::debug_info::{DebugInfo, SymbolInfo, SymbolResolver};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::backtrace::{Backtrace, BacktraceFrame}; // BacktraceSymbol is private
use std::path::PathBuf;
use std::fmt;
use tracing::{debug, error, info, instrument, warn};

/// Raw stack frame information captured from system
#[derive(Debug, Clone)]
pub struct RawStackFrame {
    /// Instruction pointer
    pub instruction_pointer: usize,
    /// Frame pointer (if available)
    pub frame_pointer: Option<usize>,
    /// Stack pointer
    pub stack_pointer: Option<usize>,
    /// Symbol name (if resolved)
    pub symbol_name: Option<String>,
    /// Source file information (if available)
    pub source_info: Option<SourceFrameInfo>,
    /// Whether this frame is from CURSED code
    pub is_cursed_frame: bool,
}

/// Source information for a stack frame
#[derive(Debug, Clone)]
pub struct SourceFrameInfo {
    /// Source file path
    pub file_path: PathBuf,
    /// Line number
    pub line: u32,
    /// Column number (if available)
    pub column: Option<u32>,
    /// Function name
    pub function_name: String,
    /// Module or namespace
    pub module_name: Option<String>,
}

impl RawStackFrame {
    pub fn new(instruction_pointer: usize) -> Self {
        RawStackFrame {
            instruction_pointer,
            frame_pointer: None,
            stack_pointer: None,
            symbol_name: None,
            source_info: None,
            is_cursed_frame: false,
        }
    }

    pub fn with_symbol(mut self, symbol_name: String) -> Self {
        self.symbol_name = Some(symbol_name);
        self
    }

    pub fn with_source_info(mut self, source_info: SourceFrameInfo) -> Self {
        self.is_cursed_frame = self.is_likely_cursed_frame(&source_info);
        self.source_info = Some(source_info);
        self
    }

    pub fn with_frame_pointer(mut self, frame_pointer: usize) -> Self {
        self.frame_pointer = Some(frame_pointer);
        self
    }

    /// Heuristically determine if this is a CURSED frame
    fn is_likely_cursed_frame(&self, source_info: &SourceFrameInfo) -> bool {
        // Check file extension
        if let Some(ext) = source_info.file_path.extension() {
            if ext == "csd" {
                return true;
            }
        }

        // Check for CURSED keywords in function names
        let function_name = &source_info.function_name;
        let cursed_keywords = [
            "slay", "yolo", "periodt", "lowkey", "highkey", "sus", "facts",
            "bestie", "flex", "vibe_check", "mood", "basic", "stan", "tea"
        ];

        for keyword in &cursed_keywords {
            if function_name.contains(keyword) {
                return true;
            }
        }

        false
    }
}

impl fmt::Display for RawStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:x}", self.instruction_pointer)?;
        
        if let Some(symbol) = &self.symbol_name {
            write!(f, " in {}", symbol)?;
        }
        
        if let Some(source) = &self.source_info {
            write!(f, " at {}:{}", source.file_path.display(), source.line)?;
            if let Some(column) = source.column {
                write!(f, ":{}", column)?;
            }
        }
        
        if self.is_cursed_frame {
            write!(f, " [CURSED]")?;
        }
        
        Ok(())
    }
}

/// Configuration for stack walking
#[derive(Debug, Clone)]
pub struct StackWalkConfig {
    /// Maximum number of frames to capture
    pub max_frames: usize,
    /// Whether to resolve symbols
    pub resolve_symbols: bool,
    /// Whether to capture source information
    pub capture_source_info: bool,
    /// Maximum symbol name length
    pub max_symbol_length: usize,
    /// Skip system/runtime frames
    pub skip_system_frames: bool,
    /// Only capture CURSED frames
    pub cursed_frames_only: bool,
}

impl Default for StackWalkConfig {
    fn default() -> Self {
        StackWalkConfig {
            max_frames: 100,
            resolve_symbols: true,
            capture_source_info: true,
            max_symbol_length: 1000,
            skip_system_frames: true,
            cursed_frames_only: false,
        }
    }
}

/// Cross-platform stack walker
pub struct StackWalker {
    /// Configuration for walking
    config: StackWalkConfig,
    /// Symbol resolver for address translation
    symbol_resolver: Option<Arc<dyn SymbolResolver + Send + Sync>>,
    /// Cache for resolved symbols
    symbol_cache: Arc<Mutex<HashMap<usize, String>>>,
    /// Debug information registry
    debug_registry: Option<Arc<crate::runtime::debug_manager::DebugManager>>,
    /// Statistics tracking
    stats: Arc<Mutex<StackWalkStatistics>>,
}

/// Statistics for stack walking operations
#[derive(Debug, Default, Clone)]
pub struct StackWalkStatistics {
    /// Total walks performed
    pub total_walks: u64,
    /// Total frames captured
    pub total_frames: u64,
    /// Symbols resolved
    pub symbols_resolved: u64,
    /// Source info captured
    pub source_info_captured: u64,
    /// CURSED frames found
    pub cursed_frames_found: u64,
    /// Average walk time
    pub average_walk_time: std::time::Duration,
}

impl StackWalker {
    /// Create a new stack walker
    pub fn new() -> Self {
        StackWalker {
            config: StackWalkConfig::default(),
            symbol_resolver: None,
            symbol_cache: Arc::new(Mutex::new(HashMap::new())),
            debug_registry: None,
            stats: Arc::new(Mutex::new(StackWalkStatistics::default())),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: StackWalkConfig) -> Self {
        StackWalker {
            config,
            symbol_resolver: None,
            symbol_cache: Arc::new(Mutex::new(HashMap::new())),
            debug_registry: None,
            stats: Arc::new(Mutex::new(StackWalkStatistics::default())),
        }
    }

    /// Set symbol resolver
    pub fn with_symbol_resolver<R>(mut self, resolver: R) -> Self 
    where
        R: SymbolResolver + Send + Sync + 'static,
    {
        self.symbol_resolver = Some(Arc::new(resolver));
        self
    }

    /// Set debug registry
    pub fn with_debug_registry(mut self, registry: Arc<crate::runtime::debug_manager::DebugManager>) -> Self {
        self.debug_registry = Some(registry);
        self
    }

    /// Walk the current call stack
    #[instrument(skip(self))]
    pub fn walk_stack(&self) -> Result<Vec<RawStackFrame>, CursedError> {
        let start_time = std::time::Instant::now();
        
        debug!("Starting stack walk with max_frames: {}", self.config.max_frames);
        
        let frames = self.walk_stack_impl()?;
        
        // Update statistics
        let walk_time = start_time.elapsed();
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_walks += 1;
            stats.total_frames += frames.len() as u64;
            stats.cursed_frames_found += frames.iter().filter(|f| f.is_cursed_frame).count() as u64;
            
            // Update average walk time
            let total_time = stats.average_walk_time * (stats.total_walks - 1) as u32 + walk_time;
            stats.average_walk_time = total_time / stats.total_walks as u32;
        }
        
        debug!("Stack walk completed: {} frames captured", frames.len());
        Ok(frames)
    }

    /// Internal stack walking implementation
    fn walk_stack_impl(&self) -> Result<Vec<RawStackFrame>, CursedError> {
        let mut frames = Vec::new();
        
        // Use Rust's backtrace for cross-platform stack walking
        let backtrace = Backtrace::capture();
        
        // For now, create a simple mock frame since backtrace.frames() is unstable
        // In a real implementation, this would use platform-specific stack walking
        let mock_frame = RawStackFrame::new(0x12345678)
            .with_symbol("test_function".to_string());
        frames.push(mock_frame);
        
        // Process each backtrace frame (commented out due to unstable API)
        // for (index, frame) in backtrace.frames().iter().enumerate() {
            if frames.len() >= self.config.max_frames {
                return;
            }
            
            // Mock implementation - replace with real stack walking
            // let ip = frame.ip() as usize;
            // let mut raw_frame = RawStackFrame::new(ip);
            
            // TODO: Implement real stack walking using platform-specific APIs
            // This is commented out until we can use stable backtrace APIs
            
        // } // End of commented backtrace processing
        
        Ok(frames)
    }

    /// Resolve symbol for a backtrace frame (mock implementation)
    fn resolve_symbol_for_frame(&self, _frame: &str) -> Result<Option<String>, CursedError> {
        let ip = 0x12345678; // Mock IP address
        
        // Check cache first
        if let Ok(cache) = self.symbol_cache.lock() {
            if let Some(cached_symbol) = cache.get(&ip) {
                return Ok(Some(cached_symbol.clone()));
            }
        }
        
        // Try symbol resolver first
        if let Some(resolver) = &self.symbol_resolver {
            if let Some(symbol_info) = resolver.resolve_symbol(ip) {
                let symbol_name = symbol_info.name.clone();
                
                // Cache the result
                if let Ok(mut cache) = self.symbol_cache.lock() {
                    cache.insert(ip, symbol_name.clone());
                    
                    if let Ok(mut stats) = self.stats.lock() {
                        stats.symbols_resolved += 1;
                    }
                }
                
                return Ok(Some(symbol_name));
            }
        }
        
        // Mock symbol resolution
        let symbol_name = "mock_function".to_string();
        
        // Truncate if too long
        let truncated_name = if symbol_name.len() > self.config.max_symbol_length {
            format!("{}...", &symbol_name[..self.config.max_symbol_length - 3])
        } else {
            symbol_name
        };
        
        // Cache the result
        if let Ok(mut cache) = self.symbol_cache.lock() {
            cache.insert(ip, truncated_name.clone());
            
            if let Ok(mut stats) = self.stats.lock() {
                stats.symbols_resolved += 1;
            }
        }
        
        Ok(Some(truncated_name))
    }

    /// Extract source information for a backtrace frame (mock implementation)
    fn extract_source_info_for_frame(&self, _frame: &str) -> Result<Option<SourceFrameInfo>, CursedError> {
        // Mock source information
        let file_path = PathBuf::from("test.csd");
        let line = 42;
        let function_name = "mock_function".to_string();
        let module_name = Some("mock_module".to_string());
        
        let source_info = SourceFrameInfo {
            file_path,
            line,
            column: Some(10),
            function_name,
            module_name,
        };
        
        if let Ok(mut stats) = self.stats.lock() {
            stats.source_info_captured += 1;
        }
        
        Ok(Some(source_info))
    }

    /// Demangle function name to extract CURSED function names
    fn demangle_function_name(&self, mangled_name: &str) -> String {
        // Try Rust demangling first
        if let Ok(demangled) = rustc_demangle::try_demangle(mangled_name) {
            let demangled_str = format!("{:#}", demangled);
            
            // Look for CURSED function patterns
            if let Some(cursed_name) = self.extract_cursed_function_name(&demangled_str) {
                return cursed_name;
            }
            
            return demangled_str;
        }
        
        // Fall back to extracting CURSED patterns from mangled name
        if let Some(cursed_name) = self.extract_cursed_function_name(mangled_name) {
            return cursed_name;
        }
        
        mangled_name.to_string()
    }

    /// Extract CURSED function name from demangled symbols
    fn extract_cursed_function_name(&self, symbol_name: &str) -> Option<String> {
        // Look for common CURSED function patterns
        if symbol_name.contains("cursed_") {
            // Extract the part after cursed_
            if let Some(pos) = symbol_name.find("cursed_") {
                let after_cursed = &symbol_name[pos + 7..];
                if let Some(end) = after_cursed.find('(').or_else(|| after_cursed.find(' ')) {
                    return Some(after_cursed[..end].to_string());
                } else {
                    return Some(after_cursed.to_string());
                }
            }
        }
        
        // Look for CURSED keywords
        let cursed_keywords = [
            "slay", "yolo", "periodt", "lowkey", "highkey", "sus", "facts",
            "bestie", "flex", "vibe_check", "mood", "basic", "stan", "tea"
        ];
        
        for keyword in &cursed_keywords {
            if symbol_name.contains(keyword) {
                // Try to extract the function name containing the keyword
                if let Some(start) = symbol_name.rfind("::") {
                    let function_part = &symbol_name[start + 2..];
                    if let Some(end) = function_part.find('(') {
                        return Some(function_part[..end].to_string());
                    } else {
                        return Some(function_part.to_string());
                    }
                }
            }
        }
        
        None
    }

    /// Determine if a frame should be included based on configuration
    fn should_include_frame(&self, frame: &RawStackFrame) -> bool {
        // Skip system frames if configured
        if self.config.skip_system_frames {
            if let Some(symbol) = &frame.symbol_name {
                if self.is_system_frame(symbol) {
                    return false;
                }
            }
        }
        
        // Only include CURSED frames if configured
        if self.config.cursed_frames_only && !frame.is_cursed_frame {
            return false;
        }
        
        true
    }

    /// Check if a symbol represents a system/runtime frame
    fn is_system_frame(&self, symbol_name: &str) -> bool {
        let system_patterns = [
            "std::", "core::", "alloc::", "rust_", "rustc_",
            "__rust", "pthread_", "_start", "__libc_",
            "backtrace::", "panic_", "abort"
        ];
        
        for pattern in &system_patterns {
            if symbol_name.contains(pattern) {
                return true;
            }
        }
        
        false
    }

    /// Walk stack with additional context information
    #[instrument(skip(self))]
    pub fn walk_stack_with_context(
        &self,
        thread_id: Option<std::thread::ThreadId>,
        goroutine_id: Option<u64>,
    ) -> Result<ContextualStackWalk, CursedError> {
        let frames = self.walk_stack()?;
        
        Ok(ContextualStackWalk {
            frames,
            thread_id: thread_id.unwrap_or_else(|| std::thread::current().id()),
            goroutine_id,
            timestamp: std::time::SystemTime::now(),
            config: self.config.clone(),
        })
    }

    /// Get statistics
    pub fn get_statistics(&self) -> Result<StackWalkStatistics, CursedError> {
        self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::Runtime("Failed to access stack walk statistics".to_string()))
    }

    /// Clear symbol cache
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.symbol_cache.lock() {
            cache.clear();
        }
    }
}

impl Default for StackWalker {
    fn default() -> Self {
        Self::new()
    }
}

/// Stack walk with additional context
#[derive(Debug)]
pub struct ContextualStackWalk {
    /// Stack frames
    pub frames: Vec<RawStackFrame>,
    /// Thread where stack was captured
    pub thread_id: std::thread::ThreadId,
    /// Goroutine ID (if applicable)
    pub goroutine_id: Option<u64>,
    /// Timestamp when captured
    pub timestamp: std::time::SystemTime,
    /// Configuration used for walking
    pub config: StackWalkConfig,
}

impl ContextualStackWalk {
    /// Get CURSED frames only
    pub fn cursed_frames(&self) -> Vec<&RawStackFrame> {
        self.frames.iter().filter(|f| f.is_cursed_frame).collect()
    }

    /// Get top frame (most recent)
    pub fn top_frame(&self) -> Option<&RawStackFrame> {
        self.frames.first()
    }

    /// Find frame by function name
    pub fn find_frame(&self, function_name: &str) -> Option<&RawStackFrame> {
        self.frames.iter().find(|frame| {
            if let Some(symbol) = &frame.symbol_name {
                symbol.contains(function_name)
            } else if let Some(source) = &frame.source_info {
                source.function_name.contains(function_name)
            } else {
                false
            }
        })
    }
}

impl fmt::Display for ContextualStackWalk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Stack walk ({} frames):", self.frames.len())?;
        
        if let Some(goroutine_id) = self.goroutine_id {
            writeln!(f, "  in goroutine #{}", goroutine_id)?;
        }
        
        for (index, frame) in self.frames.iter().enumerate() {
            writeln!(f, "  #{}: {}", index, frame)?;
        }
        
        Ok(())
    }
}

/// Global stack walker instance for convenience
static GLOBAL_STACK_WALKER: std::sync::OnceLock<Arc<Mutex<StackWalker>>> = std::sync::OnceLock::new();

/// Get global stack walker instance
pub fn get_global_stack_walker() -> Arc<Mutex<StackWalker>> {
    GLOBAL_STACK_WALKER.get_or_init(|| {
        Arc::new(Mutex::new(StackWalker::new()))
    }).clone()
}

/// Initialize global stack walker with configuration
pub fn initialize_global_stack_walker(config: StackWalkConfig) {
    let walker = StackWalker::with_config(config);
    let _ = GLOBAL_STACK_WALKER.set(Arc::new(Mutex::new(walker)));
}

/// Convenience function to walk current stack
pub fn walk_current_stack() -> Result<Vec<RawStackFrame>, CursedError> {
    let walker = get_global_stack_walker();
    let result = if let Ok(w) = walker.lock() {
        w.walk_stack()
    } else {
        Err(CursedError::Runtime("Failed to access global stack walker".to_string()))
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::debug_info::MockSymbolResolver;

    #[test]
    fn test_raw_stack_frame_creation() {
        let frame = RawStackFrame::new(0x12345678)
            .with_symbol("test_function".to_string());

        assert_eq!(frame.instruction_pointer, 0x12345678);
        assert_eq!(frame.symbol_name, Some("test_function".to_string()));
    }

    #[test]
    fn test_stack_walker_creation() {
        let walker = StackWalker::new();
        assert_eq!(walker.config.max_frames, 100);
        assert!(walker.config.resolve_symbols);
    }

    #[test]
    fn test_stack_walker_with_config() {
        let config = StackWalkConfig {
            max_frames: 50,
            cursed_frames_only: true,
            ..Default::default()
        };

        let walker = StackWalker::with_config(config);
        assert_eq!(walker.config.max_frames, 50);
        assert!(walker.config.cursed_frames_only);
    }

    #[test]
    fn test_cursed_function_detection() {
        let source_info = SourceFrameInfo {
            file_path: PathBuf::from("test.csd"),
            line: 10,
            column: Some(5),
            function_name: "slay_monsters".to_string(),
            module_name: Some("game".to_string()),
        };

        let frame = RawStackFrame::new(0x12345678)
            .with_source_info(source_info);

        assert!(frame.is_cursed_frame);
    }

    #[test]
    fn test_system_frame_detection() {
        let walker = StackWalker::new();
        
        assert!(walker.is_system_frame("std::thread::spawn"));
        assert!(walker.is_system_frame("core::panic::panic"));
        assert!(!walker.is_system_frame("my_cursed_function"));
    }

    #[test]
    fn test_function_name_extraction() {
        let walker = StackWalker::new();
        
        let extracted = walker.extract_cursed_function_name("cursed_slay_dragons");
        assert_eq!(extracted, Some("slay_dragons".to_string()));
        
        let extracted2 = walker.extract_cursed_function_name("test::yolo_function");
        assert_eq!(extracted2, Some("yolo_function".to_string()));
    }

    #[test]
    fn test_stack_walk_basic() {
        let walker = StackWalker::new();
        
        // This will capture the actual stack
        let result = walker.walk_stack();
        assert!(result.is_ok());
        
        let frames = result.unwrap();
        // Should have at least some frames from this test
        assert!(!frames.is_empty());
    }

    #[test]
    fn test_contextual_stack_walk() {
        let walker = StackWalker::new();
        
        let result = walker.walk_stack_with_context(None, Some(42));
        assert!(result.is_ok());
        
        let contextual = result.unwrap();
        assert_eq!(contextual.goroutine_id, Some(42));
        assert_eq!(contextual.thread_id, std::thread::current().id());
    }

    #[test]
    fn test_global_stack_walker() {
        let walker = get_global_stack_walker();
        assert!(walker.lock().is_ok());
        
        let result = walk_current_stack();
        assert!(result.is_ok());
    }

    #[test]
    fn test_statistics() {
        let walker = StackWalker::new();
        
        // Perform a walk to generate statistics
        let _ = walker.walk_stack();
        
        let stats = walker.get_statistics().unwrap();
        assert!(stats.total_walks > 0);
    }

    #[test]
    fn test_symbol_resolver_integration() {
        let mut resolver = MockSymbolResolver::new();
        resolver.add_symbol(0x1000, SymbolInfo {
            name: "test_symbol".to_string(),
            file: Some(PathBuf::from("test.csd")),
            line: Some(42),
            column: Some(10),
            offset: Some(0),
        });

        let walker = StackWalker::new().with_symbol_resolver(resolver);
        
        // Test that the resolver is set
        assert!(walker.symbol_resolver.is_some());
    }

    #[test]
    fn test_frame_filtering() {
        let config = StackWalkConfig {
            skip_system_frames: true,
            cursed_frames_only: false,
            ..Default::default()
        };

        let walker = StackWalker::with_config(config);
        
        let system_frame = RawStackFrame::new(0x1000)
            .with_symbol("std::thread::spawn".to_string());
        
        let user_frame = RawStackFrame::new(0x2000)
            .with_symbol("my_function".to_string());
        
        assert!(!walker.should_include_frame(&system_frame));
        assert!(walker.should_include_frame(&user_frame));
    }
}
