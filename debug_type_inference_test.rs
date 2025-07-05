use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_program = r#"
        slay add(a, b) {
            yolo a + b;
        }
        
        facts result = add(5, 3);
    "#;

    let mut codegen = cursed::LlvmCodeGeneratorReal::new()?;

    let mut lexer = cursed::Lexer::new(test_program.to_string());
    let mut parser = cursed::Parser::new(lexer)?;
    let ast = parser.parse()?;

    println!("AST parsed successfully");
    
    match codegen.compile_ast(&ast) {
        Ok(_) => {
            let ir = codegen.module().print_to_string().to_string();
            println!("Compilation successful!");
            println!("Generated IR: {}", ir);
        }
        Err(e) => {
            println!("Compilation failed: {}", e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}
