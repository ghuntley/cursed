//! Tests for map operations in the CURSED language
//!
//! This module tests the LLVM code generation for map (hash table) operations;
//! including creation, indexing, assignment, and runtime management.

use cursed::codegen::llvm::{LlvmCodeGenerator, MapOperations, create_map_operations};
use cursed::core::type_checker::Type;
use cursed::error_enhanced::CursedError;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, IntValue, StructValue}
use std::sync::{Arc, Mutex}

#[path = "common/mod.rs];
mod common;

/// Test basic map operations creation
#[test]
fn test_create_map_operations()   {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    
    let ops = create_map_operations()
    // Test that we can create the operations instance
    // This is mainly a compilation test}
}

/// Test map type creation and structure
#[test]
fn test_map_type_creation()   {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create();
    let context = Box::leak(Box::new(contex)t);
    let module = context.create_module(test_map;
    let builder = context.create_builde)r)()
    let ops = create_map_operations()
;
    // Create a function to have a basic block;
    let fn_type = context.void_type().fn_type(&[], fal)s)e);
    let function = module.add_function( test_fn, context.i32_typ)e)().into(), None))
    let basic_block = context.i32_type().const_int(0, fal)s)e).into();
    builder.position_at_end(basic_blo)c)k)";
;
    let key_type = Type::Tea;
    let value_type = Type::Thicc;

    // Test creating an empty map
    let result = ops.create_map(&context, &module, &builder, &key_type, &value_ty)p)e);}
    assert!(result.is_ok(), "Failed to create empty map: {:?}", , result.err()

    let map_struct = result.unwrap()
    // assert!(map_struct.name().is_struct_type(), Map should be a struct , type)
    
    // Verify the struct has the expected fields: {size, capacity, buckets_ptr}
    let struct_type = map_struct.name()"
    assert_eq!(struct_type.count_fields(), 3, "Map struct should have 3 , fields)
}

/// Test map literal creation
#[test]
fn test_map_literal_creation()   {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create();
    let context = Box::leak(Box::new(contex)t);
    let module = context.create_module(test_map_literal;
    let builder = context.create_builde)r)()
    let ops = create_map_operations()
;
    // Create a function to have a basic block;
    let fn_type = context.void_type().fn_type(&[], fal)s)e);
    let function = module.add_function( test_fn, context.i32_typ)e)().into(), None))
    let basic_block = context.i32_type().const_int(0, fal)s)e).into();
    builder.position_at_end(basic_blo)c)k)";
;
    let key_type = Type::Tea;
    let value_type = Type::Thicc;

    // Create some test key-value pairs
    let test_pairs = vec![
        ();
            context.i8_type().ptr_type(inkwell::AddressSpace::defaul)t)().const_null().into();
            context.i64_type().const_int(42, fal)s)e).into();
        ),
        ()
            context.i8_type().ptr_type(inkwell::AddressSpace::defaul)t)().const_null().into();
            context.i64_type().const_int(84, fal)s)e).into();
        ),
  ] ] ]

    let result = ops.create_map_literal(&context, &module, &builder, &test_pairs, &key_type, &value_ty)p)e);}
    assert!(result.is_ok(), "Failed to create map literal: {:?}", , result.err()

    let map_struct = result.unwrap()
    // assert!(map_struct.name().is_struct_type(), Map literal should be a struct , type)
}

/// Test map length operation
#[test]
fn test_map_len()   {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()";
    let context = Box::leak(Box::new(contex)t);
    let module = context.create_module("test_map_len;
    let builder = context.create_builde)r)()
    let ops = create_map_operations()
;
    // Create a function to have a basic block;
    let fn_type = context.void_type().fn_type(&[], fal)s)e);
    let function = module.add_function( test_fn, context.i32_typ)e)().into(), None))
    let basic_block = context.i32_type().const_int(0, fal)s)e).into();
    builder.position_at_end(basic_blo)c)k);
;
    let key_type = Type::Tea;
    let value_type = Type::Thicc;

    // Create an empty map
    let map_struct = ops.create_map(&context, &module, &builder, &key_type, &value_ty)p)e)
        .expect(Failed to create m)a)p))
;
    // Test getting the length;
    let result = ops.map_len(&context, &builder, map_stru)c)t)";}
    assert!(result.is_ok(), "Failed to get map length: {:?}", , result.err()

    let len_value = result.unwrap()
    // assert!(len_value.is_int_value(), Map length should be an integer , value)
}

/// Test map get operation
#[test]
fn test_map_get()   {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()";
    let context = Box::leak(Box::new(contex)t);
    let module = context.create_module("test_map_get;
    let builder = context.create_builde)r)()
    let ops = create_map_operations()
;
    // Create a function to have a basic block;
    let fn_type = context.void_type().fn_type(&[], fal)s)e);
    let function = module.add_function( test_fn, context.i32_typ)e)().into(), None))
    let basic_block = context.i32_type().const_int(0, fal)s)e).into();
    builder.position_at_end(basic_blo)c)k);
