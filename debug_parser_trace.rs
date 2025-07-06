use cursed::lexer::Lexer;
use cursed::parser::Parser;

fn main() {
    let content = "yolo x;";
    
    println!("=== DEBUGGING YOLO STATEMENT ===");
    println!("Content: {}", content);
    
    let lexer = Lexer::new(content.to_string());
    let mut parser = match Parser::new(lexer) {
        Ok(p) => p,
        Err(e) => {
            println!("Parser creation failed: {:?}", e);
            return;
        }
    };
    
    match parser.parse() {
        Ok(program) => {
            println!("✓ Parsed successfully: {:?}", program);
        },
        Err(e) => {
            println!("✗ Parse failed: {:?}", e);
        }
    }
    
    println!("\n=== DEBUGGING SUS STATEMENT ===");
    let content2 = "sus x = 42;";
    println!("Content: {}", content2);
    
    let lexer2 = Lexer::new(content2.to_string());
    let mut parser2 = match Parser::new(lexer2) {
        Ok(p) => p,
        Err(e) => {
            println!("Parser creation failed: {:?}", e);
            return;
        }
    };
    
    match parser2.parse() {
        Ok(program) => {
            println!("✓ Parsed successfully: {:?}", program);
        },
        Err(e) => {
            println!("✗ Parse failed: {:?}", e);
        }
    }
    
    println!("\n=== DEBUGGING SIMPLE EXPRESSION ===");
    let content3 = "x";
    println!("Content: {}", content3);
    
    let lexer3 = Lexer::new(content3.to_string());
    let mut parser3 = match Parser::new(lexer3) {
        Ok(p) => p,
        Err(e) => {
            println!("Parser creation failed: {:?}", e);
            return;
        }
    };
    
    match parser3.parse() {
        Ok(program) => {
            println!("✓ Parsed successfully: {:?}", program);
        },
        Err(e) => {
            println!("✗ Parse failed: {:?}", e);
        }
    }
}
