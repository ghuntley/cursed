#!/usr/bin/env rust-script

//! Test complete LLVM code generation functionality

use std::process::Command;

fn main() {
    println!("Testing CURSED LLVM Complete Code Generation");
    
    // Test 1: Function compilation
    let test_program = r#"
fn add(a: int, b: int) -> int {
    return a + b;
}

fn main() {
    let result = add(5, 3);
    vibez.spill(result);
    return 0;
}
"#;
    
    // Create test file
    std::fs::write("test_complete_func.csd", test_program).unwrap();
    
    // Test compilation
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed", "--", "test_complete_func.csd", "--emit", "llvm-ir"])
        .output()
        .expect("Failed to execute command");
    
    if output.status.success() {
        println!("✅ Function compilation test passed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Check for complete function IR
        if stdout.contains("define i32 @add(") && 
           stdout.contains("define i32 @main(") &&
           stdout.contains("call i32 @add(") &&
           stdout.contains("ret i32") {
            println!("✅ Complete function IR generation verified");
        } else {
            println!("❌ Function IR incomplete");
            println!("Generated IR:\n{}", stdout);
        }
    } else {
        println!("❌ Function compilation test failed");
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Test 2: Expression compilation
    let test_expr_program = r#"
fn test_expressions() {
    let a = 10;
    let b = 20;
    let sum = a + b;
    let product = a * b;
    let comparison = a < b;
    
    vibez.spill(sum);
    vibez.spill(product);
    vibez.spill(comparison);
}
"#;
    
    std::fs::write("test_complete_expr.csd", test_expr_program).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed", "--", "test_complete_expr.csd", "--emit", "llvm-ir"])
        .output()
        .expect("Failed to execute command");
    
    if output.status.success() {
        println!("✅ Expression compilation test passed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Check for complete expression IR
        if stdout.contains("add i32") && 
           stdout.contains("mul i32") &&
           stdout.contains("icmp") &&
           stdout.contains("alloca") &&
           stdout.contains("store") &&
           stdout.contains("load") {
            println!("✅ Complete expression IR generation verified");
        } else {
            println!("❌ Expression IR incomplete");
            println!("Generated IR:\n{}", stdout);
        }
    } else {
        println!("❌ Expression compilation test failed");
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Test 3: Member access compilation
    let test_member_program = r#"
fn test_member_access() {
    vibez.spill("Testing member access");
    vibez.spillf("Number: %d\n", 42);
}
"#;
    
    std::fs::write("test_complete_member.csd", test_member_program).unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed", "--", "test_complete_member.csd", "--emit", "llvm-ir"])
        .output()
        .expect("Failed to execute command");
    
    if output.status.success() {
        println!("✅ Member access compilation test passed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Check for member access IR
        if stdout.contains("@puts") || stdout.contains("@printf") {
            println!("✅ Complete member access IR generation verified");
        } else {
            println!("❌ Member access IR incomplete");
            println!("Generated IR:\n{}", stdout);
        }
    } else {
        println!("❌ Member access compilation test failed");
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Cleanup
    let _ = std::fs::remove_file("test_complete_func.csd");
    let _ = std::fs::remove_file("test_complete_expr.csd");
    let _ = std::fs::remove_file("test_complete_member.csd");
    
    println!("\n🎯 LLVM Complete Code Generation Test Completed");
}
