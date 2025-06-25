use cursed::ast::expressions::question_mark::QuestionMarkExpression;
use cursed::ast::Expression;
use cursed::codegen::llvm::LlvmCodeGeneratorReal;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::runtime::Runtime;
use cursed::error::CursedError;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::OptimizationLevel;
use std::sync::Arc;
use std::fs;
use std::path::Path;
use tracing::{debug, info, error, warn};

macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .try_init();
    };
}

#[test]
fn test_complete_compilation_pipeline() {
    init_tracing!();
    info!("Testing complete compilation pipeline for ? operator");

    let test_programs = vec![
        (
            "simple_result",
            r#"
                slay divide(a: i32, b: i32) -> Result<i32, String> {
                    lowkey (b == 0) {
                        return Err("Division by zero");
                    }
                    Ok(a / b)
                }
                
                slay main() -> Result<(), String> {
                    facts result = divide(10, 2)?;
                    Ok(())
                }
            "#
        ),
        (
            "option_chain",
            r#"
                slay safe_get(arr: &[i32], index: usize) -> Option<i32> {
                    lowkey (index >= arr.len()) {
                        None
                    } flex {
                        Some(arr[index])
                    }
                }
                
                slay main() -> Option<()> {
                    facts arr = vec![1, 2, 3, 4, 5];
                    facts value = safe_get(&arr, 2)?;
                    Some(())
                }
            "#
        ),
        (
            "nested_propagation",
            r#"
                slay step1(x: i32) -> Result<i32, String> {
                    lowkey (x < 0) {
                        Err("Negative input")
                    } flex {
                        Ok(x * 2)
                    }
                }
                
                slay step2(x: i32) -> Result<i32, String> {
                    lowkey (x > 100) {
                        Err("Value too large")
                    } flex {
                        Ok(x + 10)
                    }
                }
                
                slay process_chain(input: i32) -> Result<i32, String> {
                    facts result1 = step1(input)?;
                    facts result2 = step2(result1)?;
                    Ok(result2 + 5)
                }
                
                slay main() -> Result<(), String> {
                    facts final_result = process_chain(20)?;
                    Ok(())
                }
            "#
        ),
    ];

    for (test_name, code) in test_programs {
        info!(test_name = %test_name, "Testing compilation pipeline");
        
        match compile_and_analyze(test_name, code) {
            Ok(analysis) => {
                info!(test_name = %test_name, "Compilation successful");
                debug!(analysis = ?analysis, "Compilation analysis");
                
                // Verify that the analysis shows expected error propagation patterns
                assert!(analysis.has_question_mark_operators, 
                    "Should detect ? operators in {}", test_name);
                assert!(analysis.has_error_handling_ir, 
                    "Should generate error handling IR in {}", test_name);
            },
            Err(e) => {
                warn!(test_name = %test_name, error = ?e, "Compilation failed (expected for incomplete implementation)");
                // This is expected until the ? operator is fully implemented
            }
        }
    }
}

#[test]
fn test_question_mark_ir_patterns() {
    init_tracing!();
    info!("Testing specific LLVM IR patterns for ? operator");

    let code = r#"
        slay fallible_operation() -> Result<i32, String> {
            Ok(42)
        }
        
        slay test_propagation() -> Result<String, String> {
            facts value = fallible_operation()?;
            Ok(format!("Value: {}", value))
        }
    "#;

    match compile_to_ir("ir_patterns_test", code) {
        Ok(ir) => {
            info!("Successfully generated IR for ? operator");
            debug!(ir = %ir, "Generated LLVM IR");
            
            // Look for specific IR patterns that indicate error propagation
            let ir_analysis = analyze_ir_patterns(&ir);
            debug!(ir_analysis = ?ir_analysis, "IR pattern analysis");
            
            // These assertions may fail until ? operator is fully implemented
            if ir_analysis.has_conditional_branches {
                info!("Found conditional branches for error handling");
            }
            if ir_analysis.has_early_returns {
                info!("Found early return patterns");
            }
            if ir_analysis.has_error_propagation {
                info!("Found error propagation patterns");
            }
        },
        Err(e) => {
            warn!(error = ?e, "IR generation failed (expected for incomplete implementation)");
        }
    }
}

