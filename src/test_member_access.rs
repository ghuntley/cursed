// Test parsing of member access expressions

#[cfg(test)]
mod tests {
    use crate::parser::new_parser;
    use crate::ast::*;
    
    #[test]
    fn test_member_access_simple() {
        let source = "vibez.spill";
        let mut parser = new_parser(source).expect("Failed to create parser");
        let program = parser.parse_program().expect("Failed to parse simple member access");
        
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(Expression::MemberAccess(member_access)) => {
                match member_access.object.as_ref() {
                    Expression::Identifier(name) => assert_eq!(name, "vibez"),
                    _ => panic!("Expected identifier as object"),
                }
                assert_eq!(member_access.property, "spill");
            },
            _ => panic!("Expected member access expression"),
        }
    }
    
    #[test]
    fn test_member_access_with_call() {
        let source = r#"vibez.spill("hello")"#;
        let mut parser = new_parser(source).expect("Failed to create parser");
        let program = parser.parse_program().expect("Failed to parse member access call");
        
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(Expression::Call(call)) => {
                match call.function.as_ref() {
                    Expression::MemberAccess(member_access) => {
                        match member_access.object.as_ref() {
                            Expression::Identifier(name) => assert_eq!(name, "vibez"),
                            _ => panic!("Expected identifier as object"),
                        }
                        assert_eq!(member_access.property, "spill");
                    },
                    _ => panic!("Expected member access as function"),
                }
                assert_eq!(call.arguments.len(), 1);
                match &call.arguments[0] {
                    Expression::String(s) => assert_eq!(s, "hello"),
                    _ => panic!("Expected string argument"),
                }
            },
            _ => panic!("Expected call expression"),
        }
    }
    
    #[test] 
    fn test_chained_member_access() {
        let source = "obj.member.method";
        let mut parser = new_parser(source).expect("Failed to create parser");
        let program = parser.parse_program().expect("Failed to parse chained member access");
        
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(Expression::MemberAccess(outer)) => {
                assert_eq!(outer.property, "method");
                match outer.object.as_ref() {
                    Expression::MemberAccess(inner) => {
                        assert_eq!(inner.property, "member");
                        match inner.object.as_ref() {
                            Expression::Identifier(name) => assert_eq!(name, "obj"),
                            _ => panic!("Expected identifier as inner object"),
                        }
                    },
                    _ => panic!("Expected member access as outer object"),
                }
            },
            _ => panic!("Expected member access expression"),
        }
    }
    
    #[test]
    fn test_full_cursed_demo() {
        let cursed_demo = r#"
vibe main

facts greeting = "Hello from CURSED language! 💀"
facts number = 42

slay greet() {
    vibez.spill(greeting)
    vibez.spill("The answer is:")
    vibez.spill(number)
}

slay main() {
    greet()
    vibez.spill("CURSED language is working!")
    yolo 0
}
"#;
        
        let mut parser = new_parser(cursed_demo).expect("Failed to create parser");
        let program = parser.parse_program().expect("Failed to parse full CURSED demo");
        
        // Check package declaration
        assert!(program.package.is_some());
        if let Some(package) = &program.package {
            assert_eq!(package.name, "main");
        }
        
        // Should have 4 statements: 2 constants + 2 functions
        assert_eq!(program.statements.len(), 4);
        
        // Check that member access calls were parsed correctly in function bodies
        let mut found_member_access = false;
        for statement in &program.statements {
            if let Statement::Function(func) = statement {
                for stmt in &func.body {
                    if let Statement::Expression(Expression::Call(call)) = stmt {
                        if let Expression::MemberAccess(member_access) = call.function.as_ref() {
                            if let Expression::Identifier(name) = member_access.object.as_ref() {
                                if name == "vibez" && member_access.property == "spill" {
                                    found_member_access = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(found_member_access, "Expected to find vibez.spill() call in parsed program");
    }
}
