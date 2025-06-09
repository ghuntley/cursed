/// Integration tests for debug information generation
use cursed::debug::{
    DebugInfoManager, DebugConfig, SourceLocation, DebugSymbol, DebugSymbolType,
    DebugUtils, DwarfGenerator
};
use cursed::codegen::llvm::{LlvmCodeGenerator, LlvmDebugCodeGenerator};
use cursed::error::Error;
use std::path::PathBuf;

#[test]
fn test_debug_info_manager_integration() {
    let config = DebugConfig::default();
    let mut manager = DebugInfoManager::new(config);
    
    // Initialize compilation unit
    let file = PathBuf::from("test.csd");
    let result = manager.initialize_compilation_unit(file.clone(), "Test Compiler".to_string());
    assert!(result.is_ok());
    
    // Add a function
    let func_location = SourceLocation::new(file.clone(), 10, 1);
    let result = manager.begin_function("main".to_string(), func_location);
    assert!(result.is_ok());
    
    // Add variables
    let var_location = SourceLocation::new(file.clone(), 12, 5);
    let result = manager.add_variable("x".to_string(), "int".to_string(), var_location);
    assert!(result.is_ok());
    
    let param_location = SourceLocation::new(file, 10, 15);
    let result = manager.add_parameter("argc".to_string(), "int".to_string(), param_location);
    assert!(result.is_ok());
    
    // Generate debug metadata
    let metadata = manager.generate_llvm_debug_metadata();
    assert!(metadata.is_ok());
    
    let metadata_str = metadata.unwrap();
    assert!(metadata_str.contains("!DICompileUnit"));
    
    // End function
    let result = manager.end_function();
    assert!(result.is_ok());
    
    // Check statistics
    let stats = manager.statistics();
    assert!(stats.enabled);
    assert!(stats.symbol_count > 0);
    assert!(stats.dwarf_compile_units > 0);
}

#[test]
fn test_source_location_tracking() {
    let mut manager = DebugInfoManager::new(DebugConfig::default());
    let file = PathBuf::from("test.csd");
    
    // Test location stack
    let loc1 = SourceLocation::new(file.clone(), 10, 1);
    let loc2 = SourceLocation::new(file, 20, 5);
    
    manager.enter_scope(loc1.clone());
    assert_eq!(manager.current_location(), Some(&loc1));
    
    manager.enter_scope(loc2.clone());
    assert_eq!(manager.current_location(), Some(&loc2));
    
    manager.exit_scope();
    assert_eq!(manager.current_location(), Some(&loc1));
    
    manager.exit_scope();
    assert!(manager.current_location().is_none());
}

#[test]
fn test_debug_symbol_management() {
    let mut manager = DebugInfoManager::new(DebugConfig::default());
    let file = PathBuf::from("test.csd");
    
    // Add various types of symbols
    let func_loc = SourceLocation::new(file.clone(), 5, 1);
    manager.begin_function("test_func".to_string(), func_loc).unwrap();
    
    let var_loc = SourceLocation::new(file.clone(), 7, 5);
    manager.add_variable("local_var".to_string(), "string".to_string(), var_loc).unwrap();
    
    let type_loc = SourceLocation::new(file, 3, 1);
    manager.add_type("CustomType".to_string(), type_loc).unwrap();
    
    // Test symbol lookup
    let symbol = manager.lookup_symbol("local_var");
    assert!(symbol.is_some());
    assert_eq!(symbol.unwrap().name, "local_var");
    assert_eq!(symbol.unwrap().type_name, "string");
    
    // Test functions listing
    let functions = manager.functions();
    assert_eq!(functions.len(), 1);
    assert_eq!(functions[0].name, "test_func");
    
    // Test variables in current scope
    let variables = manager.current_scope_variables();
    assert_eq!(variables.len(), 1);
    assert_eq!(variables[0].name, "local_var");
    
    manager.end_function().unwrap();
}

