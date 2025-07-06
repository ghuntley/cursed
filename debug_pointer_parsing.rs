use cursed::{lexer::Lexer, parser::Parser};

fn main() {
    let source = "sus ptr @normie = @x";
    println!("Testing: {}", source);
    
    let lexer = Lexer::new(source.to_string());
    match Parser::new(lexer) {
        Ok(mut parser) => {
            match parser.parse_typed_variable_statement() {
                Ok(let_stmt) => {
                    println!("Parsed let statement: {:#?}", let_stmt);
                    if let Some(var_type) = &let_stmt.var_type {
                        println!("Variable type: {:#?}", var_type);
                    } else {
                        println!("No type annotation found");
                    }
                }
                Err(e) => println!("Parse error: {}", e),
            }
        }
        Err(e) => println!("Parser creation error: {}", e),
    }
}
