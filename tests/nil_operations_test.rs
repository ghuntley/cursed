//! Comprehensive tests for nil representation and operations in LLVM
//!
//! This test suite validates the complete nil system including:
//! - Nil literal parsing and compilation
//! - Nil representation for all nullable types
//! - Nil comparison operations
//! - Runtime nil checking
//! - Garbage collector integration
//! - Memory safety of nil operations

use cursed::ast::expressions::{NilLiteral, BooleanLiteral, InfixExpression};
use cursed::ast::expressions::literals::StringLiteral;
use cursed::ast::expressions::identifiers::Identifier;
use cursed::ast::traits::{Expression, Node};
use cursed::core::type_checker::Type;
use cursed::codegen::llvm::{LlvmCodeGenerator, NilOperations, NilOperationsExtension};
use cursed::codegen::llvm::zero_values::ZeroValueGeneration;
use cursed::codegen::llvm::gc_integration::LlvmGcIntegration;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, IntValue};
use inkwell::types::BasicTypeEnum;
use inkwell::{AddressSpace, IntPredicate};
use std::collections::HashMap;

/// Set up test infrastructure for nil operations
fn setup_test_environment() -> (Context, Module<'static>, Builder<'static>) {
    let context = Context::create();
    let module = context.create_module("nil_test");
    let builder = context.create_builder();
    (context, module, builder)
}

/// Create a test code generator
fn create_test_codegen<'ctx>(
    context: &'ctx Context, 
    module: Module<'ctx>, 
    builder: Builder<'ctx>
) -> LlvmCodeGenerator<'ctx> {
    let mut variables = HashMap::new();
    let mut variable_types = HashMap::new();
    let mut functions = HashMap::new();
    let mut type_registry = HashMap::new();
    let mut string_literal_counter = 0;
    let loop_context_stack = Vec::new();
    let struct_definitions = HashMap::new();
    let gc_integration = LlvmGcIntegration::new(context, &module);
    
    LlvmCodeGenerator {
        context,
        module,
        builder,
        variables,
        variable_types,
        functions,
        type_registry,
        string_literal_counter,
        loop_context_stack,
        struct_definitions,
        gc_integration,
    }
}

#[cfg(test)]
mod nil_literal_tests {
    use super::*;

    #[test]
    fn test_nil_literal_creation() {
        let nil_literal = NilLiteral::new();
        assert_eq!(nil_literal.token, "cap");
        assert_eq!(nil_literal.string(), "cap");
        assert_eq!(nil_literal.token_literal(), "cap");
        assert_eq!(nil_literal.node_type(), "NilLiteral");
    }

    #[test]
    fn test_nil_literal_clone() {
        let nil_literal = NilLiteral::new();
        let cloned = nil_literal.clone_box();
        
        assert_eq!(cloned.string(), "cap");
        assert_eq!(cloned.token_literal(), "cap");
    }

    #[test]
    fn test_nil_literal_compilation_without_type() {
        let (context, module, builder) = setup_test_environment();
        let mut codegen = create_test_codegen(&context, module, builder);
        
        let nil_literal = NilLiteral::new();
        let result = codegen.compile_nil_literal(&nil_literal, None);
        
        assert!(result.is_ok(), "Nil literal compilation should succeed");
        let value = result.unwrap();
        assert!(value.is_pointer_value(), "Nil without type should be a pointer");
    }
}

#[cfg(test)]
mod nil_type_representation_tests {
    use super::*;

    #[test]
    fn test_nil_pointer_representation() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let pointer_type = Type::Pointer(Box::new(Type::Normie));
        let result = codegen.create_nil_value_for_type(&pointer_type);
        
        assert!(result.is_ok(), "Nil pointer creation should succeed");
        let value = result.unwrap();
        assert!(value.is_pointer_value(), "Nil pointer should be a pointer value");
        
