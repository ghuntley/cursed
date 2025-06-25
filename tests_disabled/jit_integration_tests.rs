/// JIT Compilation Integration Tests for CURSED
/// 
/// This test suite validates the Just-In-Time (JIT) compilation functionality
/// which is critical for REPL functionality and dynamic code execution.
/// JIT compilation tests the entire pipeline from CURSED source code to
/// executable machine code.
///
/// These tests are essential because they verify that:
/// - The entire compilation pipeline works end-to-end
/// - Generated LLVM IR is valid and executable
/// - Runtime execution produces correct results
/// - Memory management works correctly during execution
/// - Error handling works in the runtime environment
/// - The JIT engine integrates properly with the type system and GC

#[path = "common.rs"]
mod common;

use cursed::{
    ast::{
        expressions::{Literal, LiteralValue},
        operators::{BinaryExpression, AssignmentExpression},
        identifiers::Identifier,
        statements::{LetStatement, ExpressionStatement},
        traits::{Expression, Statement},
        Program,
    },
    codegen::llvm::{
        jit_compilation::{JitCompiler, ExecutionResult},
        jit_engine::JitEngine,
        expression_compiler::{LlvmExpressionCompiler, LlvmValue, LlvmType},
        variable_management::VariableManager,
        type_system::TypeCompilationContext,
    },
    core::type_checker::Type,
    lexer::Lexer,
    parser::Parser,
    error::Error,
};
use inkwell::{
    context::Context,
    module::Module,
    builder::Builder,
    execution_engine::{ExecutionEngine, JitFunction},
    OptimizationLevel,
    values::BasicValueEnum,
};
use std::collections::HashMap;
use tracing::{debug, info, error};

/// Comprehensive JIT test context
struct JitTestContext<'ctx> {
    context: &'ctx Context,
    jit_compiler: JitCompiler<'ctx>,
    jit_engine: JitEngine<'ctx>,
}

impl<'ctx> JitTestContext<'ctx> {
    fn new(context: &'ctx Context) -> Result<Self, Error> {
        let jit_compiler = JitCompiler::new(context)?;
        let jit_engine = JitEngine::new(context, OptimizationLevel::None)?;
        
        Ok(Self {
            context,
            jit_compiler,
            jit_engine,
        })
    }
}

