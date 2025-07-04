#!/usr/bin/env rust

//! Test the CursedLinter integration with the actual AST

use cursed::{CursedLinter, LinterConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing CursedLinter integration...");

    // Create a linter with default configuration
    let mut linter = CursedLinter::new();
    
    // Test with simple CURSED code
    let source_code = r#"
slay_hello_world() {
    facts message = "Hello, World!";
    print(message);
    normie count = 42;
    periodt (count > 0) {
        print("Count is positive");
    }
}

slay_main() {
    slay_hello_world();
}
"#;

    // Run linting analysis
    match linter.lint_source(source_code) {
        Ok(result) => {
            println!("✅ Linter analysis successful!");
            println!("Total issues found: {}", result.issues.len());
            println!("Files analyzed: {}", result.summary.files_analyzed);
            println!("Lines analyzed: {}", result.summary.lines_analyzed);
            println!("Analysis time: {}ms", result.metrics.analysis_time_ms);
            
            // Display any issues found
            for issue in &result.issues {
                println!("  {}: {} (line {})", 
                    issue.severity.as_str().to_uppercase(),
                    issue.message,
                    issue.line
                );
            }
        },
        Err(e) => {
            println!("❌ Linter analysis failed: {}", e);
            return Err(e.into());
        }
    }

    // Test with a simple valid program
    let simple_code = r#"
slay_main() {
    facts x = 1;
    print(x);
}
"#;

    match linter.lint_source(simple_code) {
        Ok(result) => {
            println!("✅ Simple code analysis successful!");
            println!("Issues: {}", result.issues.len());
        },
        Err(e) => {
            println!("❌ Simple code analysis failed: {}", e);
        }
    }

    println!("✅ All CursedLinter integration tests completed!");
    Ok(())
}
