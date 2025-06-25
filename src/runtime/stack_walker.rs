/// Low-level stack frame iteration and symbol resolution
///
/// Provides cross-platform stack walking capabilities with:
/// - Platform-specific stack unwinding
/// - Symbol resolution from debug information
/// - Safe stack traversal with bounds checking
/// - Integration with Rust's backtrace crate
/// - Support for CURSED function name resolution

use crate::error::CursedError;
// use crate::runtime::debug_info::{DebugInfo, SymbolInfo, SymbolResolver};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::backtrace::Backtrace; // BacktraceFrame is unstable
use std::path::PathBuf;
use std::fmt;
use tracing::{debug, error, info, instrument, warn};

/// Raw stack frame information captured from system
#[derive(Debug, Clone)]
pub struct RawStackFrame {
    /// Instruction pointer
    /// Frame pointer (if available)
    /// Stack pointer
    /// Symbol name (if resolved)
    /// Source file information (if available)
    /// Whether this frame is from CURSED code
/// Source information for a stack frame
#[derive(Debug, Clone)]
pub struct SourceFrameInfo {
    /// Source file path
    /// Line number
    /// Column number (if available)
    /// Function name
    /// Module or namespace
impl RawStackFrame {
    pub fn new(instruction_pointer: usize) -> Self {
        RawStackFrame {
        }
    }

