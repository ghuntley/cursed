//! Tests for zero value initialization in CURSED
//!
//! This test suite verifies that all types in CURSED have proper zero value
//! initialization following Go semantics.

mod common;

use cursed::core::type_checker::Type;
use cursed::codegen::llvm::{LlvmCodeGenerator, zero_values::ZeroValueGeneration};
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use tracing::{debug, info};

/// Initialize tracing for the test
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .try_init();
    };
}

/// Test zero values for basic types
#[test]
fn test_basic_type_zero_values() {
    init_tracing!();
    info!("Testing basic type zero values");
    
    let context = Context::create();
    let module = context.create_module("test_zero_values");
    let builder = context.create_builder();
    let codegen = LlvmCodeGenerator::new());
    
    // Test boolean (lit) - should be false
    let lit_zero = codegen.create_zero_value(&Type::Lit).unwrap();
    assert!(lit_zero.is_int_value());
    let lit_int = lit_zero.into_int_value();
    assert_eq!(lit_int.get_zero_extended_constant().unwrap(), 0);
    debug!("lit zero value: {:?}", lit_zero);
    
    // Test integers
    let normie_zero = codegen.create_zero_value(&Type::Normie).unwrap();
    assert!(normie_zero.is_int_value());
    let normie_int = normie_zero.into_int_value();
    assert_eq!(normie_int.get_zero_extended_constant().unwrap(), 0);
    debug!("normie zero value: {:?}", normie_zero);
    
    let thicc_zero = codegen.create_zero_value(&Type::Thicc).unwrap();
    assert!(thicc_zero.is_int_value());
    let thicc_int = thicc_zero.into_int_value();
    assert_eq!(thicc_int.get_zero_extended_constant().unwrap(), 0);
    debug!("thicc zero value: {:?}", thicc_zero);
    
    // Test floats
    let snack_zero = codegen.create_zero_value(&Type::Snack).unwrap();
    assert!(snack_zero.is_float_value());
    let snack_float = snack_zero.into_float_value();
    assert_eq!(snack_float.get_constant().unwrap().0, 0.0);
    debug!("snack zero value: {:?}", snack_zero);
    
    let meal_zero = codegen.create_zero_value(&Type::Meal).unwrap();
    assert!(meal_zero.is_float_value());
    let meal_float = meal_zero.into_float_value();
    assert_eq!(meal_float.get_constant().unwrap().0, 0.0);
    debug!("meal zero value: {:?}", meal_zero);
    
    info!("Basic type zero values test passed");
}

/// Test zero values for composite types
#[test]
fn test_composite_type_zero_values() {
    init_tracing!();
    info!("Testing composite type zero values");
    
    let context = Context::create();
    let module = context.create_module("test_composite_zero");
    let builder = context.create_builder();
    let codegen = LlvmCodeGenerator::new());
    
    // Test slice (should be nil - struct with null ptr, 0 len, 0 cap)
    let slice_type = Type::Slice(Box::new(Type::Normie));
    let slice_zero = codegen.create_zero_value(&slice_type).unwrap();
    assert!(slice_zero.is_struct_value());
    debug!("slice zero value: {:?}", slice_zero);
    
    // Test map (should be nil pointer)
    let map_type = Type::Map(Box::new(Type::Tea), Box::new(Type::Normie));
    let map_zero = codegen.create_zero_value(&map_type).unwrap();
    assert!(map_zero.is_pointer_value());
    let map_ptr = map_zero.into_pointer_value();
    assert!(map_ptr.is_null());
    debug!("map zero value: {:?}", map_zero);
    
    // Test pointer (should be nil)
    let pointer_type = Type::Pointer(Box::new(Type::Normie));
    let pointer_zero = codegen.create_zero_value(&pointer_type).unwrap();
    assert!(pointer_zero.is_pointer_value());
    let ptr = pointer_zero.into_pointer_value();
    assert!(ptr.is_null());
    debug!("pointer zero value: {:?}", pointer_zero);
    
    // Test array (should be array of zero values)
    let array_type = Type::Array(Box::new(Type::Normie), 3);
    let array_zero = codegen.create_zero_value(&array_type).unwrap();
    assert!(array_zero.is_array_value());
    debug!("array zero value: {:?}", array_zero);
    
    info!("Composite type zero values test passed");
}