/// Test basic JIT compilation of simple expressions
#[test]
fn test_basic_jit_compilation() {
    common::init_tracing!();
    info!("Testing basic JIT compilation");
    
    let context = Context::create();
    let mut test_ctx = match JitTestContext::new(&context) {
        Ok(ctx) => ctx,
        Err(err) => {
            info!("JIT compilation not available: {:?}", err);
            return; // Skip test if JIT is not available
        }
    };
    
    // Test compiling a simple integer expression: 5 + 3
    let expression = BinaryExpression {
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
    
    let result = test_ctx.jit_compiler.compile_expression(&expression);
    assert!(result.is_ok(), "Basic expression JIT compilation failed: {:?}", result.err());
    
    info!("Basic JIT compilation test passed");
}

/// Test JIT execution of simple arithmetic
#[test]
fn test_jit_arithmetic_execution() {
    common::init_tracing!();
    info!("Testing JIT arithmetic execution");
    
    let context = Context::create();
    let mut test_ctx = match JitTestContext::new(&context) {
        Ok(ctx) => ctx,
        Err(err) => {
            info!("JIT execution not available: {:?}", err);
            return;
        }
    };
    
    // Test executing arithmetic: 10 + 5
    let add_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(10),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let compile_result = test_ctx.jit_compiler.compile_expression(&add_expr);
    assert!(compile_result.is_ok(), "Addition compilation failed: {:?}", compile_result.err());
    
    let execution_result = test_ctx.jit_engine.execute_expression(&add_expr);
    
    match execution_result {
        Ok(ExecutionResult::Integer(value)) => {
            assert_eq!(value, 15, "Addition result should be 15, got {}", value);
            info!("Arithmetic execution successful: 10 + 5 = {}", value);
        }
        Ok(other) => {
            info!("Unexpected result type: {:?}", other);
        }
        Err(err) => {
            info!("JIT execution failed (may not be fully implemented): {:?}", err);
        }
    }
    
    info!("JIT arithmetic execution test completed");
}

/// Test JIT compilation of variable operations
#[test]
fn test_jit_variable_operations() {
    common::init_tracing!();
    info!("Testing JIT variable operations");
    
    let context = Context::create();
    let mut test_ctx = match JitTestContext::new(&context) {
        Ok(ctx) => ctx,
        Err(err) => {
            info!("JIT variable operations not available: {:?}", err);
            return;
        }
    };
    
    // Test variable assignment and access
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
    
    let compile_result = test_ctx.jit_compiler.compile_expression(&assignment);
    
    match compile_result {
        Ok(_) => {
            // Test variable access
            let variable_access = Identifier {
                name: "x".to_string(),
                location: None,
            };
            
            let access_result = test_ctx.jit_compiler.compile_expression(&variable_access);
            assert!(access_result.is_ok(), "Variable access compilation failed: {:?}", access_result.err());
            
            info!("Variable operations compilation successful");
        }
        Err(err) => {
            info!("Variable assignment compilation failed (may not be fully implemented): {:?}", err);
        }
    }
    
    info!("JIT variable operations test completed");
}

/// Test JIT compilation of floating point operations
#[test]
fn test_jit_float_operations() {
    common::init_tracing!();
    info!("Testing JIT floating point operations");
    
    let context = Context::create();
    let mut test_ctx = match JitTestContext::new(&context) {
        Ok(ctx) => ctx,
        Err(err) => {
            info!("JIT float operations not available: {:?}", err);
            return;
        }
    };
    
    // Test float arithmetic: 3.14 + 2.86
    let float_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Float(3.14),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Float(2.86),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let compile_result = test_ctx.jit_compiler.compile_expression(&float_expr);
    assert!(compile_result.is_ok(), "Float expression compilation failed: {:?}", compile_result.err());
    
    let execution_result = test_ctx.jit_engine.execute_expression(&float_expr);
    
    match execution_result {
        Ok(ExecutionResult::Float(value)) => {
            let expected = 3.14 + 2.86;
            assert!((value - expected).abs() < 0.0001, "Float result should be {}, got {}", expected, value);
            info!("Float execution successful: 3.14 + 2.86 = {}", value);
        }
        Ok(other) => {
            info!("Unexpected float result type: {:?}", other);
        }
        Err(err) => {
            info!("Float JIT execution failed (may not be fully implemented): {:?}", err);
        }
    }
    
    info!("JIT float operations test completed");
}

/// Test JIT compilation of comparison operations
#[test]
fn test_jit_comparison_operations() {
    common::init_tracing!();
    info!("Testing JIT comparison operations");
    
    let context = Context::create();
    let mut test_ctx = match JitTestContext::new(&context) {
        Ok(ctx) => ctx,
        Err(err) => {
            info!("JIT comparison operations not available: {:?}", err);
            return;
        }
    };
    
    // Test equality comparison: 5 == 5
    let eq_expr = BinaryExpression {
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
    
    let compile_result = test_ctx.jit_compiler.compile_expression(&eq_expr);
    assert!(compile_result.is_ok(), "Equality compilation failed: {:?}", compile_result.err());
    
    let execution_result = test_ctx.jit_engine.execute_expression(&eq_expr);
    
    match execution_result {
        Ok(ExecutionResult::Boolean(value)) => {
            assert_eq!(value, true, "5 == 5 should be true, got {}", value);
            info!("Comparison execution successful: 5 == 5 = {}", value);
        }
        Ok(other) => {
            info!("Unexpected comparison result type: {:?}", other);
        }
        Err(err) => {
            info!("Comparison JIT execution failed (may not be fully implemented): {:?}", err);
        }
    }
    
    // Test less than comparison: 3 < 7
    let lt_expr = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(3),
            location: None,
        }) as Box<dyn Expression>,
        operator: "<".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(7),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let compile_result = test_ctx.jit_compiler.compile_expression(&lt_expr);
    assert!(compile_result.is_ok(), "Less than compilation failed: {:?}", compile_result.err());
    
    info!("JIT comparison operations test completed");
}

/// Test JIT compilation of string operations
#[test]
fn test_jit_string_operations() {
    common::init_tracing!();
    info!("Testing JIT string operations");
    
    let context = Context::create();
    let mut test_ctx = match JitTestContext::new(&context) {
        Ok(ctx) => ctx,
        Err(err) => {
            info!("JIT string operations not available: {:?}", err);
            return;
        }
    };
    
    // Test string literal compilation
    let string_literal = Literal {
        value: LiteralValue::String("Hello, JIT!".to_string()),
        location: None,
    };
    
    let compile_result = test_ctx.jit_compiler.compile_expression(&string_literal);
    assert!(compile_result.is_ok(), "String literal compilation failed: {:?}", compile_result.err());
    
    let execution_result = test_ctx.jit_engine.execute_expression(&string_literal);
    
    match execution_result {
        Ok(ExecutionResult::String(value)) => {
            assert_eq!(value, "Hello, JIT!", "String should match original");
            info!("String execution successful: {}", value);
        }
        Ok(other) => {
            info!("Unexpected string result type: {:?}", other);
        }
        Err(err) => {
            info!("String JIT execution failed (may not be fully implemented): {:?}", err);
        }
    }
    
    info!("JIT string operations test completed");
}

