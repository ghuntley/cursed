use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::EnhancedErrorVisualization;
use cursed::codegen::llvm::DiamondInheritanceDetection;
use cursed::codegen::llvm::interface_type_assertion_diamond_inheritance::DiamondInheritancePattern;
use cursed::error::SourceLocation;
use common::tracing::init_test_tracing;

#[path = "common/mod.rs"]
mod common;


#[test]
fn test_enhanced_visualization() {
    init_test_tracing();
    
    // Create a simple test context
    let context = inkwell::context::Context::create();
    let mut code_generator = LlvmCodeGenerator::new();
    
    // Create a mock diamond inheritance pattern
    let diamond = DiamondInheritancePattern {
        root_type_id: 101,
        base_type_id: 102,
        left_intermediate_id: 103,
        right_intermediate_id: 104,
    };
    
    // Test diamond inheritance visualization
    let visualization = code_generator.visualize_diamond_inheritance(&diamond);
    assert!(visualization.contains("Diamond Inheritance Pattern");
    assert!(visualization.contains("Type#101");
    assert!(visualization.contains("Type#102"));
    
    // Test enhanced error visualization
    let source_location = SourceLocation {
        line: 10,
        column: 15,
        file: Some("test.csd".to_string()),
        source_line: "    let x = y.(SomeType)?".to_string(),
    };
    
    let context_lines = vec![
        (9, "    // This is a test".to_string()),
        (10, "    let x = y.(SomeType)?".to_string()),
        (11, "    vibez.spill(x)".to_string()),
    ];
    
    let error = code_generator.create_enhanced_visual_error(
        "Cannot convert interface to SomeType",
        &source_location,
        "SomeType",
        Some("ActualType"),
        context_lines,
        102, // expected type ID
        101  // actual type ID
    );
    
    let formatted = code_generator.format_visual_error_message(&error);
    assert!(formatted.contains("Error"));
    assert!(formatted.contains("SomeType"));
    assert!(formatted.contains("ActualType"));
    assert!(formatted.contains("test.csd"));
}