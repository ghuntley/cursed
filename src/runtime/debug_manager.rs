/// Centralized debug information management system
///
/// Manages debug information, source file tracking, symbol resolution,
/// and provides thread-safe access to debug data for runtime debugging.

use crate::error::Error as CursedError;
use crate::runtime::debug_info::{DebugInfo, EnhancedStackFrame, VariableInfo, SymbolInfo, SymbolResolver};
use crate::error::Error;
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
    pub path: PathBuf,
    /// File content (cached)
    pub content: Option<String>,
    /// Line offsets for quick line lookup
    pub line_offsets: Vec<usize>,
    /// Last modification time
    pub modified: Option<SystemTime>,
    /// File size in bytes
    pub size: Option<u64>,
    /// Whether file is cached
    pub is_cached: bool,
}

impl SourceFile {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        let metadata = fs::metadata(&path).ok();
        
        SourceFile {
            path,
            content: None,
            line_offsets: Vec::new(),
            modified: metadata.as_ref().and_then(|m| m.modified().ok()),
            size: metadata.as_ref().map(|m| m.len()),
            is_cached: false,
        }
    }

    /// Load file content and build line index
    pub fn load_content(&mut self) -> Result<(), Error> {
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
    }

    /// Get a specific line (1-based indexing)
    pub fn get_line(&self, line_number: u32) -> Option<String> {
        let content = self.content.as_ref()?;
        let line_idx = (line_number as usize).saturating_sub(1);
        
        if line_idx >= self.line_offsets.len() {
            return None;
        }

        let start = self.line_offsets[line_idx];
        let end = if line_idx + 1 < self.line_offsets.len() {
            self.line_offsets[line_idx + 1].saturating_sub(1)
        } else {
            content.len()
        };

        content.get(start..end).map(|s| s.to_string())
    }

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
        }

        if let Some(last_modified) = self.modified {
            if let Ok(metadata) = fs::metadata(&self.path) {
                if let Ok(current_modified) = metadata.modified() {
                    return current_modified > last_modified;
                }
            }
        }

        false
    }
}

/// Function debug information
#[derive(Debug, Clone)]
pub struct FunctionDebugInfo {
    /// Function name
    pub name: String,
    /// Source file
    pub file_path: PathBuf,
    /// Start line number
    pub start_line: u32,
    /// End line number (if known)
    pub end_line: Option<u32>,
    /// Parameter information
    pub parameters: Vec<VariableInfo>,
    /// Local variables
    pub local_variables: Vec<VariableInfo>,
    /// Instruction pointer ranges
    pub ip_ranges: Vec<(usize, usize)>,
    /// Whether function is inlined
    pub is_inlined: bool,
    /// Module or namespace
    pub module_name: Option<String>,
}

impl FunctionDebugInfo {
    pub fn new(name: String, file_path: PathBuf, start_line: u32) -> Self {
        FunctionDebugInfo {
            name,
            file_path,
            start_line,
            end_line: None,
            parameters: Vec::new(),
            local_variables: Vec::new(),
            ip_ranges: Vec::new(),
            is_inlined: false,
            module_name: None,
        }
    }

    pub fn with_end_line(mut self, end_line: u32) -> Self {
        self.end_line = Some(end_line);
        self
    }

    pub fn with_parameter(mut self, param: VariableInfo) -> Self {
        self.parameters.push(param);
        self
    }

    pub fn with_local_variable(mut self, var: VariableInfo) -> Self {
        self.local_variables.push(var);
        self
    }

    pub fn with_ip_range(mut self, start: usize, end: usize) -> Self {
        self.ip_ranges.push((start, end));
        self
    }

    pub fn with_module(mut self, module_name: String) -> Self {
        self.module_name = Some(module_name);
        self
    }

    /// Check if an instruction pointer is within this function
    pub fn contains_ip(&self, ip: usize) -> bool {
        self.ip_ranges.iter().any(|(start, end)| ip >= *start && ip < *end)
    }
}

