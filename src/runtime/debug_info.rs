//! CURSED Debug Information Types
//!
//! Advanced debugging infrastructure providing:
//! - Stack trace capture and management with LLVM integration
//! - Enhanced configuration for debugging behavior
//! - Symbol information and resolution
//! - Source mapping and line number resolution
//! - Symbol table management
//! - Enhanced stack walking capabilities

use crate::error::{CursedError, SourceLocation};
use crate::debug::{DebugSymbol, DebugSymbolType};
use std::collections::{HashMap, BTreeMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, Duration};

/// Configuration for enhanced stack trace behavior
#[derive(Debug, Clone)]
pub struct EnhancedStackTraceConfig {
    /// Maximum stack depth to capture
    pub max_depth: usize,
    /// Enable symbol resolution
    pub resolve_symbols: bool,
    /// Include source file information
    pub include_source: bool,
    /// Include line number information
    pub include_line_numbers: bool,
    /// Include column number information
    pub include_columns: bool,
    /// Enable LLVM debug info integration
    pub llvm_debug_info: bool,
    /// Show function parameters in stack traces
    pub show_parameters: bool,
    /// Show local variables in stack traces
    pub show_locals: bool,
    /// Enable inline function expansion
    pub expand_inlines: bool,
    /// Include memory addresses
    pub include_addresses: bool,
    /// Enable async stack traces for goroutines
    pub async_stack_traces: bool,
    /// Capture performance metrics during tracing
    pub capture_performance: bool,
    /// Stack trace format preference
    pub format: StackTraceFormat,
}

impl Default for EnhancedStackTraceConfig {
    fn default() -> Self {
        Self {
            max_depth: 100,
            resolve_symbols: true,
            include_source: true,
            include_line_numbers: true,
            include_columns: true,
            llvm_debug_info: true,
            show_parameters: false,
            show_locals: false,
            expand_inlines: true,
            include_addresses: false,
            async_stack_traces: true,
            capture_performance: false,
            format: StackTraceFormat::Standard,
        }
    }
}

/// Stack trace output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackTraceFormat {
    /// Standard format: function at file:line
    Standard,
    /// Compact format: minimal information
    Compact,
    /// Verbose format: all available information
    Verbose,
    /// JSON format for programmatic consumption
    Json,
    /// Custom format with user-defined template
    Custom,
}

/// Comprehensive stack trace capture and management system
#[derive(Debug)]
pub struct StackTraceCapture {
    /// Configuration for stack trace behavior
    config: EnhancedStackTraceConfig,
    /// Symbol resolver for address translation
    symbol_resolver: Arc<SymbolResolver>,
    /// Cache for resolved symbols
    symbol_cache: Arc<RwLock<HashMap<u64, SymbolInfo>>>,
    /// Source file mappings
    source_mappings: Arc<RwLock<HashMap<PathBuf, SourceFileInfo>>>,
    /// LLVM debug information
    llvm_debug_info: Arc<RwLock<LlvmDebugInfo>>,
    /// Stack trace statistics
    statistics: Arc<RwLock<StackTraceStats>>,
    /// Active capture sessions
    active_sessions: Arc<RwLock<HashMap<String, CaptureSession>>>,
}

