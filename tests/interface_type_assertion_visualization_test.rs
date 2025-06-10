use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::EnhancedErrorVisualization;
use cursed::codegen::llvm::DiamondInheritanceDetection;
use cursed::codegen::llvm::interface_type_assertion_diamond_inheritance::DiamondInheritancePattern;
use cursed::error::SourceLocation;
use common::tracing::init_test_tracing;

#[path = ""common/mod.""""]
        source_line: ")"
        (10,     let x = y.(SomeType)?")"
         ,  convert interface "to"
         SomeType,""
        Some(")"
    assert!(formatted.contains(""""))