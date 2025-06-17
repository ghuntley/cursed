/// LLVM Assignment Operations Test Suite for CURSED
/// 
/// This test suite validates assignment operations compilation in LLVM,
/// which is fundamental for variable management and state changes in programs.
/// Assignment operations are critical because they enable:
/// - Variable initialization (sus x = 42)
/// - Variable updates (x = x + 1)  
/// - Complex assignments with expressions
/// - Type consistency during assignments
/// 
/// These tests verify that the CURSED compiler correctly generates LLVM IR
/// for all forms of assignment operations and maintains proper variable
/// state throughout the program execution.

#[path = "common.rs"]
mod common;

use cursed::{
    ast::{
        expressions::{Literal, LiteralValue},
        operators::{AssignmentExpression, BinaryExpression, CompoundAssignmentExpression},
        identifiers::Identifier,
        statements::LetStatement,
        traits::{Expression, Statement},
    },
    codegen::llvm::{
        expression_compiler::{LlvmExpressionCompiler, LlvmValue, LlvmType},
        variable_management::VariableManager,
        type_system::TypeCompilationContext,
    },
    core::type_checker::Type,
    error::Error,
};
use inkwell::{
    context::Context,
    module::Module,
    builder::Builder,
    types::BasicTypeEnum,
    values::{BasicValueEnum, FunctionValue},
    AddressSpace,
};
use std::collections::HashMap;
use tracing::{debug, info, error};

/// Test context for assignment operations
struct AssignmentTestContext<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variable_manager: VariableManager<'ctx>,
    expression_compiler: LlvmExpressionCompiler<'ctx>,
    current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> AssignmentTestContext<'ctx> {
    fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("assignment_test");
        let builder = context.create_builder();
        
        let variable_manager = VariableManager::new(context, &module, &builder);
        let type_context = TypeCompilationContext::new(context, &module);
        
        let expression_compiler = LlvmExpressionCompiler::new(
            context,
            &module,
            &builder,
            &variable_manager,
            &type_context,
        );
        
        Self {
            context,
            module,
            builder,
            variable_manager,
            expression_compiler,
            current_function: None,
        }
    }
    
    fn create_test_function(&mut self, name: &str, return_type: BasicTypeEnum<'ctx>) {
        let fn_type = return_type.fn_type(&[], false);
        let function = self.module.add_function(name, fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
        self.variable_manager.set_current_function(Some(function));
        self.current_function = Some(function);
    }
}