impl StackTraceCapture {
    /// Create a new stack trace capture system
    pub fn new(config: EnhancedStackTraceConfig) -> Self {
        Self {
            config,
            symbol_resolver: Arc::new(SymbolResolver::new()),
            symbol_cache: Arc::new(RwLock::new(HashMap::new())),
            source_mappings: Arc::new(RwLock::new(HashMap::new())),
            llvm_debug_info: Arc::new(RwLock::new(LlvmDebugInfo::new())),
            statistics: Arc::new(RwLock::new(StackTraceStats::new())),
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Capture current stack trace
    pub fn capture_stack_trace(&self) -> Result<Vec<StackFrame>, CursedError> {
        let start_time = if self.config.capture_performance {
            Some(SystemTime::now())
        } else {
            None
        };

        let mut frames = Vec::new();
        let mut frame_addresses = self.get_stack_addresses()?;

        // Limit to configured max depth
        if frame_addresses.len() > self.config.max_depth {
            frame_addresses.truncate(self.config.max_depth);
        }

        for (depth, address) in frame_addresses.iter().enumerate() {
            let frame = self.create_stack_frame(*address, depth)?;
            frames.push(frame);
        }

        // Update statistics
        if let Ok(mut stats) = self.statistics.write() {
            stats.traces_captured += 1;
            stats.total_frames += frames.len();
            stats.last_capture_time = Some(SystemTime::now());
            
            if let Some(start) = start_time {
                if let Ok(duration) = SystemTime::now().duration_since(start) {
                    stats.avg_capture_time = Some(
                        stats.avg_capture_time
                            .map(|avg| Duration::from_nanos((avg.as_nanos() + duration.as_nanos()) as u64 / 2))
                            .unwrap_or(duration)
                    );
                }
            }
        }

        Ok(frames)
    }

    /// Create a capture session for targeted debugging
    pub fn start_capture_session(&self, name: String, target_function: Option<String>) -> Result<String, CursedError> {
        let session_id = format!("capture_{}_{}", 
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0)).as_nanos(),
            name
        );

        let session = CaptureSession {
            id: session_id.clone(),
            name,
            start_time: SystemTime::now(),
            target_function,
            captured_traces: Vec::new(),
            config: self.config.clone(),
        };

        if let Ok(mut sessions) = self.active_sessions.write() {
            sessions.insert(session_id.clone(), session);
        }

        Ok(session_id)
    }

    /// End a capture session and return results
    pub fn end_capture_session(&self, session_id: &str) -> Result<CaptureSession, CursedError> {
        if let Ok(mut sessions) = self.active_sessions.write() {
            sessions.remove(session_id)
                .ok_or_else(|| CursedError::RuntimeError(format!("Session '{}' not found", session_id)))
        } else {
            Err(CursedError::RuntimeError("Failed to access capture sessions".to_string()))
        }
    }

    /// Capture stack trace at specific address
    pub fn capture_at_address(&self, address: u64) -> Result<StackFrame, CursedError> {
        self.create_stack_frame(address, 0)
    }

    /// Add source file mapping for better debugging
    pub fn add_source_mapping(&self, source_file: PathBuf, info: SourceFileInfo) -> Result<(), CursedError> {
        if let Ok(mut mappings) = self.source_mappings.write() {
            mappings.insert(source_file, info);
            Ok(())
        } else {
            Err(CursedError::RuntimeError("Failed to add source mapping".to_string()))
        }
    }

    /// Update LLVM debug information
    pub fn update_llvm_debug_info(&self, info: LlvmDebugInfo) -> Result<(), CursedError> {
        if let Ok(mut debug_info) = self.llvm_debug_info.write() {
            *debug_info = info;
            Ok(())
        } else {
            Err(CursedError::RuntimeError("Failed to update LLVM debug info".to_string()))
        }
    }

