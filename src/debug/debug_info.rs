/// Main debug information coordinator
use crate::debug::{
    debug_symbols::{DebugSymbolTable, DebugSymbol, DebugSymbolType}, 
    dwarf_gen::DwarfGenerator, 
    source_location::{SourceLocationManager, SourceLocation},
    DebugConfig
};
use crate::error::Error;
use std::path::PathBuf;
use tracing::{debug, info, instrument, warn};

/// Main debug information manager that coordinates all debug-related functionality
#[derive(Debug)]
pub struct DebugInfoManager {
    symbol_table: DebugSymbolTable,
    dwarf_generator: DwarfGenerator,
    location_manager: SourceLocationManager,
    config: DebugConfig,
    is_enabled: bool,
    current_function: Option<String>,
}

impl DebugInfoManager {
    /// Create a new debug information manager
    pub fn new(config: DebugConfig) -> Self {
        Self {
            symbol_table: DebugSymbolTable::new(),
            dwarf_generator: DwarfGenerator::new(),
            location_manager: SourceLocationManager::new(),
            is_enabled: config.generate_debug_info,
            config,
            current_function: None,
        }
    }

    /// Initialize debug information for a compilation unit
    #[instrument(skip(self))]
    pub fn initialize_compilation_unit(&mut self, file: PathBuf, producer: String) -> Result<(), Error> {
        if !self.is_enabled {
            return Ok(());
        }

        info!(file = ?file, producer = %producer, "Initializing debug compilation unit");
        
        self.location_manager.set_current_file(file.clone());
        self.dwarf_generator.set_compile_unit(file, producer);
        
        Ok(())
    }

    /// Enter a new scope (function, block, etc.)
    #[instrument(skip(self))]
    pub fn enter_scope(&mut self, location: SourceLocation) {
        if !self.is_enabled {
            return;
        }

        debug!(location = ?location, "Entering scope");
        self.symbol_table.enter_scope();
        self.location_manager.push_location(location);
    }

    /// Exit the current scope
    #[instrument(skip(self))]
    pub fn exit_scope(&mut self) {
        if !self.is_enabled {
            return;
        }

        debug!("Exiting scope");
        self.symbol_table.exit_scope();
        self.location_manager.pop_location();
    }

    /// Begin a function definition
    #[instrument(skip(self))]
    pub fn begin_function(&mut self, name: String, location: SourceLocation) -> Result<(), Error> {
        if !self.is_enabled {
            return Ok(());
        }

        info!(function = %name, location = ?location, "Beginning function definition");
        
        self.current_function = Some(name.clone());
        self.enter_scope(location.clone());
        
        // Add function symbol
        let function_symbol = DebugSymbol::function(name, location);
        self.symbol_table.add_symbol(function_symbol)
            .map_err(|e| Error::Compile(format!("Failed to add function symbol: {}", e)))?;
        
        Ok(())
    }

    /// End a function definition
    #[instrument(skip(self))]
    pub fn end_function(&mut self) -> Result<(), Error> {
        if !self.is_enabled {
            return Ok(());
        }

        if let Some(function_name) = &self.current_function {
            info!(function = %function_name, "Ending function definition");
        }
        
        self.current_function = None;
        self.exit_scope();
        
        Ok(())
    }

    /// Add a variable declaration
    #[instrument(skip(self))]
    pub fn add_variable(
        &mut self,
        name: String,
        type_name: String,
        location: SourceLocation,
    ) -> Result<(), Error> {
        if !self.is_enabled {
            return Ok(());
        }

        debug!(name = %name, type_name = %type_name, location = ?location, "Adding variable");
        
        let variable_symbol = DebugSymbol::variable(name, type_name, location);
        self.symbol_table.add_symbol(variable_symbol)
            .map_err(|e| Error::Compile(format!("Failed to add variable symbol: {}", e)))?;
        
        Ok(())
    }