/// Test zero values for string type
#[test]
fn test_string_zero_value() {
    init_tracing!();
    info!("Testing string zero value");
    
    let context = Context::create();
    let module = context.create_module("test_string_zero");
    let builder = context.create_builder();
    let codegen = LlvmCodeGenerator::new());
    
    // Test tea (string) - should be empty string struct
    let tea_zero = codegen.create_zero_value(&Type::Tea).unwrap();
    assert!(tea_zero.is_struct_value());
    debug!("tea zero value: {:?}", tea_zero);
    
    // Verify the string struct has the expected fields (ptr, len, cap)
    let tea_struct = tea_zero.into_struct_value();
    let ptr_field = tea_struct.get_field_at_index(0).unwrap();
    assert!(ptr_field.is_pointer_value());
    let ptr = ptr_field.into_pointer_value();
    assert!(ptr.is_null());
    
    let len_field = tea_struct.get_field_at_index(1).unwrap();
    assert!(len_field.is_int_value());
    let len = len_field.into_int_value();
    assert_eq!(len.get_zero_extended_constant().unwrap(), 0);
    
    let cap_field = tea_struct.get_field_at_index(2).unwrap();
    assert!(cap_field.is_int_value());
    let cap = cap_field.into_int_value();
    assert_eq!(cap.get_zero_extended_constant().unwrap(), 0);
    
    info!("String zero value test passed");
}

/// Test zero values for complex types
#[test]
fn test_complex_zero_value() {
    init_tracing!();
    info!("Testing complex number zero value");
    
    let context = Context::create();
    let module = context.create_module("test_complex_zero");
    let builder = context.create_builder();
    let codegen = LlvmCodeGenerator::new());
    
    // Test extra (complex) - should be 0+0i
    let extra_zero = codegen.create_zero_value(&Type::Extra).unwrap();
    assert!(extra_zero.is_struct_value());
    debug!("extra zero value: {:?}", extra_zero);
    
    // Verify the complex struct has zero real and imaginary parts
    let complex_struct = extra_zero.into_struct_value();
    let real_field = complex_struct.get_field_at_index(0).unwrap();
    assert!(real_field.is_float_value());
    let real = real_field.into_float_value();
    assert_eq!(real.get_constant().unwrap().0, 0.0);
    
    let imag_field = complex_struct.get_field_at_index(1).unwrap();
    assert!(imag_field.is_float_value());
    let imag = imag_field.into_float_value();
    assert_eq!(imag.get_constant().unwrap().0, 0.0);
    
    info!("Complex zero value test passed");
}

/// Test zero values for interface types
#[test]
fn test_interface_zero_value() {
    init_tracing!();
    info!("Testing interface zero value");
    
    let context = Context::create();
    let module = context.create_module("test_interface_zero");
    let builder = context.create_builder();
    let codegen = LlvmCodeGenerator::new());
    
    // Test interface - should be nil interface (null data ptr, null type info)
    let interface_type = Type::Unknown // Was Interface("TestInterface".to_string(), vec![]);
    let interface_zero = codegen.create_zero_value(&interface_type).unwrap();
    assert!(interface_zero.is_struct_value());
    debug!("interface zero value: {:?}", interface_zero);
    
    // Verify the interface struct has null pointers
    let interface_struct = interface_zero.into_struct_value();
    let data_field = interface_struct.get_field_at_index(0).unwrap();
    assert!(data_field.is_pointer_value());
    let data_ptr = data_field.into_pointer_value();
    assert!(data_ptr.is_null());
    
    let type_info_field = interface_struct.get_field_at_index(1).unwrap();
    assert!(type_info_field.is_pointer_value());
    let type_info_ptr = type_info_field.into_pointer_value();
    assert!(type_info_ptr.is_null());
    
    info!("Interface zero value test passed");
}

/// Test zero value descriptions
#[test]
fn test_zero_value_descriptions() {
    init_tracing!();
    info!("Testing zero value descriptions");
    
    // Test basic type descriptions
    assert_eq!(Type::Lit.zero_value_description(), "false");
    assert_eq!(Type::Normie.zero_value_description(), "0");
    assert_eq!(Type::Snack.zero_value_description(), "0.0");
    assert_eq!(Type::Tea.zero_value_description(), "\"\"");
    assert_eq!(Type::Extra.zero_value_description(), "0+0i");
    
    // Test composite type descriptions
    let slice_type = Type::Slice(Box::new(Type::Normie));
    assert_eq!(slice_type.zero_value_description(), "nil");
    
    let map_type = Type::Map(Box::new(Type::Tea), Box::new(Type::Normie));
    assert_eq!(map_type.zero_value_description(), "nil");
    
    let pointer_type = Type::Pointer(Box::new(Type::Normie));
    assert_eq!(pointer_type.zero_value_description(), "nil");
    
    let interface_type = Type::Unknown // Was Interface("TestInterface".to_string(), vec![]);
    assert_eq!(interface_type.zero_value_description(), "nil");
    
    // Test array description
    let array_type = Type::Array(Box::new(Type::Normie), 3);
    assert_eq!(array_type.zero_value_description(), "[3]{0, 0, 0}");
    
    info!("Zero value descriptions test passed");
}