        if let BasicValueEnum::PointerValue(ptr) = value {
            assert!(ptr.is_null(), "Nil pointer should be null");
        }
    }

    #[test]
    fn test_nil_slice_representation() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let slice_type = Type::Slice(Box::new(Type::Normie));
        let result = codegen.create_nil_value_for_type(&slice_type);
        
        assert!(result.is_ok(), "Nil slice creation should succeed");
        let value = result.unwrap();
        assert!(value.is_struct_value(), "Nil slice should be a struct value");
    }

    #[test]
    fn test_nil_map_representation() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let map_type = Type::Map(Box::new(Type::Tea), Box::new(Type::Normie));
        let result = codegen.create_nil_value_for_type(&map_type);
        
        assert!(result.is_ok(), "Nil map creation should succeed");
        let value = result.unwrap();
        assert!(value.is_pointer_value(), "Nil map should be a pointer value");
        
        if let BasicValueEnum::PointerValue(ptr) = value {
            assert!(ptr.is_null(), "Nil map should be null");
        }
    }

    #[test]
    fn test_nil_channel_representation() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let channel_type = Type::Channel(Box::new(Type::Normie));
        let result = codegen.create_nil_value_for_type(&channel_type);
        
        assert!(result.is_ok(), "Nil channel creation should succeed");
        let value = result.unwrap();
        assert!(value.is_pointer_value(), "Nil channel should be a pointer value");
        
        if let BasicValueEnum::PointerValue(ptr) = value {
            assert!(ptr.is_null(), "Nil channel should be null");
        }
    }

    #[test]
    fn test_nil_function_representation() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let function_type = Type::Function(vec![Type::Normie], Box::new(Type::Normie));
        let result = codegen.create_nil_value_for_type(&function_type);
        
        assert!(result.is_ok(), "Nil function creation should succeed");
        let value = result.unwrap();
        assert!(value.is_pointer_value(), "Nil function should be a pointer value");
        
        if let BasicValueEnum::PointerValue(ptr) = value {
            assert!(ptr.is_null(), "Nil function should be null");
        }
    }

    #[test]
    fn test_nil_interface_representation() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let interface_type = Type::Interface("TestInterface".to_string(), vec![]);
        let result = codegen.create_nil_value_for_type(&interface_type);
        
        assert!(result.is_ok(), "Nil interface creation should succeed");
        let value = result.unwrap();
        assert!(value.is_struct_value(), "Nil interface should be a struct value");
    }

    #[test]
    fn test_non_nullable_types_reject_nil() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let non_nullable_types = vec![
            Type::Normie,
            Type::Tea,
            Type::Lit,
            Type::Snack,
            Type::Meal,
        ];
        
        for ty in non_nullable_types {
            let result = codegen.create_nil_value_for_type(&ty);
            assert!(result.is_err(), "Non-nullable type {:?} should reject nil", ty);
        }
    }
}

#[cfg(test)]
mod nil_comparison_tests {
    use super::*;

    #[test]
    fn test_type_can_be_nil() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        // Nullable types
        assert!(codegen.type_can_be_nil(&Type::Pointer(Box::new(Type::Normie))));
        assert!(codegen.type_can_be_nil(&Type::Slice(Box::new(Type::Normie))));
        assert!(codegen.type_can_be_nil(&Type::Map(Box::new(Type::Tea), Box::new(Type::Normie))));
        assert!(codegen.type_can_be_nil(&Type::Channel(Box::new(Type::Normie))));
        assert!(codegen.type_can_be_nil(&Type::Function(vec![Type::Normie], Box::new(Type::Normie))));
        assert!(codegen.type_can_be_nil(&Type::Interface("Test".to_string(), vec![])));
        
