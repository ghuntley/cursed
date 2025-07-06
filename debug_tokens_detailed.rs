use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <cursed_file>", args[0]);
        return;
    }

    let filename = &args[1];
    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            return;
        }
    };

    println!("=== ANALYZING FILE: {} ===", filename);
    println!("Content:");
    for (i, line) in content.lines().enumerate() {
        println!("{:2}: {}", i + 1, line);
    }
    println!();

    // Create lexer and analyze tokens
    let mut lexer = cursed::lexer::Lexer::new(&content);
    
    println!("=== TOKEN ANALYSIS ===");
    let mut tokens = Vec::new();
    let mut token_count = 0;
    
    loop {
        let token = lexer.next_token();
        token_count += 1;
        println!("#{:2} {:?}", token_count, token);
        
        if matches!(token, cursed::lexer::Token::EOF) {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    
    println!();
    println!("=== PARSER SIMULATION ===");
    
    // Simulate parsing manually to understand where it fails
    let mut position = 0;
    while position < tokens.len() {
        let token = &tokens[position];
        println!("Position {}: Processing token {:?}", position, token);
        
        match token {
            cursed::lexer::Token::Identifier(name) if name == "vibe" => {
                println!("  -> Found 'vibe' declaration");
                position += 1;
                if position < tokens.len() {
                    if let cursed::lexer::Token::Identifier(target) = &tokens[position] {
                        println!("  -> Target: {}", target);
                        position += 1;
                    }
                }
            },
            cursed::lexer::Token::Identifier(name) if name == "slay" => {
                println!("  -> Found 'slay' function declaration");
                position += 1;
                // Expect function name
                if position < tokens.len() {
                    if let cursed::lexer::Token::Identifier(func_name) = &tokens[position] {
                        println!("  -> Function name: {}", func_name);
                        position += 1;
                    }
                }
                // Expect (
                if position < tokens.len() {
                    if let cursed::lexer::Token::LeftParen = &tokens[position] {
                        println!("  -> Found opening paren");
                        position += 1;
                    }
                }
                // Expect )
                if position < tokens.len() {
                    if let cursed::lexer::Token::RightParen = &tokens[position] {
                        println!("  -> Found closing paren");
                        position += 1;
                    }
                }
                // Expect {
                if position < tokens.len() {
                    if let cursed::lexer::Token::LeftBrace = &tokens[position] {
                        println!("  -> Found opening brace");
                        position += 1;
                    }
                }
            },
            cursed::lexer::Token::Identifier(name) if name == "sus" => {
                println!("  -> Found 'sus' variable declaration");
                position += 1;
                // Expect variable name
                if position < tokens.len() {
                    if let cursed::lexer::Token::Identifier(var_name) = &tokens[position] {
                        println!("  -> Variable name: {}", var_name);
                        position += 1;
                    }
                }
                // Expect =
                if position < tokens.len() {
                    if let cursed::lexer::Token::Assign = &tokens[position] {
                        println!("  -> Found assignment operator");
                        position += 1;
                    }
                }
                // Expect value
                if position < tokens.len() {
                    println!("  -> Expecting value, found: {:?}", &tokens[position]);
                    position += 1;
                }
                // Expect ;
                if position < tokens.len() {
                    if let cursed::lexer::Token::Semicolon = &tokens[position] {
                        println!("  -> Found semicolon");
                        position += 1;
                    }
                }
            },
            cursed::lexer::Token::Identifier(name) if name == "yolo" => {
                println!("  -> Found 'yolo' print statement");
                position += 1;
                // Expect expression
                if position < tokens.len() {
                    println!("  -> Expecting expression, found: {:?}", &tokens[position]);
                    position += 1;
                }
                // Expect ;
                if position < tokens.len() {
                    if let cursed::lexer::Token::Semicolon = &tokens[position] {
                        println!("  -> Found semicolon");
                        position += 1;
                    }
                }
            },
            cursed::lexer::Token::RightBrace => {
                println!("  -> Found closing brace");
                position += 1;
            },
            cursed::lexer::Token::EOF => {
                println!("  -> End of file");
                break;
            },
            _ => {
                println!("  -> Unexpected token");
                position += 1;
            }
        }
    }
}
