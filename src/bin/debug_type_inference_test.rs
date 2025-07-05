use cursed::*;
use cursed::error::CursedError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_program = r#"
        slay add(a, b) {
            yolo a + b;
        }
        
        facts result = add(5, 3);
    "#;

    println!("Testing type inference compilation...");

    let mut codegen = LlvmCodeGeneratorReal::new()?;

    let mut lexer = Lexer::new(test_program.to_string());
    let mut parser = Parser::new(lexer)?;
    let ast = parser.parse()?;

    println!("AST parsed successfully");
    
    match codegen.compile_ast(&ast) {
        Ok(_) => {
            let ir = codegen.module().print_to_string().to_string();
            println!("Compilation successful!");
            println!("Generated IR: {}", ir);
            if ir.is_empty() {
                println!("Warning: Generated IR is empty");
            }
        }
        Err(e) => {
            println!("Compilation failed: {}", e);
            println!("Error details: {:?}", e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}
