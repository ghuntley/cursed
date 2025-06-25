// Test program to verify minimal CURSED functionality

fn main() {
    // Test the minimal lexer and parser
    let source = r#"
facts x = 42;
facts name = "CURSED";
slay greet(name) {
    facts greeting = "Hello";
}
"#;

    println!("🔤 Testing Minimal CURSED Functionality");
    println!("Source code:");
    println!("{}", source);
    println!();

    // Test tokenization
    println!("🔍 Tokenizing...");
    match cursed::tokenize(source) {
        Ok(tokens) => {
            println!("✅ Found {} tokens:", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                println!("  {}: {:?} '{}'", i + 1, token.token_type, token.literal);
            }
        }
        Err(e) => {
            println!("❌ Tokenization failed: {}", e);
            return;
        }
    }
    println!();

    // Test parsing
    println!("🔍 Parsing...");
    match cursed::parse(source) {
        Ok(program) => {
            println!("✅ Parsed successfully!");
            println!("Program has {} statements:", program.statements.len());
            for (i, stmt) in program.statements.iter().enumerate() {
                println!("  {}: {:?}", i + 1, stmt);
            }
        }
        Err(e) => {
            println!("❌ Parsing failed: {}", e);
            return;
        }
    }
    println!();

    // Test basic functionality
    println!("🎯 Testing basic run functionality...");
    match cursed::run(source) {
        Ok(_) => {
            println!("✅ Run completed successfully!");
        }
        Err(e) => {
            println!("❌ Run failed: {}", e);
        }
    }
    println!();

    println!("🎉 Minimal CURSED functionality test completed!");
}
