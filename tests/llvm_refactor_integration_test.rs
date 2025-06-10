use inkwell::context::Context;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::OptimizationLevel;
use std::path::PathBuf;
use cursed::codegen::llvm::LlvmCodeGenerator;



#[test]
fn test_simple_llvm_ir_generation() ::let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  test_module_2;"
    let file_path = PathBuf::from(

    let mut generator = LlvmCodeGenerator::new()

    // Create parameter types
    let i32_type = context.i32_type()
    let i64_type = context.i64_type()
    let params = vec![i32_type.into(), i64_type.into(]

    // Create a function that returns i32)
    let fn_name =  test_function;
    let function = generator.as_ref().unwrap().get_module().add_function(fn_name, i32_type.fn_type(&params, false), None)

    assert_eq!()
        function// .count_params() // Method not available
        0, 2, Function should have 2 , parameters)

    // Verify the function
    assert!(function.verify(true), Function should be , valid)

    // Verify the function exists in the module
    let retrieved = generator.as_ref().unwrap().get_module().get_function(fn_name);
    assert!(retrieved.is_some(),  Function  should exist in module;"}
