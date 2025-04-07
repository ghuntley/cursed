//! LLVM 17 Migration Tests
//! Verifies compatibility with LLVM 17 APIs

use inkwell::context::Context;
use inkwell::OptimizationLevel;

#[test]
fn test_llvm17_build_load() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create a simple function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    
    builder.position_at_end(basic_block);
    
    // Test build_load with LLVM 17 signature (requires type parameter)
    let alloca = builder.build_alloca(i32_type, "test_var").unwrap();
    builder.build_store(alloca, i32_type.const_int(42, false)).unwrap();
    let loaded = builder.build_load(i32_type, alloca, "loaded_var").unwrap();
    
    // Use the loaded value
    builder.build_return(Some(&loaded)).unwrap();
    
    // Verify the function
    assert!(function.verify(true));
}

#[test]
fn test_llvm17_build_struct_gep() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create a struct type
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    let struct_type = context.struct_type(&[i32_type.into(), i64_type.into()], false);
    
    // Create a simple function
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_struct_gep", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    
    builder.position_at_end(basic_block);
    
    // Test build_struct_gep with LLVM 17 signature
    let alloca = builder.build_alloca(struct_type, "test_struct").unwrap();
    let field_ptr = builder.build_struct_gep(struct_type, alloca, 0, "field_ptr").unwrap();
    builder.build_store(field_ptr, i32_type.const_int(42, false)).unwrap();
    
    // Load the field and return it
    let loaded = builder.build_load(i32_type, field_ptr, "loaded_field").unwrap();
    builder.build_return(Some(&loaded)).unwrap();
    
    // Verify the function
    assert!(function.verify(true));
}

#[test]
fn test_llvm17_build_call() {
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create a function to call
    let i32_type = context.i32_type();
    let callee_type = i32_type.fn_type(&[i32_type.into()], false);
    let callee = module.add_function("callee", callee_type, None);
    
    // Implement the callee function
    let callee_block = context.append_basic_block(callee, "entry");
    builder.position_at_end(callee_block);
    let param = callee.get_nth_param(0).unwrap().into_int_value();
    let result = builder.build_int_add(param, i32_type.const_int(1, false), "add_one").unwrap();
    builder.build_return(Some(&result)).unwrap();
    
    // Create a caller function
    let caller_type = i32_type.fn_type(&[], false);
    let caller = module.add_function("caller", caller_type, None);
    let caller_block = context.append_basic_block(caller, "entry");
    
    builder.position_at_end(caller_block);
    
    // Test build_call with LLVM 17 signature
    let args = &[i32_type.const_int(41, false).into()];
    let call = builder.build_call(callee, args, "call_result").unwrap();
    let call_result = call.try_as_basic_value().left().unwrap();
    
    builder.build_return(Some(&call_result)).unwrap();
    
    // Verify the functions
    assert!(callee.verify(true));
    assert!(caller.verify(true));
}