//! Enhanced Debug Information System for CURSED
//! 
//! This module provides comprehensive debugging support including:
//! - Detailed source location tracking with line/column/file information
//! - Stack trace capture and formatting
//! - Debug symbol generation and management
//! - Enhanced error context with source code snippets
//! - DWARF debug information generation
//! - Runtime debugging support

use crate::error::{CursedError, SourceLocation};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Enhanced debug information with comprehensive source tracking
#[derive(Debug, Clone)]
pub struct EnhancedDebugInfo {
    pub source_location: SourceLocation,
    pub function_name: Option<String>,
    pub variable_name: Option<String>,
    pub expression_type: Option<String>,
    pub stack_trace: Vec<StackFrame>,
    pub source_context: Option<SourceContext>,
    pub debug_symbols: Vec<DebugSymbol>,
    pub runtime_state: RuntimeState,
}

/// Stack frame information for detailed stack traces
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub source_location: SourceLocation,
    pub local_variables: HashMap<String, VariableDebugInfo>,
    pub instruction_pointer: Option<u64>,
    pub frame_pointer: Option<u64>,
}

/// Variable debug information
#[derive(Debug, Clone)]
pub struct VariableDebugInfo {
    pub name: String,
    pub var_type: String,
    pub value: Option<String>,
    pub memory_location: Option<u64>,
    pub scope_level: u32,
}

/// Source context for displaying code around errors
#[derive(Debug, Clone)]
pub struct SourceContext {
    pub file_content: String,
    pub line_before: Option<String>,
    pub error_line: String,
    pub line_after: Option<String>,
    pub column_pointer: String,
}

/// Debug symbol information
#[derive(Debug, Clone)]
pub struct DebugSymbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub source_location: SourceLocation,
    pub memory_address: Option<u64>,
    pub size: Option<u64>,
    pub type_info: Option<TypeInfo>,
}

/// Symbol type classification
#[derive(Debug, Clone)]
pub enum SymbolType {
    Function,
    Variable,
    Parameter,
    Struct,
    Interface,
    Constant,
    Module,
}

/// Type information for debug symbols
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub type_name: String,
    pub size: u64,
    pub alignment: u64,
    pub fields: Vec<FieldInfo>,
}

/// Field information for composite types
#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: String,
    pub offset: u64,
    pub size: u64,
}

/// Runtime state information
#[derive(Debug, Clone)]
pub struct RuntimeState {
    pub goroutine_id: Option<u64>,
    pub heap_size: u64,
    pub stack_size: u64,
    pub gc_cycles: u64,
    pub active_channels: u64,
    pub panic_count: u64,
}

/// Enhanced debug information manager
pub struct EnhancedDebugManager {
    pub debug_info_map: HashMap<String, EnhancedDebugInfo>,
    pub source_files: HashMap<String, String>,
    pub symbol_table: HashMap<String, DebugSymbol>,
    pub stack_traces: Vec<Vec<StackFrame>>,
    pub debug_enabled: bool,
    pub verbose_mode: bool,
    pub source_maps: HashMap<String, SourceMap>,
}

/// Source map for tracking locations
#[derive(Debug, Clone)]
pub struct SourceMap {
    pub file_path: String,
    pub line_mappings: HashMap<u32, String>,
    pub column_mappings: HashMap<u32, HashMap<u32, String>>,
}

impl EnhancedDebugManager {
    /// Create new enhanced debug manager
    pub fn new() -> Self {
        Self {
            debug_info_map: HashMap::new(),
            source_files: HashMap::new(),
            symbol_table: HashMap::new(),
            stack_traces: Vec::new(),
            debug_enabled: true,
            verbose_mode: false,
            source_maps: HashMap::new(),
        }
    }

    /// Enable debug information generation
    pub fn enable_debug(&mut self) {
        self.debug_enabled = true;
    }

    /// Enable verbose debug mode
    pub fn enable_verbose(&mut self) {
        self.verbose_mode = true;
    }

