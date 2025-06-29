use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast::*;

fn main() -> Result<(), cursed::error::CursedError> {
    let source = r#"vibez.spill("hello")"#;
    
    println!("Testing parsing: {}", source);
    
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    
    println!("Parsed program: {:#?}", program);
    
    // Verify we got a member access expression
    if let Some(Statement::Expression(Expression::Call(call_expr))) = program.statements.first() {
        if let Expression::MemberAccess(member_access) = call_expr.function.as_ref() {
            println!("✅ Successfully parsed member access:");
            println!("   Object: {:?}", member_access.object);
            println!("   Property: {}", member_access.property);
        } else {
            println!("❌ Expected member access, got: {:?}", call_expr.function);
        }
    } else {
        println!("❌ Expected call expression with member access");
    }
    
    Ok(())
}
