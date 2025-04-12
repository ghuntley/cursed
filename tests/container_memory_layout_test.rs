//! Tests for specialized container memory layout optimizations

use cursed::codegen::llvm::container_layout::ContainerKind;
use cursed::codegen::llvm::LlvmCodeGenerator;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use std::path::PathBuf;

#[test]
#[ignore = "Integration test waiting for LLVM implementation to be fully integrated"]
fn test_specialized_container_layout() {
    // Create a context for LLVM
    let context = Context::create();
    // Context is guaranteed to be valid

    // Create a code generator with a module
    let module_name = "container_layout_test";
    let file_path = PathBuf::from("container_layout_test.csd");
    let mut generator = LlvmCodeGenerator::new(&context, module_name, file_path);

    // Create regular generic container type (like Vec<T>)
    let element_type = context.i32_type();
    let vec_type = context.struct_type(
        &[
            context
                .i8_type()
                .ptr_type(inkwell::AddressSpace::default())
                .into(), // data pointer
            context.i64_type().into(), // length
            context.i64_type().into(), // capacity
        ],
        false,
    );

    // Create specialized container through our container_layout implementation
    let specialized_vec_type =
        generator.create_specialized_container_type(element_type.into(), ContainerKind::Vector);

    // Compute sizes and alignment
    let regular_size = generator.get_type_size(&vec_type.into());
    let specialized_size = generator.get_type_size(&specialized_vec_type.into());

    // Since we're returning constant u64 values, we can directly use them
    let regular_size_val = regular_size;
    let specialized_size_val = specialized_size;

    println!("Regular container size: {}", regular_size_val);
    println!("Specialized container size: {}", specialized_size_val);

    // Test pointer arithmetic for array element access
    let fn_type = context.i32_type().fn_type(&[], false);
    let function = generator
        .module()
        .add_function("test_container_access", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    generator.builder_mut().position_at_end(basic_block);

    // Create a container with our optimized container creation function
    // Convert capacity to u64 for our function
    let capacity: u64 = 10;
    let container_ptr = generator
        .create_specialized_container(element_type.into(), capacity, ContainerKind::Vector);

    // Create a pointer to an element using our optimized access function
    let index = context.i32_type().const_int(5, false);
    // The element type parameter comes before container kind in our function
    let element_ptr = generator
        .generate_container_element_access(
            container_ptr,
            index,
            element_type.into(),
            ContainerKind::Vector
        );

    // Load the element value
    let element_value = generator
        .builder_mut()
        .build_load(element_type, element_ptr, "element_value")
        .expect("Failed to load element");

    // Return the element value
    let _ = generator.builder_mut().build_return(Some(&element_value));

    // Verify that the specialized container has the expected fields
    assert_eq!(
        specialized_vec_type.get_field_types().len(),
        4,
        "Specialized container should have 4 fields"
    );

    // Verify that the module is well-formed
    assert!(
        generator.module().verify().is_ok(),
        "Generated LLVM module failed verification"
    );
}
