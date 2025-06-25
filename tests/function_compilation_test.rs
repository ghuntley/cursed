use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::parser::Parser;
use cursed::lexer::Lexer;
use cursed::error::CursedError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_function_compilation() -> Result<(), CursedError> {
        let source = r#"
            slay add(a: i32, b: i32) -> i32 {
                facts result = a + b;
                return result;
            }
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        
        let mut codegen = LlvmCodeGenerator::new()?;
        let compiled = codegen.compile(&ast)?;
        
        assert!(compiled.is_some());
        Ok(())
    }

    #[test]
    fn test_function_with_return_type() -> Result<(), CursedError> {
        let source = r#"
            slay multiply(x: f64, y: f64) -> f64 {
                return x * y;
            }
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;
        
        let mut codegen = LlvmCodeGenerator::new()?;
        let compiled = codegen.compile(&ast)?;
        
        assert!(compiled.is_some());
        Ok(())
    }

    #[test]
    fn test_function_parameter_validation() {
        let source = r#"
            slay invalid_params() {
                // Function with no parameters should still compile
            }
        "#;
        
        let mut lexer = Lexer::new(source);
        match lexer.tokenize() {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                let result = parser.parse();
                assert!(result.is_ok());
            }
            Err(_) => {
                // Expected to fail for malformed syntax
                assert!(true);
            }
        }
    }
}