/// Configuration for debug manager
#[derive(Debug, Clone)]
pub struct DebugManagerConfig {
    /// Whether to cache source files
    pub cache_source_files: bool,
    /// Maximum number of files to cache
    pub max_cached_files: usize,
    /// Whether to auto-reload modified files
    pub auto_reload_files: bool,
    /// Cache expiration time
    pub cache_expiration: Duration,
    /// Whether to resolve symbols automatically
    pub auto_resolve_symbols: bool,
    /// Maximum symbol resolution depth
    pub max_symbol_depth: usize,
}

impl Default for DebugManagerConfig {
    fn default() -> Self {
        DebugManagerConfig {
            cache_source_files: true,
            max_cached_files: 100,
            auto_reload_files: true,
            cache_expiration: Duration::from_secs(300), // 5 minutes
            auto_resolve_symbols: true,
            max_symbol_depth: 50,
        }
    }
}

/// Statistics for debug manager
#[derive(Debug, Default, Clone)]
pub struct DebugManagerStats {
    /// Number of source files tracked
    pub files_tracked: usize,
    /// Number of cached files
    pub files_cached: usize,
    /// Number of functions tracked
    pub functions_tracked: usize,
    /// Number of symbol resolutions
    pub symbol_resolutions: u64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
    /// Number of file reloads
    pub file_reloads: u64,
}