#[test]
fn test_dwarf_generation() {
    let mut generator = DwarfGenerator::new();
    let file = PathBuf::from("test.csd");
    
    // Set up compile unit
    generator.set_compile_unit(file.clone(), "CURSED Compiler".to_string());
    
    // Create debug symbols
    let func_location = SourceLocation::new(file.clone(), 10, 1);
    let var_location = SourceLocation::new(file, 15, 5);
    
    let symbols = vec![
        DebugSymbol::function("main".to_string(), func_location),
        DebugSymbol::variable("x".to_string(), "int".to_string(), var_location),
    ];
    
    generator.add_symbols(symbols);
    
    // Generate LLVM metadata
    let metadata = generator.generate_llvm_metadata();
    
    assert!(metadata.contains("!DICompileUnit"));
    assert!(metadata.contains("!DISubprogram"));
    assert!(metadata.contains("!DIFile"));
    assert!(metadata.contains("CURSED Compiler"));
    
    // Check statistics
    let stats = generator.statistics();
    assert_eq!(stats.compile_units, 1);
    assert_eq!(stats.subprograms, 1);
    assert_eq!(stats.variables, 1);
}

#[test]
fn test_llvm_debug_code_generator() {
    let config = DebugConfig::default();
    let mut generator = LlvmDebugCodeGenerator::new(config);
    let file = PathBuf::from("test.csd");
    
    // Initialize debug info
    let result = generator.initialize_debug_info(file.clone(), "Test Compiler".to_string());
    assert!(result.is_ok());
    
    // Generate function with debug info
    let func_location = SourceLocation::new(file.clone(), 10, 1);
    let func_ir = generator.begin_function_with_debug("test_func".to_string(), func_location);
    assert!(func_ir.is_ok());
    
    let ir = func_ir.unwrap();
    assert!(ir.contains("define i32 @test_func"));
    assert!(ir.contains("!dbg"));
    
    // Generate variable with debug info
    let var_location = SourceLocation::new(file, 12, 5);
    let var_ir = generator.generate_variable_with_debug(
        "x".to_string(),
        "int".to_string(),
        var_location,
    );
    assert!(var_ir.is_ok());
    
    let var_ir_str = var_ir.unwrap();
    assert!(var_ir_str.contains("alloca i32"));
    assert!(var_ir_str.contains("llvm.dbg.declare"));
    
    // End function
    let end_ir = generator.end_function_with_debug();
    assert!(end_ir.is_ok());
    
    // Generate complete module
    let metadata = generator.generate_debug_metadata();
    assert!(metadata.is_ok());
    
    let metadata_str = metadata.unwrap();
    assert!(metadata_str.contains("llvm.dbg.declare"));
    assert!(metadata_str.contains("Debug Info Version"));
}

#[test]
fn test_enhanced_llvm_code_generator() {
    let debug_config = DebugConfig::default();
    let mut generator = LlvmCodeGenerator::new_with_debug(debug_config).unwrap();
    
    assert!(generator.debug_enabled());
    
    // Generate basic IR
    let basic_ir = generator.generate_ir("").unwrap();
    assert!(basic_ir.contains("Generated by CURSED Compiler"));
    assert!(basic_ir.contains("cursed_debug_print_int"));
    
    // Generate IR with full debug info
    let file = PathBuf::from("test.csd");
    let debug_ir = generator.generate_ir_with_debug(file, "").unwrap();
    
    assert!(debug_ir.contains("ModuleID"));
    assert!(debug_ir.contains("define i32 @main"));
    assert!(debug_ir.contains("!DICompileUnit"));
    
    // Test debug statistics
    let stats = generator.debug_statistics();
    assert!(!stats.is_empty());
    
    // Test validation
    let validation = generator.validate_debug();
    assert!(validation.is_ok());
}

#[test]
fn test_debug_utilities() {
    let config = DebugConfig::default();
    let debug_manager = DebugInfoManager::new(config);
    
    // Test stack trace formatting
    let addresses = vec![0x1000, 0x2000, 0x3000];
    let trace = DebugUtils::format_stack_trace(&debug_manager, &addresses);
    
    assert_eq!(trace.len(), 3);
    assert!(trace[0].contains("#0"));
    assert!(trace[0].contains("0x0000000000001000"));
    
    // Test location creation
    let location = DebugUtils::create_location("test.csd", 42, 10);
    assert_eq!(location.line, 42);
    assert_eq!(location.column, 10);
    assert_eq!(location.file_name(), "test.csd");
    
    // Test GDB command generation
    let executable = PathBuf::from("/tmp/test");
    let commands = DebugUtils::generate_gdb_commands(&executable, &debug_manager);
    
    assert!(!commands.is_empty());
    assert!(commands.iter().any(|cmd| cmd.contains("file /tmp/test")));
    assert!(commands.iter().any(|cmd| cmd.contains("set print pretty")));
    
    // Test VS Code configuration
    let source_root = PathBuf::from("/tmp/src");
    let vscode_config = DebugUtils::generate_vscode_launch_config(&executable, &source_root);
    
    assert!(vscode_config.is_object());
    assert!(vscode_config["configurations"].is_array());
}