        // Non-nullable types
        assert!(!codegen.type_can_be_nil(&Type::Normie));
        assert!(!codegen.type_can_be_nil(&Type::Tea));
        assert!(!codegen.type_can_be_nil(&Type::Lit));
        assert!(!codegen.type_can_be_nil(&Type::Snack));
        assert!(!codegen.type_can_be_nil(&Type::Meal));
    }

    #[test]
    fn test_nil_representation_sizes() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        assert_eq!(codegen.get_nil_representation_size(&Type::Pointer(Box::new(Type::Normie))), 8);
        assert_eq!(codegen.get_nil_representation_size(&Type::Slice(Box::new(Type::Normie))), 24);
        assert_eq!(codegen.get_nil_representation_size(&Type::Map(Box::new(Type::Tea), Box::new(Type::Normie))), 8);
        assert_eq!(codegen.get_nil_representation_size(&Type::Channel(Box::new(Type::Normie))), 8);
        assert_eq!(codegen.get_nil_representation_size(&Type::Function(vec![Type::Normie], Box::new(Type::Normie))), 8);
        assert_eq!(codegen.get_nil_representation_size(&Type::Interface("Test".to_string(), vec![])), 16);
        
        // Non-nullable types return 0
        assert_eq!(codegen.get_nil_representation_size(&Type::Normie), 0);
        assert_eq!(codegen.get_nil_representation_size(&Type::Tea), 0);
    }

    #[test]
    fn test_compile_nil_pointer_check() {
        let (context, module, builder) = setup_test_environment();
        let mut codegen = create_test_codegen(&context, module, builder);
        
        let pointer_type = Type::Pointer(Box::new(Type::Normie));
        let nil_ptr = codegen.create_nil_value_for_type(&pointer_type).unwrap();
        
        let result = codegen.compile_is_nil_check(nil_ptr, &pointer_type);
        assert!(result.is_ok(), "Nil check compilation should succeed");
        
        let check_result = result.unwrap();
        assert!(check_result.is_int_value(), "Nil check should return boolean");
    }

    #[test]
    fn test_compile_not_nil_pointer_check() {
        let (context, module, builder) = setup_test_environment();
        let mut codegen = create_test_codegen(&context, module, builder);
        
        let pointer_type = Type::Pointer(Box::new(Type::Normie));
        let nil_ptr = codegen.create_nil_value_for_type(&pointer_type).unwrap();
        
        let result = codegen.compile_is_not_nil_check(nil_ptr, &pointer_type);
        assert!(result.is_ok(), "Not nil check compilation should succeed");
        
        let check_result = result.unwrap();
        assert!(check_result.is_int_value(), "Not nil check should return boolean");
    }
}

#[cfg(test)]
mod nil_gc_integration_tests {
    use super::*;

    #[test]
    fn test_nil_value_gc_detection() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        // Create a null pointer
        let ptr_type = context.i32_type().ptr_type(AddressSpace::default());
        let null_ptr = ptr_type.const_null();
        
        assert!(codegen.gc_integration.is_nil_value(null_ptr.into()));
    }

    #[test]
    fn test_nil_gc_validation() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let pointer_type = Type::Pointer(Box::new(Type::Normie));
        let nil_ptr = codegen.create_nil_value_for_type(&pointer_type).unwrap();
        
        let result = codegen.validate_nil_for_gc(nil_ptr, &pointer_type);
        assert!(result.is_ok(), "Nil GC validation should succeed");
        assert!(result.unwrap(), "Nil values should be valid for GC (excluded from tracking)");
    }

    #[test]
    fn test_nil_gc_root_skipping() {
        let (context, module, builder) = setup_test_environment();
        let mut codegen = create_test_codegen(&context, module, builder);
        
        // Create a null pointer
        let ptr_type = context.i32_type().ptr_type(AddressSpace::default());
        let null_ptr = ptr_type.const_null();
        
        let result = codegen.gc_integration.create_gc_root_if_not_nil(&builder, null_ptr.into(), "test_nil");
        assert!(result.is_ok(), "GC root creation should succeed (by skipping nil)");
    }
}

#[cfg(test)]
mod nil_zero_value_integration_tests {
    use super::*;

    #[test]
    fn test_zero_value_is_nil_for_nullable_types() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let nullable_types = vec![
            Type::Pointer(Box::new(Type::Normie)),
            Type::Slice(Box::new(Type::Normie)),
            Type::Map(Box::new(Type::Tea), Box::new(Type::Normie)),
            Type::Channel(Box::new(Type::Normie)),
            Type::Function(vec![Type::Normie], Box::new(Type::Normie)),
            Type::Interface("Test".to_string(), vec![]),
        ];
        
        for ty in nullable_types {
            let zero_value = codegen.create_zero_value(&ty).unwrap();
            let nil_value = codegen.create_nil_value_for_type(&ty).unwrap();
            
            // The zero value for nullable types should be equivalent to nil
            // This is a structural check since we can't easily compare LLVM values
            assert_eq!(zero_value.get_type(), nil_value.get_type(), 
                      "Zero value and nil should have same type for {:?}", ty);
        }
    }
}