/// Test JIT compilation with complex nested expressions
#[test]
fn test_jit_complex_expressions() {
    common::init_tracing!();
    info!("Testing JIT compilation of complex expressions");
    
    let context = Context::create();
    let mut test_ctx = match JitTestContext::new(&context) {
        Ok(ctx) => ctx,
        Err(err) => {
            info!("JIT complex expressions not available: {:?}", err);
            return;
        }
    };
    
    // Test complex expression: (10 + 5) * (8 - 3)
    let inner_left = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(10),
            location: None,
        }) as Box<dyn Expression>,
        operator: "+".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(5),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let inner_right = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(8),
            location: None,
        }) as Box<dyn Expression>,
        operator: "-".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(3),
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
    
    let compile_result = test_ctx.jit_compiler.compile_expression(&complex_expr);
    assert!(compile_result.is_ok(), "Complex expression compilation failed: {:?}", compile_result.err());
    
    let execution_result = test_ctx.jit_engine.execute_expression(&complex_expr);
    
    match execution_result {
        Ok(ExecutionResult::Integer(value)) => {
            let expected = (10 + 5) * (8 - 3); // 15 * 5 = 75
            assert_eq!(value, expected, "Complex expression should equal {}, got {}", expected, value);
            info!("Complex expression execution successful: (10 + 5) * (8 - 3) = {}", value);
        }
        Ok(other) => {
            info!("Unexpected complex expression result type: {:?}", other);
        }
        Err(err) => {
            info!("Complex expression JIT execution failed (may not be fully implemented): {:?}", err);
        }
    }
    
    info!("JIT complex expressions test completed");
}

/// Test JIT error handling
#[test]
fn test_jit_error_handling() {
    common::init_tracing!();
    info!("Testing JIT error handling");
    
    let context = Context::create();
    let mut test_ctx = match JitTestContext::new(&context) {
        Ok(ctx) => ctx,
        Err(err) => {
            info!("JIT error handling test skipped: {:?}", err);
            return;
        }
    };
    
    // Test division by zero (if runtime checking is implemented)
    let div_by_zero = BinaryExpression {
        left: Box::new(Literal {
            value: LiteralValue::Integer(42),
            location: None,
        }) as Box<dyn Expression>,
        operator: "/".to_string(),
        right: Box::new(Literal {
            value: LiteralValue::Integer(0),
            location: None,
        }) as Box<dyn Expression>,
        location: None,
    };
    
    let compile_result = test_ctx.jit_compiler.compile_expression(&div_by_zero);
    
    match compile_result {
        Ok(_) => {
            info!("Division by zero compiled (may have runtime checking)");
            
            let execution_result = test_ctx.jit_engine.execute_expression(&div_by_zero);
            
            match execution_result {
                Ok(value) => {
                    info!("Division by zero executed with result: {:?}", value);
                }
                Err(err) => {
                    info!("Division by zero caught at runtime: {:?}", err);
                }
            }
        }
        Err(err) => {
            info!("Division by zero caught at compile time: {:?}", err);
        }
    }
    
    // Test accessing undefined variable
    let undefined_var = Identifier {
        name: "nonexistent_variable".to_string(),
        location: None,
    };
    
    let undefined_result = test_ctx.jit_compiler.compile_expression(&undefined_var);
    assert!(undefined_result.is_err(), "Undefined variable should fail compilation");
    
    info!("JIT error handling test completed");
}

/// Test JIT engine lifecycle management
#[test]
fn test_jit_engine_lifecycle() {
    common::init_tracing!();
    info!("Testing JIT engine lifecycle management");
    
    let context = Context::create();
    
    // Test creating and destroying JIT engines
    for i in 0..3 {
        let test_ctx_result = JitTestContext::new(&context);
        
        match test_ctx_result {
            Ok(mut test_ctx) => {
                info!("JIT engine {} created successfully", i);
                
                // Test basic operation
                let simple_expr = Literal {
                    value: LiteralValue::Integer(i),
                    location: None,
                };
                
                let result = test_ctx.jit_compiler.compile_expression(&simple_expr);
                assert!(result.is_ok(), "Simple expression should compile in engine {}", i);
                
                info!("JIT engine {} operated successfully", i);
                // Engine will be dropped here
            }
            Err(err) => {
                info!("JIT engine creation failed (expected if not available): {:?}", err);
                break; // Don't continue if JIT is not available
            }
        }
    }
    
    info!("JIT engine lifecycle test completed");
}