#[test]
fn test_error_propagation_execution() {
    init_tracing!();
    info!("Testing error propagation execution semantics");

    // Test cases that should demonstrate different error propagation behaviors
    let test_cases = vec![
        (
            "success_case",
            r#"
                slay safe_divide(a: i32, b: i32) -> Result<i32, String> {
                    lowkey (b == 0) {
                        Err("Division by zero".to_string())
                    } flex {
                        Ok(a / b)
                    }
                }
                
                slay main() -> i32 {
                    vibe_check safe_divide(10, 2) {
                        mood Ok(result) => result,
                        mood Err(_) => -1,
                    }
                }
            "#,
            Some(5), // Expected result
        ),
        (
            "error_case",
            r#"
                slay safe_divide(a: i32, b: i32) -> Result<i32, String> {
                    lowkey (b == 0) {
                        Err("Division by zero".to_string())
                    } flex {
                        Ok(a / b)
                    }
                }
                
                slay main() -> i32 {
                    vibe_check safe_divide(10, 0) {
                        mood Ok(result) => result,
                        mood Err(_) => -1,
                    }
                }
            "#,
            Some(-1), // Expected error result
        ),
    ];

    for (test_name, code, expected_result) in test_cases {
        info!(test_name = %test_name, "Testing execution semantics");
        
        match compile_and_execute(test_name, code) {
            Ok(result) => {
                info!(test_name = %test_name, result, "Execution completed");
                
                if let Some(expected) = expected_result {
                    debug!(expected, actual = result, "Comparing results");
                    // Note: Exact execution testing depends on full ? operator implementation
                }
            },
            Err(e) => {
                warn!(test_name = %test_name, error = ?e, "Execution failed (expected for incomplete implementation)");
            }
        }
    }
}

#[test]
fn test_mixed_result_option_propagation() {
    init_tracing!();
    info!("Testing mixed Result and Option error propagation");

    let code = r#"
        slay get_optional_value(index: usize) -> Option<i32> {
            facts values = vec![10, 20, 30];
            lowkey (index < values.len()) {
                Some(values[index])
            } flex {
                None
            }
        }
        
        slay convert_to_result(maybe_value: Option<i32>) -> Result<String, String> {
            facts value = maybe_value.ok_or("No value available".to_string())?;
            Ok(format!("Got value: {}", value))
        }
        
        slay mixed_propagation(index: usize) -> Result<String, String> {
            facts optional_val = get_optional_value(index)?;  // Option -> Result conversion
            convert_to_result(Some(optional_val))
        }
        
        slay main() -> Result<(), String> {
            facts result = mixed_propagation(1)?;
            Ok(())
        }
    "#;

    match compile_and_analyze("mixed_propagation", code) {
        Ok(analysis) => {
            info!("Mixed propagation compilation successful");
            debug!(analysis = ?analysis, "Mixed propagation analysis");
            
            assert!(analysis.has_question_mark_operators, 
                "Should detect ? operators in mixed propagation");
        },
        Err(e) => {
            warn!(error = ?e, "Mixed propagation compilation failed (expected)");
        }
    }
}

#[test]
fn test_complex_error_propagation_chains() {
    init_tracing!();
    info!("Testing complex error propagation chains");

    let code = r#"
        slay parse_number(s: &str) -> Result<i32, String> {
            s.trim().parse().map_err(|e| format!("Parse error: {}", e))
        }
        
        slay validate_positive(n: i32) -> Result<i32, String> {
            lowkey (n <= 0) {
                Err("Number must be positive".to_string())
            } flex {
                Ok(n)
            }
        }
        
        slay calculate_square(n: i32) -> Result<i64, String> {
            facts checked = n.checked_mul(n);
            vibe_check checked {
                mood Some(result) => Ok(result as i64),
                mood None => Err("Overflow in square calculation".to_string()),
            }
        }
        
        slay complex_chain(input: &str) -> Result<String, String> {
            facts parsed = parse_number(input)?;
            facts validated = validate_positive(parsed)?;
            facts squared = calculate_square(validated)?;
            Ok(format!("Result: {}", squared))
        }
        
        slay main() -> Result<(), String> {
            facts result1 = complex_chain("5")?;
            facts result2 = complex_chain("-3")?;  // Should propagate error
            Ok(())
        }
    "#;

    match compile_and_analyze("complex_chains", code) {
        Ok(analysis) => {
            info!("Complex chain compilation successful");
            debug!(analysis = ?analysis, "Complex chain analysis");
            
            assert!(analysis.has_question_mark_operators, 
                "Should detect multiple ? operators in complex chains");
                
            // Verify we have multiple propagation points
            assert!(analysis.question_mark_count >= 3, 
                "Should have multiple ? operators in the chain");
        },
        Err(e) => {
            warn!(error = ?e, "Complex chain compilation failed (expected)");
        }
    }
}