    pub fn with_symbol(mut self, symbol_name: String) -> Self {
        self.symbol_name = Some(symbol_name);
        self
    pub fn with_source_info(mut self, source_info: SourceFrameInfo) -> Self {
        self.is_cursed_frame = self.is_likely_cursed_frame(&source_info);
        self.source_info = Some(source_info);
        self
    pub fn with_frame_pointer(mut self, frame_pointer: usize) -> Self {
        self.frame_pointer = Some(frame_pointer);
        self
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
        if let Some(source) = &self.source_info {
            write!(f, " at {}:{}", source.file_path.display(), source.line)?;
            if let Some(column) = source.column {
                write!(f, ":{}", column)?;
            }
        }
        
        if self.is_cursed_frame {
            write!(f, " [CURSED]")?;
        Ok(())
    }
}

/// Configuration for stack walking
#[derive(Debug, Clone)]
pub struct StackWalkConfig {
    /// Maximum number of frames to capture
    /// Whether to resolve symbols
    /// Whether to capture source information
    /// Maximum symbol name length
    /// Skip system/runtime frames
    /// Only capture CURSED frames
impl Default for StackWalkConfig {
    fn default() -> Self {
        StackWalkConfig {
        }
    }
/// Cross-platform stack walker
pub struct StackWalker {
    /// Configuration for walking
    /// Symbol resolver for address translation
    /// Cache for resolved symbols
    /// Debug information registry
    /// Statistics tracking
/// Statistics for stack walking operations
#[derive(Debug, Default, Clone)]
pub struct StackWalkStatistics {
    /// Total walks performed
    /// Total frames captured
    /// Symbols resolved
    /// Source info captured
    /// CURSED frames found
    /// Average walk time
impl StackWalker {
    /// Create a new stack walker
    pub fn new() -> Self {
        StackWalker {
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: StackWalkConfig) -> Self {
        StackWalker {
        }
    }

    /// Set symbol resolver
    pub fn with_symbol_resolver<R>(mut self, resolver: R) -> Self 
    where
    {
        self.symbol_resolver = Some(Arc::new(resolver));
        self
    /// Set debug registry
    pub fn with_debug_registry(mut self, registry: Arc<crate::runtime::debug_manager::DebugManager>) -> Self {
        self.debug_registry = Some(registry);
        self
    /// Walk the current call stack
    #[instrument(skip(self))]
    pub fn walk_stack(&self) -> crate::error::Result<()> {
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
        debug!("Stack walk completed: {} frames captured", frames.len());
        Ok(frames)
    /// Internal stack walking implementation
    fn walk_stack_impl(&self) -> crate::error::Result<()> {
        let mut frames = Vec::new();
        
        // Use platform-specific stack walking implementation
        #[cfg(target_os = "linux")]
        {
            self.walk_stack_linux(&mut frames)?;
        #[cfg(target_os = "macos")]
        {
            self.walk_stack_macos(&mut frames)?;
        #[cfg(target_os = "windows")]
        {
            self.walk_stack_windows(&mut frames)?;
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            self.walk_stack_generic(&mut frames)?;
        // Filter frames based on configuration
        let filtered_frames: Vec<RawStackFrame> = frames
            .into_iter()
            .filter(|frame| self.should_include_frame(frame))
            .take(self.config.max_frames)
            .collect();
        
        Ok(filtered_frames)
    /// Linux-specific stack walking using backtrace
    #[cfg(target_os = "linux")]
    fn walk_stack_linux(&self, frames: &mut Vec<RawStackFrame>) -> crate::error::Result<()> {
        use std::ffi::c_void;
        use std::mem;
        
        // Capture backtrace using libc::backtrace
        const MAX_FRAMES: usize = 256;
        let mut buffer: [*mut c_void; MAX_FRAMES] = [std::ptr::null_mut(); MAX_FRAMES];
        
        let num_frames = unsafe {
            libc::backtrace(buffer.as_mut_ptr(), MAX_FRAMES as i32)
        
        if num_frames < 0 {
            return Err(CursedError::Runtime("Failed to capture backtrace".to_string()));
        debug!("Captured {} frames on Linux", num_frames);
        
        // Get symbol names
        let symbols = unsafe {
            libc::backtrace_symbols(buffer.as_ptr(), num_frames)
        
        if symbols.is_null() {
            warn!("Failed to get symbol names for backtrace");
        // Process each frame
        for i in 0..num_frames as usize {
            if frames.len() >= self.config.max_frames {
                break;
            let ip = buffer[i] as usize;
            let mut raw_frame = RawStackFrame::new(ip);
            
            // Get symbol name if available
            if !symbols.is_null() {
                let symbol_ptr = unsafe { *symbols.add(i) };
                if !symbol_ptr.is_null() {
                    let symbol_cstr = unsafe { std::ffi::CStr::from_ptr(symbol_ptr) };
                    if let Ok(symbol_str) = symbol_cstr.to_str() {
                        let demangled = self.demangle_function_name(symbol_str);
                        raw_frame = raw_frame.with_symbol(demangled);
                    }
                }
            // Try to resolve additional symbol information
            if self.config.resolve_symbols {
                if let Ok(Some(resolved_symbol)) = self.resolve_symbol_for_frame_addr(ip) {
                    raw_frame = raw_frame.with_symbol(resolved_symbol);
                }
            }
            
            // Try to get source information
            if self.config.capture_source_info {
                if let Ok(Some(source_info)) = self.extract_source_info_for_addr(ip) {
                    raw_frame = raw_frame.with_source_info(source_info);
                }
            }
            
            frames.push(raw_frame);
        // Free symbol names
        if !symbols.is_null() {
            unsafe {
                libc::free(symbols as *mut c_void);
            }
        }
        
        Ok(())
    /// macOS-specific stack walking using backtrace
    #[cfg(target_os = "macos")]
    fn walk_stack_macos(&self, frames: &mut Vec<RawStackFrame>) -> crate::error::Result<()> {
        use std::ffi::c_void;
        
        // macOS uses similar backtrace API to Linux
        const MAX_FRAMES: usize = 256;
        let mut buffer: [*mut c_void; MAX_FRAMES] = [std::ptr::null_mut(); MAX_FRAMES];
        
        let num_frames = unsafe {
            libc::backtrace(buffer.as_mut_ptr(), MAX_FRAMES as i32)
        
        if num_frames < 0 {
            return Err(CursedError::Runtime("Failed to capture backtrace on macOS".to_string()));
        debug!("Captured {} frames on macOS", num_frames);
        
        // Get symbol names
        let symbols = unsafe {
            libc::backtrace_symbols(buffer.as_ptr(), num_frames)
        
        // Process frames similar to Linux
        for i in 0..num_frames as usize {
            if frames.len() >= self.config.max_frames {
                break;
            let ip = buffer[i] as usize;
            let mut raw_frame = RawStackFrame::new(ip);
            
            // Get symbol name if available
            if !symbols.is_null() {
                let symbol_ptr = unsafe { *symbols.add(i) };
                if !symbol_ptr.is_null() {
                    let symbol_cstr = unsafe { std::ffi::CStr::from_ptr(symbol_ptr) };
                    if let Ok(symbol_str) = symbol_cstr.to_str() {
                        let demangled = self.demangle_function_name(symbol_str);
                        raw_frame = raw_frame.with_symbol(demangled);
                    }
                }
            // Try to resolve additional information
            if self.config.resolve_symbols {
                if let Ok(Some(resolved_symbol)) = self.resolve_symbol_for_frame_addr(ip) {
                    raw_frame = raw_frame.with_symbol(resolved_symbol);
                }
            }
            
            if self.config.capture_source_info {
                if let Ok(Some(source_info)) = self.extract_source_info_for_addr(ip) {
                    raw_frame = raw_frame.with_source_info(source_info);
                }
            }
            
            frames.push(raw_frame);
        // Free symbol names
        if !symbols.is_null() {
            unsafe {
                libc::free(symbols as *mut c_void);
            }
        }
        
        Ok(())
    /// Windows-specific stack walking using StackWalk64
    #[cfg(target_os = "windows")]
    fn walk_stack_windows(&self, frames: &mut Vec<RawStackFrame>) -> crate::error::Result<()> {
        use std::ffi::c_void;
        use std::mem;
        
        // Windows stack walking using StackWalk64 API
        // This is a simplified implementation - full Windows support would require
        // more complex integration with Windows debugging APIs
        
        debug!("Attempting Windows stack walk");
        
        // For now, use a basic implementation that captures current context
        // In a full implementation, this would use:
        // - GetCurrentProcess/GetCurrentThread
        // - StackWalk64 with proper CONTEXT structure
        // - Symbol resolution using DbgHelp.dll
        
        // Placeholder implementation using basic frame pointer walking
        let mut frame_ptr: *const usize = std::ptr::null();
        
        // Get current frame pointer (this is architecture-specific)
        #[cfg(target_arch = "x86_64")]
        unsafe {
            std::arch::asm!("mov {}, rbp", out(reg) frame_ptr);
        #[cfg(target_arch = "x86")]
        unsafe {
            std::arch::asm!("mov {}, ebp", out(reg) frame_ptr);
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        {
            warn!("Inline assembly stack walking not supported on this architecture");
            return Ok(());
        let mut frame_count = 0;
        let mut current_frame = frame_ptr;
        
        // Walk the frame chain
        while !current_frame.is_null() && frame_count < self.config.max_frames {
            let ip = unsafe {
                if current_frame.is_null() {
                    break;
                }
                *current_frame.add(1) // Return address is at [rbp + 8]
            
            if ip == 0 {
                break;
            let mut raw_frame = RawStackFrame::new(ip);
            
            // Try to resolve symbol information
            if self.config.resolve_symbols {
                if let Ok(Some(symbol)) = self.resolve_symbol_for_frame_addr(ip) {
                    raw_frame = raw_frame.with_symbol(symbol);
                }
            }
            
            if self.config.capture_source_info {
                if let Ok(Some(source_info)) = self.extract_source_info_for_addr(ip) {
                    raw_frame = raw_frame.with_source_info(source_info);
                }
            }
            
            frames.push(raw_frame);
            
            // Move to next frame
            current_frame = unsafe {
                if current_frame.is_null() {
                    std::ptr::null()
                } else {
                    *current_frame as *const usize
                }
            
            frame_count += 1;
            
            // Safety check to prevent infinite loops
            if frame_count > 1000 {
                warn!("Stack walk exceeded safety limit on Windows");
                break;
            }
        }
        
        debug!("Captured {} frames on Windows", frame_count);
        Ok(())
    /// Generic stack walking fallback for unsupported platforms
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    fn walk_stack_generic(&self, frames: &mut Vec<RawStackFrame>) -> crate::error::Result<()> {
        warn!("Stack walking not fully supported on this platform, using generic implementation");
        
        // Try to use Rust's backtrace if available
        let backtrace = Backtrace::capture();
        let backtrace_str = format!("{}", backtrace);
        
        // Parse the backtrace string to extract frame information
        for (index, line) in backtrace_str.split("\n").enumerate() {
            if index >= self.config.max_frames {
                break;
            if line.trim().is_empty() {
                continue;
            // Try to extract address and symbol from backtrace line
            if let Some(ip) = self.extract_address_from_backtrace_line(line) {
                let mut raw_frame = RawStackFrame::new(ip);
                
                // Try to extract symbol name from the line
                if let Some(symbol) = self.extract_symbol_from_backtrace_line(line) {
                    raw_frame = raw_frame.with_symbol(symbol);
                frames.push(raw_frame);
            }
        }
        
        Ok(())
    /// Extract address from backtrace line
    fn extract_address_from_backtrace_line(&self, line: &str) -> Option<usize> {
        // Look for hex addresses in the line
        if let Some(start) = line.find("0x") {
            let addr_str = &line[start..];
            if let Some(end) = addr_str.find(' ') {
                let addr_str = &addr_str[..end];
                if let Ok(addr) = usize::from_str_radix(&addr_str[2..], 16) {
                    return Some(addr);
                }
            }
        }
        None
    /// Extract symbol name from backtrace line
    fn extract_symbol_from_backtrace_line(&self, line: &str) -> Option<String> {
        // Look for symbol patterns in backtrace lines
        if let Some(start) = line.find(" - ") {
            let symbol_part = &line[start + 3..];
            if let Some(end) = symbol_part.find('\n') {
                return Some(symbol_part[..end].trim().to_string());
            } else {
                return Some(symbol_part.trim().to_string());
            }
        }
        None
    /// Resolve symbol for a specific instruction pointer address
    fn resolve_symbol_for_frame_addr(&self, ip: usize) -> crate::error::Result<()> {
        // Check cache first
        if let Ok(cache) = self.symbol_cache.lock() {
            if let Some(cached_symbol) = cache.get(&ip) {
                return Ok(Some(cached_symbol.clone()));
            }
        }
        
        // Try symbol resolver first
        if let Some(resolver) = &self.symbol_resolver {
            if let Some(symbol_info) = resolver.resolve_symbol(ip) {
                let symbol_name = self.demangle_function_name(&symbol_info.name);
                
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
        
        // Try using addr2line for better symbol resolution on Unix systems
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            if let Ok(Some(symbol)) = self.resolve_symbol_with_addr2line(ip) {
                let demangled = self.demangle_function_name(&symbol);
                
                // Cache the result
                if let Ok(mut cache) = self.symbol_cache.lock() {
                    cache.insert(ip, demangled.clone());
                    
                    if let Ok(mut stats) = self.stats.lock() {
                        stats.symbols_resolved += 1;
                    }
                }
                
                return Ok(Some(demangled));
            }
        }
        
        // No symbol found
        Ok(None)
    /// Resolve symbol using addr2line utility on Unix systems
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn resolve_symbol_with_addr2line(&self, ip: usize) -> crate::error::Result<()> {
        use std::process::Command;
        
        // Get the current executable path
        let exe_path = std::env::current_exe()
            .map_err(|e| CursedError::Runtime(format!("Failed to get executable path: {}", e)))?;
        
        // Run addr2line to resolve the symbol
        let output = Command::new("addr2line")
            .arg("-e")
            .arg(&exe_path)
            .arg("-f")  // Show function names
            .arg("-C")  // Demangle C++ symbols
            .arg(format!("0x{:x}", ip))
            .output();
        
        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.trim().split('\n').collect();
                
                if lines.len() >= 1 && lines[0] != "??" {
                    return Ok(Some(lines[0].to_string()));
                }
            }
            _ => {
                // addr2line failed or not available, fall back to other methods
                debug!("addr2line resolution failed for address 0x{:x}", ip);
            }
        }
        
        Ok(None)
    /// Extract source information for a specific instruction pointer address
    fn extract_source_info_for_addr(&self, ip: usize) -> crate::error::Result<()> {
        // Try symbol resolver first if available
        if let Some(resolver) = &self.symbol_resolver {
            if let Some(symbol_info) = resolver.resolve_symbol(ip) {
                if let (Some(file), Some(line)) = (&symbol_info.file, symbol_info.line) {
                    let source_info = SourceFrameInfo {
                    
                    if let Ok(mut stats) = self.stats.lock() {
                        stats.source_info_captured += 1;
                    return Ok(Some(source_info));
                }
            }
        // Try debug registry if available
        if let Some(debug_registry) = &self.debug_registry {
            // This would integrate with the debug manager to get source info
            // For now, we'll leave this as a placeholder for future integration
            debug!("Debug registry available but not yet integrated for address 0x{:x}", ip);
        // Try using addr2line for source information on Unix systems
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            if let Ok(Some(source_info)) = self.extract_source_info_with_addr2line(ip) {
                if let Ok(mut stats) = self.stats.lock() {
                    stats.source_info_captured += 1;
                }
                return Ok(Some(source_info));
            }
        }
        
        // Try using DWARF debug information
        if let Ok(Some(source_info)) = self.extract_source_info_from_dwarf(ip) {
            if let Ok(mut stats) = self.stats.lock() {
                stats.source_info_captured += 1;
            }
            return Ok(Some(source_info));
        // No source information found
        Ok(None)
    /// Extract source information using addr2line utility
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn extract_source_info_with_addr2line(&self, ip: usize) -> crate::error::Result<()> {
        use std::process::Command;
        
        // Get the current executable path
        let exe_path = std::env::current_exe()
            .map_err(|e| CursedError::Runtime(format!("Failed to get executable path: {}", e)))?;
        
        // Run addr2line to get source location
        let output = Command::new("addr2line")
            .arg("-e")
            .arg(&exe_path)
            .arg("-f")  // Show function names
            .arg("-C")  // Demangle C++ symbols
            .arg(format!("0x{:x}", ip))
            .output();
        
        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.trim().split('\n').collect();
                
                if lines.len() >= 2 {
                    let function_name = lines[0].to_string();
                    let location = lines[1];
                    
                    // Parse location in format "file:line:column"
                    if let Some(colon_pos) = location.rfind(':') {
                        let (file_and_line, _column_str) = location.split_at(colon_pos);
                        
                        if let Some(line_colon_pos) = file_and_line.rfind(':') {
                            let (file_path_str, line_str) = file_and_line.split_at(line_colon_pos);
                            let line_str = &line_str[1..]; // Remove the ':'
                            
                            if let Ok(line_num) = line_str.parse::<u32>() {
                                let source_info = SourceFrameInfo {
                                    column: None, // addr2line doesn't always provide column info
                                
                                return Ok(Some(source_info));
                            }
                        }
                    }
                }
            }
            _ => {
                debug!("addr2line source info resolution failed for address 0x{:x}", ip);
            }
        }
        
        Ok(None)
    /// Extract source information from DWARF debug information
    fn extract_source_info_from_dwarf(&self, ip: usize) -> crate::error::Result<()> {
        // This would integrate with a DWARF parser library like gimli
        // For now, this is a placeholder for future DWARF integration
        debug!("DWARF debug info parsing not yet implemented for address 0x{:x}", ip);
        Ok(None)
    /// Extract module name from symbol name
    fn extract_module_name(&self, symbol_name: &str) -> Option<String> {
        // Try to extract module/namespace from symbol
        if let Some(last_colon) = symbol_name.rfind("::") {
            let module_part = &symbol_name[..last_colon];
            if let Some(second_last_colon) = module_part.rfind("::") {
                return Some(module_part[second_last_colon + 2..].to_string());
            } else {
                return Some(module_part.to_string());
            }
        }
        
        None
    /// Demangle function name to extract CURSED function names
    fn demangle_function_name(&self, mangled_name: &str) -> String {
        // Try Rust demangling first
        if let Ok(demangled) = rustc_demangle::try_demangle(mangled_name) {
            let demangled_str = format!("{:#}", demangled);
            
            // Look for CURSED function patterns
            if let Some(cursed_name) = self.extract_cursed_function_name(&demangled_str) {
                return cursed_name;
            return demangled_str;
        // Fall back to extracting CURSED patterns from mangled name
        if let Some(cursed_name) = self.extract_cursed_function_name(mangled_name) {
            return cursed_name;
        mangled_name.to_string()
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
        // Look for CURSED keywords
        let cursed_keywords = [
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
    /// Determine if a frame should be included based on configuration
    fn should_include_frame(&self, frame: &RawStackFrame) -> bool {
        // Skip system frames if configured
        if self.config.skip_system_frames {
            if let Some(symbol) = &frame.symbol_name {
                if self.is_system_frame(symbol) {
                    return false;
                }
            }
        // Only include CURSED frames if configured
        if self.config.cursed_frames_only && !frame.is_cursed_frame {
            return false;
        true
    /// Check if a symbol represents a system/runtime frame
    fn is_system_frame(&self, symbol_name: &str) -> bool {
        let system_patterns = [
            "backtrace::", "panic_", "abort"
        ];
        
        for pattern in &system_patterns {
            if symbol_name.contains(pattern) {
                return true;
            }
        }
        
        false
    /// Walk stack with additional context information
    #[instrument(skip(self))]
    pub fn walk_stack_with_context(
    ) -> crate::error::Result<()> {
        let frames = self.walk_stack()?;
        
        Ok(ContextualStackWalk {
        })
    /// Get statistics
    pub fn get_statistics(&self) -> crate::error::Result<()> {
        self.stats.lock()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::Runtime("Failed to access stack walk statistics".to_string()))
    /// Clear symbol cache
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.symbol_cache.lock() {
            cache.clear();
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
    /// Thread where stack was captured
    /// Goroutine ID (if applicable)
    /// Timestamp when captured
    /// Configuration used for walking
impl ContextualStackWalk {
    /// Get CURSED frames only
    pub fn cursed_frames(&self) -> Vec<&RawStackFrame> {
        self.frames.iter().filter(|f| f.is_cursed_frame).collect()
    /// Get top frame (most recent)
    pub fn top_frame(&self) -> Option<&RawStackFrame> {
        self.frames.first()
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
        for (index, frame) in self.frames.iter().enumerate() {
            writeln!(f, "  #{}: {}", index, frame)?;
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
/// Initialize global stack walker with configuration
pub fn initialize_global_stack_walker(config: StackWalkConfig) {
    let walker = StackWalker::with_config(config);
    let _ = GLOBAL_STACK_WALKER.set(Arc::new(Mutex::new(walker)));
/// Convenience function to walk current stack
pub fn walk_current_stack() -> crate::error::Result<()> {
    let walker = get_global_stack_walker();
    let result = if let Ok(w) = walker.lock() {
        w.walk_stack()
    } else {
        Err(CursedError::Runtime("Failed to access global stack walker".to_string()))
    result
