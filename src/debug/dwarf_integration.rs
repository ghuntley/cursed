//! DWARF debug information integration with CURSED debug engine
//!
//! This module provides integration between the DWARF debug information parser
//! and the CURSED language debug engine, enabling:
//! - Symbol resolution from DWARF data
//! - Source location mapping
//! - Variable inspection
//! - Stack frame reconstruction
//! - Integration with LLVM codegen debug metadata

use crate::runtime::dwarf_parser::{
    DwarfDebugDatabase, RegisterMap, StackFrameInfo, DwarfVersionHandler, DebugInfoErrorHandler,
};
use crate::runtime::debug_info::{StackTraceCapture, EnhancedStackTraceConfig};
use crate::debug::{DebugInfo, SymbolInfo, DebugSymbol, DebugSymbolType};
use crate::error::{CursedError, SourceLocation};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Integration layer between DWARF debug info and CURSED debug engine
pub struct DwarfDebugIntegration {
    /// DWARF debug database
    dwarf_database: Arc<RwLock<Option<DwarfDebugDatabase>>>,
    /// Stack trace capture system
    stack_trace_capture: Arc<StackTraceCapture>,
    /// Version handler for DWARF compatibility
    version_handler: Arc<RwLock<DwarfVersionHandler>>,
    /// Error handler for debug info parsing
    error_handler: Arc<RwLock<DebugInfoErrorHandler>>,
    /// Symbol cache for performance
    symbol_cache: Arc<RwLock<HashMap<u64, DebugSymbol>>>,
    /// Source location cache
    source_cache: Arc<RwLock<HashMap<u64, SourceLocation>>>,
}