    /// Add source file for debug information
    pub fn add_source_file(&mut self, file_path: &str) -> Result<(), CursedError> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| CursedError::Io(format!("Failed to read source file {}: {}", file_path, e)))?;
        
        self.source_files.insert(file_path.to_string(), content.clone());
        
        // Create source map
        let source_map = self.create_source_map(file_path, &content)?;
        self.source_maps.insert(file_path.to_string(), source_map);
        
        Ok(())
    }

    /// Create source map from file content
    fn create_source_map(&self, file_path: &str, content: &str) -> Result<SourceMap, CursedError> {
        let mut line_mappings = HashMap::new();
        let mut column_mappings = HashMap::new();
        
        for (line_num, line) in content.lines().enumerate() {
            let line_num = line_num as u32 + 1;
            line_mappings.insert(line_num, line.to_string());
            
            let mut col_map = HashMap::new();
            for (col_num, ch) in line.chars().enumerate() {
                let col_num = col_num as u32 + 1;
                col_map.insert(col_num, ch.to_string());
            }
            column_mappings.insert(line_num, col_map);
        }
        
        Ok(SourceMap {
            file_path: file_path.to_string(),
            line_mappings,
            column_mappings,
        })
    }

    /// Add debug symbol
    pub fn add_debug_symbol(&mut self, symbol: DebugSymbol) {
        self.symbol_table.insert(symbol.name.clone(), symbol);
    }

    /// Get source context around error location
    pub fn get_source_context(&self, location: &SourceLocation) -> Result<SourceContext, CursedError> {
        let file_content = self.source_files.get(&location.file)
            .ok_or_else(|| CursedError::General(format!("Source file not found: {}", location.file)))?;
        
        let lines: Vec<&str> = file_content.lines().collect();
        let line_index = location.line.saturating_sub(1);
        
        if line_index >= lines.len() {
            return Err(CursedError::General(format!("Line {} out of bounds in file {}", location.line, location.file)));
        }
        
        let error_line = lines[line_index].to_string();
        let line_before = if line_index > 0 { 
            Some(lines[line_index - 1].to_string()) 
        } else { 
            None 
        };
        let line_after = if line_index + 1 < lines.len() { 
            Some(lines[line_index + 1].to_string()) 
        } else { 
            None 
        };
        
        // Create column pointer
        let column_pointer = if location.column > 0 {
            format!("{}^", " ".repeat(location.column.saturating_sub(1)))
        } else {
            "^".to_string()
        };
        
        Ok(SourceContext {
            file_content: file_content.clone(),
            line_before,
            error_line,
            line_after,
            column_pointer,
        })
    }

    /// Capture current stack trace
    pub fn capture_stack_trace(&mut self) -> Result<Vec<StackFrame>, CursedError> {
        let mut stack_frames = Vec::new();
        
        // In a real implementation, this would use platform-specific stack unwinding
        // For now, we'll create a mock stack trace
        let mock_frame = StackFrame {
            function_name: "current_function".to_string(),
            source_location: SourceLocation {
                file: "main.csd".to_string(),
                line: 10,
                column: 5,
            },
            local_variables: HashMap::new(),
            instruction_pointer: Some(0x1000),
            frame_pointer: Some(0x2000),
        };
        
        stack_frames.push(mock_frame);
        self.stack_traces.push(stack_frames.clone());
        
        Ok(stack_frames)
    }

    /// Format enhanced error message with debug information
    pub fn format_error_message(&self, error: &CursedError, location: &SourceLocation) -> Result<String, CursedError> {
        let mut message = String::new();
        
        // Add error header
        message.push_str(&format!("Error: {}\n", error));
        message.push_str(&format!("  --> {}:{}:{}\n", location.file, location.line, location.column));
        
        // Add source context
        if let Ok(context) = self.get_source_context(location) {
            message.push_str("   |\n");
            
            // Add line before if available
            if let Some(line_before) = &context.line_before {
                message.push_str(&format!("{:3} | {}\n", location.line - 1, line_before));
            }
            
            // Add error line
            message.push_str(&format!("{:3} | {}\n", location.line, context.error_line));
            
            // Add column pointer
            message.push_str(&format!("   | {}\n", context.column_pointer));
            
            // Add line after if available
            if let Some(line_after) = &context.line_after {
                message.push_str(&format!("{:3} | {}\n", location.line + 1, line_after));
            }
        }
        
        // Add stack trace if available
        if let Some(stack_trace) = self.stack_traces.last() {
            message.push_str("\nStack trace:\n");
            for (i, frame) in stack_trace.iter().enumerate() {
                message.push_str(&format!("  {}: {} at {}:{}:{}\n", 
                    i, frame.function_name, frame.source_location.file, 
                    frame.source_location.line, frame.source_location.column));
            }
        }
        
        // Add debug symbols if verbose mode
        if self.verbose_mode {
            message.push_str("\nDebug symbols:\n");
            for (name, symbol) in &self.symbol_table {
                message.push_str(&format!("  {}: {:?} at {}:{}:{}\n", 
                    name, symbol.symbol_type, symbol.source_location.file,
                    symbol.source_location.line, symbol.source_location.column));
            }
        }
        
        Ok(message)
    }

    /// Generate DWARF debug information
    pub fn generate_dwarf_debug_info(&self) -> Result<Vec<u8>, CursedError> {
        let mut dwarf_data = Vec::new();
        
        // DWARF header
        dwarf_data.extend_from_slice(&[0u8; 4]); // unit_length (will be filled later)
        dwarf_data.extend_from_slice(&[4u8, 0u8]); // version (DWARF v4)
        dwarf_data.extend_from_slice(&[0u8; 4]); // debug_abbrev_offset
        dwarf_data.push(8u8); // address_size
        
        // Compilation unit DIE
        dwarf_data.push(0x11); // DW_TAG_compile_unit
        
        // Add compilation unit attributes
        for (file_path, _) in &self.source_files {
            // DW_AT_name
            dwarf_data.push(0x03);
            dwarf_data.extend_from_slice(file_path.as_bytes());
            dwarf_data.push(0x00);
            
            // DW_AT_language (DW_LANG_C99 as placeholder)
            dwarf_data.push(0x13);
            dwarf_data.push(0x0C);
        }
        
        // Add function DIEs
        for (name, symbol) in &self.symbol_table {
            if matches!(symbol.symbol_type, SymbolType::Function) {
                // DW_TAG_subprogram
                dwarf_data.push(0x2E);
                
                // DW_AT_name
                dwarf_data.push(0x03);
                dwarf_data.extend_from_slice(name.as_bytes());
                dwarf_data.push(0x00);
                
                // DW_AT_decl_line
                dwarf_data.push(0x3A);
                dwarf_data.extend_from_slice(&(symbol.source_location.line as u32).to_le_bytes());
                
                // DW_AT_decl_column
                dwarf_data.push(0x39);
                dwarf_data.extend_from_slice(&(symbol.source_location.column as u32).to_le_bytes());
            }
        }
        
        // Add variable DIEs
        for (name, symbol) in &self.symbol_table {
            if matches!(symbol.symbol_type, SymbolType::Variable) {
                // DW_TAG_variable
                dwarf_data.push(0x34);
                
                // DW_AT_name
                dwarf_data.push(0x03);
                dwarf_data.extend_from_slice(name.as_bytes());
                dwarf_data.push(0x00);
                
                // DW_AT_decl_line
                dwarf_data.push(0x3A);
                dwarf_data.extend_from_slice(&(symbol.source_location.line as u32).to_le_bytes());
            }
        }
        
        // End compilation unit
        dwarf_data.push(0x00);
        
        Ok(dwarf_data)
    }

    /// Generate debug symbols for native compilation
    pub fn generate_debug_symbols(&self) -> Result<String, CursedError> {
        let mut symbols = String::new();
        
        // Add debug section header
        symbols.push_str(".section .debug_info\n");
        symbols.push_str(".long .Ldebug_info_end - .Ldebug_info_start\n");
        symbols.push_str(".Ldebug_info_start:\n");
        symbols.push_str(".short 4\n"); // DWARF version
        symbols.push_str(".long .Ldebug_abbrev\n");
        symbols.push_str(".byte 8\n"); // address size
        
        // Add compilation unit
        symbols.push_str(".uleb128 1\n"); // DW_TAG_compile_unit
        for (file_path, _) in &self.source_files {
            symbols.push_str(&format!(".string \"{}\"\n", file_path));
        }
        
        // Add function symbols
        for (name, symbol) in &self.symbol_table {
            if matches!(symbol.symbol_type, SymbolType::Function) {
                symbols.push_str(".uleb128 2\n"); // DW_TAG_subprogram
                symbols.push_str(&format!(".string \"{}\"\n", name));
                symbols.push_str(&format!(".long {}\n", symbol.source_location.line));
                symbols.push_str(&format!(".long {}\n", symbol.source_location.column));
            }
        }
        
        symbols.push_str(".byte 0\n"); // end of DIEs
        symbols.push_str(".Ldebug_info_end:\n");
        
        // Add abbreviation section
        symbols.push_str(".section .debug_abbrev\n");
        symbols.push_str(".Ldebug_abbrev:\n");
        symbols.push_str(".uleb128 1\n"); // abbrev code
        symbols.push_str(".uleb128 0x11\n"); // DW_TAG_compile_unit
        symbols.push_str(".byte 1\n"); // has children
        symbols.push_str(".uleb128 0x03\n"); // DW_AT_name
        symbols.push_str(".uleb128 0x08\n"); // DW_FORM_string
        symbols.push_str(".byte 0\n"); // end of attributes
        symbols.push_str(".byte 0\n");
        
        symbols.push_str(".uleb128 2\n"); // abbrev code
        symbols.push_str(".uleb128 0x2E\n"); // DW_TAG_subprogram
        symbols.push_str(".byte 0\n"); // no children
        symbols.push_str(".uleb128 0x03\n"); // DW_AT_name
        symbols.push_str(".uleb128 0x08\n"); // DW_FORM_string
        symbols.push_str(".uleb128 0x3A\n"); // DW_AT_decl_line
        symbols.push_str(".uleb128 0x0B\n"); // DW_FORM_data4
        symbols.push_str(".uleb128 0x39\n"); // DW_AT_decl_column
        symbols.push_str(".uleb128 0x0B\n"); // DW_FORM_data4
        symbols.push_str(".byte 0\n"); // end of attributes
        symbols.push_str(".byte 0\n");
        
        symbols.push_str(".byte 0\n"); // end of abbreviations
        
        Ok(symbols)
    }

    /// Add runtime debugging support
    pub fn add_runtime_debug_info(&mut self, location: &SourceLocation, runtime_state: RuntimeState) {
        let debug_info = EnhancedDebugInfo {
            source_location: location.clone(),
            function_name: None,
            variable_name: None,
            expression_type: None,
            stack_trace: Vec::new(),
            source_context: self.get_source_context(location).ok(),
            debug_symbols: self.symbol_table.values().cloned().collect(),
            runtime_state,
        };
        
        let key = format!("{}:{}:{}", location.file, location.line, location.column);
        self.debug_info_map.insert(key, debug_info);
    }
}

impl Default for EnhancedDebugManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self {
            goroutine_id: None,
            heap_size: 0,
            stack_size: 0,
            gc_cycles: 0,
            active_channels: 0,
            panic_count: 0,
        }
    }
}

// Convenience functions for creating debug information
pub fn create_debug_symbol(name: &str, symbol_type: SymbolType, location: SourceLocation) -> DebugSymbol {
    DebugSymbol {
        name: name.to_string(),
        symbol_type,
        source_location: location,
        memory_address: None,
        size: None,
        type_info: None,
    }
}

pub fn create_stack_frame(function_name: &str, location: SourceLocation) -> StackFrame {
    StackFrame {
        function_name: function_name.to_string(),
        source_location: location,
        local_variables: HashMap::new(),
        instruction_pointer: None,
        frame_pointer: None,
    }
}

pub fn create_variable_debug_info(name: &str, var_type: &str, value: Option<String>) -> VariableDebugInfo {
    VariableDebugInfo {
        name: name.to_string(),
        var_type: var_type.to_string(),
        value,
        memory_location: None,
        scope_level: 0,
    }
}


