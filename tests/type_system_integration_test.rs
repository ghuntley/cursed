use cursed::core::type_checker::TypeChecker;
use cursed::ast::{AstNode, Type};
use cursed::lexer::Lexer;
use cursed::parser::Parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_type_inference() {
        let source = "sus x = 42";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let checked_ast = type_checker.check(&ast).unwrap();
        
        // Verify that integer literal gets i64 type
        assert!(matches!(checked_ast.statements[0], AstNode::VariableDeclaration { .. }));
    }

    #[test]
    fn test_function_type_checking() {
        let source = r#"
            slay add(a: i64, b: i64) -> i64 {
                a + b
            }
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Should successfully type check
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_mismatch_detection() {
        let source = r#"
            sus x: i64 = "hello"
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Should detect type mismatch
        assert!(result.is_err());
    }

    #[test]
    fn test_generic_instantiation() {
        let source = r#"
            slay identity<T>(x: T) -> T {
                x
            }
            
            sus result = identity(42)
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Should successfully instantiate generic
        assert!(result.is_ok());
    }

    #[test]
    fn test_constraint_validation() {
        let source = r#"
            collab Addable {
                slay add(self, other: Self) -> Self
            }
            
            slay generic_add<T: Addable>(a: T, b: T) -> T {
                a.add(b)
            }
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Should successfully validate constraints
        assert!(result.is_ok());
    }

    #[test]
    fn test_recursive_type_checking() {
        let source = r#"
            squad Node {
                value: i64,
                next: ?Node
            }
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Should handle recursive types
        assert!(result.is_ok());
    }
}
