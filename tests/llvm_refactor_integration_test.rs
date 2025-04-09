use std::path::PathBuf;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::types::BasicMetadataTypeEnum;

use cursed::codegen::LlvmCodeGenerator;

#[test]
fn test_simple_llvm_ir_generation() {
    // Set up manually
    let context = Context::create();
    let module_name = "test_module";
    let file_path = PathBuf::from("test.csd");
    
    let mut generator = LlvmCodeGenerator::new(&context, module_name, file_path);
    
    // Create a main function directly with LLVM API
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = generator.module().add_function("main", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    
    // Position the builder at the start of the entry block
    generator.builder().position_at_end(basic_block);
    
    // Create a simple function body: return 42;
    let ret_val = i32_type.const_int(42, false);
    generator.builder().build_return(Some(&ret_val)).unwrap();
    
    // Verify the module
    let module = generator.get_module();
    assert!(module.verify().is_ok(), "Generated LLVM module failed verification");
    
    // Verify the module contains the main function
    let main_function = module.get_function("main");
    assert!(main_function.is_some(), "No main function found in the module");
    
    // Verify the main function has the expected return type
    let main_fn = main_function.unwrap();
    assert_eq!(main_fn.count_params(), 0, "Main function should have no parameters");
    
    // Get function return type
    let return_type = main_fn.get_type().get_return_type().unwrap();
    assert!(return_type.is_int_type(), "Main function should return an int type");
    
    // Check that the int return type is 32-bit (normie)
    let int_type = return_type.into_int_type();
    assert_eq!(int_type.get_bit_width(), 32, "Main function should return a 32-bit int (normie)");
    
    // Test the IR output
    let ir = module.print_to_string().to_string();
    assert!(ir.contains("define i32 @main()"), "IR should contain main function definition");
    assert!(ir.contains("ret i32 42"), "IR should contain return instruction with value 42");
}

#[test]
fn test_create_function() {
    let context = Context::create();
    let module_name = "test_module_2";
    let file_path = PathBuf::from("test.csd");
    
    let mut generator = LlvmCodeGenerator::new(&context, module_name, file_path);
    
    // Create parameter types
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    let params = vec![i32_type.into(), i64_type.into()];
    
    // Create a function that returns i32
    let fn_name = "test_function";
    let function = generator.create_function(
        fn_name,
        &params,
        i32_type.into(),
        false
    );
    
    assert_eq!(function.count_params(), 2, "Function should have 2 parameters");
    
    // Verify the function
    assert!(function.verify(true), "Function should be valid");
    
    // Verify the function exists in the module
    let retrieved = generator.get_module().get_function(fn_name);
    assert!(retrieved.is_some(), "Function should exist in module");
}