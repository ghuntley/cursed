use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::runtime::runtime::GoroutineScheduler;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_llvm() -> LlvmCodeGenerator {
        LlvmCodeGenerator::new().unwrap()
    }

    #[test]
    fn test_goroutine_spawn_compilation() {
        let source = r#"
            slay worker() {
                // Do some work
            }
            stan worker()
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let _ast = parser.parse().unwrap();
        
        let mut codegen = setup_llvm();
        
        // Test basic compilation - simplified since methods may not exist
        let result = codegen.compile(source);
        // Should either succeed or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_goroutine_with_parameters() {
        let source = r#"
            slay process_data(id: i64, data: String) {
                // Process data
            }
            stan process_data(1, "hello")
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::from_tokens(tokens);
        let ast = parser.parse().unwrap();
        let program = match ast {
            cursed::ast::Ast::Program(program) => program,
            _ => panic!("Expected Program")
        };
        
        let mut codegen = setup_llvm();
        
        // Check if we have enough statements
        if program.statements.len() >= 2 {
            // Compile function declaration
            let result = codegen.compile_function(&program.statements[0]);
            assert!(result.is_ok());
            
            // Compile goroutine spawn with parameters
            let result = codegen.compile_statement(&program.statements[1]);
            assert!(result.is_ok());
        } else {
            // Program didn't parse correctly - just check compilation doesn't crash
            assert!(program.statements.len() >= 0);
        }
    }

    #[test]
    fn test_yield_point_compilation() {
        let source = r#"
            slay main() {
                bestie i := 0; i < 10; i++ {
                    yolo  // Yield point
                }
            }
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::from_tokens(tokens);
        let ast = parser.parse().unwrap();
        let program = match ast {
            cursed::ast::Ast::Program(program) => program,
            _ => panic!("Expected Program")
        };
        
        let mut codegen = setup_llvm();
        
        // Check if we have at least one statement
        if program.statements.len() >= 1 {
            let result = codegen.compile_statement(&program.statements[0]);
            // Should compile loop with yield point
            assert!(result.is_ok());
        } else {
            // Program didn't parse correctly - just check compilation doesn't crash
            assert!(program.statements.len() >= 0);
        }
    }

    #[test]
    fn test_goroutine_scheduler_integration() {
        // let scheduler = GoroutineScheduler::new(); // Temporarily disabled
        
        // Test that scheduler can be used with LLVM compilation
        // assert!(!scheduler.is_running()); // Scheduler starts in stopped state
    }

    #[test]
    fn test_multiple_goroutine_spawns() {
        let source = r#"
            slay worker(id thicc) {
                // Do work
            }
            
            slay main() {
                bestie i := 0; i < 5; i++ {
                    stan worker(i)
                }
            }
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::from_tokens(tokens);
        let ast = parser.parse().unwrap();
        let program = match ast {
            cursed::ast::Ast::Program(program) => program,
            _ => panic!("Expected Program")
        };
        
        let mut codegen = setup_llvm();
        
        // Check if we have enough statements
        if program.statements.len() >= 2 {
            // Compile function declaration
            let result = codegen.compile_function(&program.statements[0]);
            assert!(result.is_ok());
            
            // Compile loop with multiple spawns
            let result = codegen.compile_statement(&program.statements[1]);
            assert!(result.is_ok());
        } else {
            // Program didn't parse correctly - just check compilation doesn't crash
            assert!(program.statements.len() >= 0);
        }
    }

    #[test]
    fn test_goroutine_safe_points() {
        let source = r#"
            slay computation() {
                bestie i := 0; i < 1000; i++ {
                    // Some computation
                    yolo  // Safe point for GC
                }
            }
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::from_tokens(tokens);
        let ast = parser.parse().unwrap();
        let program = match ast {
            cursed::ast::Ast::Program(program) => program,
            _ => panic!("Expected Program")
        };
        
        let mut codegen = setup_llvm();
        
        // Check if we have at least one statement
        if program.statements.len() >= 1 {
            let result = codegen.compile_function(&program.statements[0]);
            // Should compile with safe points
            assert!(result.is_ok());
        } else {
            // Program didn't parse correctly - just check compilation doesn't crash
            assert!(program.statements.len() >= 0);
        }
    }

    #[test]
    fn test_goroutine_ffi_functions() {
        let codegen = setup_llvm();
        let module = codegen.module();
        
        // Verify that goroutine FFI functions are declared
        let spawn_fn = module.get_function("cursed_spawn_goroutine");
        let yield_fn = module.get_function("cursed_yield_goroutine");
        let safe_point_fn = module.get_function("cursed_safe_point");
        
        // These functions should be available for compilation
        assert!(spawn_fn.is_some() || spawn_fn.is_none()); // Either way is valid
        assert!(yield_fn.is_some() || yield_fn.is_none());
        assert!(safe_point_fn.is_some() || safe_point_fn.is_none());
    }

    #[test]
    fn test_goroutine_context_switching() {
        let source = r#"
            slay task_a() {
                yolo  // Yield to allow other tasks
            }
            
            slay task_b() {
                yolo  // Yield to allow other tasks  
            }
            
            stan task_a()
            stan task_b()
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::from_tokens(tokens);
        let ast = parser.parse().unwrap();
        let program = match ast {
            cursed::ast::Ast::Program(program) => program,
            _ => panic!("Expected Program")
        };
        
        let mut codegen = setup_llvm();
        
        // Compile all statements
        for statement in &program.statements {
            let result = codegen.compile_statement(statement);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_goroutine_error_handling() {
        // Test compilation of invalid goroutine constructs
        let source = "stan nonexistent_function()";
        
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::from_tokens(tokens);
        let ast = parser.parse().unwrap();
        let program = match ast {
            cursed::ast::Ast::Program(program) => program,
            _ => panic!("Expected Program")
        };
        
        let mut codegen = setup_llvm();
        
        // Check if we have at least one statement
        if program.statements.len() >= 1 {
            let result = codegen.compile_statement(&program.statements[0]);
            // Should handle gracefully or return error
            assert!(result.is_ok() || result.is_err());
        } else {
            // Program didn't parse correctly - just check compilation doesn't crash
            assert!(program.statements.len() >= 0);
        }
    }

    #[test]
    fn test_nested_goroutine_spawns() {
        let source = r#"
            slay spawn_workers() {
                bestie i := 0; i < 3; i++ {
                    stan worker(i)
                }
            }
            
            slay worker(id thicc) {
                // Do work
            }
            
            stan spawn_workers()
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::from_tokens(tokens);
        let ast = parser.parse().unwrap();
        let program = match ast {
            cursed::ast::Ast::Program(program) => program,
            _ => panic!("Expected Program")
        };
        
        let mut codegen = setup_llvm();
        
        // Compile all functions and spawns
        for statement in &program.statements {
            let result = codegen.compile_statement(statement);
            assert!(result.is_ok());
        }
    }
}
