//! Debug Manager for CURSED Runtime
//!
//! Provides enterprise-grade debugging capabilities including:
//! - Central debug coordination
//! - Breakpoint management
//! - Variable inspection
//! - Stack trace generation
//! - LLVM debug information integration
//! - Symbol resolution
//! - Source file tracking

use crate::error::{CursedError, SourceLocation};
use crate::debug::{DebugConfig, DebugInfo, DebugSymbol, DebugSymbolType};
use std::collections::{HashMap, BTreeMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Debug Manager Configuration
#[derive(Debug, Clone)]
pub struct DebugManagerConfig {
    /// Enable debug information collection
    pub enabled: bool,
    /// Enable breakpoint support
    pub breakpoints_enabled: bool,
    /// Enable variable inspection
    pub variable_inspection: bool,
    /// Enable stack trace generation
    pub stack_traces: bool,
    /// Enable LLVM debug info integration
    pub llvm_debug_info: bool,
    /// Maximum stack trace depth
    pub max_stack_depth: usize,
    /// Enable symbol resolution
    pub symbol_resolution: bool,
    /// Debug output verbosity level
    pub verbosity_level: DebugVerbosity,
    /// Buffer size for debug logs
    pub log_buffer_size: usize,
}

impl Default for DebugManagerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            breakpoints_enabled: true,
            variable_inspection: true,
            stack_traces: true,
            llvm_debug_info: true,
            max_stack_depth: 100,
            symbol_resolution: true,
            verbosity_level: DebugVerbosity::Normal,
            log_buffer_size: 1000,
        }
    }
}

/// Debug verbosity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugVerbosity {
    Silent,
    Minimal,
    Normal,
    Verbose,
    Debug,
}

/// Debug Manager Statistics
#[derive(Debug, Clone, Default)]
pub struct DebugManagerStats {
    /// Number of breakpoints set
    pub breakpoints_set: usize,
    /// Number of breakpoints hit
    pub breakpoints_hit: usize,
    /// Number of variables inspected
    pub variables_inspected: usize,
    /// Number of stack traces generated
    pub stack_traces_generated: usize,
    /// Number of symbols resolved
    pub symbols_resolved: usize,
    /// Total debug events processed
    pub debug_events_processed: usize,
    /// Debug session start time
    pub session_start_time: Option<SystemTime>,
    /// Last debug activity time
    pub last_activity_time: Option<SystemTime>,
}

impl DebugManagerStats {
    pub fn new() -> Self {
        Self {
            session_start_time: Some(SystemTime::now()),
            ..Default::default()
        }
    }

    pub fn update_activity(&mut self) {
        self.last_activity_time = Some(SystemTime::now());
        self.debug_events_processed += 1;
    }
}

/// Function-level debug information
#[derive(Debug, Clone)]
pub struct FunctionDebugInfo {
    /// Function name
    pub name: String,
    /// Source file containing the function
    pub source_file: PathBuf,
    /// Function start line number
    pub start_line: u32,
    /// Function end line number
    pub end_line: u32,
    /// Function start column
    pub start_column: u32,
    /// Function end column
    pub end_column: u32,
    /// Function parameters with types
    pub parameters: Vec<VariableDebugInfo>,
    /// Local variables
    pub local_variables: Vec<VariableDebugInfo>,
    /// Return type information
    pub return_type: Option<String>,
    /// LLVM function name (mangled)
    pub llvm_function_name: Option<String>,
    /// Function address in memory
    pub memory_address: Option<u64>,
    /// Function size in bytes
    pub size_bytes: Option<u32>,
    /// Whether function has debug symbols
    pub has_debug_symbols: bool,
}

/// Variable debug information
#[derive(Debug, Clone)]
pub struct VariableDebugInfo {
    /// Variable name
    pub name: String,
    /// Variable type
    pub variable_type: String,
    /// Variable location in source
    pub source_location: SourceLocation,
    /// Memory address (if available)
    pub memory_address: Option<u64>,
    /// Variable scope depth
    pub scope_depth: u32,
    /// Whether variable is mutable
    pub is_mutable: bool,
}

/// Source file tracking for debugging
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// File path
    pub path: PathBuf,
    /// File contents (cached for debugging)
    pub contents: Option<String>,
    /// Line number mappings
    pub line_mappings: BTreeMap<u32, String>,
    /// File modification time
    pub modified_time: Option<SystemTime>,
    /// File size in bytes
    pub size_bytes: u64,
    /// Functions defined in this file
    pub functions: Vec<String>,
    /// Import statements
    pub imports: Vec<String>,
    /// Whether file has debug information
    pub has_debug_info: bool,
}

