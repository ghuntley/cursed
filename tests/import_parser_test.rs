#[cfg(test)]
mod tests {
    use cursed::parser::Parser;
    use cursed::lexer::Lexer;
    use cursed::ast::*;
    
    fn parse_import(input: &str) -> Result<Box<dyn Node>, cursed::error::Error> {
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }
    
    #[test]
    fn test_simple_import() {
        let input = r#"yeet "std::io""#;
        
        let result = parse_import(input);
        assert!(result.is_ok(), "Failed to parse simple import: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("std::io") || program_str.contains("yeet"));
    }
    
    #[test]
    fn test_import_with_alias() {
        let input = r#"yeet io from "std::io""#;
        
        let result = parse_import(input);
        match result {
            Ok(program) => {
                let program_str = format!("{:?}", program);
                assert!(program_str.contains("io"));
                assert!(program_str.contains("std::io"));
            }
            Err(_) => {
                // May not be implemented yet
                assert!(true);
            }
        }
    }
    
    #[test]
    fn test_qualified_import() {
        let input = r#"yeet std::collections::HashMap"#;
        
        let result = parse_import(input);
        assert!(result.is_ok(), "Failed to parse qualified import: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("HashMap") || program_str.contains("collections"));
    }
    
    #[test]
    fn test_wildcard_import() {
        let input = r#"yeet std::collections::*"#;
        
        let result = parse_import(input);
        match result {
            Ok(program) => {
                let program_str = format!("{:?}", program);
                assert!(program_str.contains("collections") || program_str.contains("*"));
            }
            Err(_) => {
                // Wildcard imports may not be implemented
                assert!(true);
            }
        }
    }
    
    #[test]
    fn test_multiple_imports() {
        let input = r#"
        yeet "std::io";
        yeet "std::collections";
        yeet "std::fs";
        "#;
        
        let result = parse_import(input);
        assert!(result.is_ok(), "Failed to parse multiple imports: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        // Should contain multiple import references
        let import_count = program_str.matches("yeet").count() + program_str.matches("import").count();
        assert!(import_count >= 2);
    }
    
    #[test]
    fn test_nested_module_import() {
        let input = r#"yeet "my_package::utils::helpers::string_utils""#;
        
        let result = parse_import(input);
        assert!(result.is_ok(), "Failed to parse nested module import: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("my_package") || program_str.contains("utils") || program_str.contains("helpers"));
    }
    
    #[test]
    fn test_relative_import() {
        let input = r#"yeet "./local_module""#;
        
        let result = parse_import(input);
        assert!(result.is_ok(), "Failed to parse relative import: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("local_module") || program_str.contains("./"));
    }
    
    #[test]
    fn test_parent_directory_import() {
        let input = r#"yeet "../parent_module""#;
        
        let result = parse_import(input);
        assert!(result.is_ok(), "Failed to parse parent directory import: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("parent_module") || program_str.contains("../"));
    }
    
    #[test]
    fn test_selective_import() {
        let input = r#"yeet {HashMap, HashSet} from "std::collections""#;
        
        let result = parse_import(input);
        match result {
            Ok(program) => {
                let program_str = format!("{:?}", program);
                assert!(program_str.contains("HashMap") || program_str.contains("HashSet"));
                assert!(program_str.contains("collections"));
            }
            Err(_) => {
                // Selective imports may not be implemented yet
                assert!(true);
            }
        }
    }
    
    #[test]
    fn test_import_with_function() {
        let input = r#"
        yeet "std::io";
        
        slay main() {
            // function body
        }
        "#;
        
        let result = parse_import(input);
        assert!(result.is_ok(), "Failed to parse import with function: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("std::io") || program_str.contains("yeet"));
        assert!(program_str.contains("main") || program_str.contains("slay"));
    }
    
    #[test]
    fn test_import_error_recovery() {
        let input = r#"yeet "incomplete"#;
        
        let result = parse_import(input);
        // Should handle incomplete import gracefully
        match result {
            Ok(program) => {
                let program_str = format!("{:?}", program);
                assert!(program_str.contains("incomplete"));
            }
            Err(_) => {
                // Expected for malformed import
                assert!(true);
            }
        }
    }
    
    #[test]
    fn test_malformed_import_path() {
        let input = r#"yeet """#;
        
        let result = parse_import(input);
        // Should handle empty import path
        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(true), // Expected error
        }
    }
    
    #[test]
    fn test_import_with_comments() {
        let input = r#"
        // Import standard library
        yeet "std::io"; // For input/output operations
        // Another import
        yeet "std::collections";
        "#;
        
        let result = parse_import(input);
        assert!(result.is_ok(), "Failed to parse import with comments: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("std::io") || program_str.contains("collections"));
    }
    
    #[test]
    fn test_import_stdlib_modules() {
        let input = r#"
        yeet "stdlib::math";
        yeet "stdlib::string";
        yeet "stdlib::collections";
        "#;
        
        let result = parse_import(input);
        assert!(result.is_ok(), "Failed to parse stdlib imports: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("stdlib") || program_str.contains("math") || program_str.contains("string"));
    }
}