    /// Get stack trace statistics
    pub fn get_statistics(&self) -> Result<StackTraceStats, CursedError> {
        self.statistics.read()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::RuntimeError("Failed to read statistics".to_string()))
    }

    /// Configure stack trace behavior
    pub fn update_config(&mut self, config: EnhancedStackTraceConfig) {
        self.config = config;
    }

    // Private helper methods

    /// Get stack frame addresses using platform-specific methods
    fn get_stack_addresses(&self) -> Result<Vec<u64>, CursedError> {
        // Platform-specific stack walking implementation
        // This is a simplified version - real implementation would use
        // libunwind, backtrace, or similar libraries
        
        #[cfg(unix)]
        {
            self.get_unix_stack_addresses()
        }
        #[cfg(windows)]
        {
            self.get_windows_stack_addresses()
        }
        #[cfg(not(any(unix, windows)))]
        {
            // Fallback implementation
            Ok(vec![0x1000, 0x2000, 0x3000]) // Dummy addresses for compilation
        }
    }

    #[cfg(unix)]
    fn get_unix_stack_addresses(&self) -> Result<Vec<u64>, CursedError> {
        // Use libunwind or backtrace to get actual stack addresses
        // For now, return dummy data for compilation
        Ok(vec![0x401000, 0x401200, 0x401400])
    }

    #[cfg(windows)]
    fn get_windows_stack_addresses(&self) -> Result<Vec<u64>, CursedError> {
        // Use Windows debugging APIs to get stack addresses
        // For now, return dummy data for compilation
        Ok(vec![0x401000, 0x401200, 0x401400])
    }

    /// Create a stack frame from an address
    fn create_stack_frame(&self, address: u64, depth: usize) -> Result<StackFrame, CursedError> {
        let symbol_info = if self.config.resolve_symbols {
            self.symbol_resolver.resolve_address(address)?
        } else {
            SymbolInfo::unknown(address)
        };

        let source_location = if self.config.include_source {
            self.resolve_source_location(address)?
        } else {
            None
        };

        Ok(StackFrame {
            depth,
            address: if self.config.include_addresses { Some(address) } else { None },
            symbol_info,
            source_location,
            parameters: if self.config.show_parameters { 
                self.get_function_parameters(address)? 
            } else { 
                Vec::new() 
            },
            local_variables: if self.config.show_locals { 
                self.get_local_variables(address)? 
            } else { 
                Vec::new() 
            },
            inline_info: if self.config.expand_inlines { 
                self.get_inline_info(address)? 
            } else { 
                Vec::new() 
            },
        })
    }

    /// Resolve source location from address
    fn resolve_source_location(&self, address: u64) -> Result<Option<SourceLocation>, CursedError> {
        if let Ok(debug_info) = self.llvm_debug_info.read() {
            debug_info.get_source_location(address)
        } else {
            Ok(None)
        }
    }

    /// Get function parameters at address
    fn get_function_parameters(&self, _address: u64) -> Result<Vec<ParameterInfo>, CursedError> {
        // TODO: Implement parameter extraction from debug info
        Ok(Vec::new())
    }

    /// Get local variables at address
    fn get_local_variables(&self, _address: u64) -> Result<Vec<LocalVariableInfo>, CursedError> {
        // TODO: Implement local variable extraction from debug info
        Ok(Vec::new())
    }

    /// Get inline function information
    fn get_inline_info(&self, _address: u64) -> Result<Vec<InlineInfo>, CursedError> {
        // TODO: Implement inline function information extraction
        Ok(Vec::new())
    }
}

/// Individual stack frame information
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Frame depth (0 = current frame)
    pub depth: usize,
    /// Memory address (if available)
    pub address: Option<u64>,
    /// Symbol information
    pub symbol_info: SymbolInfo,
    /// Source location information
    pub source_location: Option<SourceLocation>,
    /// Function parameters
    pub parameters: Vec<ParameterInfo>,
    /// Local variables
    pub local_variables: Vec<LocalVariableInfo>,
    /// Inline function information
    pub inline_info: Vec<InlineInfo>,
}

/// Function parameter information for debugging
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
    /// Parameter value (if available)
    pub value: Option<String>,
    /// Parameter location in memory
    pub location: Option<u64>,
}

/// Local variable information for debugging
#[derive(Debug, Clone)]
pub struct LocalVariableInfo {
    /// Variable name
    pub name: String,
    /// Variable type
    pub var_type: String,
    /// Variable value (if available)
    pub value: Option<String>,
    /// Variable scope
    pub scope: String,
    /// Memory location
    pub location: Option<u64>,
}

/// Inline function information
#[derive(Debug, Clone)]
pub struct InlineInfo {
    /// Inline function name
    pub function_name: String,
    /// Source location where inlined
    pub inline_site: SourceLocation,
    /// Original function location
    pub original_location: SourceLocation,
}

/// Symbol information for debugging addresses
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    /// Symbol name (function, variable, etc.)
    pub name: String,
    /// Symbol type (function, variable, type, etc.)
    pub symbol_type: SymbolType,
    /// Base address of the symbol
    pub address: u64,
    /// Size of the symbol in bytes
    pub size: u32,
    /// Offset from base address
    pub offset: u32,
    /// Source file containing the symbol
    pub source_file: Option<PathBuf>,
    /// Line number in source file
    pub line_number: Option<u32>,
    /// Column number in source file
    pub column_number: Option<u32>,
    /// Module/library containing the symbol
    pub module_name: Option<String>,
    /// Whether this symbol has debugging information
    pub has_debug_info: bool,
    /// Symbol visibility (public, private, etc.)
    pub visibility: SymbolVisibility,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl SymbolInfo {
    /// Create an unknown symbol info for an address
    pub fn unknown(address: u64) -> Self {
        Self {
            name: format!("unknown_0x{:x}", address),
            symbol_type: SymbolType::Unknown,
            address,
            size: 0,
            offset: 0,
            source_file: None,
            line_number: None,
            column_number: None,
            module_name: None,
            has_debug_info: false,
            visibility: SymbolVisibility::Unknown,
            metadata: HashMap::new(),
        }
    }

    /// Create a function symbol info
    pub fn function(name: String, address: u64, size: u32) -> Self {
        Self {
            name,
            symbol_type: SymbolType::Function,
            address,
            size,
            offset: 0,
            source_file: None,
            line_number: None,
            column_number: None,
            module_name: None,
            has_debug_info: false,
            visibility: SymbolVisibility::Public,
            metadata: HashMap::new(),
        }
    }
}