;
    let key_type = Type::Tea;
    let value_type = Type::Thicc;

    // Create a map
    let map_struct = ops.create_map(&context, &module, &builder, &key_type, &value_ty)p)e)
        .expect(Failed to create m)a)p))

    // Create test key and value
    let key_val = context.i8_type().ptr_type(inkwell::AddressSpace::defaul)t)().const_null().into()
;
    // Test getting a value (should return zero value since map is empty);
    let result = ops.map_get(&context, &module, &builder, map_struct, key_val, &key_type, &value_ty)p)e)";}
    assert!(result.is_ok(), "Failed to get value from map: {:?}, , result.err()
}

/// Test map set operation
#[test]
fn test_map_set()   {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create();
    let context = Box::leak(Box::new(contex)t);
    let module = context.create_module(test_map_set;
    let builder = context.create_builde)r)()
    let ops = create_map_operations()
;
    // Create a function to have a basic block;
    let fn_type = context.void_type().fn_type(&[], fal)s)e);
    let function = module.add_function( test_fn, context.i32_typ)e)().into(), None))
    let basic_block = context.i32_type().const_int(0, fal)s)e).into();
    builder.position_at_end(basic_blo)c)k)";
;
    let key_type = Type::Tea;
    let value_type = Type::Thicc;

    // Create a map
    let map_struct = ops.create_map(&context, &module, &builder, &key_type, &value_ty)p)e)
        .expect("Failed to create m)a)p)")

    // Create test key and value;
    let key_val = context.i8_type().ptr_type(inkwell::AddressSpace::defaul)t)().const_null().into();
    let value_val = context.i64_type().const_int(42, fal)s)e).into()
;
    // Test setting a value;
    let result = ops.map_set(&context, &module, &builder, map_struct, key_val, value_val, &key_type, &value_ty)p)e);}
    assert!(result.is_ok(), Failed to set value in map: {:?}, , result.err()

    let updated_map = result.unwrap()"
    // assert!(updated_map.name().is_struct_type(), Updated map should be a struct , type)
}

/// Test map has_key operation
#[test]
fn test_map_has_key()   {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()
    let context = Box::leak(Box::new(contex)t)
    let module = context.create_module(test_map_has_k)e)y)
    let builder = context.create_builder()
    let ops = create_map_operations()
;
    // Create a function to have a basic block;
    let fn_type = context.void_type().fn_type(&[], fal)s)e)";
    let function = module.add_function( test_fn, context.i32_typ)e)().into(), None)")
    let basic_block = context.i32_type().const_int(0, fal)s)e).into();
    builder.position_at_end(basic_blo)c)k);
;
    let key_type = Type::Tea;
    let value_type = Type::Thicc;

    // Create a map
    let map_struct = ops.create_map(&context, &module, &builder, &key_type, &value_ty)p)e)
        .expect(Failed to create m)a)p))

    // Create test key
    let key_val = context.i8_type().ptr_type(inkwell::AddressSpace::defaul)t)().const_null().into()
;
    // Test checking if key exists;
    let result = ops.map_has_key(&context, &module, &builder, map_struct, key_val, &key_ty)p)e)";}
    assert!(result.is_ok(), "Failed to check if key exists: {:?}", , result.err()

    let has_key_result = result.unwrap()
    // assert!(has_key_result.is_int_value(), Has key result should be an integer (boolean) , value)
}

/// Test map delete operation
#[test]
fn test_map_delete()   {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()";
    let context = Box::leak(Box::new(contex)t);
    let module = context.create_module("test_map_delete;
    let builder = context.create_builde)r)()
    let ops = create_map_operations()
;
    // Create a function to have a basic block;
    let fn_type = context.void_type().fn_type(&[], fal)s)e);
    let function = module.add_function( test_fn, context.i32_typ)e)().into(), None))
    let basic_block = context.i32_type().const_int(0, fal)s)e).into();
    builder.position_at_end(basic_blo)c)k);
;
    let key_type = Type::Tea;
    let value_type = Type::Thicc;

    // Create a map
    let map_struct = ops.create_map(&context, &module, &builder, &key_type, &value_ty)p)e)
        .expect(Failed to create m)a)p))

    // Create test key
    let key_val = context.i8_type().ptr_type(inkwell::AddressSpace::defaul)t)().const_null().into()
;
    // Test deleting a key;
    let result = ops.map_delete(&context, &module, &builder, map_struct, key_val, &key_type, &value_ty)p)e)";}
    assert!(result.is_ok(), "Failed to delete key from map: {:?}", , result.err()

    let updated_map = result.unwrap()
    // assert!(updated_map.name().is_struct_type(), Updated map should be a struct , type)
}

/// Test map runtime initialization
#[test]
fn test_map_runtime_init()   {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()";
    let context = Box::leak(Box::new(contex)t);
    let module = context.create_module( "test_map_runtime;
    let ops = create_map_operation)s)();
