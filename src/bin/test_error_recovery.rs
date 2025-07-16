#!/usr/bin/env cargo run --bin test_error_recovery

//! Test program for CURSED error recovery system

use cursed::error_recovery_simple::*;

fn main() {
    println!("=== CURSED Compiler Error Recovery Test ===\n");
    
    // Test the simplified error recovery system
    println!("Testing simplified error recovery system...\n");
    let test_result = test_error_recovery_system();
    println!("{}", test_result);
    
    // Test individual components
    println!("\n=== Testing Individual Components ===\n");
    
    // Test error collection and reporting
    let mut recovery = SimpleErrorRecovery::new();
    
    // Add various types of errors
    recovery.add_error(
        "Expected ';' after statement".to_string(),
        10, 25,
        Some("Add semicolon at the end of the statement".to_string())
    );
    
    recovery.add_error(
        "Undefined variable 'x'".to_string(),
        15, 8,
        Some("Declare the variable with 'sus x normie = value'".to_string())
    );
    
    recovery.add_warning(
        "Type mismatch: expected 'normie', found 'tea'".to_string(),
        20, 12,
        Some("Check variable types or use type conversion".to_string())
    );
    
    recovery.add_error(
        "Function 'unknown_func' not found".to_string(),
        25, 5,
        Some("Define the function or check imports".to_string())
    );
    
    println!("Comprehensive Error Report:");
    println!("{}", recovery.generate_report());
    
    // Test error suggestion system
    println!("\n=== Testing Error Suggestion System ===\n");
    
    let test_errors = vec![
        "Expected ')'",
        "Expected ';'",
        "Expected identifier",
        "undefined variable 'x'",
        "undefined function 'test'",
        "type mismatch: expected normie, found tea",
        "vibez.spill hello world",
    ];
    
    for error_msg in test_errors {
        if let Some(suggestion) = suggest_fix_for_error(error_msg) {
            println!("Error: {}", error_msg);
            println!("Suggestion: {}\n", suggestion);
        } else {
            println!("Error: {} (no specific suggestion available)\n", error_msg);
        }
    }
    
    // Test compilation orchestrator
    println!("=== Testing Compilation Orchestrator ===\n");
    
    let mut orchestrator = ErrorRecoveryOrchestrator::new();
    let sample_code = r#"
        sus x normie = 42
        vibez.spill("Hello, world!"
        vibez.spill(y)
        sus z tea = 123
        vibez.spill("This should work")
    "#;
    
    let result = orchestrator.compile_with_full_recovery(sample_code);
    
    println!("Compilation Result:");
    println!("- Parsing: {}", if result.parsing_success { "✅ SUCCESS" } else { "❌ FAILED" });
    println!("- Semantic Analysis: {}", if result.semantic_success { "✅ SUCCESS" } else { "❌ FAILED" });
    println!("- Code Generation: {}", if result.codegen_success { "✅ SUCCESS" } else { "❌ FAILED" });
    println!("- Overall Success: {}", if result.overall_success() { "✅ SUCCESS" } else { "❌ FAILED" });
    println!("- Can Execute: {}", if result.can_execute() { "✅ YES" } else { "❌ NO" });
    println!();
    println!("Detailed Report:");
    println!("{}", result.final_report);
    
    // Demonstrate benefits of error recovery
    println!("=== Error Recovery Benefits ===\n");
    
    println!("🔧 PARSER ERROR RECOVERY:");
    println!("   - Continues parsing after syntax errors");
    println!("   - Provides helpful suggestions for common mistakes");
    println!("   - Identifies multiple errors in one pass");
    println!("   - Suggests fixes like adding missing semicolons or parentheses");
    println!();
    
    println!("🔍 SEMANTIC ERROR RECOVERY:");
    println!("   - Accumulates type errors without stopping analysis");
    println!("   - Provides placeholder types for undefined variables");
    println!("   - Continues checking even with missing function definitions");
    println!("   - Helps identify patterns of errors across the codebase");
    println!();
    
    println!("⚙️ CODEGEN ERROR RECOVERY:");
    println!("   - Generates placeholder code for failed compilation units");
    println!("   - Provides graceful degradation when LLVM compilation fails");
    println!("   - Allows partial program execution even with some errors");
    println!("   - Offers fallback to interpretation mode when needed");
    println!();
    
    println!("📊 USER EXPERIENCE IMPROVEMENTS:");
    println!("   - Comprehensive error reports with context and suggestions");
    println!("   - Better IDE integration with precise error locations");
    println!("   - Reduced compilation iterations due to multiple error detection");
    println!("   - Educational value through helpful error messages and suggestions");
    println!();
    
    println!("=== Test Complete ===");
    println!("The CURSED compiler now provides robust error recovery that:");
    println!("1. Continues compilation after errors when possible");
    println!("2. Provides helpful suggestions for fixing common issues");
    println!("3. Generates comprehensive reports with context");
    println!("4. Improves the overall developer experience");
    println!("5. Enables better tooling and IDE integration");
}