/// Types of symbols for debugging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    /// Function symbol
    Function,
    /// Variable symbol
    Variable,
    /// Type definition
    Type,
    /// Module/namespace
    Module,
    /// Label
    Label,
    /// Unknown symbol type
    Unknown,
}

/// Symbol visibility levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolVisibility {
    /// Public symbol (exported)
    Public,
    /// Private symbol (internal)
    Private,
    /// Protected symbol
    Protected,
    /// Local symbol
    Local,
    /// Unknown visibility
    Unknown,
}

/// Symbol resolver for translating addresses to symbols
#[derive(Debug)]
pub struct SymbolResolver {
    /// Symbol table mapping addresses to symbols
    symbol_table: Arc<RwLock<BTreeMap<u64, SymbolInfo>>>,
    /// Module information
    modules: Arc<RwLock<HashMap<String, ModuleInfo>>>,
    /// Symbol lookup cache
    lookup_cache: Arc<RwLock<HashMap<u64, SymbolInfo>>>,
    /// Resolution statistics
    stats: Arc<RwLock<ResolverStats>>,
}

impl SymbolResolver {
    /// Create a new symbol resolver
    pub fn new() -> Self {
        Self {
            symbol_table: Arc::new(RwLock::new(BTreeMap::new())),
            modules: Arc::new(RwLock::new(HashMap::new())),
            lookup_cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(ResolverStats::new())),
        }
    }

    /// Add a symbol to the resolver
    pub fn add_symbol(&self, symbol: SymbolInfo) -> Result<(), CursedError> {
        if let Ok(mut table) = self.symbol_table.write() {
            table.insert(symbol.address, symbol);
            Ok(())
        } else {
            Err(CursedError::RuntimeError("Failed to add symbol".to_string()))
        }
    }

    /// Resolve an address to symbol information
    pub fn resolve_address(&self, address: u64) -> Result<SymbolInfo, CursedError> {
        // Check cache first
        if let Ok(cache) = self.lookup_cache.read() {
            if let Some(symbol) = cache.get(&address) {
                if let Ok(mut stats) = self.stats.write() {
                    stats.cache_hits += 1;
                }
                return Ok(symbol.clone());
            }
        }

        // Search symbol table
        let symbol_info = if let Ok(table) = self.symbol_table.read() {
            table.range(..=address)
                .next_back()
                .and_then(|(base_addr, symbol)| {
                    let offset = address - base_addr;
                    if offset < symbol.size as u64 {
                        let mut resolved = symbol.clone();
                        resolved.offset = offset as u32;
                        Some(resolved)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| SymbolInfo::unknown(address))
        } else {
            SymbolInfo::unknown(address)
        };

        // Update cache
        if let Ok(mut cache) = self.lookup_cache.write() {
            cache.insert(address, symbol_info.clone());
            
            // Limit cache size
            if cache.len() > 10000 {
                let oldest_key = *cache.keys().next().unwrap();
                cache.remove(&oldest_key);
            }
        }

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.resolutions += 1;
            if symbol_info.symbol_type != SymbolType::Unknown {
                stats.successful_resolutions += 1;
            }
        }

        Ok(symbol_info)
    }

    /// Load symbols from a module
    pub fn load_module_symbols(&self, module_name: String, module_info: ModuleInfo) -> Result<(), CursedError> {
        // Add module info
        if let Ok(mut modules) = self.modules.write() {
            modules.insert(module_name.clone(), module_info.clone());
        }

        // Add symbols from module
        for symbol in module_info.symbols {
            self.add_symbol(symbol)?;
        }

        Ok(())
    }

    /// Get resolver statistics
    pub fn get_stats(&self) -> Result<ResolverStats, CursedError> {
        self.stats.read()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::RuntimeError("Failed to read resolver stats".to_string()))
    }

    /// Clear symbol cache
    pub fn clear_cache(&self) -> Result<(), CursedError> {
        if let Ok(mut cache) = self.lookup_cache.write() {
            cache.clear();
            Ok(())
        } else {
            Err(CursedError::RuntimeError("Failed to clear symbol cache".to_string()))
        }
    }
}

