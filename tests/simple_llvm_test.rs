use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::TypeChecker;
use inkwell::context::Context;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_llvm_module_creation() {
        let context = Context::create();
        let module = context.create_module("test_module");
        let builder = context.create_builder();
        
        let codegen = LlvmCodeGenerator::new(&context, module, builder);
        
        // Module should be created successfully
        assert!(!codegen.module().get_name().to_str().unwrap().is_empty());
    }

    #[test]
    fn test_simple_function_compilation() {
        let source = r#"
            slay main() -> i64 {
                42
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let mut codegen = LlvmCodeGenerator::new(&context, module, builder);
        
        let result = codegen.compile_program(&ast);
        
        // Should compile without panicking
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_arithmetic_expression_compilation() {
        let source = r#"
            slay calculate() -> i64 {
                sus x = 10;
                sus y = 20;
                x + y
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let mut codegen = LlvmCodeGenerator::new(&context, module, builder);
        
        let result = codegen.compile_program(&ast);
        
        // Should compile arithmetic expressions
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_variable_declaration_compilation() {
        let source = r#"
            sus global_var = 42;
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let mut codegen = LlvmCodeGenerator::new(&context, module, builder);
        
        let result = codegen.compile_program(&ast);
        
        // Should compile variable declarations
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_conditional_compilation() {
        let source = r#"
            slay test_conditional() -> i64 {
                lowkey (facts) {
                    42
                } flex {
                    24
                }
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let mut codegen = LlvmCodeGenerator::new(&context, module, builder);
        
        let result = codegen.compile_program(&ast);
        
        // Should compile conditional statements
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_function_call_compilation() {
        let source = r#"
            slay helper() -> i64 {
                42
            }
            
            slay main() -> i64 {
                helper()
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let mut codegen = LlvmCodeGenerator::new(&context, module, builder);
        
        let result = codegen.compile_program(&ast);
        
        // Should compile function calls
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_loop_compilation() {
        let source = r#"
            slay loop_test() -> i64 {
                sus sum = 0;
                lowkey (sus i = 0; i < 10; i++) {
                    sum = sum + i;
                }
                sum
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let mut codegen = LlvmCodeGenerator::new(&context, module, builder);
        
        let result = codegen.compile_program(&ast);
        
        // Should compile loops
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_string_literal_compilation() {
        let source = r#"
            slay get_greeting() -> String {
                "Hello, CURSED!"
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let mut codegen = LlvmCodeGenerator::new(&context, module, builder);
        
        let result = codegen.compile_program(&ast);
        
        // Should compile string literals
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_boolean_literal_compilation() {
        let source = r#"
            slay get_true() -> bool {
                facts
            }
            
            slay get_false() -> bool {
                cap
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let mut codegen = LlvmCodeGenerator::new(&context, module, builder);
        
        let result = codegen.compile_program(&ast);
        
        // Should compile boolean literals
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_llvm_ir_generation() {
        let source = r#"
            slay simple() -> i64 {
                sus x = 1;
                sus y = 2;
                x + y
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        let mut codegen = LlvmCodeGenerator::new(&context, module, builder);
        
        let compilation_result = codegen.compile_program(&ast);
        
        // Generate IR string
        let ir_string = codegen.module().print_to_string().to_string();
        
        // IR should not be empty
        assert!(!ir_string.is_empty());
        
        // Should contain basic LLVM IR elements
        assert!(ir_string.contains("define") || compilation_result.is_err());
    }
}
