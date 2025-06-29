use cursed::execution::CursedExecutionEngine;
use std::fs;

fn main() {
    println!("Testing CURSED full compilation pipeline...");
    
    // Read the demo program
    let source = match fs::read_to_string("test_cursed_demo.csd") {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading test_cursed_demo.csd: {}", e);
            return;
        }
    };

    println!("Source code from test_cursed_demo.csd:");
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
