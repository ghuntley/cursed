/// Simple Debug System Tests
/// 
/// Basic functionality tests for the CURSED debugging system including
/// debug info creation, symbol management, and source location tracking.

use cursed::debug::{
    EnhancedDebugInfo, DebugInfoRegistry, SymbolMetadata, SymbolType, SymbolVisibility,
    SourceMap, TypeDebugInfo, TypeKind, FieldDebugInfo, ScopeInfo, ScopeType
};
use cursed::runtime::debug_info::{DebugInfo, VariableInfo};
use cursed::runtime::debug_manager::{DebugManager, SourceFile};
use std::path::PathBuf;
use std::collections::HashMap;

#[path = "common.rs"]
mod common;

macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{info, debug, error};

    #[test]
    fn test_debug_info_creation() {
        init_tracing!();
        info!("Testing basic debug info creation");

        let debug_info = DebugInfo::new("test.csd", 42, 10, "test_function".to_string());
        
        assert_eq!(debug_info.line, 42);
        assert_eq!(debug_info.column, 10);
        assert_eq!(debug_info.function_name, "test_function");
        assert!(debug_info.file_path.to_string_lossy().contains("test.csd"));

        debug!("Basic debug info creation passed");
    }

    #[test]
    fn test_enhanced_debug_info_creation() {
        init_tracing!();
        info!("Testing enhanced debug info creation");

        let enhanced_info = EnhancedDebugInfo::new(
            "test.csd", 
            42, 
            10, 
            "test_function".to_string()
        );
        
        assert_eq!(enhanced_info.debug_info.line, 42);
        assert_eq!(enhanced_info.debug_info.column, 10);
        assert_eq!(enhanced_info.debug_info.function_name, "test_function");
        assert!(enhanced_info.is_user_code());
        
        let location_str = enhanced_info.location_string();
        assert!(location_str.contains("test.csd"));
        assert!(location_str.contains("42"));
        assert!(location_str.contains("10"));

        debug!("Enhanced debug info creation passed");
    }

    #[test]
    fn test_symbol_metadata_creation() {
        init_tracing!();
        info!("Testing symbol metadata creation");

        // Test function metadata
        let func_metadata = SymbolMetadata::function("test_func", Some("slay"));
        assert_eq!(func_metadata.symbol_type, SymbolType::Function);
        assert_eq!(func_metadata.attributes.get("gen_z_keyword"), Some(&"slay".to_string()));
        assert!(func_metadata.tags.contains(&"function".to_string()));

        // Test variable metadata with Gen Z mapping
        let var_metadata = SymbolMetadata::variable("test_var", "i32");
        assert_eq!(var_metadata.symbol_type, SymbolType::Variable);
        assert_eq!(var_metadata.attributes.get("type"), Some(&"i32".to_string()));
        assert_eq!(var_metadata.attributes.get("gen_z_type"), Some(&"sus".to_string()));

        // Test Gen Z type mappings
        let bool_metadata = SymbolMetadata::variable("flag", "bool");
        assert_eq!(bool_metadata.attributes.get("gen_z_type"), Some(&"facts".to_string()));

        let float_metadata = SymbolMetadata::variable("value", "f64");
        assert_eq!(float_metadata.attributes.get("gen_z_type"), Some(&"vibes".to_string()));

        let string_metadata = SymbolMetadata::variable("text", "String");
        assert_eq!(string_metadata.attributes.get("gen_z_type"), Some(&"tea".to_string()));

        debug!("Symbol metadata creation tests passed");
    }

    #[test]
    fn test_source_map_functionality() {
        init_tracing!();
        info!("Testing source map functionality");

        let mut source_map = SourceMap::new(PathBuf::from("test.csd"));
        
        // Add multiple source ranges
        source_map.add_range(10, 5, 8, 3, 15);
        source_map.add_range(20, 0, 18, 2, 10);
        source_map.add_range(30, 10, 28, 5, 20);
        
        // Test mapping within range
        let mapped = source_map.map_to_original(10, 10);
        assert_eq!(mapped, Some((8, 8))); // offset of 5 from column 5
        
        // Test mapping at range start
        let mapped_start = source_map.map_to_original(10, 5);
        assert_eq!(mapped_start, Some((8, 3)));
        
        // Test mapping outside range
        let mapped_outside = source_map.map_to_original(15, 10);
        assert_eq!(mapped_outside, None);
        
        // Test second range
        let mapped_second = source_map.map_to_original(20, 5);
        assert_eq!(mapped_second, Some((18, 7))); // offset of 5 from column 2

        debug!("Source map functionality tests passed");
    }

    #[test]
    fn test_type_debug_info() {
        init_tracing!();
        info!("Testing type debug info");

        let type_info = TypeDebugInfo::new("TestStruct".to_string(), TypeKind::Struct)
            .with_field(FieldDebugInfo::new("field1".to_string(), "i32".to_string()))
            .with_field(FieldDebugInfo::new("field2".to_string(), "String".to_string()))
            .with_type_parameter("T".to_string());
        
        assert_eq!(type_info.type_name, "TestStruct");
        assert_eq!(type_info.type_kind, TypeKind::Struct);
        assert_eq!(type_info.fields.len(), 2);
        assert_eq!(type_info.fields[0].name, "field1");
        assert_eq!(type_info.fields[0].field_type, "i32");
        assert_eq!(type_info.fields[1].name, "field2");
        assert_eq!(type_info.fields[1].field_type, "String");
        assert_eq!(type_info.type_parameters.len(), 1);
        assert_eq!(type_info.type_parameters[0], "T");

        debug!("Type debug info tests passed");
    }

    #[test]
    fn test_scope_info_creation() {
        init_tracing!();
        info!("Testing scope info creation");

        let mut scope = ScopeInfo::function_scope(1);
        scope.start_location = Some((10, 5));
        scope.end_location = Some((50, 10));
        
        let var_info = VariableInfo::new("test_var".to_string(), "i32".to_string());
        scope.add_variable("test_var".to_string(), var_info);
        
        assert_eq!(scope.scope_type, ScopeType::Function);
        assert_eq!(scope.depth, 1);
        assert!(scope.has_variable("test_var"));
        assert!(!scope.has_variable("nonexistent_var"));
        assert_eq!(scope.start_location, Some((10, 5)));
        assert_eq!(scope.end_location, Some((50, 10)));

        // Test block scope
        let block_scope = ScopeInfo::new();
        assert_eq!(block_scope.scope_type, ScopeType::Block);
        assert_eq!(block_scope.depth, 0);

        debug!("Scope info creation tests passed");
    }

    #[test]
    fn test_debug_info_registry_basic() {
        init_tracing!();
        info!("Testing debug info registry basic operations");

        let registry = DebugInfoRegistry::new();
        
        // Register debug information
        let debug_info = EnhancedDebugInfo::new("test.csd", 42, 10, "test_function".to_string());
        let location_key = "test.csd:42:10".to_string();
        
        let result = registry.register_debug_info(location_key.clone(), debug_info);
        assert!(result.is_ok());
        
        // Retrieve debug information
        let retrieved = registry.get_debug_info(&location_key);
        assert!(retrieved.is_ok());
        let retrieved_info = retrieved.unwrap();
        assert!(retrieved_info.is_some());
        
        let info = retrieved_info.unwrap();
        assert_eq!(info.debug_info.line, 42);
        assert_eq!(info.debug_info.column, 10);
        assert_eq!(info.debug_info.function_name, "test_function");

        debug!("Debug info registry basic operations passed");
    }

    #[test]
    fn test_debug_info_registry_symbols() {
        init_tracing!();
        info!("Testing debug info registry symbol operations");

        let registry = DebugInfoRegistry::new();
        
        // Register multiple symbols
        let func_metadata = SymbolMetadata::function("test_function", Some("slay"));
        let var_metadata = SymbolMetadata::variable("test_var", "i32");
        
        registry.register_symbol("module::test_function".to_string(), func_metadata).unwrap();
        registry.register_symbol("module::test_var".to_string(), var_metadata).unwrap();
        
        // Test symbol retrieval
        let func_symbol = registry.get_symbol("module::test_function").unwrap();
        assert!(func_symbol.is_some());
        assert_eq!(func_symbol.unwrap().symbol_type, SymbolType::Function);
        
        let var_symbol = registry.get_symbol("module::test_var").unwrap();
        assert!(var_symbol.is_some());
        assert_eq!(var_symbol.unwrap().symbol_type, SymbolType::Variable);
        
        // Test symbol search
        let matches = registry.find_symbols("test").unwrap();
        assert_eq!(matches.len(), 2);
        
        let function_matches = registry.find_symbols("function").unwrap();
        assert_eq!(function_matches.len(), 1);

        debug!("Debug info registry symbol operations passed");
    }

    #[test]
    fn test_debug_info_registry_types() {
        init_tracing!();
        info!("Testing debug info registry type operations");

        let registry = DebugInfoRegistry::new();
        
        // Register type information
        let type_info = TypeDebugInfo::new("TestStruct".to_string(), TypeKind::Struct)
            .with_field(FieldDebugInfo::new("field1".to_string(), "i32".to_string()));
        
        registry.register_type("TestStruct".to_string(), type_info).unwrap();
        
        // Retrieve type information
        let retrieved_type = registry.get_type("TestStruct").unwrap();
        assert!(retrieved_type.is_some());
        
        let type_info = retrieved_type.unwrap();
        assert_eq!(type_info.type_name, "TestStruct");
        assert_eq!(type_info.type_kind, TypeKind::Struct);
        assert_eq!(type_info.fields.len(), 1);

        debug!("Debug info registry type operations passed");
    }

    #[test]
    fn test_debug_info_registry_scopes() {
        init_tracing!();
        info!("Testing debug info registry scope operations");

        let registry = DebugInfoRegistry::new();
        
        // Create scope
        let scope_info = ScopeInfo::function_scope(1);
        let scope_id = registry.create_scope(scope_info).unwrap();
        assert!(scope_id > 0);
        
        // Retrieve scope
        let retrieved_scope = registry.get_scope(scope_id).unwrap();
        assert!(retrieved_scope.is_some());
        
        let scope = retrieved_scope.unwrap();
        assert_eq!(scope.scope_type, ScopeType::Function);
        assert_eq!(scope.depth, 1);

        debug!("Debug info registry scope operations passed");
    }

    #[test]
    fn test_debug_statistics() {
        init_tracing!();
        info!("Testing debug statistics");

        let registry = DebugInfoRegistry::new();
        
        // Add some debug information
        let debug_info = EnhancedDebugInfo::new("test.csd", 42, 10, "test_function".to_string());
        registry.register_debug_info("test:42:10".to_string(), debug_info).unwrap();
        
        let symbol_metadata = SymbolMetadata::function("test_func", Some("slay"));
        registry.register_symbol("test_func".to_string(), symbol_metadata).unwrap();
        
        let type_info = TypeDebugInfo::new("TestStruct".to_string(), TypeKind::Struct);
        registry.register_type("TestStruct".to_string(), type_info).unwrap();
        
        let scope_info = ScopeInfo::function_scope(1);
        registry.create_scope(scope_info).unwrap();
        
        // Get statistics
        let stats = registry.get_statistics().unwrap();
        assert_eq!(stats.debug_info_count, 1);
        assert_eq!(stats.symbol_count, 1);
        assert_eq!(stats.type_count, 1);
        assert_eq!(stats.scope_count, 1);
        
        // Test display formatting
        let stats_str = format!("{}", stats);
        assert!(stats_str.contains("Debug Info: 1"));
        assert!(stats_str.contains("Symbols: 1"));
        assert!(stats_str.contains("Types: 1"));
        assert!(stats_str.contains("Scopes: 1"));

        debug!("Debug statistics tests passed");
    }

    #[test]
    fn test_source_file_functionality() {
        init_tracing!();
        info!("Testing source file functionality");

        // Create a temporary test file
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_debug.csd");
        let test_content = "line 1\nline 2\nline 3\nline 4\n";
        std::fs::write(&test_file, test_content).unwrap();
        
        let mut source_file = SourceFile::new(&test_file);
        assert!(!source_file.is_cached);
        assert!(source_file.content.is_none());
        
        // Load content
        let load_result = source_file.load_content();
        assert!(load_result.is_ok());
        assert!(source_file.is_cached);
        assert!(source_file.content.is_some());
        
        // Test line retrieval
        let line_1 = source_file.get_line(1);
        assert_eq!(line_1, Some("line 1".to_string()));
        
        let line_2 = source_file.get_line(2);
        assert_eq!(line_2, Some("line 2".to_string()));
        
        let line_nonexistent = source_file.get_line(10);
        assert_eq!(line_nonexistent, None);
        
        // Test context lines
        let context_lines = source_file.get_lines_with_context(2, 1);
        assert!(context_lines.is_some());
        let lines = context_lines.unwrap();
        assert_eq!(lines.len(), 3); // lines 1, 2, 3
        assert_eq!(lines[0], (1, "line 1".to_string()));
        assert_eq!(lines[1], (2, "line 2".to_string()));
        assert_eq!(lines[2], (3, "line 3".to_string()));
        
        // Cleanup
        std::fs::remove_file(&test_file).ok();

        debug!("Source file functionality tests passed");
    }

    #[test]
    fn test_debug_manager_basic() {
        init_tracing!();
        info!("Testing debug manager basic functionality");

        let mut debug_manager = DebugManager::new();
        
        // Test configuration
        assert!(!debug_manager.is_enabled());
        debug_manager.enable();
        assert!(debug_manager.is_enabled());
        debug_manager.disable();
        assert!(!debug_manager.is_enabled());
        
        // Test debug information addition
        let debug_info = DebugInfo::new("test.csd", 42, 10, "test_function".to_string());
        let location_key = "test.csd:42:10".to_string();
        
        let add_result = debug_manager.add_debug_info(location_key.clone(), debug_info);
        assert!(add_result.is_ok());
        
        // Test retrieval
        let retrieved = debug_manager.get_debug_info(&location_key);
        assert!(retrieved.is_some());
        
        let info = retrieved.unwrap();
        assert_eq!(info.line, 42);
        assert_eq!(info.column, 10);
        assert_eq!(info.function_name, "test_function");

        debug!("Debug manager basic functionality tests passed");
    }

    #[test]
    fn test_qualified_symbol_names() {
        init_tracing!();
        info!("Testing qualified symbol names");

        // Test with module name
        let mut debug_info = EnhancedDebugInfo::new("test.csd", 42, 10, "test_function".to_string());
        debug_info.debug_info.module_name = Some("test_module".to_string());
        
        let qualified_name = debug_info.qualified_symbol_name();
        assert_eq!(qualified_name, "test_module::test_function");
        
        // Test without module name
        let debug_info_no_module = EnhancedDebugInfo::new("test.csd", 42, 10, "test_function".to_string());
        let simple_name = debug_info_no_module.qualified_symbol_name();
        assert_eq!(simple_name, "test_function");

        debug!("Qualified symbol names tests passed");
    }

    #[test]
    fn test_user_code_detection() {
        init_tracing!();
        info!("Testing user code detection");

        // CURSED source file (.csd extension)
        let cursed_info = EnhancedDebugInfo::new("user_code.csd", 42, 10, "user_function".to_string());
        assert!(cursed_info.is_user_code());
        
        // Non-CURSED source file
        let other_info = EnhancedDebugInfo::new("system_code.rs", 42, 10, "system_function".to_string());
        assert!(!other_info.is_user_code());
        
        // File without extension
        let no_ext_info = EnhancedDebugInfo::new("noext", 42, 10, "function".to_string());
        assert!(!no_ext_info.is_user_code());

        debug!("User code detection tests passed");
    }

    #[test]
    fn test_error_handling() {
        init_tracing!();
        info!("Testing error handling in debug system");

        let registry = DebugInfoRegistry::new();
        
        // Test error handling for non-existent items
        let non_existent_debug = registry.get_debug_info("non_existent_key");
        assert!(non_existent_debug.is_ok());
        assert!(non_existent_debug.unwrap().is_none());
        
        let non_existent_symbol = registry.get_symbol("non_existent_symbol");
        assert!(non_existent_symbol.is_ok());
        assert!(non_existent_symbol.unwrap().is_none());
        
        let non_existent_type = registry.get_type("NonExistentType");
        assert!(non_existent_type.is_ok());
        assert!(non_existent_type.unwrap().is_none());
        
        let non_existent_scope = registry.get_scope(99999);
        assert!(non_existent_scope.is_ok());
        assert!(non_existent_scope.unwrap().is_none());

        debug!("Error handling tests passed");
    }
}
