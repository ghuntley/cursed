use cursed::{
    lexer::Lexer,
    parser::Parser,
    execution::CursedExecutionEngine,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test basic lexing
    let source = r#"
        slay main() {
            sus x = 42
            vibez.spill("Hello, Cursed!")
            yolo x
        }
    "#;
    
    println!("🔍 Testing CURSED compilation pipeline...");
    
    // Lexer test
    println!("1. Lexing...");
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.tokenize()?;
    println!("   Found {} tokens", tokens.len());
    
    // Parser test  
    println!("2. Parsing...");
    let lexer2 = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer2)?;
    let program = parser.parse_program()?;
    println!("   Parsed {} statements", program.statements.len());
    
    // Execution test
    println!("3. Executing...");
    let mut engine = CursedExecutionEngine::new()?;
    let result = engine.execute(source)?;
    println!("   Result: {:?}", result);
    
    println!("✅ All tests passed!");
    
    Ok(())
}
