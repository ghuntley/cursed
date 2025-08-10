#!/usr/bin/env cargo run --bin cursed_demo

//! CURSED Language Demo Binary
//! 
//! Demonstrates the core functionality of the CURSED programming language
//! including Gen-Z syntax keywords and traditional constructs.

use std::env;
use std::process;
use cursed;

fn main() {
    // Initialize CURSED runtime (includes logging)
    cursed::init();
    
    let args: Vec<String> = env::args().collect();
    
    // Determine which demo to run
    let demo_file = if args.len() > 1 {
        &args[1]
    } else {
        "cursed_simple_demo.csd"
    };
    
    println!("🚀 CURSED Language Demo");
    println!("=======================");
    println!("Running: {}", demo_file);
    println!();
    
    // Test parser first
    match test_parser() {
        Ok(_) => println!("✅ Parser test passed"),
        Err(e) => {
            eprintln!("❌ Parser test failed: {}", e);
            process::exit(1);
        }
    }
    
    // Run the demo file
    match cursed::run_file(demo_file) {
        Ok(_) => {
            println!();
            println!("✅ Demo completed successfully!");
            println!();
            println!("🎉 CURSED language features working:");
            println!("   • Gen-Z keywords (slay, yolo, facts, sus, lowkey)");
            println!("   • Traditional syntax compatibility");
            println!("   • Function definitions and calls");
            println!("   • Variable declarations and assignments");
            println!("   • Control flow statements");
            println!("   • String and number literals");
        }
        Err(e) => {
            eprintln!("❌ Demo execution failed: {}", e);
            process::exit(1);
        }
    }
}

/// Test the parser with a simple CURSED program
fn test_parser() -> Result<(), Box<dyn std::error::Error>> {
    let test_code = r#"
        facts test_var = "Hello CURSED!"
        sus counter = 42
        
        slay test_function() {
            yolo test_var
        }
    "#;
    
    // Create parser using the new_parser function
    let mut parser = cursed::new_parser(test_code)?;
    
    // Parse the program
    let program = parser.parse_program()?;
    
    // Verify we parsed some statements
    if program.statements.is_empty() {
        return Err("No statements parsed".into());
    }
    
    println!("📝 Parsed {} statements", program.statements.len());
    
    Ok(())
}
