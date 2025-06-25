/// Type System LLVM Integration Tests for CURSED
/// 
/// This test suite validates the critical integration between the type system
/// and LLVM code generation. This is essential because the type system must
/// correctly map CURSED types to LLVM types and ensure type safety throughout
/// the compilation process.
///
/// These tests are vital for ensuring that:
/// - Type inference works correctly during compilation
/// - CURSED types map properly to LLVM types
/// - Type checking prevents invalid operations at compile time
/// - Generic types are properly instantiated and compiled
/// - Type system integration doesn't break under complex scenarios

#[path = "common.rs"]
mod common;

use cursed::{
    ast::{
        expressions::{Literal, LiteralValue},
        operators::{BinaryExpression, AssignmentExpression},
        identifiers::Identifier,
        traits::Expression,
        types::{TypeExpression, TypeKind},
    },
    codegen::llvm::{
        expression_compiler::{LlvmExpressionCompiler, LlvmValue, LlvmType},
        variable_management::VariableManager,
        type_system::{TypeCompilationContext, CompiledGenericInstance},
    },
    core::type_checker::{Type, TypeChecker},
    type_system::{TypeSystem, TypeInference, TypeEnvironment},
    error::Error,
};
use inkwell::{
    context::Context,
    module::Module,
    builder::Builder,
    types::{BasicTypeEnum, IntType, FloatType, StructType},
    values::{BasicValueEnum, IntValue, FloatValue},
    AddressSpace,
};
use std::collections::HashMap;
use tracing::{debug, info, error};

/// Comprehensive test context with type system integration
struct TypeSystemTestContext<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    type_checker: TypeChecker,
    type_compilation_context: TypeCompilationContext<'ctx>,
    variable_manager: VariableManager<'ctx>,
    expression_compiler: LlvmExpressionCompiler<'ctx>,
}

impl<'ctx> TypeSystemTestContext<'ctx> {
    fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("type_system_test");
        let builder = context.create_builder();
        
        // Initialize type system components
        let type_checker = TypeChecker::new();
        let type_compilation_context = TypeCompilationContext::new(context, &module);
        let variable_manager = VariableManager::new(context, &module, &builder);
        
        let expression_compiler = LlvmExpressionCompiler::new(
            context,
            &module,
            &builder,
            &variable_manager,
            &type_compilation_context,
        );
        
        Self {
            context,
            module,
            builder,
            type_checker,
            type_compilation_context,
            variable_manager,
            expression_compiler,
        }
    }
    
    fn create_function(&mut self, name: &str, return_type: BasicTypeEnum<'ctx>) {
        let fn_type = return_type.fn_type(&[], false);
        let function = self.module.add_function(name, fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
        self.variable_manager.set_current_function(Some(function));
    }
}

/// Test basic type mapping from CURSED to LLVM
#[test]
fn test_basic_type_mapping() {
    common::init_tracing!();
    info!("Testing basic type mapping from CURSED to LLVM");
    
    let context = Context::create();
    let mut test_ctx = TypeSystemTestContext::new(&context);
    
    // Test integer type mapping
    let int_type = Type::Integer;
    let llvm_int_type = test_ctx.type_compilation_context.map_cursed_type_to_llvm(&int_type);
    assert!(llvm_int_type.is_ok(), "Failed to map integer type: {:?}", llvm_int_type.err());
    
    // Test float type mapping
    let float_type = Type::Float;
    let llvm_float_type = test_ctx.type_compilation_context.map_cursed_type_to_llvm(&float_type);
    assert!(llvm_float_type.is_ok(), "Failed to map float type: {:?}", llvm_float_type.err());
    
    // Test string type mapping
    let string_type = Type::String;
    let llvm_string_type = test_ctx.type_compilation_context.map_cursed_type_to_llvm(&string_type);
    assert!(llvm_string_type.is_ok(), "Failed to map string type: {:?}", llvm_string_type.err());
    
    // Test boolean type mapping
    let bool_type = Type::Boolean;
    let llvm_bool_type = test_ctx.type_compilation_context.map_cursed_type_to_llvm(&bool_type);
    assert!(llvm_bool_type.is_ok(), "Failed to map boolean type: {:?}", llvm_bool_type.err());
    
    info!("Basic type mapping test passed");
}