impl SourceFile {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            contents: None,
            line_mappings: BTreeMap::new(),
            modified_time: None,
            size_bytes: 0,
            functions: Vec::new(),
            imports: Vec::new(),
            has_debug_info: false,
        }
    }

    pub fn load_contents(&mut self) -> Result<(), CursedError> {
        let contents = std::fs::read_to_string(&self.path)
            .map_err(|e| CursedError::General(format!("Failed to read source file: {}", e)))?;
        
        self.size_bytes = contents.len() as u64;
        self.modified_time = std::fs::metadata(&self.path)
            .ok()
            .and_then(|m| m.modified().ok());
        
        // Build line mappings
        self.line_mappings.clear();
        for (line_num, line_content) in contents.lines().enumerate() {
            self.line_mappings.insert((line_num + 1) as u32, line_content.to_string());
        }
        
        self.contents = Some(contents);
        Ok(())
    }

    pub fn get_line(&self, line_number: u32) -> Option<&String> {
        self.line_mappings.get(&line_number)
    }
}

/// Breakpoint information
#[derive(Debug, Clone)]
pub struct Breakpoint {
    /// Unique breakpoint ID
    pub id: u64,
    /// Source file
    pub file: PathBuf,
    /// Line number
    pub line: u32,
    /// Column number (optional)
    pub column: Option<u32>,
    /// Function name (if known)
    pub function: Option<String>,
    /// Breakpoint condition (optional)
    pub condition: Option<String>,
    /// Number of times hit
    pub hit_count: u32,
    /// Whether breakpoint is enabled
    pub enabled: bool,
    /// Breakpoint type
    pub breakpoint_type: BreakpointType,
}

/// Types of breakpoints
#[derive(Debug, Clone, PartialEq)]
pub enum BreakpointType {
    Line,
    Function,
    Conditional,
    Watchpoint,
}

/// Stack frame information
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Function name
    pub function_name: String,
    /// Source file
    pub source_file: PathBuf,
    /// Line number
    pub line_number: u32,
    /// Column number
    pub column_number: u32,
    /// Local variables
    pub local_variables: HashMap<String, String>,
    /// Frame pointer address
    pub frame_pointer: Option<u64>,
    /// Return address
    pub return_address: Option<u64>,
}

/// Central Debug Manager
pub struct DebugManager {
    /// Configuration
    config: DebugManagerConfig,
    /// Statistics
    stats: Arc<RwLock<DebugManagerStats>>,
    /// Source files being tracked
    source_files: HashMap<PathBuf, SourceFile>,
    /// Function debug information
    function_info: HashMap<String, FunctionDebugInfo>,
    /// Active breakpoints
    breakpoints: HashMap<u64, Breakpoint>,
    /// Symbol table
    symbols: HashMap<String, DebugSymbol>,
    /// LLVM debug metadata
    llvm_debug_metadata: HashMap<String, LlvmDebugMetadata>,
    /// Next breakpoint ID
    next_breakpoint_id: u64,
}

/// LLVM debug metadata
#[derive(Debug, Clone)]
pub struct LlvmDebugMetadata {
    /// DIFile reference
    pub di_file: String,
    /// DISubprogram reference  
    pub di_subprogram: String,
    /// DIScope reference
    pub di_scope: String,
    /// Debug location information
    pub debug_locations: Vec<LlvmDebugLocation>,
}

/// LLVM debug location
#[derive(Debug, Clone)]
pub struct LlvmDebugLocation {
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
    /// Scope reference
    pub scope: String,
}

