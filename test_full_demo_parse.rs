use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast::*;

fn main() -> Result<(), cursed::error::CursedError> {
    let source = std::fs::read_to_string("test_member_clean.csd")
        .expect("Failed to read test_member_clean.csd");
    
    println!("Testing parsing of test_cursed_demo.csd");
    
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    
    println!("✅ Successfully parsed full demo program!");
    println!("Statements: {}", program.statements.len());
    
    // Check for member access in the statements
    let mut member_access_count = 0;
    for stmt in &program.statements {
        if let Statement::Function(func) = stmt {
            println!("Function: {}", func.name);
            for body_stmt in &func.body {
                if let Statement::Expression(Expression::Call(call)) = body_stmt {
                    if let Expression::MemberAccess(member) = call.function.as_ref() {
                        member_access_count += 1;
                        println!("  Member access: {:?}.{}", member.object, member.property);
                    }
                }
            }
        }
    }
    
    println!("Found {} member access expressions", member_access_count);
    
    Ok(())
}
