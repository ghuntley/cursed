use cursed::core::type_checker::Type as CursedType;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::values::{BasicValueEnum, FunctionValue};
use std::collections::HashMap;

// Test automatic code generation for interface method dispatching
//
// This test verifies that our automatic interface method dispatch code generation
// works correctly, both for static interface implementations and dynamic lookups.

use cursed::codegen::llvm::{
    LlvmCodeGenerator, 
    AutoInterfaceDispatcher, 
    AutoInterfaceDispatchExtension,
    AutoInterfaceDispatcherIntegration,
    InterfaceImplementation,
    StringUtilsExtension
};

mod common;

#[test]
fn test_auto_interface_implementation() -> Result<(), Error> {
    common::tracing::setup();
    
    // Create a new LLVM context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", std::path::PathBuf::from("test.csd"));
    
    // Initialize the auto interface dispatcher with comprehensive integration
    codegen.init_auto_interface_dispatcher_integration()?;
    
    // Define an interface with a single method
    let greeter_methods = vec![
        (
            "greet".to_string(),
            vec![CursedType::Tea],  // Parameters: [name: string]
            Some(CursedType::Tea),   // Return type: string
        ),
    ];
    
    // Register the interface with the code generator
    codegen.register_interface("Greeter", greeter_methods, vec![])?;
    
    // Create a struct implementation of the interface
    let struct_name = "Person";
    
    // Create a method for the struct
    let greet_fn_type = context.i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // self pointer
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // name parameter
        ], false);
    
    let greet_fn = codegen.module().add_function(
        &format!("{}.greet", struct_name),
        greet_fn_type,
        None,
    );
    
    // Create a basic block and function body
    let basic_block = context.append_basic_block(greet_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Simply return the name parameter as the result (echo function)
    let param = greet_fn.get_nth_param(1).unwrap().into_pointer_value();
    builder.build_return(Some(&param)).unwrap();
    
    // Add the method to a map
    let mut methods = HashMap::new();
    methods.insert("greet".to_string(), greet_fn);
    
    // Auto-generate the interface implementation
    codegen.auto_generate_interface_implementation(
        struct_name,
        "Greeter",
        methods,
    )?;
    
    // Create Person instance and convert to Greeter interface
    let person_struct_type = context.struct_type(&[], false);
    let person_instance = codegen.builder().build_alloca(person_struct_type, "person_instance").unwrap();
    
    let person_type = CursedType::Struct(struct_name.to_string(), vec![]);
    let greeter_interface = codegen.create_interface_value(
        person_instance,
        &person_type,
        "Greeter",
    )?;
    
    // Create a string constant for the name parameter
    let name_str = codegen.create_string_constant("Alice")?;
    
    // Use the auto-generated method dispatch code
    let result = codegen.auto_generate_method_dispatch(
        greeter_interface,
        "Greeter",
        "greet",
        &[name_str.into()],
    )?;
    
    // Verify we got a result
    assert!(result.is_some(), "Expected a result from method call");
    
    // Try direct dispatch as well
    let direct_result = codegen.auto_generate_direct_dispatch(
        person_instance,
        &person_type,
        "Greeter",
        "greet",
        &[name_str.into()],
    )?;
    
    assert!(direct_result.is_some(), "Expected a result from direct dispatch");
    
    Ok(())
}

