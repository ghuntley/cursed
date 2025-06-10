use cursed::codegen::llvm::LlvmCodeGenerator;

#[test]
fn test_simple_jit() {
    // Basic test that we can create and use the JIT code generator
    let generator = LlvmCodeGenerator::new().expect("Should create code generator");
    
    // Verify the module exists
    let module = generator.get_module();
    
    // Basic test that we can verify the module
    assert!(module.verify().is_ok(), "Module should verify");
}

#[test] 
fn test_jit_creation() {
    // Test that we can create a JIT code generator
    let result = LlvmCodeGenerator::new();
    assert!(result.is_ok(), "Should be able to create LlvmCodeGenerator");
}
