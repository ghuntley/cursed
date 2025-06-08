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
    // Updated API signature now returns a Result<BasicTypeEnum, Error>
    let specialized_vec_type = {
        let mut manager = generator.container_layout_manager();
        manager.create_specialized_container(element_type.into(), ContainerKind::Vector)
            .expect("Failed to create specialized container")
    };

    // Compute sizes and alignment
    // Updated API now uses memory_layout_manager().get_type_size() instead of get_abi_size
    let regular_size = {
        let manager = generator.memory_layout_manager();
        manager.get_type_size(&vec_type.into())
    };
    let specialized_size = {
        let manager = generator.memory_layout_manager();
        manager.get_type_size(&specialized_vec_type.into())
    };

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
    
    // Set the current function context
    generator.set_current_function(function);

    // Create a container with our optimized container creation function
    // Convert capacity to u64 for our function
    let capacity: u64 = 10;
    let container_ptr = {
        let mut manager = generator.container_layout_manager();
        manager.create_container_instance(element_type.into(), capacity, ContainerKind::Vector)
            .expect("Failed to create container instance")
    };

    // Create a pointer to an element using our optimized access function
    let index = context.i32_type().const_int(5, false);
    // Updated API now takes element_ptr first followed by index, element_type, and container_kind
    let element_ptr = {
        let mut manager = generator.container_layout_manager();
        manager.get_element_pointer(
            container_ptr,
            index.into(), 
            element_type.into(),
            ContainerKind::Vector
        )
        .expect("Failed to get element pointer")
    };

    // Load the element value - using standard build_load API
    let element_value = generator
        .builder_mut()
        .build_load(element_type, element_ptr, "element_value")
        .expect("Failed to load element");

    // Return the element value
    let _ = generator.builder_mut().build_return(Some(&element_value));

    // Verify that the specialized container has the expected fields
    // Updated API - struct_type().get_field_types() vs. get_field_types()
    let fields = match specialized_vec_type {
        inkwell::types::BasicTypeEnum::StructType(struct_type) => struct_type.get_field_types(),
        _ => panic!("Expected specialized_vec_type to be a struct type"),
    };
    assert_eq!(
        fields.len(),
        4,
        "Specialized container should have 4 fields"
    );

    // Verify that the module is well-formed
    assert!(
        generator.module().verify().is_ok(),
        "Generated LLVM module failed verification"
    );
}