    /// Add a function parameter
    #[instrument(skip(self))]
    pub fn add_parameter(
        &mut self,
        name: String,
        type_name: String,
        location: SourceLocation,
    ) -> Result<(), Error> {
        if !self.is_enabled {
            return Ok(());
        }

        debug!(name = %name, type_name = %type_name, location = ?location, "Adding parameter");
        
        let parameter_symbol = DebugSymbol::parameter(name, type_name, location);
        self.symbol_table.add_symbol(parameter_symbol)
            .map_err(|e| Error::Compile(format!("Failed to add parameter symbol: {}", e)))?;
        
        Ok(())
    }

    /// Add a type definition
    #[instrument(skip(self))]
    pub fn add_type(
        &mut self,
        name: String,
        location: SourceLocation,
    ) -> Result<(), Error> {
        if !self.is_enabled {
            return Ok(());
        }

        debug!(name = %name, location = ?location, "Adding type definition");
        
        let type_symbol = DebugSymbol::type_def(name, location);
        self.symbol_table.add_symbol(type_symbol)
            .map_err(|e| Error::Compile(format!("Failed to add type symbol: {}", e)))?;
        
        Ok(())
    }

    /// Set current source location for subsequent operations
    #[instrument(skip(self))]
    pub fn set_current_location(&mut self, location: SourceLocation) {
        if !self.is_enabled {
            return;
        }

        self.location_manager.push_location(location);
    }

    /// Get the current source location
    pub fn current_location(&self) -> Option<&SourceLocation> {
        self.location_manager.current_location()
    }

    /// Generate LLVM debug metadata
    #[instrument(skip(self))]
    pub fn generate_llvm_debug_metadata(&mut self) -> Result<String, Error> {
        if !self.is_enabled {
            return Ok(String::new());
        }

        info!("Generating LLVM debug metadata");
        
        // Export symbols to DWARF generator
        let symbols = self.symbol_table.export_for_dwarf();
        self.dwarf_generator.add_symbols(symbols);
        
        // Generate the metadata
        let metadata = self.dwarf_generator.generate_llvm_metadata();
        
        debug!(metadata_size = metadata.len(), "Generated debug metadata");
        Ok(metadata)
    }

    /// Generate debug location for an LLVM instruction
    pub fn generate_debug_location(&self, location: &SourceLocation) -> String {
        if !self.is_enabled {
            return String::new();
        }

        self.dwarf_generator.generate_debug_location(location)
    }

    /// Generate line number information
    pub fn generate_line_table(&self) -> Vec<(u32, String)> {
        if !self.is_enabled {
            return Vec::new();
        }

        self.dwarf_generator.generate_line_table()
    }

    /// Look up a symbol by name
    pub fn lookup_symbol(&self, name: &str) -> Option<&DebugSymbol> {
        if !self.is_enabled {
            return None;
        }

        self.symbol_table.lookup_symbol(name)
    }

    /// Get symbols at a specific location
    pub fn symbols_at_location(&self, location: &SourceLocation) -> Vec<&DebugSymbol> {
        if !self.is_enabled {
            return Vec::new();
        }

        self.symbol_table.symbols_at_location(location)
    }

    /// Get all functions
    pub fn functions(&self) -> Vec<&DebugSymbol> {
        if !self.is_enabled {
            return Vec::new();
        }

        self.symbol_table.symbols_by_type(DebugSymbolType::Function)
    }

    /// Get all variables in current scope
    pub fn current_scope_variables(&self) -> Vec<&DebugSymbol> {
        if !self.is_enabled {
            return Vec::new();
        }

        self.symbol_table.current_scope_symbols()
            .into_iter()
            .filter(|s| s.symbol_type == DebugSymbolType::Variable)
            .collect()
    }

