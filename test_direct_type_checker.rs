use cursed::*;
use cursed::error::CursedError;

fn main() {
    let test_program = r#"
        slay add(a, b) {
            yolo a + b;
        }
    "#;
    
    println!("Testing program: {}", test_program);
    
    let mut lexer = Lexer::new(test_program.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    let ast = parser.parse().unwrap();
    
    println!("AST parsed successfully");
    
    // Test the type checker directly
    let mut type_checker = cursed::type_system::checker::TypeChecker::new();
    match type_checker.check_program(&ast) {
        Ok(_) => {
            println!("SUCCESS: Type checking passed!");
        }
        Err(errors) => {
            println!("ERROR: Type checking failed with {} errors:", errors.len());
            for error in errors {
                println!("  - {}", error.message);
            }
        }
    }
}
