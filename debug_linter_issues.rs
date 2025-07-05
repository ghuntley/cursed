use cursed::tools::linter::CursedLinter;

fn main() {
    let mut linter = CursedLinter::default();
    
    // Test basic functionality
    let source1 = r#"
vibe test

slay main() {
    sus x = 42
    print(x)
}
"#;
    
    println!("=== Testing basic functionality ===");
    match linter.lint_source(source1) {
        Ok(results) => {
            println!("Found {} issues:", results.issues.len());
            for issue in &results.issues {
                println!("  {}: {} ({}:{})", 
                    issue.rule_id, 
                    issue.message, 
                    issue.line,
                    issue.column
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    // Test Go-style keywords
    let source2 = "func main() { var x = 42; return x; }";
    
    println!("\n=== Testing Go-style keyword detection ===");
    match linter.lint_source(source2) {
        Ok(results) => {
            println!("Found {} issues:", results.issues.len());
            for issue in &results.issues {
                println!("  {}: {} ({}:{})", 
                    issue.rule_id, 
                    issue.message, 
                    issue.line,
                    issue.column
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    // Test line length
    let source3 = "sus very_long_variable_name_that_definitely_exceeds_normal_limits = 42";
    
    println!("\n=== Testing line length detection ===");
    match linter.lint_source(source3) {
        Ok(results) => {
            println!("Found {} issues:", results.issues.len());
            for issue in &results.issues {
                println!("  {}: {} ({}:{})", 
                    issue.rule_id, 
                    issue.message, 
                    issue.line,
                    issue.column
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