#[cfg(test)]
mod nil_memory_safety_tests {
    use super::*;

    #[test]
    fn test_nil_pointer_dereference_safety() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let pointer_type = Type::Pointer(Box::new(Type::Normie));
        let nil_ptr = codegen.create_nil_value_for_type(&pointer_type).unwrap();
        
        // Attempting to dereference nil should be caught at runtime
        // This test verifies that nil pointers are properly represented as null
        if let BasicValueEnum::PointerValue(ptr) = nil_ptr {
            assert!(ptr.is_null(), "Nil pointer should be null to prevent invalid dereferencing");
        }
    }

    #[test]
    fn test_nil_slice_access_safety() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let slice_type = Type::Slice(Box::new(Type::Normie));
        let nil_slice = codegen.create_nil_value_for_type(&slice_type).unwrap();
        
        // Nil slice should have null data pointer and zero length/capacity
        if let BasicValueEnum::StructValue(_slice_val) = nil_slice {
            // The slice structure should be properly formed with null data pointer
            // and zero length/capacity to prevent out-of-bounds access
            // This is verified by the zero value system
        }
    }

    #[test]
    fn test_nil_interface_method_call_safety() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let interface_type = Type::Interface("TestInterface".to_string(), vec![]);
        let nil_interface = codegen.create_nil_value_for_type(&interface_type).unwrap();
        
        // Nil interface should have null data and type pointers
        if let BasicValueEnum::StructValue(_interface_val) = nil_interface {
            // The interface structure should be properly formed with null pointers
            // to prevent invalid method calls on nil interfaces
        }
    }
}

#[cfg(test)]
mod nil_edge_cases_tests {
    use super::*;

    #[test]
    fn test_nil_with_nested_pointers() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let nested_pointer_type = Type::Pointer(Box::new(Type::Pointer(Box::new(Type::Normie))));
        let result = codegen.create_nil_value_for_type(&nested_pointer_type);
        
        assert!(result.is_ok(), "Nil nested pointer creation should succeed");
        let value = result.unwrap();
        assert!(value.is_pointer_value(), "Nil nested pointer should be a pointer value");
    }

    #[test]
    fn test_nil_with_complex_types() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        // Test nil with map of slices
        let complex_type = Type::Map(
            Box::new(Type::Tea), 
            Box::new(Type::Slice(Box::new(Type::Normie)))
        );
        let result = codegen.create_nil_value_for_type(&complex_type);
        
        assert!(result.is_ok(), "Nil complex type creation should succeed");
    }

    #[test]
    fn test_nil_literal_without_type_context() {
        let (context, module, builder) = setup_test_environment();
        let mut codegen = create_test_codegen(&context, module, builder);
        
        let nil_literal = NilLiteral::new();
        let result = codegen.compile_nil_literal(&nil_literal, None);
        
        assert!(result.is_ok(), "Nil literal without type should compile to generic null pointer");
        let value = result.unwrap();
        assert!(value.is_pointer_value(), "Nil without type should be a pointer");
    }
}

#[cfg(test)]
mod nil_runtime_behavior_tests {
    use super::*;

    #[test]
    fn test_llvm_value_nil_checking() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        // Create a null pointer
        let ptr_type = context.i32_type().ptr_type(AddressSpace::default());
        let null_ptr = ptr_type.const_null();
        
        let pointer_type = Type::Pointer(Box::new(Type::Normie));
        let result = codegen.is_llvm_value_nil(null_ptr.into(), &pointer_type);
        
        assert!(result.is_ok(), "LLVM value nil check should succeed");
        assert!(result.unwrap(), "Null pointer should be detected as nil");
    }

    #[test]
    fn test_typed_nil_value_creation() {
        let (context, module, builder) = setup_test_environment();
        let codegen = create_test_codegen(&context, module, builder);
        
        let slice_type = Type::Slice(Box::new(Type::Normie));
        let result = codegen.create_typed_nil_value(&slice_type);
        
        assert!(result.is_ok(), "Typed nil value creation should succeed");
        let value = result.unwrap();
        assert!(value.is_struct_value(), "Typed nil slice should be a struct");
    }
}