/// Test has_zero_value method
#[test]
fn test_has_zero_value() {
    init_tracing!();
    info!("Testing has_zero_value method");
    
    // Types that have zero values
    assert!(Type::Lit.has_zero_value());
    assert!(Type::Normie.has_zero_value());
    assert!(Type::Tea.has_zero_value());
    assert!(Type::Slice(Box::new(Type::Normie)).has_zero_value());
    assert!(Type::Map(Box::new(Type::Tea), Box::new(Type::Normie)).has_zero_value());
    assert!(Type::Pointer(Box::new(Type::Normie)).has_zero_value());
    assert!(Type::Unknown // Was Interface("TestInterface".to_string(), vec![]).has_zero_value());
    assert!(Type::Struct("TestStruct".to_string(), vec![]).has_zero_value());
    
    // Types that don't have zero values
    assert!(!Type::Unknown.has_zero_value());
    
    info!("has_zero_value test passed");
}

/// Test zero value memory operations
#[test]
fn test_zero_value_memory_operations() {
    init_tracing!();
    info!("Testing zero value memory operations");
    
    let context = Context::create();
    let module = context.create_module("test_memory_zero");
    let builder = context.create_builder();
    let codegen = LlvmCodeGenerator::new());
    
    // Create a function to test memory operations
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_zero_memory", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Test zero allocation for arrays
    let array_ptr = codegen.zero_allocate_array(&Type::Normie, 5).unwrap();
    assert!(array_ptr.is_pointer_value());
    debug!("zero allocated array: {:?}", array_ptr);
    
    // Test zero initialization of memory
    let int_type = context.i32_type();
    let ptr = builder.build_alloca(int_type, "test_var").unwrap();
    codegen.zero_initialize_memory(ptr.into(), &Type::Normie).unwrap();
    debug!("zero initialized memory: {:?}", ptr);
    
    info!("Zero value memory operations test passed");
}

/// Test LLVM type zero values
#[test]
fn test_llvm_type_zero_values() {
    init_tracing!();
    info!("Testing LLVM type zero values");
    
    let context = Context::create();
    let module = context.create_module("test_llvm_zero");
    let builder = context.create_builder();
    let codegen = LlvmCodeGenerator::new());
    
    // Test basic LLVM types
    let i32_zero = codegen.create_zero_value_for_llvm_type(context.i32_type().into());
    assert!(i32_zero.is_int_value());
    debug!("i32 zero: {:?}", i32_zero);
    
    let f64_zero = codegen.create_zero_value_for_llvm_type(context.f64_type().into());
    assert!(f64_zero.is_float_value());
    debug!("f64 zero: {:?}", f64_zero);
    
    let ptr_zero = codegen.create_zero_value_for_llvm_type(
        context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()
    );
    assert!(ptr_zero.is_pointer_value());
    assert!(ptr_zero.into_pointer_value().is_null());
    debug!("pointer zero: {:?}", ptr_zero);
    
    // Test struct type
    let struct_type = context.struct_type(&[
        context.i32_type().into(),
        context.f32_type().into(),
    ], false);
    let struct_zero = codegen.create_zero_value_for_llvm_type(struct_type.into());
    assert!(struct_zero.is_struct_value());
    debug!("struct zero: {:?}", struct_zero);
    
    // Test array type
    let array_type = context.i32_type().array_type(3);
    let array_zero = codegen.create_zero_value_for_llvm_type(array_type.into());
    assert!(array_zero.is_array_value());
    debug!("array zero: {:?}", array_zero);
    
    info!("LLVM type zero values test passed");
}

/// Test error cases
#[test]
fn test_zero_value_errors() {
    init_tracing!();
    info!("Testing zero value error cases");
    
    let context = Context::create();
    let module = context.create_module("test_errors");
    let builder = context.create_builder();
    let codegen = LlvmCodeGenerator::new());
    
    // Test unknown type
    let unknown_result = codegen.create_zero_value(&Type::Unknown);
    assert!(unknown_result.is_err());
    debug!("Unknown type error: {:?}", unknown_result.err());
    
    info!("Zero value error cases test passed");
}

/// Integration test for zero values in variable declarations
#[test]
fn test_zero_value_variable_integration() {
    init_tracing!();
    info!("Testing zero value integration with variables");
    
    let context = Context::create();
    let module = context.create_module("test_integration");
    let builder = context.create_builder();
    let codegen = LlvmCodeGenerator::new());
    
    // Create a function for testing
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_variables", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Test variable declaration with zero initialization
    let var_type = context.i32_type();
    let var_ptr = builder.build_alloca(var_type, "test_var").unwrap();
    let zero_value = codegen.create_zero_value_for_llvm_type(var_type.into());
    builder.build_store(var_ptr, zero_value).unwrap();
    
    // Load and verify the value is zero
    let loaded_value = builder.build_load(var_type, var_ptr, "loaded").unwrap();
    assert!(loaded_value.is_int_value());
    debug!("loaded zero value: {:?}", loaded_value);
    
    info!("Zero value variable integration test passed");
}
