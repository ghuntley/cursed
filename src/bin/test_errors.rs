//! Test application for demonstrating the structured error system

use cursed::error::{ErrorReporter, StructuredError, ErrorCode, ErrorSourceLocation};
use cursed::error::cli::{FileAwareErrorReporter, ErrorCliOptions};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::Path;
use std::fs;

fn main() {
    println!("🔥 CURSED Structured Error System Demo");
    println!("======================================");
    println!();
    
    // Test basic error reporting
    println!("1. Basic Error Reporting:");
    test_basic_error_reporting();
    
    println!();
    println!("2. Error Code Explanations:");
    test_error_explanations();
    
    println!();
    println!("3. Lexer Error Testing:");
    test_lexer_errors();
    
    println!();
    println!("4. Parser Error Testing:");
    test_parser_errors();
    
    println!();
    println!("5. File-based Error Testing:");
    test_file_errors();
}

fn test_basic_error_reporting() {
    let mut reporter = ErrorReporter::new();
    
    // Create some test errors
    let error1 = StructuredError::syntax_error("Missing semicolon")
        .with_location(ErrorSourceLocation {
            file: "test.csd".to_string(),
            line: 10,
            column: 15,
            length: 1,
            source_line: Some("    sus x = 42".to_string()),
        })
        .with_suggestions(vec!["Add a semicolon at the end of the statement".to_string()]);
    
    let error2 = StructuredError::type_mismatch("normie", "tea", 15, 20)
        .with_suggestions(vec![
            "Convert the string to a number".to_string(),
            "Change the variable type to 'tea'".to_string(),
        ]);
    
    reporter.add_error(error1);
    reporter.add_error(error2);
    
    reporter.print_all();
}

fn test_error_explanations() {
    let reporter = ErrorReporter::new();
    
    println!("Explaining E0001 (Unexpected Token):");
    reporter.print_explanation(ErrorCode::E0001);
    
    println!("Explaining E0100 (Type Mismatch):");
    reporter.print_explanation(ErrorCode::E0100);
}

fn test_lexer_errors() {
    println!("Testing lexer with broken CURSED code:");
    
    // Test with unterminated string
    let broken_code1 = r#"sus message = "unterminated string"#;
    test_code_with_lexer("Unterminated String", broken_code1);
    
    // Test with invalid character
    let broken_code2 = "sus x = 42 @@ invalid";
    test_code_with_lexer("Invalid Character", broken_code2);
    
    // Test with invalid escape sequence
    let broken_code3 = r#"sus text = "bad \q escape""#;
    test_code_with_lexer("Invalid Escape", broken_code3);
}

fn test_parser_errors() {
    println!("Testing parser with broken CURSED code:");
    
    // Test with missing closing brace
    let broken_code1 = r#"
    slay test() {
        sus x = 42
        fr fr missing closing brace
    "#;
    test_code_with_parser("Missing Closing Brace", broken_code1);
    
    // Test with unexpected token
    let broken_code2 = r#"
    slay test(( {
        sus x = 42
    }
    "#;
    test_code_with_parser("Unexpected Token", broken_code2);
}

fn test_file_errors() {
    if Path::new("test_error_demo.csd").exists() {
        println!("Testing with demo error file:");
        
        let options = ErrorCliOptions {
            no_color: false,
            max_errors: 10,
            explain: None,
            context_lines: 2,
            json: false,
            list_error_codes: false,
        };
        
        let mut reporter = FileAwareErrorReporter::new(&options);
        
        if let Ok(content) = fs::read_to_string("test_error_demo.csd") {
            test_file_content(&mut reporter, "test_error_demo.csd", &content);
            reporter.print_all();
        }
    } else {
        println!("Demo file test_error_demo.csd not found");
    }
}

fn test_code_with_lexer(description: &str, code: &str) {
    println!("  {} - Testing: {}", description, code.trim());
    
    let mut lexer = Lexer::new(code.to_string());
    match lexer.tokenize() {
        Ok(_) => println!("    ✅ No lexer errors (unexpected)"),
        Err(e) => println!("    ❌ Error: {}", e),
    }
    println!();
}

fn test_code_with_parser(description: &str, code: &str) {
    println!("  {} - Testing: {}", description, code.trim().replace('\n', " "));
    
    let mut lexer = Lexer::new(code.to_string());
    match Parser::new(lexer) {
        Ok(mut parser) => {
            match parser.parse() {
                Ok(_) => println!("    ✅ No parser errors (unexpected)"),
                Err(e) => println!("    ❌ Error: {}", e),
            }
        }
        Err(e) => println!("    ❌ Lexer error: {}", e),
    }
    println!();
}

fn test_file_content(reporter: &mut FileAwareErrorReporter, file_path: &str, content: &str) {
    // Create some mock errors based on the content analysis
    if content.contains("This string is missing a quote") {
        let error = StructuredError::unterminated_string(7, 20);
        reporter.add_error_with_file(error, file_path);
    }
    
    if content.contains("undefined_variable") {
        let error = StructuredError::unknown_variable("undefined_variable", 10, 17);
        reporter.add_error_with_file(error, file_path);
    }
    
    if content.contains("nonexistent_function") {
        let error = StructuredError::function_not_found("nonexistent_function", 16, 15);
        reporter.add_error_with_file(error, file_path);
    }
    
    if content.contains("Invalid \\q escape") {
        let error = StructuredError::invalid_escape_sequence("q", 19, 25);
        reporter.add_error_with_file(error, file_path);
    }
    
    if content.contains("no cap") && !content.contains("on god") {
        let error = StructuredError::unterminated_block_comment(26, 5);
        reporter.add_error_with_file(error, file_path);
    }
}
