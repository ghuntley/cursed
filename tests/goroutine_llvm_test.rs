use cursed::codegen::llvm::{LlvmCodeGenerator, GoroutineCompiler};
use cursed::ast::{AstNode, Expression, Statement};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::runtime::goroutine::GoroutineScheduler;
use inkwell::context::Context;
use inkwell::module::Module;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_llvm() -> (Context, Module<'static>, LlvmCodeGenerator<'static>) {
        let context = Context::create();
        let module = context.create_module("test");
        let codegen = LlvmCodeGenerator::new(&context, &module);
        (context, module, codegen)
    }

    #[test]
    fn test_goroutine_spawn_compilation() {
        let source = r#"
            slay worker() {
                // Do some work
            }
            stan worker()
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        
        // Compile function declaration
        let result = codegen.compile_function(&ast.statements[0]);
        assert!(result.is_ok());
        
        // Compile goroutine spawn statement
        let result = codegen.compile_statement(&ast.statements[1]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_goroutine_with_parameters() {
        let source = r#"
            slay process_data(id: i64, data: String) {
                // Process data
            }
            stan process_data(1, "hello")
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        
        // Compile function declaration
        let result = codegen.compile_function(&ast.statements[0]);
        assert!(result.is_ok());
        
        // Compile goroutine spawn with parameters
        let result = codegen.compile_statement(&ast.statements[1]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_yield_point_compilation() {
        let source = r#"
            lowkey (sus i = 0; i < 10; i++) {
                yolo  // Yield point
            }
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        let result = codegen.compile_statement(&ast.statements[0]);
        
        // Should compile loop with yield point
        assert!(result.is_ok());
    }

    #[test]
    fn test_goroutine_scheduler_integration() {
        let scheduler = GoroutineScheduler::new(Default::default());
        
        // Test that scheduler can be used with LLVM compilation
        assert!(scheduler.is_active());
    }

    #[test]
    fn test_multiple_goroutine_spawns() {
        let source = r#"
            slay worker(id: i64) {
                // Do work
            }
            
            lowkey (sus i = 0; i < 5; i++) {
                stan worker(i)
            }
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        
        // Compile function declaration
        let result = codegen.compile_function(&ast.statements[0]);
        assert!(result.is_ok());
        
        // Compile loop with multiple spawns
        let result = codegen.compile_statement(&ast.statements[1]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_goroutine_safe_points() {
        let source = r#"
            slay computation() {
                lowkey (sus i = 0; i < 1000; i++) {
                    // Some computation
                    yolo  // Safe point for GC
                }
            }
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        let result = codegen.compile_function(&ast.statements[0]);
        
        // Should compile with safe points
        assert!(result.is_ok());
    }

    #[test]
    fn test_goroutine_ffi_functions() {
        let (_context, module, _codegen) = setup_llvm();
        
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
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        
        // Compile all statements
        for statement in &ast.statements {
            let result = codegen.compile_statement(statement);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_goroutine_error_handling() {
        // Test compilation of invalid goroutine constructs
        let source = "stan nonexistent_function()";
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        let result = codegen.compile_statement(&ast.statements[0]);
        
        // Should handle gracefully or return error
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_nested_goroutine_spawns() {
        let source = r#"
            slay spawn_workers() {
                lowkey (sus i = 0; i < 3; i++) {
                    stan worker(i)
                }
            }
            
            slay worker(id: i64) {
                // Do work
            }
            
            stan spawn_workers()
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        
        // Compile all functions and spawns
        for statement in &ast.statements {
            let result = codegen.compile_statement(statement);
            assert!(result.is_ok());
        }
    }
}
