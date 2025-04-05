use cursed::ast::{Statement, Expression};
use cursed::lexer::Lexer;
use cursed::parser::Parser;

#[test]
fn test_parse_generic_type() {
    let input = "be_like Box[T] squad { value T }";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    assert_eq!(program.statements.len(), 1, "Program should have 1 statement");
    
    // The statement should be a struct declaration with generic type parameters
    if let Some(squad_stmt) = program.statements[0].as_any().downcast_ref::<cursed::ast::SquadStatement>() {
        assert_eq!(squad_stmt.name.value, "Box");
        assert_eq!(squad_stmt.type_parameters.len(), 1);
        assert_eq!(squad_stmt.type_parameters[0].value, "T");
        assert_eq!(squad_stmt.fields.len(), 1);
        assert_eq!(squad_stmt.fields[0].name.value, "value");
        assert_eq!(squad_stmt.fields[0].type_name.value, "T");
    } else {
        panic!("Statement is not a SquadStatement");
    }
}

#[test]
fn test_parse_multiple_generic_type_parameters() {
    let input = "be_like Pair[A, B] squad { first A second B }";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    assert_eq!(program.statements.len(), 1, "Program should have 1 statement");
    
    // The statement should be a struct declaration with multiple generic type parameters
    if let Some(squad_stmt) = program.statements[0].as_any().downcast_ref::<cursed::ast::SquadStatement>() {
        assert_eq!(squad_stmt.name.value, "Pair");
        assert_eq!(squad_stmt.type_parameters.len(), 2);
        assert_eq!(squad_stmt.type_parameters[0].value, "A");
        assert_eq!(squad_stmt.type_parameters[1].value, "B");
        assert_eq!(squad_stmt.fields.len(), 2);
        assert_eq!(squad_stmt.fields[0].name.value, "first");
        assert_eq!(squad_stmt.fields[0].type_name.value, "A");
        assert_eq!(squad_stmt.fields[1].name.value, "second");
        assert_eq!(squad_stmt.fields[1].type_name.value, "B");
    } else {
        panic!("Statement is not a SquadStatement");
    }
}

#[test]
fn test_parse_generic_function() {
    let input = "slay identity[T](x T) T { yolo x }";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    assert_eq!(program.statements.len(), 1, "Program should have 1 statement");
    
    // The statement should be a function declaration with generic type parameters
    if let Some(expr_stmt) = program.statements[0].as_any().downcast_ref::<cursed::ast::ExpressionStatement>() {
        if let Some(assign_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<cursed::ast::AssignmentExpression>() {
            assert_eq!(assign_expr.name.value, "identity");
            
            if let Some(func_expr) = assign_expr.value.as_any().downcast_ref::<cursed::ast::FunctionLiteral>() {
                assert_eq!(func_expr.type_parameters.len(), 1);
                assert_eq!(func_expr.type_parameters[0].value, "T");
                assert_eq!(func_expr.parameters.len(), 1);
                assert_eq!(func_expr.parameters[0].value, "x");
                // Check for the return type as well
                assert!(func_expr.body.statements.len() > 0);
            } else {
                panic!("Expression is not a FunctionLiteral");
            }
        } else {
            panic!("Expression is not an AssignmentExpression");
        }
    } else {
        panic!("Statement is not an ExpressionStatement");
    }
}

#[test]
fn test_parse_generic_function_with_multiple_parameters() {
    let input = "slay map[T, U](input T, fn stan(T) U) U { yolo fn(input) }";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    assert_eq!(program.statements.len(), 1, "Program should have 1 statement");
    
    // The statement should be a function declaration with multiple generic type parameters
    if let Some(expr_stmt) = program.statements[0].as_any().downcast_ref::<cursed::ast::ExpressionStatement>() {
        if let Some(assign_expr) = expr_stmt.expression.as_ref().unwrap().as_any().downcast_ref::<cursed::ast::AssignmentExpression>() {
            assert_eq!(assign_expr.name.value, "map");
            
            if let Some(func_expr) = assign_expr.value.as_any().downcast_ref::<cursed::ast::FunctionLiteral>() {
                assert_eq!(func_expr.type_parameters.len(), 2);
                assert_eq!(func_expr.type_parameters[0].value, "T");
                assert_eq!(func_expr.type_parameters[1].value, "U");
                assert_eq!(func_expr.parameters.len(), 2);
                assert_eq!(func_expr.parameters[0].value, "input");
                assert_eq!(func_expr.parameters[1].value, "fn");
                // Check for the return type as well
                assert!(func_expr.body.statements.len() > 0);
            } else {
                panic!("Expression is not a FunctionLiteral");
            }
        } else {
            panic!("Expression is not an AssignmentExpression");
        }
    } else {
        panic!("Statement is not an ExpressionStatement");
    }
}

#[test]
fn test_parse_generic_type_instantiation() {
    let input = "sus box = Box[normie]{value: 42}";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    assert_eq!(program.statements.len(), 1, "Program should have 1 statement");
    
    // The statement should be a variable declaration with a generic type instantiation
    if let Some(let_stmt) = program.statements[0].as_any().downcast_ref::<cursed::ast::LetStatement>() {
        assert_eq!(let_stmt.name.value, "box");
        
        if let Some(be_like_expr) = let_stmt.value.as_ref().unwrap().as_any().downcast_ref::<cursed::ast::BeLikeExpression>() {
            assert_eq!(be_like_expr.struct_name.value, "Box");
            assert_eq!(be_like_expr.type_arguments.len(), 1);
            
            // Check that the type argument is 'normie'
            if let Some(type_arg) = be_like_expr.type_arguments[0].as_any().downcast_ref::<cursed::ast::Identifier>() {
                assert_eq!(type_arg.value, "normie");
            } else {
                panic!("Type argument is not an Identifier");
            }
            
            // Check the struct fields
            assert_eq!(be_like_expr.fields.len(), 1);
            assert_eq!(be_like_expr.fields[0].0, "value");
        } else {
            panic!("Expression is not a BeLikeExpression");
        }
    } else {
        panic!("Statement is not a LetStatement");
    }
}