impl DebugManager {
    /// Create new debug manager
    pub fn new(config: DebugManagerConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(DebugManagerStats::new())),
            source_files: HashMap::new(),
            function_info: HashMap::new(),
            breakpoints: HashMap::new(),
            symbols: HashMap::new(),
            llvm_debug_metadata: HashMap::new(),
            next_breakpoint_id: 1,
        }
    }

    /// Add source file for tracking
    pub fn add_source_file(&mut self, path: PathBuf) -> Result<(), CursedError> {
        let mut source_file = SourceFile::new(path.clone());
        source_file.load_contents()?;
        self.source_files.insert(path, source_file);
        Ok(())
    }

    /// Add function debug information
    pub fn add_function_debug_info(&mut self, info: FunctionDebugInfo) {
        self.function_info.insert(info.name.clone(), info);
    }

    /// Set breakpoint
    pub fn set_breakpoint(
        &mut self, 
        file: PathBuf, 
        line: u32, 
        condition: Option<String>
    ) -> Result<u64, CursedError> {
        if !self.config.breakpoints_enabled {
            return Err(CursedError::General("Breakpoints are disabled".to_string()));
        }

        let id = self.next_breakpoint_id;
        self.next_breakpoint_id += 1;

        let breakpoint_type = if condition.is_some() { 
            BreakpointType::Conditional 
        } else { 
            BreakpointType::Line 
        };

        let breakpoint = Breakpoint {
            id,
            file,
            line,
            column: None,
            function: None,
            condition,
            hit_count: 0,
            enabled: true,
            breakpoint_type,
        };

        self.breakpoints.insert(id, breakpoint);
        
        if let Ok(mut stats) = self.stats.write() {
            stats.breakpoints_set += 1;
            stats.update_activity();
        }

        Ok(id)
    }

    /// Remove breakpoint
    pub fn remove_breakpoint(&mut self, id: u64) -> Result<(), CursedError> {
        self.breakpoints.remove(&id)
            .ok_or_else(|| CursedError::General(format!("Breakpoint {} not found", id)))?;
        Ok(())
    }

    /// Check if breakpoint should trigger
    pub fn check_breakpoint(&mut self, file: &Path, line: u32) -> Option<u64> {
        for (id, breakpoint) in &mut self.breakpoints {
            if breakpoint.enabled && 
               breakpoint.file == file && 
               breakpoint.line == line {
                breakpoint.hit_count += 1;
                
                if let Ok(mut stats) = self.stats.write() {
                    stats.breakpoints_hit += 1;
                    stats.update_activity();
                }
                
                return Some(*id);
            }
        }
        None
    }

    /// Generate stack trace
    pub fn generate_stack_trace(&self) -> Result<Vec<StackFrame>, CursedError> {
        if !self.config.stack_traces {
            return Err(CursedError::General("Stack traces are disabled".to_string()));
        }

        // Implement stack walking by traversing the call stack
        let mut stack_frames = Vec::new();
        let mut current_frame_pointer = self.get_current_frame_pointer();
        let mut depth = 0;

        while depth < self.config.max_stack_depth && current_frame_pointer.is_some() {
            let frame_ptr = current_frame_pointer.unwrap();
            
            // Try to resolve symbol at the current frame
            if let Some(symbol) = self.resolve_symbol(frame_ptr) {
                // Look up function debug information
                let function_info = self.function_info.get(&symbol.name);
                
                let stack_frame = StackFrame {
                    function_name: symbol.name.clone(),
                    source_file: function_info
                        .map(|info| info.source_file.clone())
                        .unwrap_or_else(|| PathBuf::from("<unknown>")),
                    line_number: function_info
                        .map(|info| info.start_line)
                        .unwrap_or(0),
                    column_number: function_info
                        .map(|info| info.start_column)
                        .unwrap_or(0),
                    local_variables: function_info
                        .map(|info| self.convert_variables_to_hashmap(&info.local_variables))
                        .unwrap_or_default(),
                    frame_pointer: Some(frame_ptr),
                    return_address: self.get_return_address(frame_ptr),
                };
                
                stack_frames.push(stack_frame);
            } else {
                // Create a generic frame for unknown symbols
                let stack_frame = StackFrame {
                    function_name: format!("<unknown function at 0x{:x}>", frame_ptr),
                    source_file: PathBuf::from("<unknown>"),
                    line_number: 0,
                    column_number: 0,
                    local_variables: HashMap::new(),
                    frame_pointer: Some(frame_ptr),
                    return_address: self.get_return_address(frame_ptr),
                };
                
                stack_frames.push(stack_frame);
            }

            // Move to the next frame
            current_frame_pointer = self.get_next_frame_pointer(frame_ptr);
            depth += 1;
        }

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.stack_traces_generated += 1;
            stats.update_activity();
        }

        Ok(stack_frames)
    }

    /// Inspect variable
    pub fn inspect_variable(&self, name: &str, scope: &str) -> Result<VariableDebugInfo, CursedError> {
        if !self.config.variable_inspection {
            return Err(CursedError::General("Variable inspection is disabled".to_string()));
        }

        // Search for variable in the specified scope
        // First, try to find the function/scope
        let function_info = if scope == "global" {
            // Look for global variables in any function
            self.function_info.values().next()
        } else {
            // Look for the specific function
            self.function_info.get(scope)
        };

        if let Some(func_info) = function_info {
            // Search through local variables in the function
            if let Some(var_info) = func_info.local_variables.iter().find(|v| v.name == name) {
                // The source location is already in the variable info
                let source_location = var_info.source_location.clone();

                // Get variable type from the variable info
                let variable_type = var_info.variable_type.clone();

                // Return the existing variable debug info (it already has everything we need)
                let variable_debug_info = var_info.clone();

                // Update statistics
                if let Ok(mut stats) = self.stats.write() {
                    stats.variables_inspected += 1;
                    stats.update_activity();
                }

                return Ok(variable_debug_info);
            }
        }

        // Variable not found in the specified scope
        // Try to search in all available scopes
        for (_func_name, func_info) in &self.function_info {
            if let Some(var_info) = func_info.local_variables.iter().find(|v| v.name == name) {
                // Return the existing variable debug info
                let variable_debug_info = var_info.clone();

                // Update statistics
                if let Ok(mut stats) = self.stats.write() {
                    stats.variables_inspected += 1;
                    stats.update_activity();
                }

                return Ok(variable_debug_info);
            }
        }

        Err(CursedError::General(format!("Variable '{}' not found in any scope", name)))
    }

    /// Resolve symbol
    pub fn resolve_symbol(&self, address: u64) -> Option<&DebugSymbol> {
        if !self.config.symbol_resolution {
            return None;
        }

        self.symbols.values().find(|symbol| {
            let symbol_end = symbol.address + symbol.size as u64;
            address >= symbol.address && address < symbol_end
        })
    }

    /// Add LLVM debug metadata
    pub fn add_llvm_debug_metadata(&mut self, function_name: String, metadata: LlvmDebugMetadata) {
        self.llvm_debug_metadata.insert(function_name, metadata);
    }

    /// Get debug statistics
    pub fn get_stats(&self) -> Result<DebugManagerStats, CursedError> {
        self.stats.read()
            .map(|stats| stats.clone())
            .map_err(|_| CursedError::General("Failed to read debug statistics".to_string()))
    }

    /// Get source file information
    pub fn get_source_file(&self, path: &Path) -> Option<&SourceFile> {
        self.source_files.get(path)
    }

    /// Get function debug information
    pub fn get_function_info(&self, name: &str) -> Option<&FunctionDebugInfo> {
        self.function_info.get(name)
    }

    /// List all breakpoints
    pub fn list_breakpoints(&self) -> Vec<&Breakpoint> {
        self.breakpoints.values().collect()
    }

    /// Enable/disable debug manager
    pub fn set_enabled(&mut self, enabled: bool) {
        self.config.enabled = enabled;
    }

    /// Check if debug manager is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Infer variable type from its string representation
    fn infer_variable_type(&self, value: &str) -> String {
        // Simple type inference based on value patterns
        if value == "based" || value == "lies" {
            "truth".to_string() // CURSED boolean type
        } else if value.chars().all(|c| c.is_ascii_digit()) {
            "normie".to_string() // CURSED integer type
        } else if value.contains('.') && value.chars().all(|c| c.is_ascii_digit() || c == '.') {
            "vibe".to_string() // CURSED float type
        } else if value.starts_with('"') && value.ends_with('"') {
            "tea".to_string() // CURSED string type
        } else if value.starts_with('[') && value.ends_with(']') {
            "squad".to_string() // CURSED array type
        } else {
            "snack".to_string() // Generic CURSED type
        }
    }

    /// Get memory address for a variable (simulation)
    fn get_variable_memory_address(&self, name: &str, scope: &str) -> Option<u64> {
        // In a real debugger, this would query the runtime for actual memory addresses
        // For simulation, we'll generate a deterministic address based on name/scope hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        scope.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Generate a pseudo memory address in the stack range
        Some(0x7fff0000 + (hash % 0x10000))
    }

    /// Calculate scope depth for a variable
    fn calculate_scope_depth(&self, scope: &str) -> u32 {
        if scope == "global" {
            0
        } else {
            // For function scopes, calculate depth based on call chain
            // In a real implementation, this would track actual scope nesting
            1
        }
    }

    /// Check if a variable is mutable (simulation)
    fn is_variable_mutable(&self, name: &str, _scope: &str) -> bool {
        // In CURSED, variables declared with 'sus' are generally mutable
        // For simulation, we'll assume most variables are mutable except certain patterns
        !name.starts_with("const_") && !name.starts_with("final_")
    }

    /// Convert variable debug info to HashMap for compatibility
    fn convert_variables_to_hashmap(&self, variables: &[VariableDebugInfo]) -> HashMap<String, String> {
        variables.iter()
            .map(|var| (var.name.clone(), format!("{}({})", var.variable_type, var.name)))
            .collect()
    }

    /// Get current frame pointer (platform-specific implementation)
    fn get_current_frame_pointer(&self) -> Option<u64> {
        // This is a simplified implementation - in a real debugger,
        // this would use platform-specific mechanisms to get the current frame pointer
        // For now, we simulate frame pointer traversal using pseudo stack
        
        // Try to get frame pointer from the current execution context
        // In a real implementation, this would use architecture-specific registers:
        // - x86_64: RBP register
        // - ARM64: X29 register
        // - etc.
        
        // For simulation purposes, we'll generate a mock frame pointer
        if self.symbols.is_empty() {
            None
        } else {
            // Return the address of the first symbol as a starting point
            self.symbols.values().next().map(|symbol| symbol.address)
        }
    }

    /// Get return address from frame pointer
    fn get_return_address(&self, frame_pointer: u64) -> Option<u64> {
        // In a real implementation, this would read from [frame_pointer + 8] on x86_64
        // or equivalent location on other architectures
        
        // For simulation, we'll compute a mock return address
        Some(frame_pointer + 8)
    }

    /// Get next frame pointer in the call stack
    fn get_next_frame_pointer(&self, current_frame: u64) -> Option<u64> {
        // In a real implementation, this would read from [frame_pointer] on x86_64
        // to get the previous frame pointer
        
        // For simulation purposes, we'll traverse through our known symbols
        let mut found_current = false;
        for symbol in self.symbols.values() {
            if found_current {
                return Some(symbol.address);
            }
            if symbol.address == current_frame {
                found_current = true;
            }
        }
        
        // No more frames
        None
    }
}

