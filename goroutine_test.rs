// Test goroutine parsing specifically
use cursed::parser::Parser;
use cursed::lexer::Lexer;
use cursed::ast::Statement;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = r#"
vibe main

slay worker() {
    yolo
}

slay main() {
    stan worker()
    yolo
}
"#;

    println!("Testing goroutine parsing...");
    println!("Source code:\n{}", source);

    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;

    println!("\nProgram parsed successfully!");
    println!("Statements: {}", program.statements.len());
    
    for (i, stmt) in program.statements.iter().enumerate() {
        match stmt {
            Statement::Goroutine(goroutine_stmt) => {
                println!("  Statement {}: Goroutine(expression: {:?})", i + 1, goroutine_stmt.expression);
            },
            _ => {
                println!("  Statement {}: {:?}", i + 1, std::mem::discriminant(stmt));
            }
        }
    }
    
    Ok(())
}
