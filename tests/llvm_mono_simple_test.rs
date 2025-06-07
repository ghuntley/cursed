use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::function_monomorphization::FunctionMonomorphization;
use inkwell::context::Context;
use std::path::PathBuf;

// Simple test for function monomorphization


#[test]
fn test_monomorphization_type_to_llvm_type() {
    // Create a context and code generator
    let context = Context::create();
    let generator = LlvmCodeGenerator::new(&context, "test_types", PathBuf::from("test.csd"));

    // Test converting different type names to LLVM types
    let normie_type = generator.monomorphization_type_to_llvm_type("Normie")
        .expect("Failed to convert Normie type");
    assert!(normie_type.is_int_type(), "Normie should be an integer type");

    let thicc_type = generator.monomorphization_type_to_llvm_type("Thicc")
        .expect("Failed to convert Thicc type");
    assert!(thicc_type.is_int_type(), "Thicc should be an integer type");

    let snack_type = generator.monomorphization_type_to_llvm_type("Snack")
        .expect("Failed to convert Snack type");
    assert!(snack_type.is_float_type(), "Snack should be a float type");

    let tea_type = generator.monomorphization_type_to_llvm_type("Tea")
        .expect("Failed to convert Tea type");
    assert!(tea_type.is_pointer_type(), "Tea should be a pointer type");
}