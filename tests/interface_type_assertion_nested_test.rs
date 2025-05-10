//! Comprehensive nested interface type assertion test
//!
//! This test validates that interface type assertions work correctly with nested interface
//! hierarchies where types implement multiple interfaces with inheritance relationships.

use cursed::error::Error;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::auto_interface_dispatcher::{AutoInterfaceDispatcher, AutoInterfaceDispatchExtension};
use cursed::codegen::llvm::auto_interface_dispatcher_integration::AutoInterfaceDispatcherIntegration;
use cursed::codegen::llvm::interface_implementation::InterfaceImplementation;
use cursed::codegen::llvm::type_assertion::TypeAssertion;
use cursed::core::type_checker::Type as CursedType;
use inkwell::context::Context;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use std::collections::HashMap;

mod common;

#[path = "tracing_setup.rs"]
mod tracing_setup;

/// Define a more complex interface hierarchy to test nested type assertions
/// We'll define several interfaces with relationships:
/// - Drawable: base interface for things that can be drawn
/// - Movable: interface for things that can change position
/// - AnimatedObject: extends both Drawable and Movable
/// - Collidable: interface for things that can be checked for collisions
/// - GameObject: extends AnimatedObject and Collidable for a complete game object
#[test]
fn test_nested_interface_type_assertions() -> Result<(), Error> {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    // Create a new LLVM context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_nested_interfaces")?;
    
    // Initialize the interface components
    codegen.init_auto_interface_dispatcher_integration()?;
    
    // Define the base interfaces
    
    // Drawable interface
    let drawable_methods = vec![
        (
            "draw".to_string(),
            vec![],  // No parameters besides self
            Some(CursedType::Lit),  // Return type: bool to indicate success
        ),
    ];
    codegen.register_interface("Drawable", drawable_methods, vec![])?;
    
    // Movable interface
    let movable_methods = vec![
        (
            "move_to".to_string(),
            vec![CursedType::Normie, CursedType::Normie],  // Parameters: x, y coordinates
            Some(CursedType::Lit),  // Return type: bool to indicate success
        ),
    ];
    codegen.register_interface("Movable", movable_methods, vec![])?;
    
    // AnimatedObject interface (extends Drawable and Movable)
    let animated_object_methods = vec![
        (
            "animate".to_string(),
            vec![CursedType::Tea],  // Parameters: animation name
            Some(CursedType::Lit),  // Return type: bool to indicate success
        ),
    ];
    codegen.register_interface("AnimatedObject", animated_object_methods, vec!["Drawable".to_string(), "Movable".to_string()])?;
    
    // Collidable interface
    let collidable_methods = vec![
        (
            "check_collision".to_string(),
            vec![CursedType::Interface("Collidable".to_string(), vec![])],  // Parameter: another collidable
            Some(CursedType::Lit),  // Return type: bool indicating collision
        ),
    ];
    codegen.register_interface("Collidable", collidable_methods, vec![])?;
    
    // GameObject interface (extends AnimatedObject and Collidable)
    let game_object_methods = vec![
        (
            "get_id".to_string(),
            vec![],  // No parameters
            Some(CursedType::Normie),  // Return type: int32 id
        ),
    ];
    codegen.register_interface("GameObject", game_object_methods, 
        vec!["AnimatedObject".to_string(), "Collidable".to_string()])?;
    
    // Create a concrete implementation: Sprite struct that implements GameObject
    let struct_name = "Sprite";
    
    // Implement all required methods
    
    // Create the draw method
    let draw_fn_type = context.bool_type()
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // self pointer
        ], false);
    let draw_fn = codegen.module().add_function(&format!("{}.draw", struct_name), draw_fn_type, None);
    let basic_block = context.append_basic_block(draw_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    builder.build_return(Some(&context.bool_type().const_int(1, false))).unwrap();
    
    // Create the move_to method
    let move_to_fn_type = context.bool_type()
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // self pointer
            context.i32_type().into(),  // x coordinate
            context.i32_type().into(),  // y coordinate
        ], false);
    let move_to_fn = codegen.module().add_function(&format!("{}.move_to", struct_name), move_to_fn_type, None);
    let basic_block = context.append_basic_block(move_to_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    builder.build_return(Some(&context.bool_type().const_int(1, false))).unwrap();
    
    // Create the animate method
    let animate_fn_type = context.bool_type()
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // self pointer
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // animation name
        ], false);
    let animate_fn = codegen.module().add_function(&format!("{}.animate", struct_name), animate_fn_type, None);
    let basic_block = context.append_basic_block(animate_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    builder.build_return(Some(&context.bool_type().const_int(1, false))).unwrap();
    
    // Create the check_collision method
    let check_collision_fn_type = context.bool_type()
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // self pointer
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // other collidable
        ], false);
    let check_collision_fn = codegen.module().add_function(&format!("{}.check_collision", struct_name), check_collision_fn_type, None);
    let basic_block = context.append_basic_block(check_collision_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    builder.build_return(Some(&context.bool_type().const_int(0, false))).unwrap();  // No collision by default
    
    // Create the get_id method
    let get_id_fn_type = context.i32_type()
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // self pointer
        ], false);
    let get_id_fn = codegen.module().add_function(&format!("{}.get_id", struct_name), get_id_fn_type, None);
    let basic_block = context.append_basic_block(get_id_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    builder.build_return(Some(&context.i32_type().const_int(42, false))).unwrap();  // Return ID 42
    
    // Register all interface implementations
    
    // Sprite implements Drawable
    let drawable_methods = HashMap::from([("draw".to_string(), draw_fn)]);
    codegen.auto_generate_interface_implementation(struct_name, "Drawable", drawable_methods)?;
    
    // Sprite implements Movable
    let movable_methods = HashMap::from([("move_to".to_string(), move_to_fn)]);
    codegen.auto_generate_interface_implementation(struct_name, "Movable", movable_methods)?;
    
    // Sprite implements AnimatedObject
    let animated_object_methods = HashMap::from([("animate".to_string(), animate_fn)]);
    codegen.auto_generate_interface_implementation(struct_name, "AnimatedObject", animated_object_methods)?;
    
    // Sprite implements Collidable
    let collidable_methods = HashMap::from([("check_collision".to_string(), check_collision_fn)]);
    codegen.auto_generate_interface_implementation(struct_name, "Collidable", collidable_methods)?;
    
    // Sprite implements GameObject
    let game_object_methods = HashMap::from([("get_id".to_string(), get_id_fn)]);
    codegen.auto_generate_interface_implementation(struct_name, "GameObject", game_object_methods)?;
    
    // Create the sprite instance
    let sprite_struct_type = context.struct_type(&[], false);
    let sprite_instance = codegen.builder.build_alloca(sprite_struct_type, "sprite_instance").unwrap();
    
    // Convert Sprite to GameObject interface
    let sprite_type = CursedType::Struct(struct_name.to_string(), vec![]);
    let game_object_interface = codegen.create_interface_value(
        sprite_instance,
        &sprite_type,
        "GameObject",
    )?;
    
    // Now perform nested type assertions to test the full hierarchy
    
    // 1. Assert GameObject -> Sprite (concrete type)
    // This should succeed since the GameObject is actually a Sprite
    let sprite_assertion = codegen.type_assert(
        game_object_interface, 
        "GameObject", 
        &CursedType::Struct(struct_name.to_string(), vec![]),
    )?;
    
    assert!(sprite_assertion.0.is_some(), "Expected type assertion to Sprite to succeed");
    assert!(sprite_assertion.1, "Expected success flag to be true for Sprite assertion");
    
    // 2. Convert GameObject -> AnimatedObject (parent interface)
    // This should succeed since GameObject extends AnimatedObject
    let animated_object_assertion = codegen.type_assert(
        game_object_interface,
        "GameObject",
        &CursedType::Interface("AnimatedObject".to_string(), vec![]),
    )?;
    
    assert!(animated_object_assertion.0.is_some(), "Expected type assertion to AnimatedObject to succeed");
    assert!(animated_object_assertion.1, "Expected success flag to be true for AnimatedObject assertion");
    
    // 3. Convert GameObject -> Drawable (grandparent interface)
    // This should succeed since GameObject extends AnimatedObject which extends Drawable
    let drawable_assertion = codegen.type_assert(
        game_object_interface,
        "GameObject",
        &CursedType::Interface("Drawable".to_string(), vec![]),
    )?;
    
    assert!(drawable_assertion.0.is_some(), "Expected type assertion to Drawable to succeed");
    assert!(drawable_assertion.1, "Expected success flag to be true for Drawable assertion");
    
    // 4. Try an invalid assertion to a non-implemented interface
    // Let's create a new interface that isn't implemented
    let resizable_methods = vec![
        (
            "resize".to_string(),
            vec![CursedType::Normie, CursedType::Normie],  // Parameters: width, height
            Some(CursedType::Lit),  // Return type: bool indicating success
        ),
    ];
    codegen.register_interface("Resizable", resizable_methods, vec![])?;
    
    // This should fail since GameObject doesn't implement Resizable
    let resizable_assertion = codegen.type_assert(
        game_object_interface,
        "GameObject",
        &CursedType::Interface("Resizable".to_string(), vec![]),
    )?;
    
    assert!(resizable_assertion.0.is_none(), "Expected type assertion to Resizable to fail");
    assert!(!resizable_assertion.1, "Expected success flag to be false for Resizable assertion");
    
    // 5. Test multi-level assertions (AnimatedObject -> Drawable)
    let drawable_from_animated = codegen.type_assert(
        animated_object_assertion.0.unwrap().into_pointer_value(),
        "AnimatedObject",
        &CursedType::Interface("Drawable".to_string(), vec![]),
    )?;
    
    assert!(drawable_from_animated.0.is_some(), "Expected multi-level type assertion to succeed");
    assert!(drawable_from_animated.1, "Expected success flag to be true for multi-level assertion");
    
    // 6. Test method calls after type assertion
    // Call draw() on the drawable interface
    let draw_result = codegen.auto_generate_method_dispatch(
        drawable_from_animated.0.unwrap().into_pointer_value(),
        "Drawable",
        "draw",
        &[],
    )?;
    
    assert!(draw_result.is_some(), "Expected draw method call to succeed");
    
    Ok(())
}

