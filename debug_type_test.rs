use cursed::*;
use cursed::error::CursedError;

fn main() {
    let test_program = r#"
        slay add(a, b) {
            yolo a + b;
        }
        
        facts result = add(5, 3);
    "#;
    
    let mut codegen = LlvmCodeGeneratorReal::new().unwrap();
    let mut lexer = Lexer::new(test_program.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    let ast = parser.parse().unwrap();

    match codegen.compile_ast(&ast) {
        Ok(_) => {
            let ir = codegen.module().print_to_string().to_string();
            println!("SUCCESS: Generated IR:\n{}", ir);
        }
        Err(error) => {
            println!("ERROR: Compilation failed: {}", error);
            println!("ERROR: Debug info: {:?}", error);
        }
    }
}