/// Main debug information manager
pub struct DebugManager {
    /// Configuration
    config: DebugManagerConfig,
    /// Source file cache
    source_files: Arc<RwLock<HashMap<PathBuf, SourceFile>>>,
    /// Function debug information
    functions: Arc<RwLock<HashMap<String, FunctionDebugInfo>>>,
    /// Instruction pointer to function mapping
    ip_to_function: Arc<RwLock<HashMap<usize, String>>>,
    /// Symbol resolver
    symbol_resolver: Arc<Mutex<Option<Box<dyn SymbolResolver + Send + Sync>>>>,
    /// Debug statistics
    stats: Arc<Mutex<DebugManagerStats>>,
    /// Source location cache
    location_cache: Arc<RwLock<HashMap<usize, DebugInfo>>>,
}

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
            config: DebugManagerConfig::default(),
            source_files: Arc::new(RwLock::new(HashMap::new())),
            functions: Arc::new(RwLock::new(HashMap::new())),
            ip_to_function: Arc::new(RwLock::new(HashMap::new())),
            symbol_resolver: Arc::new(Mutex::new(None)),
            stats: Arc::new(Mutex::new(DebugManagerStats::default())),
            location_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create debug manager with custom configuration
    pub fn with_config(config: DebugManagerConfig) -> Self {
        DebugManager {
            config,
            source_files: Arc::new(RwLock::new(HashMap::new())),
            functions: Arc::new(RwLock::new(HashMap::new())),
            ip_to_function: Arc::new(RwLock::new(HashMap::new())),
            symbol_resolver: Arc::new(Mutex::new(None)),
            stats: Arc::new(Mutex::new(DebugManagerStats::default())),
            location_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Set symbol resolver
    pub fn set_symbol_resolver<R>(&self, resolver: R) -> Result<(), Error>
    where
        R: SymbolResolver + Send + Sync + 'static,
    {
        if let Ok(mut resolver_lock) = self.symbol_resolver.lock() {
            *resolver_lock = Some(Box::new(resolver));
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to set symbol resolver".to_string()))
        }
    }

    /// Register a source file
    pub fn register_source_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let path = path.as_ref().to_path_buf();
        let mut source_file = SourceFile::new(&path);

        if self.config.cache_source_files {
            source_file.load_content()?;
        }

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
    pub fn register_function(&self, function_info: FunctionDebugInfo) -> Result<(), Error> {
        let function_name = function_info.name.clone();
        
        // Update IP to function mapping
        if let Ok(mut ip_map) = self.ip_to_function.write() {
            for (start_ip, end_ip) in &function_info.ip_ranges {
                for ip in *start_ip..*end_ip {
                    ip_map.insert(ip, function_name.clone());
                }
            }
        }

        if let Ok(mut functions) = self.functions.write() {
            functions.insert(function_name, function_info);
            
            // Update stats
            if let Ok(mut stats) = self.stats.lock() {
                stats.functions_tracked = functions.len();
            }
            
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to register function".to_string()))
        }
    }

    /// Get source file content
    pub fn get_source_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
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
                }
                
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
    pub fn get_function(&self, name: &str) -> Result<(), Error> {
        if let Ok(functions) = self.functions.read() {
            Ok(functions.get(name).cloned())
        } else {
            Err(CursedError::Runtime("Failed to access function information".to_string()))
        }
    }

    /// Add function debug information
    pub fn add_function_debug(&self, name: String, debug_info: crate::runtime::debug_info::DebugInfo) -> Result<(), Error> {
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
    pub fn get_function_by_ip(&self, ip: usize) -> Result<(), Error> {
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
        }

        Ok(None)
    }

    /// Resolve symbol information for an instruction pointer
    pub fn resolve_symbol(&self, ip: usize) -> Result<(), Error> {
        // Check cache first
        if let Ok(cache) = self.location_cache.read() {
            if let Some(debug_info) = cache.get(&ip) {
                let symbol_info = SymbolInfo {
                    name: debug_info.function_name.clone(),
                    file: Some(debug_info.file_path.clone()),
                    line: Some(debug_info.line),
                    column: Some(debug_info.column),
                    offset: debug_info.instruction_pointer.map(|base| ip.saturating_sub(base)),
                };
                
                if let Ok(mut stats) = self.stats.lock() {
                    stats.cache_hits += 1;
                }
                
                return Ok(Some(symbol_info));
            }
        }

        // Try symbol resolver
        if let Ok(resolver_lock) = self.symbol_resolver.lock() {
            if let Some(resolver) = resolver_lock.as_ref() {
                let symbol = resolver.resolve_symbol(ip);
                
                if let Ok(mut stats) = self.stats.lock() {
                    stats.symbol_resolutions += 1;
                }
                
                return Ok(symbol);
            }
        }

        // Try function lookup
        if let Some(function_info) = self.get_function_by_ip(ip)? {
            let symbol_info = SymbolInfo {
                name: function_info.name,
                file: Some(function_info.file_path),
                line: Some(function_info.start_line),
                column: Some(1),
                offset: None,
            };
            
            return Ok(Some(symbol_info));
        }

        if let Ok(mut stats) = self.stats.lock() {
            stats.cache_misses += 1;
        }

        Ok(None)
    }

    /// Get source code snippet around a location
    pub fn get_source_snippet(
        &self,
        file_path: &Path,
        line: u32,
        context_lines: u32,
    ) -> Result<(), Error> {
        if let Some(source_file) = self.get_source_file(file_path)? {
            if let Some(lines) = source_file.get_lines_with_context(line, context_lines) {
                let mut snippet = String::new();
                
                for (line_num, line_content) in lines {
                    let marker = if line_num == line { ">" } else { " " };
                    snippet.push_str(&format!("{} {:4} | {}\n", marker, line_num, line_content));
                }
                
                Ok(snippet)
            } else {
                Err(CursedError::Runtime(format!("Line {} not found in file {}", line, file_path.display())))
            }
        } else {
            Err(CursedError::Runtime(format!("Source file not found: {}", file_path.display())))
        }
    }

    /// Create enhanced stack frame from instruction pointer
    pub fn create_enhanced_frame(&self, ip: usize, frame_index: usize) -> Result<(), Error> {
        if let Some(symbol_info) = self.resolve_symbol(ip)? {
            let debug_info = DebugInfo::new(
                symbol_info.file.as_ref().unwrap_or(&PathBuf::from("unknown")),
                symbol_info.line.unwrap_or(0),
                symbol_info.column.unwrap_or(0),
                symbol_info.name.clone(),
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
    pub fn cache_location(&self, ip: usize, debug_info: DebugInfo) -> Result<(), Error> {
        if let Ok(mut cache) = self.location_cache.write() {
            cache.insert(ip, debug_info);
            Ok(())
        } else {
            Err(CursedError::Runtime("Failed to cache debug location".to_string()))
        }
    }

    /// Get debug manager statistics
    pub fn get_statistics(&self) -> Result<(), Error> {
        if let Ok(stats) = self.stats.lock() {
            Ok(stats.clone())
        } else {
            Err(CursedError::Runtime("Failed to get statistics".to_string()))
        }
    }

    /// Clear caches
    pub fn clear_caches(&self) -> Result<(), Error> {
        if let Ok(mut cache) = self.location_cache.write() {
            cache.clear();
        }

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
        }

        Ok(())
    }
}

impl Default for DebugManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_source_file_creation() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.csd");
        
        // Create a test file
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "facts hello = \"world\"").unwrap();
        writeln!(file, "sus count = 42").unwrap();

        let mut source_file = SourceFile::new(&file_path);
        assert!(!source_file.is_cached);
        
        source_file.load_content().unwrap();
        assert!(source_file.is_cached);
        assert!(source_file.content.is_some());
        assert_eq!(source_file.line_offsets.len(), 3); // 0 + 2 lines
    }

    #[test]
    fn test_source_file_line_access() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.csd");
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "line 1").unwrap();
        writeln!(file, "line 2").unwrap();
        writeln!(file, "line 3").unwrap();

        let mut source_file = SourceFile::new(&file_path);
        source_file.load_content().unwrap();

        assert_eq!(source_file.get_line(1), Some("line 1".to_string()));
        assert_eq!(source_file.get_line(2), Some("line 2".to_string()));
        assert_eq!(source_file.get_line(3), Some("line 3".to_string()));
        assert_eq!(source_file.get_line(4), None);
    }

    #[test]
    fn test_function_debug_info() {
        let file_path = PathBuf::from("test.csd");
        let mut func_info = FunctionDebugInfo::new("test_func".to_string(), file_path, 10)
            .with_end_line(20)
            .with_ip_range(0x1000, 0x2000);

        let param = VariableInfo::new("param1".to_string(), "sus".to_string());
        func_info = func_info.with_parameter(param);

        assert_eq!(func_info.name, "test_func");
        assert_eq!(func_info.start_line, 10);
        assert_eq!(func_info.end_line, Some(20));
        assert!(func_info.contains_ip(0x1500));
        assert!(!func_info.contains_ip(0x3000));
        assert_eq!(func_info.parameters.len(), 1);
    }

    #[test]
    fn test_debug_manager_creation() {
        let manager = DebugManager::new();
        assert!(manager.get_statistics().is_ok());
        
        let custom_config = DebugManagerConfig {
            cache_source_files: false,
            ..Default::default()
        };
        let custom_manager = DebugManager::with_config(custom_config);
        assert!(!custom_manager.config.cache_source_files);
    }

    #[test]
    fn test_debug_manager_source_file_registration() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.csd");
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "facts test = true").unwrap();

        let manager = DebugManager::new();
        assert!(manager.register_source_file(&file_path).is_ok());
        
        let retrieved = manager.get_source_file(&file_path).unwrap();
        assert!(retrieved.is_some());
        assert!(retrieved.unwrap().is_cached);
    }

    #[test]
    fn test_debug_manager_function_registration() {
        let manager = DebugManager::new();
        let file_path = PathBuf::from("test.csd");
        
        let func_info = FunctionDebugInfo::new("test_function".to_string(), file_path, 10)
            .with_ip_range(0x1000, 0x2000);

        assert!(manager.register_function(func_info).is_ok());
        
        let retrieved = manager.get_function("test_function").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "test_function");
        
        let by_ip = manager.get_function_by_ip(0x1500).unwrap();
        assert!(by_ip.is_some());
        assert_eq!(by_ip.unwrap().name, "test_function");
    }

    #[test]
    fn test_debug_manager_statistics() {
        let manager = DebugManager::new();
        let stats = manager.get_statistics().unwrap();
        
        assert_eq!(stats.files_tracked, 0);
        assert_eq!(stats.functions_tracked, 0);
        assert_eq!(stats.cache_hits, 0);
        assert_eq!(stats.cache_misses, 0);
    }

    #[test]
    fn test_source_snippet_extraction() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.csd");
        
        let mut file = File::create(&file_path).unwrap();
        for i in 1..=10 {
            writeln!(file, "line {}", i).unwrap();
        }

        let manager = DebugManager::new();
        manager.register_source_file(&file_path).unwrap();
        
        let snippet = manager.get_source_snippet(&file_path, 5, 2).unwrap();
        assert!(snippet.contains("line 3"));
        assert!(snippet.contains("line 7"));
        assert!(snippet.contains("> 5"));
    }
}
