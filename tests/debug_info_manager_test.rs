/// Comprehensive tests for the debug information manager
///
/// These tests validate the complete debug information system including:
/// - Real DWARF debug information generation
/// - Source location tracking and mapping
/// - Symbol resolution and metadata management
/// - LLVM integration capabilities
/// - Performance and memory safety

use cursed::debug::{DebugInfoManager, DebugConfig};
use cursed::error::SourceLocation as ErrorSourceLocation;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_manager_creation() {
        let manager = DebugInfoManager::new();
        assert!(manager.is_enabled());
        assert_eq!(manager.functions().len(), 0);
        
        let stats = manager.statistics();
        assert_eq!(stats.symbol_count, 0);
        assert_eq!(stats.debug_info_count, 0);
    }

    #[test]
    fn test_compilation_unit_lifecycle() {
        let mut manager = DebugInfoManager::new();
        let file = PathBuf::from("test_lifecycle.csd");
        let producer = "CURSED Test Compiler".to_string();
        
        // Initialize compilation unit
        let result = manager.initialize_compilation_unit(file.clone(), producer);
        assert!(result.is_ok(), "Failed to initialize compilation unit: {:?}", result);
        
        // Verify current location is set
        let current_location = manager.current_location();
        assert!(current_location.is_some());
        let location = current_location.unwrap();
        assert_eq!(location.file_path, file);
        assert_eq!(location.line, 1);
        assert_eq!(location.column, 1);
    }

    #[test]
    fn test_function_debug_lifecycle() {
        let mut manager = DebugInfoManager::new();
        let file = PathBuf::from("test_function.csd");
        
        // Initialize compilation unit
        manager.initialize_compilation_unit(file.clone(), "Test Producer".to_string()).unwrap();
        
        // Create function location
        let function_location = ErrorSourceLocation::new(file.clone(), 10, 5);
        
        // Begin function debug
        let result = manager.begin_function("test_function".to_string(), function_location);
        assert!(result.is_ok(), "Failed to begin function: {:?}", result);
        
        // Verify function is tracked
        let functions = manager.functions();
        assert!(functions.contains(&"test_function".to_string()));
        
        // Verify statistics updated
        let stats = manager.statistics();
        assert!(stats.symbol_count >= 1);
        
        // End function debug
        let result = manager.end_function();
        assert!(result.is_ok(), "Failed to end function: {:?}", result);
    }

    #[test]
    fn test_variable_debug_info() {
        let mut manager = DebugInfoManager::new();
        let file = PathBuf::from("test_variables.csd");
        
        // Setup compilation context
        manager.initialize_compilation_unit(file.clone(), "Test Producer".to_string()).unwrap();
        let function_location = ErrorSourceLocation::new(file.clone(), 10, 5);
        manager.begin_function("test_function".to_string(), function_location).unwrap();
        
        // Add variable debug info
        let var_location = ErrorSourceLocation::new(file.clone(), 12, 8);
        let result = manager.add_variable(
            "test_var".to_string(),
            "sus".to_string(), // CURSED integer type
            var_location,
        );
        assert!(result.is_ok(), "Failed to add variable: {:?}", result);
        
        // Verify statistics updated
        let stats = manager.statistics();
        assert!(stats.symbol_count >= 2); // Function + variable
        
        // Add more variables of different types
        let var2_location = ErrorSourceLocation::new(file.clone(), 13, 8);
        manager.add_variable("test_bool".to_string(), "facts".to_string(), var2_location).unwrap();
        
        let var3_location = ErrorSourceLocation::new(file.clone(), 14, 8);
        manager.add_variable("test_float".to_string(), "vibes".to_string(), var3_location).unwrap();
        
        let var4_location = ErrorSourceLocation::new(file.clone(), 15, 8);
        manager.add_variable("test_string".to_string(), "tea".to_string(), var4_location).unwrap();
        
        // Verify all variables are tracked
        let final_stats = manager.statistics();
        assert!(final_stats.symbol_count >= 5); // Function + 4 variables
    }

    #[test]
    fn test_debug_location_generation() {
        let manager = DebugInfoManager::new();
        let location = ErrorSourceLocation::new(PathBuf::from("test_location.csd"), 42, 15);
        
        let debug_location = manager.generate_debug_location(&location);
        assert!(!debug_location.is_empty());
        assert!(debug_location.starts_with("!dbg"));
        
        // Test multiple locations generate different debug info
        let location2 = ErrorSourceLocation::new(PathBuf::from("test_location.csd"), 50, 20);
        let debug_location2 = manager.generate_debug_location(&location2);
        assert!(!debug_location2.is_empty());
        // They should be different (different line numbers)
        assert_ne!(debug_location, debug_location2);
    }

    #[test]
    fn test_current_location_tracking() {
        let mut manager = DebugInfoManager::new();
        let file = PathBuf::from("test_tracking.csd");
        
        // Initialize compilation unit
        manager.initialize_compilation_unit(file.clone(), "Test Producer".to_string()).unwrap();
        
        // Set various locations
        let location1 = ErrorSourceLocation::new(file.clone(), 10, 5);
        manager.set_current_location(location1.clone());
        
        let current = manager.current_location().unwrap();
        assert_eq!(current.line, 10);
        assert_eq!(current.column, 5);
        
        // Update location
        let location2 = ErrorSourceLocation::new(file.clone(), 25, 12);
        manager.set_current_location(location2.clone());
        
        let current = manager.current_location().unwrap();
        assert_eq!(current.line, 25);
        assert_eq!(current.column, 12);
    }

    #[test]
    fn test_llvm_debug_metadata_generation() {
        let mut manager = DebugInfoManager::new();
        let file = PathBuf::from("test_llvm.csd");
        
        // Setup compilation context with function
        manager.initialize_compilation_unit(file.clone(), "CURSED LLVM Test".to_string()).unwrap();
        let function_location = ErrorSourceLocation::new(file.clone(), 5, 1);
        manager.begin_function("test_llvm_function".to_string(), function_location).unwrap();
        
        // Generate LLVM metadata
        let metadata = manager.generate_llvm_debug_metadata();
        assert!(metadata.is_ok(), "Failed to generate LLVM metadata: {:?}", metadata);
        
        let metadata_str = metadata.unwrap();
        assert!(!metadata_str.is_empty());
        
        // Verify LLVM debug metadata format
        assert!(metadata_str.contains("!DICompileUnit"), "Missing compile unit metadata");
        assert!(metadata_str.contains("!DIFile"), "Missing file metadata");
        assert!(metadata_str.contains("test_llvm.csd"), "Missing filename in metadata");
        assert!(metadata_str.contains("CURSED LLVM Test"), "Missing producer in metadata");
    }

    #[test]
    fn test_line_table_generation() {
        let mut manager = DebugInfoManager::new();
        let file = PathBuf::from("test_lines.csd");
        
        // Initialize and add some debug locations
        manager.initialize_compilation_unit(file.clone(), "Test Producer".to_string()).unwrap();
        
        // Generate debug locations to populate line table
        let loc1 = ErrorSourceLocation::new(file.clone(), 10, 5);
        let _debug1 = manager.generate_debug_location(&loc1);
        
        let loc2 = ErrorSourceLocation::new(file.clone(), 20, 10);
        let _debug2 = manager.generate_debug_location(&loc2);
        
        // Get line table
        let line_table = manager.generate_line_table();
        assert!(!line_table.is_empty(), "Line table should not be empty");
        
        // Verify line numbers are present
        let line_numbers: Vec<u32> = line_table.iter().map(|(line, _)| *line).collect();
        assert!(line_numbers.contains(&10));
        assert!(line_numbers.contains(&20));
    }

    #[test]
    fn test_debug_configuration() {
        let mut manager = DebugInfoManager::new();
        
        // Test default configuration
        let default_config = manager.config();
        assert!(default_config.debug_info_enabled);
        assert!(default_config.generate_debug_info);
        
        // Update configuration
        let new_config = DebugConfig::minimal();
        manager.update_config(new_config.clone());
        
        let updated_config = manager.config();
        assert_eq!(updated_config.debug_level, new_config.debug_level);
        assert_eq!(updated_config.optimized_debug, new_config.optimized_debug);
        
        // Test disabling debug info
        let disabled_config = DebugConfig::none();
        manager.update_config(disabled_config);
        assert!(!manager.is_enabled());
    }

    #[test]
    fn test_debug_validation() {
        let mut manager = DebugInfoManager::new();
        let file = PathBuf::from("test_validation.csd");
        
        // Empty manager should validate successfully
        let result = manager.validate();
        assert!(result.is_ok(), "Empty manager validation failed: {:?}", result);
        
        // Add valid debug information
        manager.initialize_compilation_unit(file.clone(), "Test Producer".to_string()).unwrap();
        let function_location = ErrorSourceLocation::new(file.clone(), 10, 5);
        manager.begin_function("valid_function".to_string(), function_location).unwrap();
        
        // Should still validate successfully
        let result = manager.validate();
        assert!(result.is_ok(), "Valid debug info validation failed: {:?}", result);
    }

    #[test]
    fn test_clear_debug_info() {
        let mut manager = DebugInfoManager::new();
        let file = PathBuf::from("test_clear.csd");
        
        // Add debug information
        manager.initialize_compilation_unit(file.clone(), "Test Producer".to_string()).unwrap();
        let function_location = ErrorSourceLocation::new(file.clone(), 10, 5);
        manager.begin_function("test_function".to_string(), function_location.clone()).unwrap();
        manager.add_variable("test_var".to_string(), "sus".to_string(), function_location).unwrap();
        
        // Verify we have debug info
        assert!(!manager.functions().is_empty());
        let stats_before = manager.statistics();
        assert!(stats_before.symbol_count > 0);
        
        // Clear debug information
        manager.clear();
        
        // Verify everything is cleared
        assert!(manager.functions().is_empty());
        let stats_after = manager.statistics();
        assert_eq!(stats_after.symbol_count, 0);
        assert_eq!(stats_after.debug_info_count, 0);
    }

    #[test]
    fn test_multiple_functions() {
        let mut manager = DebugInfoManager::new();
        let file = PathBuf::from("test_multi_functions.csd");
        
        // Initialize compilation unit
        manager.initialize_compilation_unit(file.clone(), "Test Producer".to_string()).unwrap();
        
        // Add multiple functions
        let function_names = vec!["function_one", "function_two", "function_three"];
        
        for (i, name) in function_names.iter().enumerate() {
            let location = ErrorSourceLocation::new(file.clone(), (i as u32 + 1) * 10, 1);
            manager.begin_function(name.to_string(), location.clone()).unwrap();
            
            // Add a variable to each function
            let var_location = ErrorSourceLocation::new(file.clone(), (i as u32 + 1) * 10 + 2, 5);
            manager.add_variable(
                format!("var_{}", i),
                "sus".to_string(),
                var_location,
            ).unwrap();
            
            manager.end_function().unwrap();
        }
        
        // Verify all functions are tracked
        let functions = manager.functions();
        for name in &function_names {
            assert!(functions.contains(&name.to_string()));
        }
        
        // Verify statistics
        let stats = manager.statistics();
        assert!(stats.symbol_count >= function_names.len() * 2); // Functions + variables
    }

    #[test]
    fn test_nested_scopes() {
        let mut manager = DebugInfoManager::new();
        let file = PathBuf::from("test_scopes.csd");
        
        // Initialize compilation unit
        manager.initialize_compilation_unit(file.clone(), "Test Producer".to_string()).unwrap();
        
        // Begin function (creates function scope)
        let function_location = ErrorSourceLocation::new(file.clone(), 10, 1);
        manager.begin_function("test_scope_function".to_string(), function_location).unwrap();
        
        // Add variables at different scope levels
        let var1_location = ErrorSourceLocation::new(file.clone(), 12, 5);
        manager.add_variable("outer_var".to_string(), "sus".to_string(), var1_location).unwrap();
        
        // Variables in nested scopes would be handled by the scope tracking
        let var2_location = ErrorSourceLocation::new(file.clone(), 15, 9);
        manager.add_variable("inner_var".to_string(), "facts".to_string(), var2_location).unwrap();
        
        // End function (pops function scope)
        manager.end_function().unwrap();
        
        // Verify debug info is properly structured
        let functions = manager.functions();
        assert!(functions.contains(&"test_scope_function".to_string()));
        
        let stats = manager.statistics();
        assert!(stats.symbol_count >= 3); // Function + 2 variables
    }

    #[test]
    fn test_cursed_type_system_integration() {
        let mut manager = DebugInfoManager::new();
        let file = PathBuf::from("test_cursed_types.csd");
        
        // Initialize compilation unit
        manager.initialize_compilation_unit(file.clone(), "CURSED Type Test".to_string()).unwrap();
        let function_location = ErrorSourceLocation::new(file.clone(), 5, 1);
        manager.begin_function("test_cursed_types".to_string(), function_location).unwrap();
        
        // Test all CURSED Gen Z types
        let cursed_types = vec![
            ("suspicious_number", "sus"),     // integer
            ("absolute_truth", "facts"),      // boolean  
            ("good_vibes", "vibes"),          // float
            ("spilled_tea", "tea"),           // string
        ];
        
        for (i, (var_name, type_name)) in cursed_types.iter().enumerate() {
            let var_location = ErrorSourceLocation::new(file.clone(), 10 + i as u32, 5);
            let result = manager.add_variable(
                var_name.to_string(),
                type_name.to_string(),
                var_location,
            );
            assert!(result.is_ok(), "Failed to add CURSED type {}: {:?}", type_name, result);
        }
        
        // Verify all types are properly handled
        let stats = manager.statistics();
        assert!(stats.symbol_count >= cursed_types.len() + 1); // Variables + function
        
        manager.end_function().unwrap();
    }

    #[test]
    fn test_debug_disabled_behavior() {
        let mut manager = DebugInfoManager::new();
        
        // Disable debug info
        let disabled_config = DebugConfig::none();
        manager.update_config(disabled_config);
        assert!(!manager.is_enabled());
        
        let file = PathBuf::from("test_disabled.csd");
        
        // Operations should succeed but not generate debug info
        let result = manager.initialize_compilation_unit(file.clone(), "Test".to_string());
        assert!(result.is_ok());
        
        let function_location = ErrorSourceLocation::new(file.clone(), 10, 1);
        let result = manager.begin_function("test_function".to_string(), function_location.clone());
        assert!(result.is_ok());
        
        let result = manager.add_variable("test_var".to_string(), "sus".to_string(), function_location);
        assert!(result.is_ok());
        
        // But debug metadata should be minimal/empty
        let metadata = manager.generate_llvm_debug_metadata().unwrap();
        assert!(metadata.is_empty() || metadata.trim().is_empty());
        
        let line_table = manager.generate_line_table();
        assert!(line_table.is_empty());
    }

    #[test]
    fn test_error_handling() {
        let mut manager = DebugInfoManager::new();
        
        // Test operations without initialization should handle gracefully
        let file = PathBuf::from("test_error.csd");
        let location = ErrorSourceLocation::new(file, 10, 1);
        
        // These should work even without full initialization due to internal handling
        let result = manager.begin_function("test".to_string(), location.clone());
        assert!(result.is_ok()); // Should handle gracefully
        
        let result = manager.add_variable("var".to_string(), "sus".to_string(), location);
        assert!(result.is_ok()); // Should handle gracefully
    }

    #[test]
    fn test_performance_with_many_symbols() {
        let mut manager = DebugInfoManager::new();
        let file = PathBuf::from("test_performance.csd");
        
        manager.initialize_compilation_unit(file.clone(), "Performance Test".to_string()).unwrap();
        
        // Add many functions and variables to test performance
        let num_functions = 100;
        let vars_per_function = 10;
        
        for func_i in 0..num_functions {
            let function_location = ErrorSourceLocation::new(file.clone(), func_i * 20 + 10, 1);
            let function_name = format!("function_{}", func_i);
            
            manager.begin_function(function_name, function_location).unwrap();
            
            for var_i in 0..vars_per_function {
                let var_location = ErrorSourceLocation::new(file.clone(), func_i * 20 + 12 + var_i, 5);
                let var_name = format!("var_{}_{}", func_i, var_i);
                
                manager.add_variable(var_name, "sus".to_string(), var_location).unwrap();
            }
            
            manager.end_function().unwrap();
        }
        
        // Verify all symbols are tracked
        let stats = manager.statistics();
        assert!(stats.symbol_count >= num_functions * (vars_per_function + 1));
        
        let functions = manager.functions();
        assert_eq!(functions.len(), num_functions as usize);
        
        // Test that metadata generation still works with many symbols
        let metadata = manager.generate_llvm_debug_metadata();
        assert!(metadata.is_ok());
    }
}