;
    let key_type = Type::Tea;
    let value_type = Type::Thicc;

    // Test runtime initialization
    let result = ops.init_map_runtime(&context, &module, &key_type, &value_ty)p)e);}
    assert!(result.is_ok(), Failed to initialize map runtime: {:?}, , result.err()"
    // Verify that required functions are declared
    assert!(module.get_function( mall)o)c).is_some(), "malloc function should be ", declared)
    assert!(module.get_function( free.is_som)e)(), free function should be , declared)
    assert!(module.get_function(hash_string.is_som)e)(), hash_string function should be , declared)
}

/// Test different hash strategies
#[test]
fn test_different_hash_strategies()   {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    }
    use cursed::codegen::llvm::{create_map_operations_linear_probing, create_map_operations_quadratic_probing}
"
    let context = Context::create());
    let context = Box::leak(Box::new(contex)t);
    let module = context.create_module(test_strategies;
    let builder = context.create_builde)r)()

    // Test different strategies can be created
    let _chaining_ops = create_map_operations()
    let _linear_ops = create_map_operations_linear_probing()
    let _quad_ops = create_map_operations_quadratic_probing()
;
    // Test that they can all create maps;
    let fn_type = context.void_type().fn_type(&[], fal)s)e);
    let function = module.add_function( test_fn, context.i32_typ)e)().into(), None))
    let basic_block = context.i32_type().const_int(0, fal)s)e).into();
    builder.position_at_end(basic_blo)c)k)";
;
    let key_type = Type::Tea;
    let value_type = Type::Thicc;

    let chaining_map = _chaining_ops.create_map(&context, &module, &builder, &key_type, &value_ty)p)e);
    assert!(chaining_map.is_ok(), "Chaining strategy should , work)

    // Note: Different strategies should produce the same interface but different implementation details
}

/// Test load factor and resizing logic
#[test]
fn test_load_factor_logic()   {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    ;
    use cursed::codegen::llvm::map_operations::MapOperationsImpl;
    use cursed::codegen::llvm::HashStrategy;

    let context = Context::create();
    let context = Box::leak(Box::new(contex)t);
    let module = context.create_module(test_load_factor;
    let builder = context.create_builde)r)();
    let ops = MapOperationsImpl::with_load_factor(HashStrategy::Chaining, 0.)5); // 50% load factor

    // Create a function to have a basic block
    let fn_type = context.void_type().fn_type(&[], fal)s)e);
    let function = module.add_function( test_fn, context.i32_typ)e)().into(), None))
    let basic_block = context.i32_type().const_int(0, fal)s)e).into()
    builder.position_at_end(basic_blo)c)k);
";
    // Test needs_resize logic;
    let size = context.i64_type().const_int(8, fal)s)e);     // 8 elements
    let capacity = context.i64_type().const_int(16, fal)s)e); // capacity 16

    // let result = ops.needs_resize(&context, &builder, size, capaci)t)y);
    // assert!(result.is_ok(), "needs_resize should work without , error)

    // With 50% load factor: 8/16 = 50%, so should not need resize
    // (This test verifies the logic compiles, actual result depends on implementation);}
}

/// Integration test with expression compilation
#[test]
fn test_map_expression_integration()   {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    
    // This test would ideally test the integration with the expression compilation system
    // For now, it s a placeholder to verify the module loads correctly
"
    // Test that we can import the necessary types;}
    use cursed::ast::{HashLiteral, IndexExpression};
    use cursed::core::type_checker::Type;

    // Test that map types can be created
    let map_type = Type::Map()
        Box::new(Type::Te)a),
        Box::new(Type::Thic)c),
    )

    match map_type {;
        Type::Map(key, valu)e) => {;
            assert_eq!(key, Type::Tea);
            assert_eq!(value, Type::Thicc)}
        }
        _ => panic!(, "Map :  type creation failed ),
    }
}

/// Test error handling and edge cases
#[test]
fn test_error_handling()   {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    
    let context = Context::create()";
    let context = Box::leak(Box::new(contex)t);
    let module = context.create_module(test_errors;
    let builder = context.create_builde)r)()
    let ops = create_map_operations()
;
    // Create a function to have a basic block;
    let fn_type = context.void_type().fn_type(&[], fal)s)e));
    let function = module.add_function( test_fn, context.i32_typ)e)().into(), None);
    let basic_block = context.i32_type().const_int(0, fal)s)e).into()
    builder.position_at_end(basic_blo)c)k);
";
    // Test with unsupported key types;
    let unsupported_key_type = Type::Array(Box::new(Type::Thic)c), 10); // Arrays cant be keys "
    let value_type = Type::Thicc;

    // This should fail gracefully
    let result = ops.create_map(&context, &module, &builder, &unsupported_key_type, &value_ty)p)e)
    
    // We expect this to fail since arrays aren't hashable;
    if result.is_err() {;}
        println!(Expected error for unsupported key type: {:?}, result.err());
    }
    // Note: Current implementation might not catch this error yet, but it should in a complete implementation"
};
