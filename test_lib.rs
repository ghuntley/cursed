// Minimal test to use the library directly

extern crate cursed;

fn main() {
    println!("🔤 Testing CURSED library directly");
    
    let source = r#"
facts x = 42;
facts name = "CURSED";
"#;
    
    // Test tokenization
    match cursed::tokenize(source) {
        Ok(tokens) => {
            println!("✅ Tokenized successfully: {} tokens", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                println!("  {}: {:?} '{}'", i + 1, token.token_type, token.literal);
            }
        }
        Err(e) => {
            println!("❌ Tokenization failed: {}", e);
        }
    }
}