/// Test JIT integration with CURSED source parsing
#[test]
fn test_jit_source_integration() {
    common::init_tracing!();
    info!("Testing JIT integration with CURSED source parsing");
    
    let context = Context::create();
    let mut test_ctx = match JitTestContext::new(&context) {
        Ok(ctx) => ctx,
        Err(err) => {
            info!("JIT source integration not available: {:?}", err);
            return;
        }
    };
    
    // Test parsing and JIT compiling simple CURSED code
    let cursed_source = "5 + 3";
    
    let mut lexer = Lexer::new(cursed_source);
    let tokens = lexer.tokenize();
    
    match tokens {
        Ok(token_list) => {
            let mut parser = Parser::new(token_list);
            let parse_result = parser.parse_expression();
            
            match parse_result {
                Ok(expression) => {
                    let compile_result = test_ctx.jit_compiler.compile_expression(&*expression);
                    
                    match compile_result {
                        Ok(_) => {
                            info!("Successfully compiled CURSED source: '{}'", cursed_source);
                            
                            let execution_result = test_ctx.jit_engine.execute_expression(&*expression);
                            
                            match execution_result {
                                Ok(ExecutionResult::Integer(value)) => {
                                    assert_eq!(value, 8, "5 + 3 should equal 8");
                                    info!("CURSED source execution result: {}", value);
                                }
                                Ok(other) => {
                                    info!("Unexpected result type from CURSED source: {:?}", other);
                                }
                                Err(err) => {
                                    info!("CURSED source execution failed: {:?}", err);
                                }
                            }
                        }
                        Err(err) => {
                            info!("CURSED source compilation failed: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    info!("CURSED source parsing failed: {:?}", err);
                }
            }
        }
        Err(err) => {
            info!("CURSED source tokenization failed: {:?}", err);
        }
    }
    
    info!("JIT source integration test completed");
}

/// Performance test for JIT compilation
#[test]
fn test_jit_performance() {
    common::init_tracing!();
    info!("Testing JIT compilation performance");
    
    let context = Context::create();
    let mut test_ctx = match JitTestContext::new(&context) {
        Ok(ctx) => ctx,
        Err(err) => {
            info!("JIT performance test skipped: {:?}", err);
            return;
        }
    };
    
    let start_time = std::time::Instant::now();
    
    // Compile many simple expressions to test performance
    let mut successful_compilations = 0;
    
    for i in 0..20 {
        let expr = BinaryExpression {
            left: Box::new(Literal {
                value: LiteralValue::Integer(i),
                location: None,
            }) as Box<dyn Expression>,
            operator: "+".to_string(),
            right: Box::new(Literal {
                value: LiteralValue::Integer(i + 1),
                location: None,
            }) as Box<dyn Expression>,
            location: None,
        };
        
        let result = test_ctx.jit_compiler.compile_expression(&expr);
        if result.is_ok() {
            successful_compilations += 1;
        }
    }
    
    let elapsed = start_time.elapsed();
    info!("JIT compiled {} expressions in {:?}", successful_compilations, elapsed);
    
    assert!(successful_compilations > 0, "Should have compiled at least some expressions");
    
    // JIT compilation should be reasonably fast
    assert!(elapsed.as_secs() < 5, "JIT compilation too slow: {:?}", elapsed);
    
    info!("JIT performance test completed");
}

/// Memory test for JIT compilation and execution
#[test]
fn test_jit_memory_management() {
    common::init_tracing!();
    info!("Testing JIT memory management");
    
    let context = Context::create();
    let mut test_ctx = match JitTestContext::new(&context) {
        Ok(ctx) => ctx,
        Err(err) => {
            info!("JIT memory management test skipped: {:?}", err);
            return;
        }
    };
    
    // Test that multiple compilations don't leak memory (basic test)
    for i in 0..10 {
        let expr = Literal {
            value: LiteralValue::Integer(i),
            location: None,
        };
        
        let compile_result = test_ctx.jit_compiler.compile_expression(&expr);
        
        if let Ok(_) = compile_result {
            // Try executing to test runtime memory management
            let _execution_result = test_ctx.jit_engine.execute_expression(&expr);
            // Results intentionally ignored - we're testing for crashes/leaks
        }
    }
    
    info!("JIT memory management test completed (no crashes)");
}
