use cursed::*;
use cursed::error::CursedError;

#[test]
fn test_basic_arithmetic_inference() {
    let test_program = r#"
        slay add(a, b) {
            yolo a + b;
        }
    "#;
    
    test_type_checking_passes(test_program);
}

#[test] 
fn test_multiple_operations_inference() {
    let test_program = r#"
        slay calculate(x, y, z) {
            sus sum = x + y;
            sus product = sum * z;
            yolo product - 10;
        }
    "#;
    
    test_type_checking_passes(test_program);
}

#[test]
fn test_mixed_typed_parameters() {
    let test_program = r#"
        slay mixed_add(a normie, b) {
            yolo a + b;
        }
    "#;
    
    test_type_checking_passes(test_program);
}

#[test]
fn test_arithmetic_with_literals() {
    let test_program = r#"
        slay add_literal(a) {
            yolo a + 42;
        }
    "#;
    
    test_type_checking_passes(test_program);
}

#[test]
fn test_comparison_operations() {
    let test_program = r#"
        slay compare(a, b) {
            yolo a < b;
        }
    "#;
    
    test_type_checking_passes(test_program);
}

#[test]
fn test_nested_expressions() {
    let test_program = r#"
        slay nested_calc(a, b, c) {
            yolo (a + b) * (c - a);
        }
    "#;
    
    test_type_checking_passes(test_program);
}

fn test_type_checking_passes(test_program: &str) {
    println!("Testing program: {}", test_program);
    
    let mut lexer = Lexer::new(test_program.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    let ast = parser.parse().unwrap();
    
    println!("AST parsed successfully");
    
    // Test the type checker directly
    let program = match ast {
        cursed::ast::Ast::Program(program) => program,
        _ => panic!("Expected Program")
    };
    let mut type_checker = cursed::type_system::checker::TypeChecker::new();
    match type_checker.check_program(&program) {
        Ok(_) => {
            println!("SUCCESS: Type checking passed!");
        }
        Err(errors) => {
            println!("ERROR: Type checking failed with {} errors:", errors.len());
            for error in &errors {
                println!("  - {}", error.message);
            }
            panic!("Type checking failed: {:?}", errors);
        }
    }
}
