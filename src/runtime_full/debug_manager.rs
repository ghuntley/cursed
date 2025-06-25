/// Centralized debug information management system
///
/// Manages debug information, source file tracking, symbol resolution,
/// and provides thread-safe access to debug data for runtime debugging.

use crate::error::CursedError;
// use crate::runtime::debug_info::{DebugInfo, EnhancedStackFrame, VariableInfo, SymbolInfo, SymbolResolver};
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, Duration};

/// Source file information
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// File path
    /// File content (cached)
    /// Line offsets for quick line lookup
    /// Last modification time
    /// File size in bytes
    /// Whether file is cached
impl SourceFile {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        let metadata = fs::metadata(&path).ok();
        
        SourceFile {
        }
    }

    /// Load file content and build line index
    pub fn load_content(&mut self) -> crate::error::Result<()> {
        let content = fs::read_to_string(&self.path)
            .map_err(|e| CursedError::Runtime(format!("Failed to read file {}: {}", self.path.display(), e)))?;

        // Build line offsets for quick line lookup
        let mut offsets = vec![0];
        for (i, ch) in content.char_indices() {
            if ch == '\n' {
                offsets.push(i + 1);
            }
        }

        self.content = Some(content);
        self.line_offsets = offsets;
        self.is_cached = true;

        Ok(())
    /// Get a specific line (1-based indexing)
    pub fn get_line(&self, line_number: u32) -> Option<String> {
        let content = self.content.as_ref()?;
        let line_idx = (line_number as usize).saturating_sub(1);
        
        if line_idx >= self.line_offsets.len() {
            return None;
        let start = self.line_offsets[line_idx];
        let end = if line_idx + 1 < self.line_offsets.len() {
            self.line_offsets[line_idx + 1].saturating_sub(1)
        } else {
            content.len()

        content.get(start..end).map(|s| s.to_string())
    /// Get a range of lines with context
    pub fn get_lines_with_context(&self, line_number: u32, context: u32) -> Option<Vec<(u32, String)>> {
        let content = self.content.as_ref()?;
        let start_line = line_number.saturating_sub(context).max(1);
        let end_line = line_number + context;
        
        let mut lines = Vec::new();
        for line_num in start_line..=end_line {
            if let Some(line_content) = self.get_line(line_num) {
                lines.push((line_num, line_content));
            }
        }

        if lines.is_empty() { None } else { Some(lines) }
    }

    /// Check if file needs reloading
    pub fn needs_reload(&self) -> bool {
        if !self.is_cached {
            return true;
        if let Some(last_modified) = self.modified {
            if let Ok(metadata) = fs::metadata(&self.path) {
                if let Ok(current_modified) = metadata.modified() {
                    return current_modified > last_modified;
                }
            }
        false
    }
}

/// Function debug information
#[derive(Debug, Clone)]
pub struct FunctionDebugInfo {
    /// Function name
    /// Source file
    /// Start line number
    /// End line number (if known)
    /// Parameter information
    /// Local variables
    /// Instruction pointer ranges
    /// Whether function is inlined
    /// Module or namespace
impl FunctionDebugInfo {
    pub fn new(name: String, file_path: PathBuf, start_line: u32) -> Self {
        FunctionDebugInfo {
        }
    }

    pub fn with_end_line(mut self, end_line: u32) -> Self {
        self.end_line = Some(end_line);
        self
    pub fn with_parameter(mut self, param: VariableInfo) -> Self {
        self.parameters.push(param);
        self
    pub fn with_local_variable(mut self, var: VariableInfo) -> Self {
        self.local_variables.push(var);
        self
    pub fn with_ip_range(mut self, start: usize, end: usize) -> Self {
        self.ip_ranges.push((start, end));
        self
    pub fn with_module(mut self, module_name: String) -> Self {
        self.module_name = Some(module_name);
        self
    /// Check if an instruction pointer is within this function
    pub fn contains_ip(&self, ip: usize) -> bool {
        self.ip_ranges.iter().any(|(start, end)| ip >= *start && ip < *end)
    }
}

/// Configuration for debug manager
#[derive(Debug, Clone)]
pub struct DebugManagerConfig {
    /// Whether to cache source files
    /// Maximum number of files to cache
    /// Whether to auto-reload modified files
    /// Cache expiration time
    /// Whether to resolve symbols automatically
    /// Maximum symbol resolution depth
impl Default for DebugManagerConfig {
    fn default() -> Self {
        DebugManagerConfig {
            cache_expiration: Duration::from_secs(300), // 5 minutes
        }
    }
/// Statistics for debug manager
#[derive(Debug, Default, Clone)]
pub struct DebugManagerStats {
    /// Number of source files tracked
    /// Number of cached files
    /// Number of functions tracked
    /// Number of symbol resolutions
    /// Number of cache hits
    /// Number of cache misses
    /// Number of file reloads
/// Main debug information manager
pub struct DebugManager {
    /// Configuration
    /// Source file cache
    /// Function debug information
    /// Instruction pointer to function mapping
    /// Symbol resolver
    /// Debug statistics
    /// Source location cache
impl std::fmt::Debug for DebugManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DebugManager")
            .field("config", &self.config)
            .field("source_files", &"<cached source files>")
            .field("functions", &"<function debug info>")
            .field("ip_to_function", &"<ip mapping>")
            .field("symbol_resolver", &"<symbol resolver>")
            .field("stats", &"<debug stats>")
            .field("location_cache", &"<location cache>")
            .finish()
    }
}

impl DebugManager {
    /// Create a new debug manager
    pub fn new() -> Self {
        DebugManager {
        }
    }

    /// Create debug manager with custom configuration
    pub fn with_config(config: DebugManagerConfig) -> Self {
        DebugManager {
        }
    }

    /// Set symbol resolver
    pub fn set_symbol_resolver<R>(&self, resolver: R) -> crate::error::Result<()>
    where
    {
        if let Ok(mut resolver_lock) = self.symbol_resolver.lock() {
            *resolver_lock = Some(Box::new(resolver));
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to set symbol resolver".to_string()))
        }
    }

    /// Register a source file
    pub fn register_source_file<P: AsRef<Path>>(&self, path: P) -> crate::error::Result<()> {
        let path = path.as_ref().to_path_buf();
        let mut source_file = SourceFile::new(&path);

        if self.config.cache_source_files {
            source_file.load_content()?;
        if let Ok(mut files) = self.source_files.write() {
            files.insert(path, source_file);
            
            // Update stats
            if let Ok(mut stats) = self.stats.lock() {
                stats.files_tracked = files.len();
                if self.config.cache_source_files {
                    stats.files_cached += 1;
                }
            }
            
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to register source file".to_string()))
        }
    }

    /// Register function debug information
    pub fn register_function(&self, function_info: FunctionDebugInfo) -> crate::error::Result<()> {
        let function_name = function_info.name.clone();
        
        // Update IP to function mapping
        if let Ok(mut ip_map) = self.ip_to_function.write() {
            for (start_ip, end_ip) in &function_info.ip_ranges {
                for ip in *start_ip..*end_ip {
                    ip_map.insert(ip, function_name.clone());
                }
            }
        if let Ok(mut functions) = self.functions.write() {
            functions.insert(function_name, function_info);
            
            // Update stats
            if let Ok(mut stats) = self.stats.lock() {
                stats.functions_tracked = functions.len();
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to register function".to_string()))
        }
    }

    /// Get source file content
    pub fn get_source_file<P: AsRef<Path>>(&self, path: P) -> crate::error::Result<()> {
        let path = path.as_ref().to_path_buf();
        
        if let Ok(mut files) = self.source_files.write() {
            if let Some(file) = files.get_mut(&path) {
                // Check if file needs reloading
                if self.config.auto_reload_files && file.needs_reload() {
                    file.load_content()?;
                    if let Ok(mut stats) = self.stats.lock() {
                        stats.file_reloads += 1;
                    }
                }
                
                if let Ok(mut stats) = self.stats.lock() {
                    stats.cache_hits += 1;
                Ok(Some(file.clone()))
            } else {
                if let Ok(mut stats) = self.stats.lock() {
                    stats.cache_misses += 1;
                }
                Ok(None)
            }
        } else {
            Err(CursedError::Runtime("Failed to access source files".to_string()))
        }
    }

    /// Get function debug information by name
    pub fn get_function(&self, name: &str) -> crate::error::Result<()> {
        if let Ok(functions) = self.functions.read() {
            Ok(functions.get(name).cloned())
        } else {
            Err(CursedError::Runtime("Failed to access function information".to_string()))
        }
    }

    /// Add function debug information
    pub fn add_function_debug(&self, name: String, debug_info: crate::runtime::debug_info::DebugInfo) -> crate::error::Result<()> {
        let function_debug = FunctionDebugInfo::new(name.clone(), debug_info.file_path.clone(), debug_info.line)
            .with_end_line(debug_info.line)
            .with_module(name.clone()); // Use name as module for now

        if let Ok(mut functions) = self.functions.write() {
            functions.insert(name, function_debug);
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to add function debug info".to_string()))
        }
    }

    /// Get function debug information by instruction pointer
    pub fn get_function_by_ip(&self, ip: usize) -> crate::error::Result<()> {
        // First try direct IP lookup
        if let Ok(ip_map) = self.ip_to_function.read() {
            if let Some(function_name) = ip_map.get(&ip) {
                return self.get_function(function_name);
            }
        }

        // Fallback: search through all functions
        if let Ok(functions) = self.functions.read() {
            for function_info in functions.values() {
                if function_info.contains_ip(ip) {
                    return Ok(Some(function_info.clone()));
                }
            }
        Ok(None)
    /// Resolve symbol information for an instruction pointer
    pub fn resolve_symbol(&self, ip: usize) -> crate::error::Result<()> {
        // Check cache first
        if let Ok(cache) = self.location_cache.read() {
            if let Some(debug_info) = cache.get(&ip) {
                let symbol_info = SymbolInfo {
                
                if let Ok(mut stats) = self.stats.lock() {
                    stats.cache_hits += 1;
                return Ok(Some(symbol_info));
            }
        }

        // Try symbol resolver
        if let Ok(resolver_lock) = self.symbol_resolver.lock() {
            if let Some(resolver) = resolver_lock.as_ref() {
                let symbol = resolver.resolve_symbol(ip);
                
                if let Ok(mut stats) = self.stats.lock() {
                    stats.symbol_resolutions += 1;
                return Ok(symbol);
            }
        }

        // Try function lookup
        if let Some(function_info) = self.get_function_by_ip(ip)? {
            let symbol_info = SymbolInfo {
            
            return Ok(Some(symbol_info));
        if let Ok(mut stats) = self.stats.lock() {
            stats.cache_misses += 1;
        Ok(None)
    /// Get source code snippet around a location
    pub fn get_source_snippet(
    ) -> crate::error::Result<()> {
        if let Some(source_file) = self.get_source_file(file_path)? {
            if let Some(lines) = source_file.get_lines_with_context(line, context_lines) {
                let mut snippet = String::new();
                
                for (line_num, line_content) in lines {
                    let marker = if line_num == line { ">" } else { " " };
                    snippet.push_str(&format!("{} {:4} | {}\n", marker, line_num, line_content));
                Ok(snippet)
            } else {
                Err(CursedError::Runtime(format!("Line {} not found in file {}", line, file_path.display())))
            }
        } else {
            Err(CursedError::Runtime(format!("Source file not found: {}", file_path.display())))
        }
    }

    /// Create enhanced stack frame from instruction pointer
    pub fn create_enhanced_frame(&self, ip: usize, frame_index: usize) -> crate::error::Result<()> {
        if let Some(symbol_info) = self.resolve_symbol(ip)? {
            let debug_info = DebugInfo::new(
            ).with_instruction_pointer(ip);

            let mut frame = EnhancedStackFrame::new(debug_info, frame_index);

            // Add function variables if available
            if let Some(function_info) = self.get_function_by_ip(ip)? {
                for var in function_info.local_variables {
                    frame = frame.with_variable(var);
                }
            }

            Ok(Some(frame))
        } else {
            Ok(None)
        }
    }

    /// Cache debug location
    pub fn cache_location(&self, ip: usize, debug_info: DebugInfo) -> crate::error::Result<()> {
        if let Ok(mut cache) = self.location_cache.write() {
            cache.insert(ip, debug_info);
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to cache debug location".to_string()))
        }
    }

    /// Get debug manager statistics
    pub fn get_statistics(&self) -> crate::error::Result<()> {
        if let Ok(stats) = self.stats.lock() {
            Ok(stats.clone())
        } else {
            Err(CursedError::Runtime("Failed to get statistics".to_string()))
        }
    }

    /// Clear caches
    pub fn clear_caches(&self) -> crate::error::Result<()> {
        if let Ok(mut cache) = self.location_cache.write() {
            cache.clear();
        if let Ok(mut files) = self.source_files.write() {
            for file in files.values_mut() {
                file.content = None;
                file.is_cached = false;
            }
        }

        if let Ok(mut stats) = self.stats.lock() {
            stats.cache_hits = 0;
            stats.cache_misses = 0;
            stats.files_cached = 0;
        Ok(())
    }
}

impl Default for DebugManager {
    fn default() -> Self {
        Self::new()
    }
}

