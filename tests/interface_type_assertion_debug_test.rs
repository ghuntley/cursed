use std::sync::Arc;

use cursed::codegen::llvm::interface_type_assertion_debug::{InterfaceTypeAssertionDebug, TypeAssertionDebugConfig};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::SourceLocation;

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

#[test]
fn test_type_assertion_debug_configuration() {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    // Create a code generator with debug context
    let context = inkwell::context::Context::create();
    let mut code_generator = LlvmCodeGenerator::new(&context, "test_module", std::path::PathBuf::from("test.csd"));
    
    // Configure debug settings
    let config = TypeAssertionDebugConfig {
        enable_debug: true,
        print_hierarchies: true,
        collect_metrics: true,
        break_on_failure: false,
        max_hierarchy_depth: 3,
    };
    
    // Set the configuration
    code_generator.set_type_assertion_debug_config(config);
    
    // Get the configuration back and verify
    let retrieved_config = code_generator.get_type_assertion_debug_config();
    assert!(retrieved_config.enable_debug);
    assert!(retrieved_config.print_hierarchies);
    assert!(retrieved_config.collect_metrics);
    assert!(!retrieved_config.break_on_failure);
    assert_eq!(retrieved_config.max_hierarchy_depth, 3);
}

#[test]
fn test_register_runtime_type() {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    // Create a code generator with debug context
    let context = inkwell::context::Context::create();
    let mut code_generator = LlvmCodeGenerator::new(&context, "test_module", std::path::PathBuf::from("test.csd"));
    
    // Enable debugging
    let config = TypeAssertionDebugConfig {
        enable_debug: true,
        print_hierarchies: false,
        collect_metrics: true,
        break_on_failure: false,
        max_hierarchy_depth: 2,
    };
    code_generator.set_type_assertion_debug_config(config);
    
    // Register some runtime types
    let type_id_1 = 0x1234567890ABCDEF;
    let type_id_2 = 0xFEDCBA0987654321;
    
    code_generator.register_runtime_type(type_id_1, "Person");
    code_generator.register_runtime_type(type_id_2, "Animal");
    
    // Reset statistics to ensure they're working
    code_generator.reset_type_assertion_statistics();
    
    // Log an assertion
    let source_location = SourceLocation {
        line: 42,
        column: 10,
        file: Some("test.csd".to_string()),
        source_line: "person, ok = animal.(Person)".to_string(),
    };
    
    // Create a dummy LLVM value for testing
    let dummy_value = code_generator.context.i32_type().const_int(0, false).into();
    
    // Debug a type assertion  
    let _ = code_generator.debug_type_assertion(
        dummy_value,
        "Person",
        Some(source_location)
    );
    
    // Print the statistics
    code_generator.print_type_assertion_statistics();
}