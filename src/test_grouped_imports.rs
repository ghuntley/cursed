#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_grouped_imports_parsing() {
        let source = r#"yeet ( "fmt"; "strings"; "os" )"#;
        
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();
        
        assert_eq!(program.imports.len(), 1);
        let import = &program.imports[0];
        assert_eq!(import.path, ""); // Empty path indicates grouped import
        assert_eq!(import.items.len(), 3);
        assert_eq!(import.items[0], "fmt");
        assert_eq!(import.items[1], "strings");
        assert_eq!(import.items[2], "os");
    }

    #[test]
    fn test_mixed_imports_parsing() {
        let source = r#"
yeet ( "fmt"; "strings" )
yeet "single_import"
yeet ( "math"; "net" )
"#;
        
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();
        
        assert_eq!(program.imports.len(), 3);
        
        // First import: grouped
        let import1 = &program.imports[0];
        assert_eq!(import1.path, "");
        assert_eq!(import1.items.len(), 2);
        assert_eq!(import1.items[0], "fmt");
        assert_eq!(import1.items[1], "strings");
        
        // Second import: single
        let import2 = &program.imports[1];
        assert_eq!(import2.path, "single_import");
        assert_eq!(import2.items.len(), 0);
        
        // Third import: grouped
        let import3 = &program.imports[2];
        assert_eq!(import3.path, "");
        assert_eq!(import3.items.len(), 2);
        assert_eq!(import3.items[0], "math");
        assert_eq!(import3.items[1], "net");
    }

    #[test]
    fn test_single_grouped_import() {
        let source = r#"yeet ( "single_module" )"#;
        
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();
        
        assert_eq!(program.imports.len(), 1);
        let import = &program.imports[0];
        assert_eq!(import.path, "");
        assert_eq!(import.items.len(), 1);
        assert_eq!(import.items[0], "single_module");
    }

    #[test]
    fn test_grouped_import_error_missing_semicolon() {
        let source = r#"yeet ( "fmt" "strings" )"#;
        
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer).unwrap();
        let result = parser.parse_program();
        
        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("Expected ';'"));
    }

    #[test]
    fn test_grouped_import_error_empty_group() {
        let source = r#"yeet ( )"#;
        
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();
        
        assert_eq!(program.imports.len(), 1);
        let import = &program.imports[0];
        assert_eq!(import.path, "");
        assert_eq!(import.items.len(), 0);
    }
}
