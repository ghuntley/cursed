use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_function_parsing() {
        let source = r#"
slay identity<T>(value) {
    yolo value
}
"#;
        
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse().unwrap();
        
        assert_eq!(program.statements.len(), 1);
        
        if let Statement::Function(func) = &program.statements[0] {
            assert_eq!(func.name, "identity");
            assert_eq!(func.type_parameters.len(), 1);
            assert_eq!(func.type_parameters[0].name, "T");
            assert_eq!(func.type_parameters[0].bounds.len(), 0);
            assert!(func.where_clause.is_none());
        } else {
            panic!("Expected function statement");
        }
    }

    #[test]
    fn test_generic_function_with_bounds() {
        let source = r#"
slay compare<T: Clone + Debug>(a, b) {
    yolo true
}
"#;
        
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse().unwrap();
        
        assert_eq!(program.statements.len(), 1);
        
        if let Statement::Function(func) = &program.statements[0] {
            assert_eq!(func.name, "compare");
            assert_eq!(func.type_parameters.len(), 1);
            assert_eq!(func.type_parameters[0].name, "T");
            assert_eq!(func.type_parameters[0].bounds.len(), 2);
            assert_eq!(func.type_parameters[0].bounds[0], "Clone");
            assert_eq!(func.type_parameters[0].bounds[1], "Debug");
            assert!(func.where_clause.is_none());
        } else {
            panic!("Expected function statement");
        }
    }

    #[test]
    fn test_generic_function_with_where_clause() {
        let source = r#"
slay complex<T, U>(x, y) where T: Clone, U: Debug + Send {
    yolo x
}
"#;
        
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse().unwrap();
        
        assert_eq!(program.statements.len(), 1);
        
        if let Statement::Function(func) = &program.statements[0] {
            assert_eq!(func.name, "complex");
            assert_eq!(func.type_parameters.len(), 2);
            assert_eq!(func.type_parameters[0].name, "T");
            assert_eq!(func.type_parameters[1].name, "U");
            
            // Check where clause
            assert!(func.where_clause.is_some());
            let where_clause = func.where_clause.as_ref().unwrap();
            assert_eq!(where_clause.constraints.len(), 2);
            
            assert_eq!(where_clause.constraints[0].type_name, "T");
            assert_eq!(where_clause.constraints[0].bounds.len(), 1);
            assert_eq!(where_clause.constraints[0].bounds[0], "Clone");
            
            assert_eq!(where_clause.constraints[1].type_name, "U");
            assert_eq!(where_clause.constraints[1].bounds.len(), 2);
            assert_eq!(where_clause.constraints[1].bounds[0], "Debug");
            assert_eq!(where_clause.constraints[1].bounds[1], "Send");
        } else {
            panic!("Expected function statement");
        }
    }

    #[test]
    fn test_multiple_generic_parameters() {
        let source = r#"
slay multi<T, U: Clone, V: Debug + Send + Sync>(x, y, z) {
    yolo x
}
"#;
        
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse().unwrap();
        
        assert_eq!(program.statements.len(), 1);
        
        if let Statement::Function(func) = &program.statements[0] {
            assert_eq!(func.name, "multi");
            assert_eq!(func.type_parameters.len(), 3);
            
            // T with no bounds
            assert_eq!(func.type_parameters[0].name, "T");
            assert_eq!(func.type_parameters[0].bounds.len(), 0);
            
            // U: Clone
            assert_eq!(func.type_parameters[1].name, "U");
            assert_eq!(func.type_parameters[1].bounds.len(), 1);
            assert_eq!(func.type_parameters[1].bounds[0], "Clone");
            
            // V: Debug + Send + Sync
            assert_eq!(func.type_parameters[2].name, "V");
            assert_eq!(func.type_parameters[2].bounds.len(), 3);
            assert_eq!(func.type_parameters[2].bounds[0], "Debug");
            assert_eq!(func.type_parameters[2].bounds[1], "Send");
            assert_eq!(func.type_parameters[2].bounds[2], "Sync");
        } else {
            panic!("Expected function statement");
        }
    }

    #[test]
    fn test_non_generic_function_still_works() {
        let source = r#"
slay simple(x) {
    yolo x
}
"#;
        
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse().unwrap();
        
        assert_eq!(program.statements.len(), 1);
        
        if let Statement::Function(func) = &program.statements[0] {
            assert_eq!(func.name, "simple");
            assert_eq!(func.type_parameters.len(), 0);
            assert!(func.where_clause.is_none());
        } else {
            panic!("Expected function statement");
        }
    }
}