    /// Enable or disable debug information generation
    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
        info!(enabled = enabled, "Debug information generation toggled");
    }

    /// Check if debug information generation is enabled
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    /// Get debug configuration
    pub fn config(&self) -> &DebugConfig {
        &self.config
    }

    /// Update debug configuration
    pub fn update_config(&mut self, config: DebugConfig) {
        self.is_enabled = config.generate_debug_info;
        self.config = config;
        info!(config = ?self.config, "Updated debug configuration");
    }

    /// Get debug statistics
    pub fn statistics(&self) -> DebugStatistics {
        let dwarf_stats = self.dwarf_generator.statistics();
        
        DebugStatistics {
            enabled: self.is_enabled,
            symbol_count: self.symbol_table.symbol_count(),
            scope_depth: self.symbol_table.scope_depth(),
            dwarf_compile_units: dwarf_stats.compile_units,
            dwarf_subprograms: dwarf_stats.subprograms,
            dwarf_variables: dwarf_stats.variables,
            dwarf_types: dwarf_stats.types,
        }
    }

    /// Clear all debug information
    pub fn clear(&mut self) {
        self.symbol_table.clear();
        self.dwarf_generator.clear();
        self.location_manager.clear();
        self.current_function = None;
        info!("Cleared all debug information");
    }

    /// Generate a debug report
    pub fn generate_debug_report(&self) -> DebugReport {
        let stats = self.statistics();
        let line_table = self.generate_line_table();
        let functions = self.functions().iter().map(|s| s.name.clone()).collect();
        
        DebugReport {
            statistics: stats,
            line_table,
            functions,
            current_function: self.current_function.clone(),
            current_location: self.current_location().cloned(),
        }
    }

    /// Validate debug information consistency
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Check if we have a compile unit when we have symbols
        if self.symbol_table.symbol_count() > 0 && self.dwarf_generator.statistics().compile_units == 0 {
            errors.push("Symbols exist but no compile unit defined".to_string());
        }
        
        // Check for orphaned parameters (parameters without functions)
        let parameters = self.symbol_table.symbols_by_type(DebugSymbolType::Parameter);
        let functions = self.symbol_table.symbols_by_type(DebugSymbolType::Function);
        
        if !parameters.is_empty() && functions.is_empty() {
            errors.push("Parameters exist without any functions".to_string());
        }
        
        // Check for invalid source locations
        for symbol in self.symbol_table.all_symbols() {
            if !symbol.location.is_valid() {
                errors.push(format!("Invalid source location for symbol '{}'", symbol.name));
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Default for DebugInfoManager {
    fn default() -> Self {
        Self::new(DebugConfig::default())
    }
}

/// Debug information statistics
#[derive(Debug, Clone)]
pub struct DebugStatistics {
    pub enabled: bool,
    pub symbol_count: usize,
    pub scope_depth: usize,
    pub dwarf_compile_units: usize,
    pub dwarf_subprograms: usize,
    pub dwarf_variables: usize,
    pub dwarf_types: usize,
}

impl std::fmt::Display for DebugStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Debug Stats: {} symbols, depth {}, {} CUs, {} functions, {} vars, {} types ({})",
            self.symbol_count,
            self.scope_depth,
            self.dwarf_compile_units,
            self.dwarf_subprograms,
            self.dwarf_variables,
            self.dwarf_types,
            if self.enabled { "enabled" } else { "disabled" }
        )
    }
}

/// Comprehensive debug report
#[derive(Debug, Clone)]
pub struct DebugReport {
    pub statistics: DebugStatistics,
    pub line_table: Vec<(u32, String)>,
    pub functions: Vec<String>,
    pub current_function: Option<String>,
    pub current_location: Option<SourceLocation>,
}

