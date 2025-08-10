use cursed::execution::CursedExecutionEngine;
use std::fs;

fn main() {
    println!("Testing CURSED full compilation pipeline...");
    
    // Get filename from command line or use default
    let filename = std::env::args().nth(1).unwrap_or_else(|| "test_cursed_demo.csd".to_string());
    
    // Read the program
    let source = match fs::read_to_string(&filename) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading {}: {}", filename, e);
            return;
        }
    };

    println!("Source code from {}:", filename);
    println!("{}", source);
    println!("\nStarting compilation...");
    
    // Test the full compilation pipeline
    match CursedExecutionEngine::new() {
        Ok(mut engine) => {
            println!("✅ Execution engine created successfully!");
            
            // Try to execute the program
            match engine.execute(&source) {
                Ok(result) => {
                    println!("✅ Program executed successfully!");
                    println!("Result: {:?}", result);
                }
                Err(e) => {
                    println!("❌ Execution failed: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to create execution engine: {:?}", e);
        }
    }
}
