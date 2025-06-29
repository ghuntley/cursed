use cursed::lexer::{Lexer, TokenKind};
use cursed::parser::Parser;

fn test_lexer() {
    println!("Testing CURSED lexer with Gen Z keywords...");
    let source = "slay vibe_check(sus count) { yolo count; }";
    let mut lexer = Lexer::new(source.to_string());
    
    match lexer.tokenize() {
        Ok(tokens) => {
            for token in tokens {
                println!("Token: {:?} -> '{}'", token.kind, token.lexeme);
            }
        }
        Err(e) => println!("Lexer error: {:?}", e),
    }
}

fn test_parser() {
    println!("\nTesting CURSED parser with basic program...");
    let source = r#"
    slay hello() {
        sus x = 42;
        yolo x;
    }
    "#;
    
    let lexer = Lexer::new(source.to_string());
    match Parser::new(lexer) {
        Ok(mut parser) => {
            match parser.parse_program() {
                Ok(program) => {
                    println!("Successfully parsed program with {} statements", program.statements.len());
                    println!("AST: {:#?}", program);
                }
                Err(e) => println!("Parser error: {:?}", e),
            }
        }
        Err(e) => println!("Parser creation error: {:?}", e),
    }
}

fn test_member_access() {
    println!("\nTesting member access parsing (vibez.spill())...");
    let source = "vibez.spill()";
    
    let lexer = Lexer::new(source.to_string());
    match Parser::new(lexer) {
        Ok(mut parser) => {
            match parser.parse_expression() {
                Ok(expr) => {
                    println!("Successfully parsed member access: {:#?}", expr);
                }
                Err(e) => println!("Member access parse error: {:?}", e),
            }
        }
        Err(e) => println!("Parser creation error: {:?}", e),
    }
}

fn main() {
    test_lexer();
    test_parser();
    test_member_access();
}