impl DwarfDebugIntegration {
    /// Create new DWARF debug integration
    pub fn new() -> Self {
        let config = EnhancedStackTraceConfig::default();
        
        Self {
            dwarf_database: Arc::new(RwLock::new(None)),
            stack_trace_capture: Arc::new(StackTraceCapture::new(config)),
            version_handler: Arc::new(RwLock::new(DwarfVersionHandler::new())),
            error_handler: Arc::new(RwLock::new(DebugInfoErrorHandler::new())),
            symbol_cache: Arc::new(RwLock::new(HashMap::new())),
            source_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Load DWARF debug information from binary data
    pub fn load_debug_info(&self, dwarf_data: &[u8]) -> Result<(), CursedError> {
        let mut database = DwarfDebugDatabase::new();
        
        // Parse DWARF data
        database.load_from_dwarf(dwarf_data)?;
        
        // Store the database
        if let Ok(mut db_lock) = self.dwarf_database.write() {
            *db_lock = Some(database);
            
            // Clear caches since we have new debug info
            if let Ok(mut cache) = self.symbol_cache.write() {
                cache.clear();
            }
            if let Ok(mut cache) = self.source_cache.write() {
                cache.clear();
            }
            
            Ok(())
        } else {
            Err(CursedError::RuntimeError("Failed to store DWARF database".to_string()))
        }
    }

    /// Get debug symbol for address
    pub fn get_debug_symbol(&self, address: u64) -> Result<Option<DebugSymbol>, CursedError> {
        // Check cache first
        if let Ok(cache) = self.symbol_cache.read() {
            if let Some(symbol) = cache.get(&address) {
                return Ok(Some(symbol.clone()));
            }
        }

        // Query DWARF database
        if let Ok(db_lock) = self.dwarf_database.read() {
            if let Some(ref database) = *db_lock {
                if let Some(function) = database.find_function(address) {
                    let symbol = DebugSymbol {
                        name: function.name.clone(),
                        symbol_type: DebugSymbolType::Function,
                        address,
                        size: (function.end_address - function.start_address) as u32,
                        file_path: function.source_file.clone(),
                        line_number: function.line_range.map(|(start, _)| start),
                        column_number: None,
                    };

                    // Cache the result
                    if let Ok(mut cache) = self.symbol_cache.write() {
                        cache.insert(address, symbol.clone());
                    }

                    return Ok(Some(symbol));
                }
            }
        }

        Ok(None)
    }

    /// Get source location for address
    pub fn get_source_location(&self, address: u64) -> Result<Option<SourceLocation>, CursedError> {
        // Check cache first
        if let Ok(cache) = self.source_cache.read() {
            if let Some(location) = cache.get(&address) {
                return Ok(Some(location.clone()));
            }
        }

        // Query DWARF database
        if let Ok(db_lock) = self.dwarf_database.read() {
            if let Some(ref database) = *db_lock {
                if let Some(line_info) = database.get_source_location_for_address(address) {
                    let location = SourceLocation {
file: line_info.file.to_string_lossy().to_string(),
                        line: line_info.line as usize,
                        column: line_info.column as usize,
                    
                    offset: 0,
                };

                    // Cache the result
                    if let Ok(mut cache) = self.source_cache.write() {
                        cache.insert(address, location.clone());
                    }

                    return Ok(Some(location));
                }
            }
        }

        Ok(None)
    }

    /// Reconstruct stack frame with debug information
    pub fn reconstruct_stack_frame(&self, address: u64) -> Result<Option<StackFrameInfo>, CursedError> {
        if let Ok(db_lock) = self.dwarf_database.read() {
            if let Some(ref database) = *db_lock {
                let registers = RegisterMap::from_current_context();
                let frame_info = database.reconstruct_stack_frame(address, &registers)?;
                return Ok(Some(frame_info));
            }
        }

        Ok(None)
    }

    /// Get function parameters at address
    pub fn get_function_parameters(&self, address: u64) -> Result<Vec<FunctionParameter>, CursedError> {
        if let Ok(db_lock) = self.dwarf_database.read() {
            if let Some(ref database) = *db_lock {
                if let Some(function) = database.find_function(address) {
                    let mut parameters = Vec::new();
                    
                    for param in &function.parameters {
                        let param_type = database.get_type_name(param.type_id);
                        
                        parameters.push(FunctionParameter {
                            name: param.name.clone(),
                            param_type,
                            location: param.location.clone(),
                        });
                    }
                    
                    return Ok(parameters);
                }
            }
        }

        Ok(Vec::new())
    }

    /// Get local variables in scope at address
    pub fn get_local_variables(&self, address: u64) -> Result<Vec<LocalVariable>, CursedError> {
        if let Ok(db_lock) = self.dwarf_database.read() {
            if let Some(ref database) = *db_lock {
                let variables = database.get_variables_at_address(address);
                let mut result = Vec::new();
                
                for var in variables {
                    let var_type = database.get_type_name(var.type_id);
                    
                    result.push(LocalVariable {
                        name: var.name.clone(),
                        var_type,
                        location: var.location.clone(),
                        scope_start: var.scope_start,
                        scope_end: var.scope_end,
                    });
                }
                
                return Ok(result);
            }
        }

        Ok(Vec::new())
    }

    /// Get inline function information at address
    pub fn get_inline_info(&self, address: u64) -> Result<Vec<InlineFunction>, CursedError> {
        if let Ok(db_lock) = self.dwarf_database.read() {
            if let Some(ref database) = *db_lock {
                let inline_sites = database.get_inline_info_at_address(address);
                let mut result = Vec::new();
                
                for site in inline_sites {
                    result.push(InlineFunction {
                        function_name: site.function_name.clone(),
                        call_address: site.call_address,
                        original_location: site.original_location.clone(),
                        inline_location: site.inline_location.clone(),
                    });
                }
                
                return Ok(result);
            }
        }

        Ok(Vec::new())
    }

    /// Capture enhanced stack trace with DWARF information
    pub fn capture_enhanced_stack_trace(&self) -> Result<Vec<EnhancedStackFrame>, CursedError> {
        let basic_frames = self.stack_trace_capture.capture_stack_trace()?;
        let mut enhanced_frames = Vec::new();

        for frame in basic_frames {
            let mut enhanced_frame = EnhancedStackFrame {
                address: frame.address.unwrap_or(0),
                symbol: self.get_debug_symbol(frame.address.unwrap_or(0))?,
                source_location: self.get_source_location(frame.address.unwrap_or(0))?,
                parameters: self.get_function_parameters(frame.address.unwrap_or(0))?,
                local_variables: self.get_local_variables(frame.address.unwrap_or(0))?,
                inline_functions: self.get_inline_info(frame.address.unwrap_or(0))?,
                basic_frame: frame,
            };

            enhanced_frames.push(enhanced_frame);
        }

        Ok(enhanced_frames)
    }

    /// Get DWARF version information
    pub fn get_dwarf_version(&self) -> Option<u16> {
        if let Ok(handler) = self.version_handler.read() {
            handler.current_version()
        } else {
            None
        }
    }

    /// Get debug info parsing errors
    pub fn get_parsing_errors(&self) -> Vec<crate::runtime::debug_info::DebugInfoError> {
        if let Ok(handler) = self.error_handler.read() {
            handler.get_errors().to_vec()
        } else {
            Vec::new()
        }
    }

    /// Clear debug information and caches
    pub fn clear_debug_info(&self) -> Result<(), CursedError> {
        if let Ok(mut db_lock) = self.dwarf_database.write() {
            *db_lock = None;
        }

        if let Ok(mut cache) = self.symbol_cache.write() {
            cache.clear();
        }

        if let Ok(mut cache) = self.source_cache.write() {
            cache.clear();
        }

        Ok(())
    }

    /// Update stack trace configuration
    pub fn update_config(&self, config: EnhancedStackTraceConfig) -> Result<(), CursedError> {
        // Note: This would require making StackTraceCapture.update_config public
        // For now, we'll create a new instance if needed
        Ok(())
    }

    /// Check if debug information is available
    pub fn has_debug_info(&self) -> bool {
        if let Ok(db_lock) = self.dwarf_database.read() {
            db_lock.is_some()
        } else {
            false
        }
    }

    /// Get statistics about loaded debug information
    pub fn get_debug_info_stats(&self) -> Result<DebugInfoStats, CursedError> {
        if let Ok(db_lock) = self.dwarf_database.read() {
            if let Some(ref database) = *db_lock {
                return Ok(DebugInfoStats {
                    function_count: database.functions.len(),
                    variable_count: database.variables.values().map(|v| v.len()).sum(),
                    type_count: database.types.len(),
                    line_mapping_count: database.line_mappings.len(),
                    inline_site_count: database.inline_sites.values().map(|v| v.len()).sum(),
                });
            }
        }

        Err(CursedError::RuntimeError("No debug information loaded".to_string()))
    }
}

/// Function parameter from debug information
#[derive(Debug, Clone)]
pub struct FunctionParameter {
    pub name: String,
    pub param_type: String,
    pub location: Option<Vec<u8>>,
}

/// Local variable from debug information
#[derive(Debug, Clone)]
pub struct LocalVariable {
    pub name: String,
    pub var_type: String,
    pub location: Option<Vec<u8>>,
    pub scope_start: u64,
    pub scope_end: u64,
}

/// Inline function information
#[derive(Debug, Clone)]
pub struct InlineFunction {
    pub function_name: String,
    pub call_address: u64,
    pub original_location: Option<(PathBuf, u32, u32)>,
    pub inline_location: Option<(PathBuf, u32, u32)>,
}

/// Enhanced stack frame with DWARF debug information
#[derive(Debug, Clone)]
pub struct EnhancedStackFrame {
    pub address: u64,
    pub symbol: Option<DebugSymbol>,
    pub source_location: Option<SourceLocation>,
    pub parameters: Vec<FunctionParameter>,
    pub local_variables: Vec<LocalVariable>,
    pub inline_functions: Vec<InlineFunction>,
    pub basic_frame: crate::runtime::debug_info::StackFrame,
}

/// Debug information statistics
#[derive(Debug, Clone)]
pub struct DebugInfoStats {
    pub function_count: usize,
    pub variable_count: usize,
    pub type_count: usize,
    pub line_mapping_count: usize,
    pub inline_site_count: usize,
}

/// Integration with LLVM debug metadata
impl DwarfDebugIntegration {
    /// Convert LLVM debug metadata to DWARF debug information
    pub fn import_llvm_debug_metadata(&self, metadata: &LlvmDebugMetadata) -> Result<(), CursedError> {
        // This would integrate with LLVM's debug metadata format
        // and convert it to our DWARF database format
        Ok(())
    }

    /// Export debug information for LLVM codegen
    pub fn export_llvm_debug_metadata(&self, address_range: (u64, u64)) -> Result<Vec<LlvmDebugMetadata>, CursedError> {
        let mut metadata = Vec::new();

        if let Ok(db_lock) = self.dwarf_database.read() {
            if let Some(ref database) = *db_lock {
                // Export function information in LLVM debug metadata format
                for (addr, function) in database.functions.range(address_range.0..=address_range.1) {
                    let llvm_metadata = LlvmDebugMetadata {
                        address: *addr,
                        function_name: Some(function.name.clone()),
                        file_path: function.source_file.clone().unwrap_or_else(|| PathBuf::from("unknown")),
                        line: function.line_range.map(|(start, _)| start).unwrap_or(0),
                        column: 0,
                        scope_depth: 0,
                    };
                    metadata.push(llvm_metadata);
                }

                // Export line information
                for (addr, line_info) in database.line_mappings.range(address_range.0..=address_range.1) {
                    let llvm_metadata = LlvmDebugMetadata {
                        address: *addr,
                        function_name: None,
                        file_path: line_info.file.clone(),
                        line: line_info.line,
                        column: line_info.column,
                        scope_depth: 0,
                    };
                    metadata.push(llvm_metadata);
                }
            }
        }

        Ok(metadata)
    }
}

/// LLVM debug metadata structure
#[derive(Debug, Clone)]
pub struct LlvmDebugMetadata {
    pub address: u64,
    pub function_name: Option<String>,
    pub file_path: PathBuf,
    pub line: u32,
    pub column: u32,
    pub scope_depth: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dwarf_integration_creation() {
        let integration = DwarfDebugIntegration::new();
        assert!(!integration.has_debug_info());
        assert_eq!(integration.get_dwarf_version(), None);
        assert!(integration.get_parsing_errors().is_empty());
    }

    #[test]
    fn test_debug_info_loading() {
        let integration = DwarfDebugIntegration::new();
        
        // Test with invalid data
        let invalid_data = vec![0, 1, 2, 3];
        let result = integration.load_debug_info(&invalid_data);
        
        // Should fail gracefully
        assert!(result.is_err());
    }

    #[test]
    fn test_symbol_cache_operations() {
        let integration = DwarfDebugIntegration::new();
        
        // Test getting symbol when no debug info is loaded
        let result = integration.get_debug_symbol(0x1000);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_source_location_cache() {
        let integration = DwarfDebugIntegration::new();
        
        // Test getting source location when no debug info is loaded
        let result = integration.get_source_location(0x1000);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_clear_debug_info() {
        let integration = DwarfDebugIntegration::new();
        let result = integration.clear_debug_info();
        assert!(result.is_ok());
        assert!(!integration.has_debug_info());
    }

    #[test]
    fn test_debug_info_stats_without_data() {
        let integration = DwarfDebugIntegration::new();
        let result = integration.get_debug_info_stats();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No debug information"));
    }

    #[test]
    fn test_function_parameters_without_data() {
        let integration = DwarfDebugIntegration::new();
        let result = integration.get_function_parameters(0x1000);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_local_variables_without_data() {
        let integration = DwarfDebugIntegration::new();
        let result = integration.get_local_variables(0x1000);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_inline_info_without_data() {
        let integration = DwarfDebugIntegration::new();
        let result = integration.get_inline_info(0x1000);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_enhanced_stack_trace_capture() {
        let integration = DwarfDebugIntegration::new();
        let result = integration.capture_enhanced_stack_trace();
        
        // Should succeed even without debug info
        assert!(result.is_ok());
    }

    #[test]
    fn test_llvm_metadata_export() {
        let integration = DwarfDebugIntegration::new();
        let result = integration.export_llvm_debug_metadata((0x1000, 0x2000));
        
        // Should return empty list when no debug info is loaded
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
