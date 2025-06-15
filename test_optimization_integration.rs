#!/usr/bin/env rust-script

/// Test script to verify real LLVM optimization integration works
/// 
/// This script tests that the real LLVM optimization passes are properly connected
/// to the CURSED compilation pipeline and that different optimization levels work.

use std::process::Command;
use std::fs;
use std::time::Instant;

fn main() {
    println!("🧪 Testing CURSED Real LLVM Optimization Integration");
    println!("=" .repeat(60));
    
    // Create a simple test CURSED program
    let test_program = r#"
slay main() {
    sus x = 10;
    sus y = 20;
    sus result = x + y + 5;
    facts message = "Hello, CURSED!";
    yeet result;
}
"#;
    
    // Write test program to file
    fs::write("test_optimization.csd", test_program).expect("Failed to write test program");
    
    println!("📝 Created test program: test_optimization.csd");
    println!("🎯 Testing different optimization levels...\n");
    
    let optimization_levels = vec![
        ("O0", "No optimization (debug)"),
        ("O1", "Minimal optimization"),
        ("O2", "Standard optimization (default)"),
        ("O3", "Aggressive optimization"),
        ("Os", "Size optimization"),
        ("Oz", "Aggressive size optimization"),
    ];
    
    for (level, description) in optimization_levels {
        println!("🔧 Testing optimization level: {} - {}", level, description);
        
        let start_time = Instant::now();
        
        // Test compilation with this optimization level
        let output = Command::new("cargo")
            .args(&["run", "--bin", "cursed", "--", "compile", "test_optimization.csd", "--optimization", level, "--verbose"])
            .output();
        
        let compile_time = start_time.elapsed();
        
        match output {
            Ok(result) => {
                if result.status.success() {
                    println!("  ✅ Compilation successful in {:?}", compile_time);
                    
                    // Check for optimization statistics in output
                    let stdout = String::from_utf8_lossy(&result.stdout);
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    
                    if stdout.contains("Real LLVM optimization") || stderr.contains("Real LLVM optimization") {
                        println!("  📊 Real LLVM optimization passes detected!");
                    }
                    
                    if stdout.contains("Optimization statistics") || stderr.contains("Optimization statistics") {
                        println!("  📈 Optimization statistics found!");
                    }
                    
                    // Check for specific optimization pass mentions
                    let combined_output = format!("{}{}", stdout, stderr);
                    if combined_output.contains("Functions inlined") {
                        println!("  🔄 Function inlining detected");
                    }
                    if combined_output.contains("Dead code eliminated") {
                        println!("  🗑️  Dead code elimination detected");
                    }
                    if combined_output.contains("Constants propagated") {
                        println!("  📐 Constant propagation detected");
                    }
                    if combined_output.contains("Loops unrolled") {
                        println!("  🔄 Loop unrolling detected");
                    }
                    
                } else {
                    println!("  ❌ Compilation failed");
                    println!("  Error: {}", String::from_utf8_lossy(&result.stderr));
                }
            }
            Err(e) => {
                println!("  ⚠️  Failed to run command: {}", e);
            }
        }
        
        println!();
    }
    
    // Test default optimization (should be O2)
    println!("🔧 Testing default optimization (should be O2)...");
    let start_time = Instant::now();
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed", "--", "compile", "test_optimization.csd", "--verbose"])
        .output();
    
    let compile_time = start_time.elapsed();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("  ✅ Default compilation successful in {:?}", compile_time);
                
                let combined_output = format!("{}{}", 
                    String::from_utf8_lossy(&result.stdout),
                    String::from_utf8_lossy(&result.stderr)
                );
                
                if combined_output.contains("Real LLVM optimization") {
                    println!("  📊 Real LLVM optimization passes active by default!");
                }
                
                if combined_output.contains("optimization level: O2") || combined_output.contains("release optimizations") {
                    println!("  🎯 Default optimization level confirmed as O2");
                }
            } else {
                println!("  ❌ Default compilation failed");
            }
        }
        Err(e) => {
            println!("  ⚠️  Failed to run default test: {}", e);
        }
    }
    
    // Cleanup
    let _ = fs::remove_file("test_optimization.csd");
    
    println!("\n🏁 Real LLVM Optimization Integration Test Complete!");
    println!("=" .repeat(60));
    
    // Summary
    println!("📋 Integration Summary:");
    println!("  • Real LLVM pass manager should be connected to compilation pipeline");
    println!("  • Optimization levels O0-O3, Os, Oz should work");
    println!("  • Default optimization should be O2 for release builds");
    println!("  • Optimization statistics should be logged");
    println!("  • Function inlining, DCE, constant propagation should be active");
    
    println!("\n💡 If you see optimization passes working above, the integration is successful!");
}
