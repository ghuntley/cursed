use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast::*;

fn main() -> Result<(), cursed::error::CursedError> {
    let test_cases = vec![
        "obj.prop",
        "obj.method()",
        "nested.obj.prop",
        "complex.chain.method().prop",
        "vibez.spill(\"test\")",
    ];
    
    for (i, source) in test_cases.iter().enumerate() {
        println!("Test case {}: {}", i + 1, source);
        
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;
        
        if let Some(Statement::Expression(expr)) = program.statements.first() {
            println!("  Parsed as: {:#?}", expr);
        }
        println!();
    }
    
    Ok(())
}
