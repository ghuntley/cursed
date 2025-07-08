// Debug program to test goroutine compilation failures
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::Lexer;
use cursed::parser::Parser;

fn main() {
    let source = r#"
        slay main() {
            bestie i := 0; i < 10; i++ {
                yolo  // Yield point
            }
        }
    "#;
    
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::from_tokens(tokens);
    let ast = parser.parse().unwrap();
    let program = match ast {
        cursed::ast::Ast::Program(program) => program,
        _ => panic!("Expected Program")
    };
    
    let mut codegen = LlvmCodeGenerator::new().unwrap();
    
    if program.statements.len() >= 1 {
        match codegen.compile_statement(&program.statements[0]) {
            Ok(result) => println!("Success: {}", result),
            Err(err) => println!("Error: {:?}", err),
        }
    }
}