/// Module information for symbol resolution
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    /// Module name
    pub name: String,
    /// Base address in memory
    pub base_address: u64,
    /// Module size
    pub size: u64,
    /// Path to module file
    pub file_path: Option<PathBuf>,
    /// Symbols in this module
    pub symbols: Vec<SymbolInfo>,
    /// Module type (executable, library, etc.)
    pub module_type: ModuleType,
    /// Build ID or checksum
    pub build_id: Option<String>,
}

/// Types of modules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleType {
    /// Main executable
    Executable,
    /// Shared library
    SharedLibrary,
    /// Static library
    StaticLibrary,
    /// Object file
    ObjectFile,
    /// Unknown module type
    Unknown,
}

/// Source file information for debugging
#[derive(Debug, Clone)]
pub struct SourceFileInfo {
    /// File path
    pub path: PathBuf,
    /// Line number to address mappings
    pub line_mappings: BTreeMap<u32, Vec<u64>>,
    /// Address to line number mappings
    pub address_mappings: BTreeMap<u64, u32>,
    /// Functions defined in this file
    pub functions: Vec<FunctionInfo>,
    /// File hash for integrity checking
    pub file_hash: Option<String>,
    /// Compilation timestamp
    pub compiled_at: Option<SystemTime>,
}

/// Function information within a source file
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    /// Function name
    pub name: String,
    /// Start address
    pub start_address: u64,
    /// End address
    pub end_address: u64,
    /// Start line in source
    pub start_line: u32,
    /// End line in source
    pub end_line: u32,
    /// Parameter count
    pub parameter_count: u32,
    /// Local variable count
    pub local_count: u32,
}

/// LLVM debug information integration
#[derive(Debug, Clone)]
pub struct LlvmDebugInfo {
    /// Debug metadata
    debug_metadata: HashMap<u64, LlvmDebugMetadata>,
    /// Compilation units
    compilation_units: Vec<CompilationUnit>,
    /// Type information
    type_info: HashMap<String, TypeDebugInfo>,
}

impl LlvmDebugInfo {
    /// Create new LLVM debug info
    pub fn new() -> Self {
        Self {
            debug_metadata: HashMap::new(),
            compilation_units: Vec::new(),
            type_info: HashMap::new(),
        }
    }

    /// Get source location for an address
    pub fn get_source_location(&self, address: u64) -> Result<Option<SourceLocation>, CursedError> {
        if let Some(metadata) = self.debug_metadata.get(&address) {
            Ok(Some(SourceLocation {
                file: metadata.file_path.to_string_lossy().to_string(),
                line: metadata.line as usize,
                column: metadata.column as usize,
            }))
        } else {
            Ok(None)
        }
    }

    /// Add debug metadata for an address
    pub fn add_metadata(&mut self, address: u64, metadata: LlvmDebugMetadata) {
        self.debug_metadata.insert(address, metadata);
    }
}

/// LLVM debug metadata for a specific location
#[derive(Debug, Clone)]
pub struct LlvmDebugMetadata {
    /// Source file path
    pub file_path: PathBuf,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
    /// Function name
    pub function_name: Option<String>,
    /// Scope information
    pub scope: Option<String>,
}

/// Compilation unit information
#[derive(Debug, Clone)]
pub struct CompilationUnit {
    /// Unit name
    pub name: String,
    /// Source files in this unit
    pub source_files: Vec<PathBuf>,
    /// Producer (compiler) information
    pub producer: Option<String>,
    /// Language (CURSED)
    pub language: String,
}

/// Type debug information
#[derive(Debug, Clone)]
pub struct TypeDebugInfo {
    /// Type name
    pub name: String,
    /// Type size in bytes
    pub size: u32,
    /// Type alignment
    pub alignment: u32,
    /// Type kind (struct, function, etc.)
    pub kind: TypeKind,
    /// Member information (for structs/classes)
    pub members: Vec<TypeMember>,
}

/// Type kinds for debugging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeKind {
    /// Basic type (int, float, etc.)
    Basic,
    /// Struct type
    Struct,
    /// Function type
    Function,
    /// Pointer type
    Pointer,
    /// Array type
    Array,
    /// Enum type
    Enum,
    /// Union type
    Union,
}

