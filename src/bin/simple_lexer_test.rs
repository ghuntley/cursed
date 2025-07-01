//! Simple test to verify CURSED lexer and basic parsing without external dependencies

use std::fs;

// We'll include the core components directly
use cursed::lexer::Lexer;

fn main() {
    println!("🚀 CURSED Lexer Test");
    println!("==================\n");
    
    // Read the demo file
    let demo_content = match fs::read_to_string("demo_cursed_hello.csd") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Failed to read demo file: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("📄 Demo file content (first 500 chars):");
    println!("{}", demo_content.chars().take(500).collect::<String>());
    if demo_content.len() > 500 {
        println!("... ({} more characters)", demo_content.len() - 500);
    }
    println!("\n{}", "=".repeat(50));
    
    // Test basic tokenization
    println!("\n🔍 Testing tokenization...");
    let mut lexer = Lexer::new(demo_content);
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("✅ Tokenization successful!");
            println!("📊 Found {} total tokens", tokens.len());
            
            // Count key token types
            let mut keyword_count = 0;
            let mut identifier_count = 0;
            let mut string_count = 0;
            let mut number_count = 0;
            let mut operator_count = 0;
            
            println!("\n📋 First 20 tokens:");
            for (i, token) in tokens.iter().take(20).enumerate() {
                println!("  {}: {:?} = '{}'", i, token.kind, token.lexeme);
                
                // Count by type for analysis
                use cursed::lexer::TokenKind;
                match token.kind {
                    TokenKind::Vibe | TokenKind::Slay | TokenKind::Sus | TokenKind::Yolo |
                    TokenKind::Lowkey | TokenKind::Highkey | TokenKind::Yeet | TokenKind::Facts => {
                        keyword_count += 1;
                    },
                    TokenKind::Identifier => identifier_count += 1,
                    TokenKind::String => string_count += 1,
                    TokenKind::Number => number_count += 1,
                    TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Equal => {
                        operator_count += 1;
                    },
                    _ => {},
                }
            }
            
            if tokens.len() > 20 {
                println!("  ... and {} more tokens", tokens.len() - 20);
            }
            
            println!("\n📊 Token type analysis:");
            println!("  CURSED Keywords: {}", keyword_count);
            println!("  Identifiers: {}", identifier_count);
            println!("  Strings: {}", string_count);
            println!("  Numbers: {}", number_count);
            println!("  Operators: {}", operator_count);
            
            // Look for specific expected tokens from the demo
            println!("\n🔍 Searching for key language features...");
            
            let expected_keywords = ["vibe", "main", "slay", "sus", "yolo", "lowkey", "highkey", "vibez"];
            for keyword in &expected_keywords {
                let found = tokens.iter().any(|t| t.lexeme == *keyword);
                let status = if found { "✅" } else { "❌" };
                println!("  {} Keyword '{}': {}", status, keyword, if found { "Found" } else { "Missing" });
            }
            
            let expected_identifiers = ["calculateArea", "greetUser", "demonstrateBasics", "radius", "name"];
            for identifier in &expected_identifiers {
                let found = tokens.iter().any(|t| t.lexeme == *identifier);
                let status = if found { "✅" } else { "❌" };
                println!("  {} Identifier '{}': {}", status, identifier, if found { "Found" } else { "Missing" });
            }
            
            let string_literals: Vec<_> = tokens.iter()
                .filter(|t| matches!(t.kind, cursed::lexer::TokenKind::String))
                .map(|t| &t.lexeme)
                .collect();
            
            println!("\n📝 Found {} string literals:", string_literals.len());
            for (i, string) in string_literals.iter().take(5).enumerate() {
                println!("  {}: \"{}\"", i + 1, string);
            }
            if string_literals.len() > 5 {
                println!("  ... and {} more", string_literals.len() - 5);
            }
            
            // Count functions by looking for patterns
            let mut potential_functions = 0;
            for i in 0..tokens.len().saturating_sub(2) {
                if tokens[i].lexeme == "slay" && 
                   matches!(tokens[i + 1].kind, cursed::lexer::TokenKind::Identifier) &&
                   matches!(tokens[i + 2].kind, cursed::lexer::TokenKind::LeftParen) {
                    potential_functions += 1;
                    println!("  📝 Function definition found: {}", tokens[i + 1].lexeme);
                }
            }
            
            println!("\n🎯 Analysis summary:");
            println!("  Total tokens: {}", tokens.len());
            println!("  CURSED keywords found: {}", keyword_count);
            println!("  Potential functions: {}", potential_functions);
            println!("  String literals: {}", string_literals.len());
            
            let success_score = (keyword_count > 10) as i32 + 
                               (potential_functions >= 4) as i32 + 
                               (string_literals.len() > 5) as i32 +
                               (identifier_count > 10) as i32;
            
            println!("\n🏆 Success score: {}/4", success_score);
            match success_score {
                4 => println!("🎉 EXCELLENT: All major language features tokenized correctly!"),
                3 => println!("✅ GOOD: Most language features working well"),
                2 => println!("⚠️ PARTIAL: Some issues with tokenization"),
                _ => println!("❌ NEEDS WORK: Significant tokenization problems"),
            }
            
        },
        Err(e) => {
            eprintln!("❌ Tokenization failed: {}", e);
            std::process::exit(1);
        }
    }
    
    println!("\n{}", "=".repeat(50));
    println!("Lexer test completed");
}