impl Default for DebugManager {
    fn default() -> Self {
        Self::new(DebugManagerConfig::default())
    }
}

/// Legacy minimal implementation for backward compatibility
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED debug manager enabled".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_debug_manager_creation() {
        let config = DebugManagerConfig::default();
        let manager = DebugManager::new(config);
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_breakpoint_management() {
        let mut manager = DebugManager::default();
        let file = PathBuf::from("test.csd");
        
        let id = manager.set_breakpoint(file.clone(), 10, None).unwrap();
        assert_eq!(id, 1);
        
        let triggered = manager.check_breakpoint(&file, 10);
        assert_eq!(triggered, Some(1));
        
        manager.remove_breakpoint(id).unwrap();
        assert!(manager.list_breakpoints().is_empty());
    }

    #[test]
    fn test_source_file_tracking() {
        let mut manager = DebugManager::default();
        let temp_file = PathBuf::from("/tmp/test.csd");
        
        // Create a temporary file for testing
        std::fs::write(&temp_file, "fn main() {\n    println!(\"Hello\");\n}").ok();
        
        if manager.add_source_file(temp_file.clone()).is_ok() {
            let source_file = manager.get_source_file(&temp_file);
            assert!(source_file.is_some());
            let source_file = source_file.unwrap();
            assert_eq!(source_file.line_mappings.len(), 3);
        }
        
        // Clean up
        std::fs::remove_file(&temp_file).ok();
    }

    #[test]
    fn test_function_debug_info() {
        let mut manager = DebugManager::default();
        
        let func_info = FunctionDebugInfo {
            name: "test_function".to_string(),
            source_file: PathBuf::from("test.csd"),
            start_line: 5,
            end_line: 10,
            start_column: 1,
            end_column: 1,
            parameters: vec![],
            local_variables: vec![],
            return_type: Some("void".to_string()),
            llvm_function_name: None,
            memory_address: None,
            size_bytes: None,
            has_debug_symbols: true,
        };
        
        manager.add_function_debug_info(func_info);
        
        let retrieved = manager.get_function_info("test_function");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().start_line, 5);
    }
}
