use cursed::codegen::llvm::EnhancedMonomorphization;
use cursed::codegen::llvm::IntegratedMonomorphization;
use cursed::codegen::llvm::StructMonomorphization;

#[path = "common.rs"]
mod common;

#[test]
fn test_integrated_monomorphization() {
    common::tracing::setup();
    
    // Setup test environment
    let context = inkwell::context::Context::create();
    let mut code_gen = cursed::codegen::llvm::LlvmCodeGenerator::new(
        &context,
        "test_module",
        std::path::PathBuf::from("test.csd"),
    );
    
    // Import the enhanced and integrated monomorphization traits
    
    // Create a test generic struct
    let ast_factory = common::ast_factory::AstFactory::new();
    let generic_struct = ast_factory.create_generic_struct(
        "Container",
        vec!["T"],
        vec![("value", "T"), ("next", "Container")],
    );
    
    // Create type args for specialization
    let type_args = vec![cursed::core::type_checker::Type::Normie];
    let specialized_name = "Container__Normie";
    
    // Test the integrated monomorphization
    let result = code_gen.generate_specialized_struct_with_accessors(
        &generic_struct,
        specialized_name,
        &type_args,
    );
    
    // Verify the result
    assert!(result.is_ok(), "Integrated monomorphization failed: {:?}", result.err())
    
    // Verify the accessors were created correctly by checking for their presence in the module
    let module = code_gen.module();
    
    // Check for getter functions
    let value_getter = module.get_function("Container__Normie_get_value");
    assert!(value_getter.is_some(), "Value getter function not found");
    
    let next_getter = module.get_function("Container__Normie_get_next");
    assert!(next_getter.is_some(), "Next getter function not found");
    
    // Check for setter functions
    let value_setter = module.get_function("Container__Normie_set_value");
    assert!(value_setter.is_some(), "Value setter function not found");
    
    let next_setter = module.get_function("Container__Normie_set_next");
    assert!(next_setter.is_some(), "Next setter function not found");
    
    // Print the generated LLVM IR for manual inspection
    let ir = module.print_to_string().to_string();
    println!("Generated LLVM IR:\n{}", ir);
}