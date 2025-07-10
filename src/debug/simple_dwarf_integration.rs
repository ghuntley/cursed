//! Simple DWARF debug information integration
//!
//! This provides a working integration between DWARF parsing and the debug engine.

use crate::runtime::dwarf_parser::{DwarfDebugDatabase, RegisterMap};
use crate::debug::DebugSymbol;
use crate::error::{CursedError, SourceLocation};
use std::sync::{Arc, RwLock};

/// Simple DWARF debug integration
pub struct SimpleDwarfIntegration {
    database: Arc<RwLock<Option<DwarfDebugDatabase>>>,
}

impl SimpleDwarfIntegration {
    /// Create new integration
    pub fn new() -> Self {
        Self {
            database: Arc::new(RwLock::new(None)),
        }
    }

    /// Load DWARF debug information
    pub fn load_debug_info(&self, dwarf_data: &[u8]) -> Result<(), CursedError> {
        let mut database = DwarfDebugDatabase::new();
        database.load_from_dwarf(dwarf_data)?;
        
        if let Ok(mut db_lock) = self.database.write() {
            *db_lock = Some(database);
            Ok(())
        } else {
            Err(CursedError::RuntimeError("Failed to store database".to_string()))
        }
    }

    /// Get function name at address
    pub fn get_function_name(&self, address: u64) -> Option<String> {
        if let Ok(db_lock) = self.database.read() {
            if let Some(ref database) = *db_lock {
                return database.find_function(address).map(|f| f.name.clone());
            }
        }
        None
    }

    /// Get source location for address
    pub fn get_source_location(&self, address: u64) -> Option<SourceLocation> {
        if let Ok(db_lock) = self.database.read() {
            if let Some(ref database) = *db_lock {
                if let Some(line_info) = database.get_source_location_for_address(address) {
                    return Some(SourceLocation {
                        file: line_info.file.to_string_lossy().to_string(),
                        line: line_info.line as usize,
                        column: line_info.column as usize,
                    });
                }
            }
        }
        None
    }

    /// Check if debug info is available
    pub fn has_debug_info(&self) -> bool {
        if let Ok(db_lock) = self.database.read() {
            db_lock.is_some()
        } else {
            false
        }
    }

    /// Get debug statistics
    pub fn get_stats(&self) -> (usize, usize, usize) {
        if let Ok(db_lock) = self.database.read() {
            if let Some(ref database) = *db_lock {
                return (
                    database.functions.len(),
                    database.types.len(), 
                    database.line_mappings.len()
                );
            }
        }
        (0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_integration() {
        let integration = SimpleDwarfIntegration::new();
        assert!(!integration.has_debug_info());
        
        // Load some mock data
        let mock_data = vec![0x7f, 0x45, 0x4c, 0x46];
        let _ = integration.load_debug_info(&mock_data);
        
        assert!(integration.has_debug_info());
        
        // Test function lookup
        let function_name = integration.get_function_name(0x1000);
        assert_eq!(function_name, Some("test_function".to_string()));
    }
}
