use cursed::parser::Parser;
use cursed::lexer::Lexer;

fn main() {
    println!("=== CURSED Parser Grammar Validation ===\n");
    
    // Test cases based on grammar specification
    let test_cases = vec![
        // 1. Package declaration
        ("vibe main", "Package declaration"),
        
        // 2. Import statements
        ("yeet \"fmt\"", "Simple import"),
        ("yeet tea \"strings\"", "Aliased import"),
        
        // 3. Constant declaration
        ("facts PI = 3.14", "Constant declaration"),
        ("facts (\n    PI = 3.14\n    E = 2.71\n)", "Multiple constants"),
        
        // 4. Variable declaration
        ("sus x = 5", "Variable declaration"),
        ("sus name tea = \"World\"", "Typed variable"),
        ("sus a, b = 1, 2", "Multiple variables"),
        
        // 5. Type declaration
        ("be_like Person squad {\n    name tea\n    age normie\n}", "Struct type"),
        
        // 6. Function declaration
        ("slay add(x, y normie) normie {\n    yolo x + y\n}", "Function declaration"),
        
        // 7. If statement
        ("lowkey x > 0 {\n    print(\"positive\")\n}", "If statement"),
        ("lowkey x > 0 {\n    print(\"pos\")\n} highkey {\n    print(\"neg\")\n}", "If-else"),
        
        // 8. Switch statement
        ("vibe_check x {\n    mood 1:\n        print(\"one\")\n    basic:\n        print(\"other\")\n}", "Switch statement"),
        
        // 9. For statement
        ("bestie i := 0; i < 10; i++ {\n    print(i)\n}", "For loop"),
        ("bestie _, v := flex arr {\n    print(v)\n}", "Range for"),
        
        // 10. While statement
        ("periodt x > 0 {\n    x--\n}", "While loop"),
        
        // 11. Return statement
        ("yolo x + y", "Return statement"),
        
        // 12. Control flow
        ("ghosted", "Break statement"),
        ("simp", "Continue statement"),
        
        // 13. Goroutine
        ("stan doWork()", "Goroutine"),
        
        // 14. Complex expression
        ("x.field[0](a, b)", "Complex expression"),
        
        // 15. Complete program
        ("vibe main\n\nyeet \"fmt\"\n\nslay main() {\n    print(\"Hello\")\n}", "Complete program"),
    ];
    
    let mut passed = 0;
    let mut failed = 0;
    
    for (test_code, description) in test_cases {
        print!("Testing {}: ", description);
        
        match test_parse(test_code) {
            Ok(_) => {
                println!("✓ PASS");
                passed += 1;
            }
            Err(e) => {
                println!("✗ FAIL - {}", e);
                failed += 1;
            }
        }
    }
    
    println!("\n=== Grammar Validation Results ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    println!("Total:  {}", passed + failed);
    
    if failed == 0 {
        println!("\n🎉 All grammar tests passed!");
    } else {
        println!("\n⚠️  {} tests failed", failed);
    }
    
    // Test the complete program file
    println!("\n=== Testing Complete Program File ===");
    test_complete_program();
}

fn test_parse(source: &str) -> Result<String, String> {
    let mut parser = Parser::from_source(source)
        .map_err(|e| format!("Parser creation failed: {}", e))?;
    
    match parser.parse_program() {
        Ok(program) => Ok(format!("Parsed successfully: {} statements", program.statements.len())),
        Err(e) => Err(format!("Parse error: {}", e)),
    }
}

fn test_complete_program() {
    let program_content = std::fs::read_to_string("test_parser_validation.csd")
        .unwrap_or_else(|_| {
            println!("Could not read test file, using inline content");
            r#"
vibe main

yeet "fmt"

facts PI = 3.14159

sus name tea = "World"

slay main() {
    lowkey name == "World" {
        print("Hello, World!")
    }
    yolo 0
}
"#.to_string()
        });
        
    match test_parse(&program_content) {
        Ok(result) => println!("✓ Complete program parsed: {}", result),
        Err(e) => println!("✗ Complete program failed: {}", e),
    }
}
