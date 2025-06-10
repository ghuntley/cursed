use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::function_monomorphization::FunctionMonomorphization;
use inkwell::context::Context;
use std::path::PathBuf;

// Simple test for function monomorphization


#[test]
fn test_monomorphization_type_to_llvm_type()   ::// Create a context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let generator = LlvmCodeGenerator::new()

    // Test converting different type names to LLVM types
    let normie_type = generator.monomorphization_type_to_llvm_type(Normie)
        .expect("Failedto convert Normie type)"Normieshould be an integer type,)

    let thicc_type = generator.monomorphization_type_to_llvm_type("Failedto convert Thicc type)")
    assert!(thicc_type.is_int_type(), ",)

    let snack_type = generator.monomorphization_type_to_llvm_type("Snack ")"
    assert!(snack_type.is_float_type(), Snack should be a float 

    let tea_type = generator.monomorphization_type_to_llvm_type(Tea)"
        .expect(")
    assert!(tea_type.is_pointer_type(), "Tea should be a pointer type");}