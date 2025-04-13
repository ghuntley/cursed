use std::fs;
extern crate cursed;
use cursed::lexer::Lexer;
use cursed::parser::Parser;

fn main() {
    let filename = "examples/simple_dot_call.csd";
    let input = fs::read_to_string(filename).expect("Failed to read file");
    
    println!("🔍 Lexical Analysis...");
    let mut lexer = Lexer::new(&input);
    
    println!("🔨 Parsing...");
    let mut parser = Parser::new(&mut lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");
    
    println!("✅ Successfully parsed program");
    println!("📊 Program structure:\n{}", program.string());
    
    // Print the AST structure in a more detailed way
    for (i, stmt) in program.statements.iter().enumerate() {
        println!("Statement {}: {} ({})", i, stmt.string(), std::any::type_name_of_val(stmt.as_ref()));
    }
}