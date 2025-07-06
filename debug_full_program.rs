use cursed::lexer::Lexer;
use cursed::parser::Parser;

fn main() {
    let content = "vibe main

slay main() {
    sus x = 42;
    yolo x;
}";
    
    println!("=== DEBUGGING FULL PROGRAM ===");
    for (i, line) in content.lines().enumerate() {
        println!("{:2}: {}", i + 1, line);
    }
    println!();
    
    // First, check tokenization
    let mut lexer_for_tokens = Lexer::new(content.to_string());
    match lexer_for_tokens.tokenize() {
        Ok(tokens) => {
            println!("✓ Tokenization successful ({} tokens)", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                println!("  #{:2} {:?} '{}' at line {}, col {}", 
                         i + 1, token.kind, token.lexeme, token.line, token.column);
            }
        },
        Err(e) => {
            println!("✗ Tokenization failed: {:?}", e);
            return;
        }
    }
    
    println!("\n=== PARSER ANALYSIS ===");
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
}
