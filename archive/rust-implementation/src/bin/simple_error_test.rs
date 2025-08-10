//! Simple test for demonstrating basic error improvements

use cursed::error::{ErrorReporter, StructuredError, ErrorCode};
use cursed::lexer::Lexer;

fn main() {
    println!("🔥 CURSED Simple Error System Demo");
    println!("==================================");
    
    // Test 1: Basic structured error
    println!("\n1. Basic Structured Error:");
    let error = StructuredError::syntax_error("Missing semicolon at end of statement");
    println!("   {}: {}", error.code.as_str(), error.message);
    
    // Test 2: Error with suggestions
    println!("\n2. Error with Suggestions:");
    let error = StructuredError::unknown_variable("myVar", 10, 5)
        .with_suggestions(vec!["Did you mean 'myVariable'?".to_string()]);
    println!("   {}: {}", error.code.as_str(), error.message);
    for suggestion in &error.suggestions {
        println!("   Help: {}", suggestion);
    }
    
    // Test 3: Type mismatch error
    println!("\n3. Type Mismatch Error:");
    let error = StructuredError::type_mismatch("normie", "tea", 15, 20);
    println!("   {}: {}", error.code.as_str(), error.message);
    
    // Test 4: Error reporter
    println!("\n4. Error Reporter:");
    let mut reporter = ErrorReporter::new();
    reporter.add_error(StructuredError::syntax_error("First error"));
    reporter.add_error(StructuredError::type_error("Second error"));
    println!("   Total errors: {}", reporter.error_count());
    
    // Test 5: Lexer error
    println!("\n5. Lexer Error Demo:");
    let broken_code = r#"sus x = "unterminated string"#;
    let mut lexer = Lexer::new(broken_code.to_string());
    match lexer.tokenize() {
        Ok(_) => println!("   ✅ No errors found"),
        Err(e) => println!("   ❌ Error: {}", e),
    }
    
    println!("\n✅ Error system demonstration complete!");
}
