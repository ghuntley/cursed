use std::fs;

// Add the src path to use the lexer module
mod lexer {
    include!("../src/lexer.rs");
}

use lexer::{Lexer, TokenType};

fn main() {
    let input = fs::read_to_string("test_lexer_validation.csd")
        .expect("Failed to read test file");

    let mut lexer = Lexer::new(&input);
    
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("=== LEXER VALIDATION RESULTS ===\n");
            
            let mut keyword_count = 0;
            let mut comment_count = 0;
            let mut literal_count = 0;
            
            for token in &tokens {
                match token.token_type {
                    TokenType::Slay | TokenType::Yolo | TokenType::Sus | TokenType::Facts |
                    TokenType::Lowkey | TokenType::Highkey | TokenType::Periodt | TokenType::Stan |
                    TokenType::Bestie | TokenType::Flex | TokenType::Ghosted | TokenType::Simp |
                    TokenType::Squad | TokenType::Collab | TokenType::Vibe | TokenType::Yeet |
                    TokenType::BeLike | TokenType::VibeCheck | TokenType::Mood | TokenType::Basic |
                    TokenType::YeetError | TokenType::Catch | TokenType::Normie | TokenType::Tea |
                    TokenType::Cap | TokenType::NoCap | TokenType::MainCharacter | TokenType::Dm => {
                        keyword_count += 1;
                        println!("✅ Keyword: {} -> {:?}", token.literal, token.token_type);
                    }
                    TokenType::Comment => {
                        comment_count += 1;
                        println!("✅ Comment: {}", token.literal.chars().take(50).collect::<String>());
                    }
                    TokenType::Integer | TokenType::Float | TokenType::String | TokenType::Boolean => {
                        literal_count += 1;
                        println!("✅ Literal: {} -> {:?}", token.literal, token.token_type);
                    }
                    TokenType::Eof => {
                        println!("✅ End of file reached");
                    }
                    _ => {
                        // Other tokens like operators, delimiters, etc.
                    }
                }
            }
            
            println!("\n=== SUMMARY ===");
            println!("Total tokens: {}", tokens.len());
            println!("Keywords found: {}", keyword_count);
            println!("Comments found: {}", comment_count);
            println!("Literals found: {}", literal_count);
            
            // Check for specific Gen Z keywords
            let gen_z_keywords = [
                ("vibe", TokenType::Vibe),
                ("slay", TokenType::Slay),
                ("yolo", TokenType::Yolo),
                ("sus", TokenType::Sus),
                ("facts", TokenType::Facts),
                ("lowkey", TokenType::Lowkey),
                ("highkey", TokenType::Highkey),
                ("based", TokenType::Boolean),
                ("cap", TokenType::Cap),
                ("yeet_error", TokenType::YeetError),
                ("tea", TokenType::Tea),
            ];
            
            println!("\n=== GEN Z KEYWORD VALIDATION ===");
            for (keyword, expected_type) in gen_z_keywords {
                let found = tokens.iter().any(|t| t.literal == keyword && t.token_type == expected_type);
                if found {
                    println!("✅ '{}' correctly tokenized as {:?}", keyword, expected_type);
                } else {
                    println!("❌ '{}' not found or incorrectly tokenized", keyword);
                }
            }
            
        }
        Err(e) => {
            println!("❌ Lexer error: {:?}", e);
        }
    }
}