impl std::fmt::Display for DebugReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== Debug Information Report ===")?;
        writeln!(f, "{}", self.statistics)?;
        writeln!(f, "Functions: {}", self.functions.join(", "))?;
        
        if let Some(current_func) = &self.current_function {
            writeln!(f, "Current function: {}", current_func)?;
        }
        
        if let Some(current_loc) = &self.current_location {
            writeln!(f, "Current location: {}", current_loc)?;
        }
        
        writeln!(f, "Line table entries: {}", self.line_table.len())?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_debug_info_manager_creation() {
        let config = DebugConfig::default();
        let manager = DebugInfoManager::new(config);
        
        assert!(manager.is_enabled());
        assert!(manager.current_location().is_none());
        assert_eq!(manager.statistics().symbol_count, 0);
    }

    #[test]
    fn test_compilation_unit_initialization() {
        let mut manager = DebugInfoManager::new(DebugConfig::default());
        let file = PathBuf::from("test.csd");
        let producer = "Test Compiler".to_string();
        
        let result = manager.initialize_compilation_unit(file.clone(), producer);
        assert!(result.is_ok());
        
        let stats = manager.statistics();
        assert_eq!(stats.dwarf_compile_units, 1);
    }

    #[test]
    fn test_function_management() {
        let mut manager = DebugInfoManager::new(DebugConfig::default());
        let location = SourceLocation::new(PathBuf::from("test.csd"), 10, 1);
        
        manager.begin_function("test_func".to_string(), location.clone()).unwrap();
        assert_eq!(manager.current_function, Some("test_func".to_string()));
        
        let functions = manager.functions();
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "test_func");
        
        manager.end_function().unwrap();
        assert_eq!(manager.current_function, None);
    }

    #[test]
    fn test_variable_addition() {
        let mut manager = DebugInfoManager::new(DebugConfig::default());
        let location = SourceLocation::new(PathBuf::from("test.csd"), 15, 5);
        
        manager.add_variable("x".to_string(), "int".to_string(), location).unwrap();
        
        let symbol = manager.lookup_symbol("x");
        assert!(symbol.is_some());
        assert_eq!(symbol.unwrap().name, "x");
        assert_eq!(symbol.unwrap().type_name, "int");
    }

    #[test]
    fn test_scope_management() {
        let mut manager = DebugInfoManager::new(DebugConfig::default());
        let location = SourceLocation::new(PathBuf::from("test.csd"), 10, 1);
        
        assert_eq!(manager.statistics().scope_depth, 0);
        
        manager.enter_scope(location.clone());
        assert_eq!(manager.statistics().scope_depth, 1);
        
        manager.exit_scope();
        assert_eq!(manager.statistics().scope_depth, 0);
    }

    #[test]
    fn test_debug_metadata_generation() {
        let mut manager = DebugInfoManager::new(DebugConfig::default());
        let file = PathBuf::from("test.csd");
        let location = SourceLocation::new(file.clone(), 10, 1);
        
        manager.initialize_compilation_unit(file, "Test".to_string()).unwrap();
        manager.begin_function("main".to_string(), location).unwrap();
        
        let metadata = manager.generate_llvm_debug_metadata().unwrap();
        assert!(!metadata.is_empty());
        assert!(metadata.contains("!DICompileUnit"));
    }

    #[test]
    fn test_disabled_debug_info() {
        let config = DebugConfig {
            generate_debug_info: false,
            ..Default::default()
        };
        let mut manager = DebugInfoManager::new(config);
        let location = SourceLocation::new(PathBuf::from("test.csd"), 10, 1);
        
        // Operations should succeed but not generate debug info
        manager.begin_function("test".to_string(), location.clone()).unwrap();
        manager.add_variable("x".to_string(), "int".to_string(), location).unwrap();
        
        assert_eq!(manager.statistics().symbol_count, 0);
        assert!(manager.lookup_symbol("x").is_none());
    }

    #[test]
    fn test_validation() {
        let mut manager = DebugInfoManager::new(DebugConfig::default());
        let location = SourceLocation::new(PathBuf::from("test.csd"), 10, 1);
        
        // Valid state
        manager.initialize_compilation_unit(PathBuf::from("test.csd"), "Test".to_string()).unwrap();
        manager.begin_function("main".to_string(), location).unwrap();
        
        assert!(manager.validate().is_ok());
        
        // Invalid state - clear compile unit but keep symbols
        manager.dwarf_generator.clear();
        let validation_result = manager.validate();
        assert!(validation_result.is_err());
    }
}