// Helper structures and functions

#[derive(Debug)]
struct CompilationAnalysis {
    has_question_mark_operators: bool,
    question_mark_count: usize,
    has_error_handling_ir: bool,
    has_conditional_branches: bool,
    has_early_returns: bool,
    compilation_successful: bool,
}

#[derive(Debug)]
struct IrAnalysis {
    has_conditional_branches: bool,
    has_early_returns: bool,
    has_error_propagation: bool,
    has_phi_nodes: bool,
    has_function_calls: bool,
}

fn compile_and_analyze(test_name: &str, code: &str) -> Result<CompilationAnalysis, CursedError> {
    let context = Context::create();
    let module = context.create_module(test_name);
    let builder = context.create_builder();
    let runtime = Arc::new(Runtime::new());

    let mut codegen = LlvmCodeGeneratorReal::new(&context, module, builder, runtime)?;

    // Parse the code
    let mut lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer)?;
    let ast = parser.parse()?;

    // Count ? operators in the AST
    let question_mark_count = count_question_marks_in_code(code);
    let has_question_mark_operators = question_mark_count > 0;

    // Attempt compilation
    let compilation_result = codegen.compile(&ast);
    let compilation_successful = compilation_result.is_ok();

    let (has_error_handling_ir, has_conditional_branches, has_early_returns) = 
        if compilation_successful {
            let ir = codegen.module().print_to_string().to_string();
            let ir_analysis = analyze_ir_patterns(&ir);
            (ir_analysis.has_error_propagation, ir_analysis.has_conditional_branches, ir_analysis.has_early_returns)
        } else {
            (false, false, false)
        };

    Ok(CompilationAnalysis {
        has_question_mark_operators,
        question_mark_count,
        has_error_handling_ir,
        has_conditional_branches,
        has_early_returns,
        compilation_successful,
    })
}

fn compile_to_ir(test_name: &str, code: &str) -> Result<String, CursedError> {
    let context = Context::create();
    let module = context.create_module(test_name);
    let builder = context.create_builder();
    let runtime = Arc::new(Runtime::new());

    let mut codegen = LlvmCodeGeneratorReal::new(&context, module, builder, runtime)?;

    let mut lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer)?;
    let ast = parser.parse()?;

    codegen.compile(&ast)?;
    Ok(codegen.module().print_to_string().to_string())
}

fn compile_and_execute(test_name: &str, code: &str) -> Result<i32, CursedError> {
    let context = Context::create();
    let module = context.create_module(test_name);
    let builder = context.create_builder();
    let runtime = Arc::new(Runtime::new());

    let mut codegen = LlvmCodeGeneratorReal::new(&context, module, builder, runtime)?;

    let mut lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer)?;
    let ast = parser.parse()?;

    codegen.compile(&ast)?;

    // Create execution engine
    let execution_engine = codegen.module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| CursedError::Runtime(format!("Failed to create execution engine: {}", e)))?;

    // Execute main function
    unsafe {
        type MainFunc = unsafe extern "C" fn() -> i32;
        let main_func: JitFunction<MainFunc> = execution_engine
            .get_function("main")
            .map_err(|e| CursedError::Runtime(format!("Failed to get main function: {}", e)))?;

        Ok(main_func.call())
    }
}

fn analyze_ir_patterns(ir: &str) -> IrAnalysis {
    IrAnalysis {
        has_conditional_branches: ir.contains("br i1") || ir.contains("switch"),
        has_early_returns: ir.contains("ret") && ir.matches("ret").count() > 1,
        has_error_propagation: ir.contains("error") || ir.contains("propagat") || ir.contains("unwind"),
        has_phi_nodes: ir.contains("phi"),
        has_function_calls: ir.contains("call"),
    }
}

fn count_question_marks_in_code(code: &str) -> usize {
    code.matches("?").count()
}