/// Type member information
#[derive(Debug, Clone)]
pub struct TypeMember {
    /// Member name
    pub name: String,
    /// Member type
    pub member_type: String,
    /// Offset within the type
    pub offset: u32,
    /// Member size
    pub size: u32,
}

/// Statistics for stack trace capture
#[derive(Debug, Clone)]
pub struct StackTraceStats {
    /// Number of traces captured
    pub traces_captured: u64,
    /// Total frames captured across all traces
    pub total_frames: usize,
    /// Average capture time
    pub avg_capture_time: Option<Duration>,
    /// Last capture time
    pub last_capture_time: Option<SystemTime>,
    /// Cache hit rate for symbol resolution
    pub cache_hit_rate: f64,
}

impl StackTraceStats {
    pub fn new() -> Self {
        Self {
            traces_captured: 0,
            total_frames: 0,
            avg_capture_time: None,
            last_capture_time: None,
            cache_hit_rate: 0.0,
        }
    }
}

/// Statistics for symbol resolver
#[derive(Debug, Clone)]
pub struct ResolverStats {
    /// Total resolution attempts
    pub resolutions: u64,
    /// Successful resolutions
    pub successful_resolutions: u64,
    /// Cache hits
    pub cache_hits: u64,
    /// Success rate
    pub success_rate: f64,
}

impl ResolverStats {
    pub fn new() -> Self {
        Self {
            resolutions: 0,
            successful_resolutions: 0,
            cache_hits: 0,
            success_rate: 0.0,
        }
    }
}

/// Capture session for targeted debugging
#[derive(Debug, Clone)]
pub struct CaptureSession {
    /// Session ID
    pub id: String,
    /// Session name
    pub name: String,
    /// Session start time
    pub start_time: SystemTime,
    /// Target function (if specific)
    pub target_function: Option<String>,
    /// Captured stack traces
    pub captured_traces: Vec<Vec<StackFrame>>,
    /// Session configuration
    pub config: EnhancedStackTraceConfig,
}

/// Legacy minimal implementation for backward compatibility
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED advanced debug info enabled".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_trace_capture_creation() {
        let config = EnhancedStackTraceConfig::default();
        let capture = StackTraceCapture::new(config);
        
        // Test basic functionality
        let result = capture.capture_stack_trace();
        assert!(result.is_ok());
    }

    #[test]
    fn test_symbol_resolver() {
        let resolver = SymbolResolver::new();
        
        let symbol = SymbolInfo::function("test_function".to_string(), 0x1000, 0x100);
        let add_result = resolver.add_symbol(symbol);
        assert!(add_result.is_ok());
        
        let resolved = resolver.resolve_address(0x1050);
        assert!(resolved.is_ok());
        
        let symbol_info = resolved.unwrap();
        assert_eq!(symbol_info.name, "test_function");
        assert_eq!(symbol_info.offset, 0x50);
    }

    #[test]
    fn test_enhanced_config() {
        let mut config = EnhancedStackTraceConfig::default();
        config.max_depth = 50;
        config.resolve_symbols = false;
        
        assert_eq!(config.max_depth, 50);
        assert!(!config.resolve_symbols);
    }

    #[test]
    fn test_symbol_info_creation() {
        let unknown = SymbolInfo::unknown(0x2000);
        assert_eq!(unknown.symbol_type, SymbolType::Unknown);
        assert_eq!(unknown.address, 0x2000);
        
        let function = SymbolInfo::function("main".to_string(), 0x1000, 0x200);
        assert_eq!(function.symbol_type, SymbolType::Function);
        assert_eq!(function.name, "main");
    }

    #[test]
    fn test_llvm_debug_info() {
        let mut debug_info = LlvmDebugInfo::new();
        
        let metadata = LlvmDebugMetadata {
            file_path: PathBuf::from("test.csd"),
            line: 10,
            column: 5,
            function_name: Some("test_func".to_string()),
            scope: None,
        };
        
        debug_info.add_metadata(0x1000, metadata);
        
        let location = debug_info.get_source_location(0x1000);
        assert!(location.is_ok());
        
        let source_loc = location.unwrap();
        assert!(source_loc.is_some());
        
        let loc = source_loc.unwrap();
        assert_eq!(loc.line, 10);
        assert_eq!(loc.column, 5);
    }
}
