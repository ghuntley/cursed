use std::process::Command;

fn main() {
    // Test error handling compilation
    println!("Testing error handling LLVM IR generation...");
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed", "--", "compile", "test_error_comprehensive.csd"])
        .output()
        .expect("Failed to execute command");
        
    println!("Compilation output:");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    if !output.stderr.is_empty() {
        println!("Compilation errors:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Try to read the generated IR if it exists
    if let Ok(ir_content) = std::fs::read_to_string("test_error_comprehensive.ll") {
        println!("\nGenerated LLVM IR:");
        println!("{}", ir_content);
    }
}
