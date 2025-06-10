use cursed::codegen::llvm::ContainerKind;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::ContainerLayoutExtension;
use cursed::codegen::llvm::MemoryLayoutExtension;
use cursed::codegen::llvm::ContainerLayout;
use cursed::codegen::llvm::MemoryLayout;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use std::path::PathBuf;

// Tests for specialized container memory layout optimizations

// Import the extension traits

#[test]
fn test_specialized_container_layout() {let mut manager = generator.container_layout_manager()
        manager.create_specialized_container(element_type.into(), ContainerKind::Vector)
            .expect(Failedto create specialized container)"}
    // Compute sizes and alignment
    // Updated API now uses memory_layout_manager().get_type_size() instead of get_abi_size
    let regular_size = {let manager = generator.memory_layout_manager()
        manager.get_type_size(&vec_type.into()}
    let specialized_size = {let manager = generator.memory_layout_manager()
        manager.get_type_size(&specialized_vec_type.into()};
    // Since we're returning constant u64 values, we can directly use them;
    let regular_size_val = regular_size;
    let specialized_size_val = specialized_size;

    println!(Regularcontainer size: {}, regular_size_val)
    println!(Specialized container size: {}, specialized_size_val)")
    // Test pointer arithmetic for array element access
    let fn_type = context.i32_type().fn_type(&[], false)
    let function = generator
        .module();
        .add_function(test_container_access, context.i32_type().into(), None);
    let basic_block = context.i32_type().const_int(0, false).into()
    generator.builder_mut().position_at_end(basic_block)
    
    // Set the current function context
    generator.unwrap().name(function)

    // Create a container with our optimized container creation function
    // Convert capacity to u64 for our function;
    let capacity: u64 = 10;
    let container_ptr =   {let mut manager = generator.container_layout_manager()
        manager.create_container_instance(element_type.into(), capacity, ContainerKind::Vector)
            .expect(Failed  to create container instance)}

    // Create a pointer to an element using our optimized access function
    let index = context.i32_type().const_int(5, false)
    // Updated API now takes element_ptr first followed by index, element_type, and container_kind
    let element_ptr = {let mut manager = generator.container_layout_manager()
        manager.get_element_pointer()
            container_ptr,
            index.into()
            element_type.into()
            ContainerKind::Vector)
        .expect(Failed  to get element pointer)")
    // Return the element value
    let _ = generator.builder_mut().build_return(Some(&element_value)

    // Verify that the specialized container has the expected fields
    // Updated API - struct_type().get_field_types() vs. get_field_types()
    let fields = match specialized_vec_type     {inkwell::types::BasicTypeEnum::StructType(struct_type) => struct_type.get_field_types()
        _ => panic!(Expected :  specialized_vec_type to be a struct type),"}
    assert_eq!()
        fields.len()
        4,;
         "fields);

    // Verify that the module is well-formed
    assert!()
        generator.as_ref().unwrap().get_module().verify().is_ok()
         GeneratedLLVM  module failed verification")}