#[test]
fn test_interface_inheritance_type_assertion() -> Result<(), Error> {
    // Initialize tracing
    tracing_setup::init_test_tracing();
    
    // Create a new LLVM context and code generator
    let context = Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_interface_inheritance")?;
    
    // Initialize the interface components
    codegen.init_auto_interface_dispatcher_integration()?;
    
    // Define a simple interface hierarchy for Reader -> TextReader -> FileReader
    
    // Reader interface
    let reader_methods = vec![
        (
            "read".to_string(),
            vec![],  // No parameters besides self
            Some(CursedType::Tea),  // Return type: string
        ),
    ];
    codegen.register_interface("Reader", reader_methods, vec![])?;
    
    // TextReader interface (extends Reader)
    let text_reader_methods = vec![
        (
            "read_line".to_string(),
            vec![],  // No parameters besides self
            Some(CursedType::Tea),  // Return type: string
        ),
    ];
    codegen.register_interface("TextReader", text_reader_methods, vec!["Reader".to_string()])?;
    
    // FileReader interface (extends TextReader)
    let file_reader_methods = vec![
        (
            "get_path".to_string(),
            vec![],  // No parameters besides self
            Some(CursedType::Tea),  // Return type: string
        ),
    ];
    codegen.register_interface("FileReader", file_reader_methods, vec!["TextReader".to_string()])?;
    
    // Create a concrete implementation: JsonFileReader struct that implements FileReader
    let struct_name = "JsonFileReader";
    
    // Implement all required methods
    
    // Create the read method
    let read_fn_type = context.i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // self pointer
        ], false);
    let read_fn = codegen.module().add_function(&format!("{}.read", struct_name), read_fn_type, None);
    let basic_block = context.append_basic_block(read_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    let str_val = codegen.create_string_constant("json content")?;
    builder.build_return(Some(&str_val)).unwrap();
    
    // Create the read_line method
    let read_line_fn_type = context.i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // self pointer
        ], false);
    let read_line_fn = codegen.module().add_function(&format!("{}.read_line", struct_name), read_line_fn_type, None);
    let basic_block = context.append_basic_block(read_line_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    let str_val = codegen.create_string_constant("one line of json")?;
    builder.build_return(Some(&str_val)).unwrap();
    
    // Create the get_path method
    let get_path_fn_type = context.i8_type()
        .ptr_type(inkwell::AddressSpace::default())
        .fn_type(&[
            context.i8_type().ptr_type(inkwell::AddressSpace::default()).into(),  // self pointer
        ], false);
    let get_path_fn = codegen.module().add_function(&format!("{}.get_path", struct_name), get_path_fn_type, None);
    let basic_block = context.append_basic_block(get_path_fn, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    let str_val = codegen.create_string_constant("/path/to/file.json")?;
    builder.build_return(Some(&str_val)).unwrap();
    
    // Register all interface implementations
    
    // JsonFileReader implements Reader
    let reader_methods = HashMap::from([("read".to_string(), read_fn)]);
    codegen.auto_generate_interface_implementation(struct_name, "Reader", reader_methods)?;
    
    // JsonFileReader implements TextReader
    let text_reader_methods = HashMap::from([("read_line".to_string(), read_line_fn)]);
    codegen.auto_generate_interface_implementation(struct_name, "TextReader", text_reader_methods)?;
    
    // JsonFileReader implements FileReader
    let file_reader_methods = HashMap::from([("get_path".to_string(), get_path_fn)]);
    codegen.auto_generate_interface_implementation(struct_name, "FileReader", file_reader_methods)?;
    
    // Create the JsonFileReader instance
    let json_reader_struct_type = context.struct_type(&[], false);
    let json_reader_instance = codegen.builder.build_alloca(json_reader_struct_type, "json_reader_instance").unwrap();
    
    // Convert JsonFileReader to FileReader interface
    let json_reader_type = CursedType::Struct(struct_name.to_string(), vec![]);
    let file_reader_interface = codegen.create_interface_value(
        json_reader_instance,
        &json_reader_type,
        "FileReader",
    )?;
    
    // Test type assertions through the inheritance chain
    
    // 1. FileReader -> TextReader (parent interface)
    let text_reader_assertion = codegen.type_assert(
        file_reader_interface,
        "FileReader",
        &CursedType::Interface("TextReader".to_string(), vec![]),
    )?;
    
    assert!(text_reader_assertion.0.is_some(), "Expected type assertion to TextReader to succeed");
    assert!(text_reader_assertion.1, "Expected success flag to be true for TextReader assertion");
    
    // 2. TextReader -> Reader (grandparent interface)
    let reader_assertion = codegen.type_assert(
        text_reader_assertion.0.unwrap().into_pointer_value(),
        "TextReader",
        &CursedType::Interface("Reader".to_string(), vec![]),
    )?;
    
    assert!(reader_assertion.0.is_some(), "Expected type assertion to Reader to succeed");
    assert!(reader_assertion.1, "Expected success flag to be true for Reader assertion");
    
    // 3. FileReader -> Reader (direct to grandparent, skipping middle)
    let direct_reader_assertion = codegen.type_assert(
        file_reader_interface,
        "FileReader",
        &CursedType::Interface("Reader".to_string(), vec![]),
    )?;
    
    assert!(direct_reader_assertion.0.is_some(), "Expected direct type assertion to Reader to succeed");
    assert!(direct_reader_assertion.1, "Expected success flag to be true for direct Reader assertion");
    
    // 4. Reader -> concrete type (attempt to assert back to concrete type)
    let concrete_assertion = codegen.type_assert(
        reader_assertion.0.unwrap().into_pointer_value(),
        "Reader",
        &CursedType::Struct(struct_name.to_string(), vec![]),
    )?;
    
    assert!(concrete_assertion.0.is_some(), "Expected type assertion back to concrete type to succeed");
    assert!(concrete_assertion.1, "Expected success flag to be true for concrete type assertion");
    
    // Test method calls after type assertions
    
    // Call read() on Reader interface
    let read_result = codegen.auto_generate_method_dispatch(
        reader_assertion.0.unwrap().into_pointer_value(),
        "Reader",
        "read",
        &[],
    )?;
    
    assert!(read_result.is_some(), "Expected read method call to succeed");
    
    // Call read_line() on TextReader interface
    let read_line_result = codegen.auto_generate_method_dispatch(
        text_reader_assertion.0.unwrap().into_pointer_value(),
        "TextReader",
        "read_line",
        &[],
    )?;
    
    assert!(read_line_result.is_some(), "Expected read_line method call to succeed");
    
    // Call get_path() on FileReader interface
    let get_path_result = codegen.auto_generate_method_dispatch(
        file_reader_interface,
        "FileReader",
        "get_path",
        &[],
    )?;
    
    assert!(get_path_result.is_some(), "Expected get_path method call to succeed");
    
    Ok(())
}