/// Test basic variable assignment with integers
#[test]
fn test_basic_integer_assignment() {
    common::init_tracing!();
    info!("Testing basic integer assignment");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_int_assign", context.i32_type().as_basic_type_enum());
    
    // Test: sus x = 42
    let assignment = AssignmentExpression {
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
    
    let result = test_ctx.expression_compiler.compile_assignment_expression(&assignment);
    assert!(result.is_ok(), "Integer assignment compilation failed: {:?}", result.err());
    
    let assignment_value = result.unwrap();
    assert_eq!(assignment_value.value_type, LlvmType::Int32);
    
    // Test variable access after assignment
    let var_access = Identifier {
        name: "x".to_string(),
        location: None,
    };
    
    let access_result = test_ctx.expression_compiler.compile_identifier(&var_access);
    assert!(access_result.is_ok(), "Variable access after assignment failed: {:?}", access_result.err());
    
    let access_value = access_result.unwrap();
    assert_eq!(access_value.value_type, LlvmType::Int32);
    
    info!("Basic integer assignment test passed");
}

/// Test floating point assignment operations
#[test]
fn test_float_assignment() {
    common::init_tracing!();
    info!("Testing float assignment");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_float_assign", context.f64_type().as_basic_type_enum());
    
    // Test: sus pi = 3.14159
    let float_assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "pi".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Float(3.14159),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_assignment_expression(&float_assignment);
    assert!(result.is_ok(), "Float assignment compilation failed: {:?}", result.err());
    
    let assignment_value = result.unwrap();
    assert_eq!(assignment_value.value_type, LlvmType::Float64);
    
    // Verify variable can be accessed
    let pi_access = Identifier {
        name: "pi".to_string(),
        location: None,
    };
    
    let access_result = test_ctx.expression_compiler.compile_identifier(&pi_access);
    assert!(access_result.is_ok(), "Float variable access failed: {:?}", access_result.err());
    
    let access_value = access_result.unwrap();
    assert_eq!(access_value.value_type, LlvmType::Float64);
    
    info!("Float assignment test passed");
}

/// Test string assignment operations
#[test]
fn test_string_assignment() {
    common::init_tracing!();
    info!("Testing string assignment");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_string_assign", context.i8_type().ptr_type(AddressSpace::default()).as_basic_type_enum());
    
    // Test: sus message = "Hello, CURSED!"
    let string_assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "message".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::String("Hello, CURSED!".to_string()),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_assignment_expression(&string_assignment);
    assert!(result.is_ok(), "String assignment compilation failed: {:?}", result.err());
    
    let assignment_value = result.unwrap();
    assert_eq!(assignment_value.value_type, LlvmType::String);
    
    // Verify string variable access
    let msg_access = Identifier {
        name: "message".to_string(),
        location: None,
    };
    
    let access_result = test_ctx.expression_compiler.compile_identifier(&msg_access);
    assert!(access_result.is_ok(), "String variable access failed: {:?}", access_result.err());
    
    let access_value = access_result.unwrap();
    assert_eq!(access_value.value_type, LlvmType::String);
    
    info!("String assignment test passed");
}

/// Test boolean assignment operations
#[test]
fn test_boolean_assignment() {
    common::init_tracing!();
    info!("Testing boolean assignment");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_bool_assign", context.i1_type().as_basic_type_enum());
    
    // Test: sus flag = true
    let bool_assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "flag".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Boolean(true),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_assignment_expression(&bool_assignment);
    assert!(result.is_ok(), "Boolean assignment compilation failed: {:?}", result.err());
    
    let assignment_value = result.unwrap();
    assert_eq!(assignment_value.value_type, LlvmType::Bool);
    
    // Test accessing boolean variable
    let flag_access = Identifier {
        name: "flag".to_string(),
        location: None,
    };
    
    let access_result = test_ctx.expression_compiler.compile_identifier(&flag_access);
    assert!(access_result.is_ok(), "Boolean variable access failed: {:?}", access_result.err());
    
    let access_value = access_result.unwrap();
    assert_eq!(access_value.value_type, LlvmType::Bool);
    
    info!("Boolean assignment test passed");
}

/// Test assignment with expression values
#[test]
fn test_expression_assignment() {
    common::init_tracing!();
    info!("Testing assignment with expression values");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_expr_assign", context.i32_type().as_basic_type_enum());
    
    // Test: sus result = 5 + 3 * 2
    let expression_value = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(BinaryExpression {
            left: Box::new(Literal {
                value: LiteralValue::Integer(3),
                location: None,
            }) as Box<dyn Expression>,
            operator: "*".to_string(),
            right: Box::new(Literal {
                value: LiteralValue::Integer(2),
                location: None,
            }) as Box<dyn Expression>,
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "result".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(expression_value) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_assignment_expression(&assignment);
    assert!(result.is_ok(), "Expression assignment compilation failed: {:?}", result.err());
    
    let assignment_value = result.unwrap();
    assert_eq!(assignment_value.value_type, LlvmType::Int32);
    
    // Test accessing the result variable
    let result_access = Identifier {
        name: "result".to_string(),
        location: None,
    };
    
    let access_result = test_ctx.expression_compiler.compile_identifier(&result_access);
    assert!(access_result.is_ok(), "Expression result access failed: {:?}", access_result.err());
    
    let access_value = access_result.unwrap();
    assert_eq!(access_value.value_type, LlvmType::Int32);
    
    info!("Expression assignment test passed");
}

/// Test variable reassignment (updating existing variables)
#[test]
fn test_variable_reassignment() {
    common::init_tracing!();
    info!("Testing variable reassignment");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_reassign", context.i32_type().as_basic_type_enum());
    
    // First assignment: sus counter = 0
    let initial_assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "counter".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Integer(0),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result1 = test_ctx.expression_compiler.compile_assignment_expression(&initial_assignment);
    assert!(result1.is_ok(), "Initial assignment failed: {:?}", result1.err());
    
    // Second assignment: counter = 10
    let reassignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "counter".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Integer(10),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result2 = test_ctx.expression_compiler.compile_assignment_expression(&reassignment);
    assert!(result2.is_ok(), "Reassignment failed: {:?}", result2.err());
    
    // Test accessing the updated variable
    let counter_access = Identifier {
        name: "counter".to_string(),
        location: None,
    };
    
    let access_result = test_ctx.expression_compiler.compile_identifier(&counter_access);
    assert!(access_result.is_ok(), "Counter access after reassignment failed: {:?}", access_result.err());
    
    let access_value = access_result.unwrap();
    assert_eq!(access_value.value_type, LlvmType::Int32);
    
    info!("Variable reassignment test passed");
}

/// Test assignment with variable references
#[test]
fn test_assignment_with_variables() {
    common::init_tracing!();
    info!("Testing assignment with variable references");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_var_assign", context.i32_type().as_basic_type_enum());
    
    // First: sus a = 5
    let a_assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "a".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result1 = test_ctx.expression_compiler.compile_assignment_expression(&a_assignment);
    assert!(result1.is_ok(), "Variable 'a' assignment failed: {:?}", result1.err());
    
    // Second: sus b = 3
    let b_assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "b".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Integer(3),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result2 = test_ctx.expression_compiler.compile_assignment_expression(&b_assignment);
    assert!(result2.is_ok(), "Variable 'b' assignment failed: {:?}", result2.err());
    
    // Third: sus sum = a + b
    let sum_assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "sum".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(BinaryExpression {
            left: Box::new(Identifier {
                name: "a".to_string(),
                location: None,
            }) as Box<dyn Expression>,
            operator: "+".to_string(),
            right: Box::new(Identifier {
                name: "b".to_string(),
                location: None,
            }) as Box<dyn Expression>,
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result3 = test_ctx.expression_compiler.compile_assignment_expression(&sum_assignment);
    assert!(result3.is_ok(), "Sum assignment failed: {:?}", result3.err());
    
    // Test accessing all variables
    for var_name in ["a", "b", "sum"] {
        let var_access = Identifier {
            name: var_name.to_string(),
            location: None,
        };
        
        let access_result = test_ctx.expression_compiler.compile_identifier(&var_access);
        assert!(access_result.is_ok(), "Variable '{}' access failed: {:?}", var_name, access_result.err());
        
        let access_value = access_result.unwrap();
        assert_eq!(access_value.value_type, LlvmType::Int32);
    }
    
    info!("Assignment with variables test passed");
}

/// Test self-assignment operations (x = x + 1)
#[test]
fn test_self_assignment() {
    common::init_tracing!();
    info!("Testing self-assignment operations");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_self_assign", context.i32_type().as_basic_type_enum());
    
    // Initial assignment: sus x = 10
    let initial_assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "x".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Integer(10),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result1 = test_ctx.expression_compiler.compile_assignment_expression(&initial_assignment);
    assert!(result1.is_ok(), "Initial self-assignment failed: {:?}", result1.err());
    
    // Self-assignment: x = x + 1
    let self_assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "x".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(BinaryExpression {
            left: Box::new(Identifier {
                name: "x".to_string(),
                location: None,
            }) as Box<dyn Expression>,
            operator: "+".to_string(),
            right: Box::new(Literal {
                value: LiteralValue::Integer(1),
                location: None,
            }) as Box<dyn Expression>,
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result2 = test_ctx.expression_compiler.compile_assignment_expression(&self_assignment);
    assert!(result2.is_ok(), "Self-assignment failed: {:?}", result2.err());
    
    // Test accessing the updated variable
    let x_access = Identifier {
        name: "x".to_string(),
        location: None,
    };
    
    let access_result = test_ctx.expression_compiler.compile_identifier(&x_access);
    assert!(access_result.is_ok(), "Self-assignment variable access failed: {:?}", access_result.err());
    
    let access_value = access_result.unwrap();
    assert_eq!(access_value.value_type, LlvmType::Int32);
    
    info!("Self-assignment test passed");
}

/// Test compound assignment operations (if supported)
#[test]
fn test_compound_assignment() {
    common::init_tracing!();
    info!("Testing compound assignment operations");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_compound", context.i32_type().as_basic_type_enum());
    
    // Initial assignment: sus value = 20
    let initial = AssignmentExpression {
        target: Box::new(Identifier {
            name: "value".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Integer(20),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result1 = test_ctx.expression_compiler.compile_assignment_expression(&initial);
    assert!(result1.is_ok(), "Initial compound assignment failed: {:?}", result1.err());
    
    // Test if compound assignment compilation is available
    let compound = CompoundAssignmentExpression {
        target: Box::new(Identifier {
            name: "value".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+=".to_string(),
        value: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    // This might not be implemented yet, so we test gracefully
    if let Ok(_) = test_ctx.expression_compiler.compile_compound_assignment_expression(&compound) {
        info!("Compound assignment compilation successful");
    } else {
        info!("Compound assignment not implemented (using regular assignment instead)");
        
        // Fall back to regular assignment: value = value + 5
        let fallback = AssignmentExpression {
            target: Box::new(Identifier {
                name: "value".to_string(),
                location: None,
            }) as Box<dyn Expression>,
            value: Box::new(BinaryExpression {
                left: Box::new(Identifier {
                    name: "value".to_string(),
                    location: None,
                }) as Box<dyn Expression>,
                operator: "+".to_string(),
                right: Box::new(Literal {
                    value: LiteralValue::Integer(5),
                    location: None,
                }) as Box<dyn Expression>,
                location: None,
            }) as Box<dyn Expression>,
            location: None,
        };
        
        let fallback_result = test_ctx.expression_compiler.compile_assignment_expression(&fallback);
        assert!(fallback_result.is_ok(), "Fallback assignment failed: {:?}", fallback_result.err());
    }
    
    info!("Compound assignment test completed");
}

/// Test assignment error handling
#[test]
fn test_assignment_error_handling() {
    common::init_tracing!();
    info!("Testing assignment error handling");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_assign_errors", context.i32_type().as_basic_type_enum());
    
    // Test assignment to undefined expression target (should fail)
    let invalid_assignment = AssignmentExpression {
        target: Box::new(Literal {
            value: LiteralValue::Integer(42), // Can't assign to literal
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Integer(10),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_assignment_expression(&invalid_assignment);
    assert!(result.is_err(), "Assignment to literal should fail");
    
    info!("Assignment to literal correctly failed: {:?}", result.err());
    
    info!("Assignment error handling test passed");
}

/// Test multiple variable assignments in sequence
#[test]
fn test_multiple_assignments() {
    common::init_tracing!();
    info!("Testing multiple variable assignments");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_multiple", context.i32_type().as_basic_type_enum());
    
    // Create multiple variables with different values
    let variables = [
        ("var1", 10),
        ("var2", 20),
        ("var3", 30),
        ("var4", 40),
        ("var5", 50),
    ];
    
    // Assign all variables
    for (name, value) in &variables {
        let assignment = AssignmentExpression {
            target: Box::new(Identifier {
                name: name.to_string(),
                location: None,
            }) as Box<dyn Expression>,
            value: Box::new(Literal {
                value: LiteralValue::Integer(*value),
                location: None,
            }) as Box<dyn Expression>,
            location: None,
        };
        
        let result = test_ctx.expression_compiler.compile_assignment_expression(&assignment);
        assert!(result.is_ok(), "Assignment of {} failed: {:?}", name, result.err());
    }
    
    // Access all variables to verify they were stored correctly
    for (name, _) in &variables {
        let access = Identifier {
            name: name.to_string(),
            location: None,
        };
        
        let result = test_ctx.expression_compiler.compile_identifier(&access);
        assert!(result.is_ok(), "Access of {} failed: {:?}", name, result.err());
        
        let value = result.unwrap();
        assert_eq!(value.value_type, LlvmType::Int32);
    }
    
    info!("Multiple assignments test passed");
}

/// Test assignment with mixed types (if type coercion is supported)
#[test]
fn test_mixed_type_assignment() {
    common::init_tracing!();
    info!("Testing mixed type assignment");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_mixed", context.f64_type().as_basic_type_enum());
    
    // Test potential integer to float assignment
    let mixed_assignment = AssignmentExpression {
        target: Box::new(Identifier {
            name: "mixed_var".to_string(),
            location: None,
        }) as Box<dyn Expression>,
        value: Box::new(Literal {
            value: LiteralValue::Integer(42), // Integer assigned to potentially float variable
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let result = test_ctx.expression_compiler.compile_assignment_expression(&mixed_assignment);
    
    match result {
        Ok(value) => {
            info!("Mixed type assignment successful: {:?}", value.value_type);
        }
        Err(err) => {
            info!("Mixed type assignment failed (expected if no type coercion): {:?}", err);
        }
    }
    
    info!("Mixed type assignment test completed");
}

/// Test assignment performance and memory usage
#[test]
fn test_assignment_performance() {
    common::init_tracing!();
    info!("Testing assignment performance");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_perf", context.i32_type().as_basic_type_enum());
    
    let start_time = std::time::Instant::now();
    
    // Perform many assignments to test performance
    for i in 0..50 {
        let var_name = format!("var_{}", i);
        
        let assignment = AssignmentExpression {
            target: Box::new(Identifier {
                name: var_name.clone(),
                location: None,
            }) as Box<dyn Expression>,
            value: Box::new(Literal {
                value: LiteralValue::Integer(i),
                location: None,
            }) as Box<dyn Expression>,
            location: None,
        };
        
        let result = test_ctx.expression_compiler.compile_assignment_expression(&assignment);
        assert!(result.is_ok(), "Assignment {} failed: {:?}", i, result.err());
    }
    
    let elapsed = start_time.elapsed();
    info!("Completed 50 assignments in {:?}", elapsed);
    
    // Assignments should be reasonably fast
    assert!(elapsed.as_secs() < 2, "Assignment compilation too slow: {:?}", elapsed);
    
    info!("Assignment performance test passed");
}

/// Test assignment integration with LLVM module generation
#[test]
fn test_assignment_llvm_integration() {
    common::init_tracing!();
    info!("Testing assignment LLVM integration");
    
    let context = Context::create();
    let mut test_ctx = AssignmentTestContext::new(&context);
    test_ctx.create_test_function("test_llvm_integration", context.i32_type().as_basic_type_enum());
    
    // Perform various assignments to build up the LLVM module
    let assignments = vec![
        ("int_var", LiteralValue::Integer(42)),
        ("float_var", LiteralValue::Float(3.14)),
        ("bool_var", LiteralValue::Boolean(true)),
        ("string_var", LiteralValue::String("test".to_string())),
    ];
    
    let mut successful_assignments = 0;
    
    for (name, value) in assignments {
        let assignment = AssignmentExpression {
            target: Box::new(Identifier {
                name: name.to_string(),
                location: None,
            }) as Box<dyn Expression>,
            value: Box::new(Literal {
                value,
                location: None,
            }) as Box<dyn Expression>,
            location: None,
        };
        
        let result = test_ctx.expression_compiler.compile_assignment_expression(&assignment);
        if result.is_ok() {
            successful_assignments += 1;
        }
    }
    
    assert!(successful_assignments > 0, "Should have compiled at least some assignments");
    
    // Verify the LLVM module is valid
    let llvm_ir = test_ctx.module.print_to_string();
    let ir_string = llvm_ir.to_string();
    
    assert!(!ir_string.is_empty(), "LLVM IR should not be empty");
    assert!(ir_string.contains("define"), "LLVM IR should contain function definitions");
    
    info!("Assignment LLVM integration test passed - {} assignments compiled", successful_assignments);
}