#[test]
fn test_auto_registration_of_struct_methods() -> Result<(), Error> {
    common::tracing::setup();
    
    // Create a new LLVM context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module_auto_reg", std::path::PathBuf::from("test.csd"));
    
    // Initialize the auto interface dispatcher with comprehensive integration
    codegen.init_auto_interface_dispatcher_integration()?;
    
    // Define an interface with methods
    let shape_methods = vec![
        (
            "area".to_string(),
            vec![],  // No parameters besides self
            Some(CursedType::Meal),   // Return type: float64
        ),
        (
            "perimeter".to_string(),
            vec![],  // No parameters besides self
            Some(CursedType::Meal),   // Return type: float64
        ),
    ];
    
    // Register the interface with the code generator
    codegen.register_interface("Shape", shape_methods, vec![])?;
    
    // Create a struct implementation of the interface
    let struct_name = "Rectangle";
    
    // Create the area method
    let area_fn_type = context.f64_type()
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // self pointer
        ], false);
    
    let area_fn = codegen.module().add_function(
        &format!("{}.area", struct_name),
        area_fn_type,
        None,
    );
    
    // Create a basic block and function body for area
    let basic_block = context.append_basic_block(area_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Return a constant value for area
    builder.build_return(Some(&context.f64_type().const_float(40.0))).unwrap();
    
    // Create the perimeter method
    let perimeter_fn_type = context.f64_type()
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // self pointer
        ], false);
    
    let perimeter_fn = codegen.module().add_function(
        &format!("{}.perimeter", struct_name),
        perimeter_fn_type,
        None,
    );
    
    // Create a basic block and function body for perimeter
    let basic_block = context.append_basic_block(perimeter_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Return a constant value for perimeter
    builder.build_return(Some(&context.f64_type().const_float(26.0))).unwrap();
    
    // Use the enhanced discovery and registration functionality
    codegen.discover_and_register_interface_implementations(
        struct_name,
        "Shape",
    )?;
    
    // Create Rectangle instance and convert to Shape interface
    let rect_struct_type = context.struct_type(&[], false);
    let rect_instance = codegen.builder().build_alloca(rect_struct_type, "rect_instance").unwrap();
    
    let rect_type = CursedType::Struct(struct_name.to_string(), vec![]);
    let shape_interface = codegen.create_interface_value(
        rect_instance,
        &rect_type,
        "Shape",
    )?;
    
    // Use the auto-generated method dispatch code for area
    let area_result = codegen.auto_generate_method_dispatch(
        shape_interface,
        "Shape",
        "area",
        &[],
    )?;
    
    // Verify we got a result
    assert!(area_result.is_some(), "Expected a result from area method call");
    
    // Use the auto-generated method dispatch code for perimeter
    let perimeter_result = codegen.auto_generate_method_dispatch(
        shape_interface,
        "Shape",
        "perimeter",
        &[],
    )?;
    
    // Verify we got a result
    assert!(perimeter_result.is_some(), "Expected a result from perimeter method call");
    
    Ok(())
}

#[test]
fn test_optimize_interface_call() -> Result<(), Error> {
    common::tracing::setup();
    
    // Create a new LLVM context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module_optimizer", std::path::PathBuf::from("test.csd"));
    
    // Initialize the auto interface dispatcher with comprehensive integration
    codegen.init_auto_interface_dispatcher_integration()?;
    
    // Define an interface with a method
    let speaker_methods = vec![
        (
            "speak".to_string(),
            vec![],  // No parameters besides self
            Some(CursedType::Tea),   // Return type: string
        ),
    ];
    
    // Register the interface with the code generator
    codegen.register_interface("Speaker", speaker_methods, vec![])?;
    
    // Create a struct implementation of the interface
    let struct_name = "Dog";
    
    // Create the speak method
    let speak_fn_type = context.i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // self pointer
        ], false);
    
    let speak_fn = codegen.module().add_function(
        &format!("{}.speak", struct_name),
        speak_fn_type,
        None,
    );
    
    // Create a basic block and function body
    let basic_block = context.append_basic_block(speak_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Create a string constant for the return value
    let bark_str = codegen.create_string_constant("Woof!")?;
    
    // Return the bark string
    builder.build_return(Some(&bark_str)).unwrap();
    
    // Add the method to a map
    let mut methods = HashMap::new();
    methods.insert("speak".to_string(), speak_fn);
    
    // Auto-generate the interface implementation
    codegen.auto_generate_interface_implementation(
        struct_name,
        "Speaker",
        methods,
    )?;
    
    // Create Dog instance
    let dog_struct_type = context.struct_type(&[], false);
    let dog_instance = codegen.builder().build_alloca(dog_struct_type, "dog_instance").unwrap();
    
    // Create types for testing
    let dog_type = CursedType::Struct(struct_name.to_string(), vec![]);
    let speaker_type = CursedType::Interface("Speaker".to_string(), vec![]);
    
    // Convert Dog to Speaker interface
    let speaker_interface = codegen.create_interface_value(
        dog_instance,
        &dog_type,
        "Speaker",
    )?;
    
    // Test the optimize_interface_call with struct instance (should use direct dispatch)
    let result_struct = codegen.optimize_interface_call(
        dog_instance,
        &dog_type,
        "speak",
        &[],
    )?;
    
    // Verify we got a result
    assert!(result_struct.is_some(), "Expected a result from optimized struct call");
    
    // Test the optimize_interface_call with interface instance (should use dynamic dispatch)
    let result_interface = codegen.optimize_interface_call(
        speaker_interface,
        &speaker_type,
        "speak",
        &[],
    )?;
    
    // Verify we got a result
    assert!(result_interface.is_some(), "Expected a result from optimized interface call");
    
    Ok(())
}