use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast::Type;

#[cfg(test)]
mod array_size_expressions_tests {
    use super::*;

    #[test]
    fn test_simple_array_size_literal() {
        let source = "sus arr [5]normie = [1, 2, 3, 4, 5]";
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        
        let program = parser.parse_program().unwrap();
        assert!(parser.errors().is_empty());
    }

    #[test]
    fn test_array_size_with_arithmetic() {
        let source = "sus arr [2+3]normie = [1, 2, 3, 4, 5]";
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        
        let program = parser.parse_program().unwrap();
        assert!(parser.errors().is_empty());
    }

    #[test]
    fn test_array_size_with_multiplication() {
        let source = "sus arr [3*4]normie = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]";
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        
        let program = parser.parse_program().unwrap();
        assert!(parser.errors().is_empty());
    }

    #[test]
    fn test_array_size_with_subtraction() {
        let source = "sus arr [10-3]normie = [1, 2, 3, 4, 5, 6, 7]";
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        
        let program = parser.parse_program().unwrap();
        assert!(parser.errors().is_empty());
    }

    #[test]
    fn test_array_size_with_division() {
        let source = "sus arr [20/4]normie = [1, 2, 3, 4, 5]";
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        
        let program = parser.parse_program().unwrap();
        assert!(parser.errors().is_empty());
    }

    #[test]
    fn test_array_size_with_parentheses() {
        let source = "sus arr [(2+3)*2]normie = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]";
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        
        let program = parser.parse_program().unwrap();
        assert!(parser.errors().is_empty());
    }

    #[test]
    fn test_array_with_initialization() {
        let source = r#"
            slay main() {
                sus arr [3]normie = [1, 2, 3]
                vibez.spill("Array initialized")
            }
        "#;
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        
        let program = parser.parse_program().unwrap();
        assert!(parser.errors().is_empty());
        assert_eq!(program.statements.len(), 1); // One main function
    }

    #[test]
    fn test_multiple_array_declarations_with_different_sizes() {
        let source = r#"
            slay main() {
                sus arr1 [5]normie = [1, 2, 3, 4, 5]
                sus arr2 [2*3]normie = [1, 2, 3, 4, 5, 6]
                sus arr3 [10-7]normie = [1, 2, 3]
                vibez.spill("Multiple arrays declared")
            }
        "#;
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        
        let program = parser.parse_program().unwrap();
        assert!(parser.errors().is_empty());
        assert_eq!(program.statements.len(), 1); // One main function
    }

    #[test]
    fn test_slice_vs_array_distinction() {
        let source = r#"
            slay main() {
                sus slice []normie = [1, 2, 3]
                sus array [3]normie = [1, 2, 3]
                vibez.spill("Slice and array distinguished")
            }
        "#;
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        
        let program = parser.parse_program().unwrap();
        assert!(parser.errors().is_empty());
        assert_eq!(program.statements.len(), 1); // One main function
    }
}