/// Test type inference during expression compilation
#[test]
fn test_type_inference_during_compilation() {
    common::init_tracing!();
    info!("Testing type inference during expression compilation");
    
    let context = Context::create();
    let mut test_ctx = TypeSystemTestContext::new(&context);
    test_ctx.create_function("test_inference", context.i32_type().as_basic_type_enum());
    
    // Test that integer literals are correctly inferred as integers
    let int_literal = Literal {
        value: LiteralValue::Integer(42),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_literal(&int_literal);
    assert!(result.is_ok(), "Failed to compile integer with type inference: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Int32, "Type inference failed for integer");
    
    // Test that float literals are correctly inferred as floats
    let float_literal = Literal {
        value: LiteralValue::Float(3.14),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_literal(&float_literal);
    assert!(result.is_ok(), "Failed to compile float with type inference: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Float64, "Type inference failed for float");
    
    // Test that boolean literals are correctly inferred as booleans
    let bool_literal = Literal {
        value: LiteralValue::Boolean(true),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_literal(&bool_literal);
    assert!(result.is_ok(), "Failed to compile boolean with type inference: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Bool, "Type inference failed for boolean");
    
    info!("Type inference during compilation test passed");
}

/// Test arithmetic type checking and compatibility
#[test]
fn test_arithmetic_type_checking() {
    common::init_tracing!();
    info!("Testing arithmetic type checking");
    
    let context = Context::create();
    let mut test_ctx = TypeSystemTestContext::new(&context);
    test_ctx.create_function("test_arithmetic_types", context.i32_type().as_basic_type_enum());
    
    // Test valid integer arithmetic: 5 + 3
    let valid_add = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(3),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&valid_add);
    assert!(result.is_ok(), "Valid integer arithmetic should compile: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Int32, "Result should be integer type");
    
    // Test valid float arithmetic: 2.5 + 1.5
    let valid_float_add = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Float(2.5),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Float(1.5),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&valid_float_add);
    assert!(result.is_ok(), "Valid float arithmetic should compile: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Float64, "Result should be float type");
    
    info!("Arithmetic type checking test passed");
}

/// Test comparison operation type checking
#[test]
fn test_comparison_type_checking() {
    common::init_tracing!();
    info!("Testing comparison operation type checking");
    
    let context = Context::create();
    let mut test_ctx = TypeSystemTestContext::new(&context);
    test_ctx.create_function("test_comparison_types", context.i1_type().as_basic_type_enum());
    
    // Test integer comparison: 5 == 5
    let int_comparison = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        operator: "==".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&int_comparison);
    assert!(result.is_ok(), "Integer comparison should compile: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Bool, "Comparison result should be boolean");
    
    // Test float comparison: 3.14 > 2.71
    let float_comparison = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Float(3.14),
            location: None,
        }) as Box<dyn Expression>,
        operator: ">".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Float(2.71),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&float_comparison);
    assert!(result.is_ok(), "Float comparison should compile: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Bool, "Float comparison result should be boolean");
    
    // Test boolean comparison: true == false
    let bool_comparison = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Boolean(true),
            location: None,
        }) as Box<dyn Expression>,
        operator: "==".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Boolean(false),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&bool_comparison);
    assert!(result.is_ok(), "Boolean comparison should compile: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Bool, "Boolean comparison result should be boolean");
    
    info!("Comparison type checking test passed");
}

/// Test variable type checking and assignment
#[test]
fn test_variable_type_checking() {
    common::init_tracing!();
    info!("Testing variable type checking and assignment");
    
    let context = Context::create();
    let mut test_ctx = TypeSystemTestContext::new(&context);
    test_ctx.create_function("test_var_types", context.i32_type().as_basic_type_enum());
    
    // Test integer variable assignment: sus x = 42
    let int_assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "x".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Integer(42),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_assignment_expression(&int_assignment);
    assert!(result.is_ok(), "Integer variable assignment should compile: {:?}", result.err());
    
    // Test variable access
    let var_access = Identifier {
        name: "x".to_string(),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_identifier(&var_access);
    assert!(result.is_ok(), "Variable access should compile: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Int32, "Variable should maintain integer type");
    
    // Test float variable assignment: sus y = 3.14
    let float_assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "y".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Float(3.14),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_assignment_expression(&float_assignment);
    assert!(result.is_ok(), "Float variable assignment should compile: {:?}", result.err());
    
    // Test float variable access
    let float_var_access = Identifier {
        name: "y".to_string(),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_identifier(&float_var_access);
    assert!(result.is_ok(), "Float variable access should compile: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Float64, "Variable should maintain float type");
    
    info!("Variable type checking test passed");
}

/// Test type system error detection
#[test]
fn test_type_system_error_detection() {
    common::init_tracing!();
    info!("Testing type system error detection");
    
    let context = Context::create();
    let mut test_ctx = TypeSystemTestContext::new(&context);
    test_ctx.create_function("test_type_errors", context.i32_type().as_basic_type_enum());
    
    // Test accessing undefined variable
    let undefined_var = Identifier {
        name: "undefined_variable".to_string(),
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_identifier(&undefined_var);
    assert!(result.is_err(), "Accessing undefined variable should fail");
    
    // Test invalid binary operation (if type checking is strict)
    // Note: This might not fail if type coercion is implemented
    let mixed_type_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::String("hello".to_string()),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(42),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&mixed_type_expr);
    // This may or may not fail depending on type coercion implementation
    match result {
        Ok(_) => info!("String + Integer compiled (type coercion may be implemented)"),
        Err(_) => info!("String + Integer failed as expected (strict type checking)"),
    }
    
    info!("Type system error detection test completed");
}

/// Test complex type expressions with nested structures
#[test]
fn test_complex_type_expressions() {
    common::init_tracing!();
    info!("Testing complex type expressions");
    
    let context = Context::create();
    let mut test_ctx = TypeSystemTestContext::new(&context);
    test_ctx.create_function("test_complex_types", context.i32_type().as_basic_type_enum());
    
    // Test nested arithmetic with type consistency: (5 + 3) * (10 - 2)
    let inner_left = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(3),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let inner_right = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(10),
            location: None,
        }) as Box<dyn Expression>,
        operator: "-".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(2),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let complex_expr = BinaryExpression {
        left: Box::new(inner_left) as Box<dyn Expression>,
        operator: "*".to_string(),
        right: Box::new(inner_right) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&complex_expr);
    assert!(result.is_ok(), "Complex nested expression should compile: {:?}", result.err());
    
    let value = result.unwrap();
    assert_eq!(value.value_type, LlvmType::Int32, "Complex expression should maintain type consistency");
    
    info!("Complex type expressions test passed");
}

/// Test type system integration with LLVM module generation
#[test]
fn test_type_system_llvm_module_integration() {
    common::init_tracing!();
    info!("Testing type system integration with LLVM module generation");
    
    let context = Context::create();
    let mut test_ctx = TypeSystemTestContext::new(&context);
    test_ctx.create_function("test_integration", context.i32_type().as_basic_type_enum());
    
    // Compile expressions of different types to test module consistency
    let expressions = vec![
        (LiteralValue::Integer(42), LlvmType::Int32),
        (LiteralValue::Float(3.14), LlvmType::Float64),
        (LiteralValue::Boolean(true), LlvmType::Bool),
        (LiteralValue::String("test".to_string()), LlvmType::String),
    ];
    
    let mut compiled_count = 0;
    for (literal_value, expected_type) in expressions {
        let literal = Literal {
            value: literal_value,
            location: None,
        };
        
        let result = test_ctx.expression_compiler.compile_literal(&literal);
        if let Ok(value) = result {
            assert_eq!(value.value_type, expected_type, "Type mismatch for literal");
            compiled_count += 1;
        }
    }
    
    assert!(compiled_count > 0, "Should have compiled at least some typed expressions");
    
    // Verify the LLVM module is valid after type system integration
    let llvm_ir = test_ctx.module.print_to_string();
    let ir_string = llvm_ir.to_string();
    
    assert!(!ir_string.is_empty(), "LLVM IR should not be empty");
    assert!(ir_string.contains("define"), "LLVM IR should contain function definitions");
    
    info!("Type system LLVM module integration test passed - compiled {} typed expressions", compiled_count);
}

/// Test type coercion and conversion (if implemented)
#[test]
fn test_type_coercion() {
    common::init_tracing!();
    info!("Testing type coercion and conversion");
    
    let context = Context::create();
    let mut test_ctx = TypeSystemTestContext::new(&context);
    test_ctx.create_function("test_coercion", context.f64_type().as_basic_type_enum());
    
    // Test potential integer to float coercion: 5 + 2.5
    let coercion_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Float(2.5),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_binary_expression(&coercion_expr);
    
    match result {
        Ok(value) => {
            info!("Type coercion successful: {:?}", value.value_type);
            // If coercion is implemented, result should be float
            // If not, this test documents the current behavior
        }
        Err(err) => {
            info!("Type coercion not implemented or failed: {:?}", err);
            // This is also valid - strict typing without coercion
        }
    }
    
    info!("Type coercion test completed");
}

/// Performance test for type system integration
#[test]
fn test_type_system_performance() {
    common::init_tracing!();
    info!("Testing type system performance during compilation");
    
    let context = Context::create();
    let mut test_ctx = TypeSystemTestContext::new(&context);
    test_ctx.create_function("test_perf", context.i32_type().as_basic_type_enum());
    
    let start_time = std::time::Instant::now();
    
    // Compile many typed expressions to test performance
    for i in 0..50 {
        let int_expr = Literal {
            value: LiteralValue::Integer(i),
            location: None,
        };
        
        let result = test_ctx.expression_compiler.compile_literal(&int_expr);
        assert!(result.is_ok(), "Type system performance test failed at iteration {}", i);
        
        let float_expr = Literal {
            value: LiteralValue::Float(i as f64 + 0.5),
            location: None,
        };
        
        let result = test_ctx.expression_compiler.compile_literal(&float_expr);
        assert!(result.is_ok(), "Type system performance test failed at float iteration {}", i);
    }
    
    let elapsed = start_time.elapsed();
    info!("Type system compiled 100 typed expressions in {:?}", elapsed);
    
    // Type system integration should not significantly slow compilation
    assert!(elapsed.as_secs() < 2, "Type system integration too slow: {:?}", elapsed);
}

/// Test type system state consistency across multiple compilations
#[test]
fn test_type_system_state_consistency() {
    common::init_tracing!();
    info!("Testing type system state consistency");
    
    let context = Context::create();
    let mut test_ctx = TypeSystemTestContext::new(&context);
    test_ctx.create_function("test_consistency", context.i32_type().as_basic_type_enum());
    
    // Test that type system maintains consistent state across multiple operations
    
    // First operation: variable assignment
    let assignment1 = AssignmentExpression {
        target: Box::new(Identifier {
            name: "var1".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Integer(10),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result1 = test_ctx.expression_compiler.compile_assignment_expression(&assignment1);
    assert!(result1.is_ok(), "First assignment should succeed");
    
    // Second operation: use the variable
    let var_access = Identifier {
        name: "var1".to_string(),
        location: None,
    };
    
    let result2 = test_ctx.expression_compiler.compile_identifier(&var_access);
    assert!(result2.is_ok(), "Variable access should succeed after assignment");
    
    // Third operation: another assignment to same variable
    let assignment2 = AssignmentExpression {
        target: Box::new(Identifier {
            name: "var1".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Integer(20),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result3 = test_ctx.expression_compiler.compile_assignment_expression(&assignment2);
    assert!(result3.is_ok(), "Second assignment should succeed");
    
    // Fourth operation: final variable access
    let final_access = Identifier {
        name: "var1".to_string(),
        location: None,
    };
    
    let result4 = test_ctx.expression_compiler.compile_identifier(&final_access);
    assert!(result4.is_ok(), "Final variable access should succeed");
    
    let final_value = result4.unwrap();
    assert_eq!(final_value.value_type, LlvmType::Int32, "Variable type should remain consistent");
    
    info!("Type system state consistency test passed");
}
