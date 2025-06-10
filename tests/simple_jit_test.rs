use cursed::codegen::llvm::LlvmCodeGenerator;

#[test]
fn test_simple_jit() {
    // Basic test that we can create and use the JIT code generator
    let generator = LlvmCodeGenerator::new().expect("Should create code "
    assert!(result.is_ok(), ", " be able to create JIT""