#[test]
fn test_debug_configuration() {
    // Test default configuration
    let default_config = DebugConfig::default();
    assert!(default_config.generate_debug_info);
    assert_eq!(default_config.debug_level, 2);
    assert!(default_config.has_debug_info());
    
    // Test preset configurations
    let none_config = DebugConfig::none();
    assert!(!none_config.generate_debug_info);
    assert!(!none_config.has_debug_info());
    
    let minimal_config = DebugConfig::minimal();
    assert!(minimal_config.generate_debug_info);
    assert_eq!(minimal_config.debug_level, 1);
    assert!(minimal_config.optimized_debug);
    
    let full_config = DebugConfig::full();
    assert!(full_config.generate_debug_info);
    assert_eq!(full_config.debug_level, 3);
    assert!(full_config.include_source);
    
    // Test configuration validation
    let mut invalid_config = DebugConfig::default();
    invalid_config.debug_level = 10;
    assert!(invalid_config.validate().is_err());
    
    invalid_config.debug_level = 2;
    invalid_config.dwarf_version = 1;
    assert!(invalid_config.validate().is_err());
}

#[test]
fn test_debug_with_disabled_config() {
    let disabled_config = DebugConfig::none();
    let mut generator = LlvmCodeGenerator::new_with_debug(disabled_config).unwrap();
    
    assert!(!generator.debug_enabled());
    
    // Operations should work but not generate debug info
    let ir = generator.generate_ir("").unwrap();
    assert!(ir.contains("define i32 @main"));
    assert!(!ir.contains("cursed_debug_print_int")); // Debug utilities not included
    
    let stats = generator.debug_statistics();
    assert!(stats.contains("disabled"));
}

#[test]
fn test_source_location_operations() {
    let file = PathBuf::from("test.csd");
    
    // Test basic location
    let loc = SourceLocation::new(file.clone(), 10, 5);
    assert_eq!(loc.line, 10);
    assert_eq!(loc.column, 5);
    assert!(loc.is_valid());
    assert_eq!(loc.display(), "test.csd:10:5");
    
    // Test range location
    let range_loc = SourceLocation::new_range(file, 10, 5, 15, 20);
    assert_eq!(range_loc.end_line, Some(15));
    assert_eq!(range_loc.end_column, Some(20));
    assert_eq!(range_loc.display(), "test.csd:10:5-15:20");
    
    // Test invalid location
    let invalid_loc = SourceLocation::new(PathBuf::from("test.csd"), 0, 5);
    assert!(!invalid_loc.is_valid());
}

#[test]
fn test_debug_report_generation() {
    let mut manager = DebugInfoManager::new(DebugConfig::default());
    let file = PathBuf::from("test.csd");
    
    // Set up some debug information
    manager.initialize_compilation_unit(file.clone(), "Test".to_string()).unwrap();
    
    let func_location = SourceLocation::new(file.clone(), 10, 1);
    manager.begin_function("test_func".to_string(), func_location).unwrap();
    
    let var_location = SourceLocation::new(file, 12, 5);
    manager.add_variable("x".to_string(), "int".to_string(), var_location).unwrap();
    
    // Generate report
    let report = manager.generate_debug_report();
    
    assert!(report.statistics.enabled);
    assert!(!report.functions.is_empty());
    assert_eq!(report.functions[0], "test_func");
    assert_eq!(report.current_function, Some("test_func".to_string()));
    
    // Test report display
    let report_str = format!("{}", report);
    assert!(report_str.contains("Debug Information Report"));
    assert!(report_str.contains("test_func"));
    
    manager.end_function().unwrap();
}

#[test]
fn test_debug_validation() {
    let mut manager = DebugInfoManager::new(DebugConfig::default());
    let file = PathBuf::from("test.csd");
    
    // Valid state
    manager.initialize_compilation_unit(file.clone(), "Test".to_string()).unwrap();
    manager.begin_function("main".to_string(), SourceLocation::new(file, 1, 1)).unwrap();
    
    let validation = manager.validate();
    assert!(validation.is_ok());
    
    // Invalid state - symbols without compile unit
    manager.clear();
    manager.add_variable("x".to_string(), "int".to_string(), SourceLocation::default()).unwrap();
    
    let validation = manager.validate();
    assert!(validation.is_err());
    
    let errors = validation.unwrap_err();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.contains("no compile unit")));
}
