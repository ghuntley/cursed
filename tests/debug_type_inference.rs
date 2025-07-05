use cursed::*;
use cursed::error::CursedError;

#[test]
fn debug_type_inference_simple() {
    let test_program = r#"
        slay add(a, b) {
            yolo a + b;
        }
        
        facts result = add(5, 3);
    "#;
    
    println!("Testing program: {}", test_program);
    
    let mut lexer = Lexer::new(test_program.to_string());
    let mut parser = Parser::new(lexer).unwrap();
    let ast = parser.parse().unwrap();
    
    println!("AST parsed successfully");
    
    let mut codegen = LlvmCodeGeneratorReal::new().unwrap();
    match codegen.compile_ast(&ast) {
        Ok(ir) => {
            println!("SUCCESS: Generated IR:\n{}", ir);
            println!("IR length: {}", ir.len());
            assert!(!ir.is_empty(), "IR should not be empty");
        }
        Err(error) => {
            println!("ERROR: Compilation failed: {}", error);
            panic!("Compilation failed: {}", error);
        }
    }
}
