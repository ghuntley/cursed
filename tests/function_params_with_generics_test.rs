use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast;
use cursed::ast::traits::{Node, Statement, Expression};


#[test]
fn test_function_with_generic_parameters() {
    // Test a function with generic parameters
    let input = r#"vibe test"

slay add[T](x T, y T) T {
    yolo x + y
}
"#";
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    // Print all statements for debugging
    println!("Found {} statements", program.statements.len());
    for (i, stmt) in program.statements.iter().enumerate() {
        println!("Statement {}: {}", i, stmt.string());
        
        // Print the type of each statement
        if stmt.as_any().is::<ast::FunctionStatement>() {
            println!("  Type: FunctionStatement");
        } else if stmt.as_any().is::<ast::statements::PackageStatement>() {
            println!("  Type: PackageStatement");
        } else {
            println!("  Type: Other - {}", std::any::type_name_of_val(stmt.as_any()));
        }
    }
    
    // Try to find the function statement
    let func_stmt_option = program.statements.iter()
        .filter_map(|stmt| stmt.as_any().downcast_ref::<ast::FunctionStatement>())
        .find(|func| func.name.value == "add");
        
    if let Some(func_stmt) = func_stmt_option {
        // Test continues...
    } else {
        panic!("Could not find function named 'add'");
    }
    
    let func_stmt = func_stmt_option.unwrap();
    
    // Check function name
    assert_eq!(func_stmt.name.value, "add", "Function name should be 'add'");
    
    // Check generic type parameters
    assert_eq!(func_stmt.type_parameters.len(), 1, "Should have 1 type parameter");
    assert_eq!(func_stmt.type_parameters[0].value, "T", "Type parameter should be 'T'");
    
    // Check function parameters
    assert_eq!(func_stmt.parameters.len(), 2, "Should have 2 parameters");
    assert_eq!(func_stmt.parameters[0].name.value, "x", "First parameter should be 'x'");
    assert_eq!(func_stmt.parameters[1].name.value, "y", "Second parameter should be 'y'");
    
    // Check parameter types
    assert_eq!(func_stmt.parameters[0].type_name, "T", 
              "Parameter x should have type T");
    assert_eq!(func_stmt.parameters[1].type_name, "T", 
              "Parameter y should have type T");
    
    // Check return type
    assert!(func_stmt.return_type.is_some(), "Should have a return type");
    let return_type = func_stmt.return_type.as_ref().unwrap();
    assert_eq!(return_type, "T", "Return type should be 'T'");
}

#[test]
fn test_function_with_multiple_generic_parameters() {
    // Test a function with multiple generic parameters
    let input = r#"vibe test"

slay pair[A, B](first A, second B) {
    yolo first
}
"#";
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    // Find the function statement by searching through all statements
    let func_stmt = program.statements.iter()
        .filter_map(|stmt| stmt.as_any().downcast_ref::<ast::FunctionStatement>())
        .find(|func| func.name.value == "pair")
        .expect("Could not find function named 'pair'");
    
    // Check function name
    assert_eq!(func_stmt.name.value, "pair", "Function name should be 'pair'");
    
    // Check generic type parameters
    assert_eq!(func_stmt.type_parameters.len(), 2, "Should have 2 type parameters");
    assert_eq!(func_stmt.type_parameters[0].value, "A", "First type parameter should be 'A'");
    assert_eq!(func_stmt.type_parameters[1].value, "B", "Second type parameter should be 'B'");
    
    // Check function parameters
    assert_eq!(func_stmt.parameters.len(), 2, "Should have 2 parameters");
    assert_eq!(func_stmt.parameters[0].name.value, "first", "First parameter should be 'first'");
    assert_eq!(func_stmt.parameters[1].name.value, "second", "Second parameter should be 'second'");
    
    // Check parameter types
    assert_eq!(func_stmt.parameters[0].type_name, "A", 
              "Parameter first should have type A");
    assert_eq!(func_stmt.parameters[1].type_name, "B", 
              "Parameter second should have type B");
    
    // No return type in this case
    assert!(func_stmt.return_type.is_none(), "Should not have a return type");
}

#[test]
fn test_function_with_generic_constraints() {
    // Test a function with generic constraints
    let input = r#"vibe test"

slay sort[T: Comparable](items []T) []T {
    yolo sortImplementation(items)
}
"#";
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();
    
    // Find the function statement by searching through all statements
    let func_stmt = program.statements.iter()
        .filter_map(|stmt| stmt.as_any().downcast_ref::<ast::FunctionStatement>())
        .find(|func| func.name.value == "sort")
        .expect("Could not find function named 'sort'");
    
    // Check function name
    assert_eq!(func_stmt.name.value, "sort", "Function name should be 'sort'");
    
    // Check generic type parameters
    assert_eq!(func_stmt.type_parameters.len(), 1, "Should have 1 type parameter");
    assert_eq!(func_stmt.type_parameters[0].value, "T", "Type parameter should be 'T'");
    
    // Check generic constraints
    assert!(!func_stmt.generic_constraints.is_empty(), "Should have generic constraints");
    assert_eq!(func_stmt.generic_constraints[0].type_parameter.value, "T", "Constraint should be on T");
    assert_eq!(func_stmt.generic_constraints[0].trait_name.value, "Comparable", "Constraint should be Comparable");
    
    // Check function parameters and return type
    assert_eq!(func_stmt.parameters.len(), 1, "Should have 1 parameter");
    assert_eq!(func_stmt.parameters[0].name.value, "items", "Parameter should be 'items'");
    
    assert!(func_stmt.return_type.is_some(), "Should have a return type");
}