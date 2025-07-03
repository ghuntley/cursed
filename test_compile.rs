use std::process::Command;

fn main() {
    // Test the compile functionality directly
    let source = r#"
slay main() {
    vibez.spill("Hello, CURSED world! 🎉")
}
"#;
    
    println!("Testing CURSED compilation...");
    
    // Write test source to file
    std::fs::write("test_hello.csd", source).expect("Failed to write test file");
    
    // Run our compiler
    let output = Command::new("./target/debug/cursed")
        .args(&["--compile", "test_hello.csd", "-o", "test_hello"])
        .output();
        
    match output {
        Ok(result) => {
            println!("Compile command output:");
            println!("stdout: {}", String::from_utf8_lossy(&result.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&result.stderr));
            println!("exit code: {}", result.status.code().unwrap_or(-1));
            
            // Check if executable was created
            if std::path::Path::new("test_hello").exists() {
                println!("✅ Executable created successfully!");
                
                // Try to run it
                let run_output = Command::new("./test_hello").output();
                match run_output {
                    Ok(run_result) => {
                        println!("Running executable:");
                        println!("output: {}", String::from_utf8_lossy(&run_result.stdout));
                        if !run_result.stderr.is_empty() {
                            println!("stderr: {}", String::from_utf8_lossy(&run_result.stderr));
                        }
                    },
                    Err(e) => println!("❌ Failed to run executable: {}", e),
                }
            } else {
                println!("❌ No executable created");
            }
        },
        Err(e) => println!("❌ Failed to run compiler: {}", e),
    }
    
    // Clean up
    let _ = std::fs::remove_file("test_hello.csd");
    let _ = std::fs::remove_file("test_hello");
    let _ = std::fs::remove_file("test_hello.ll");
    let _ = std::fs::remove_file("test_hello.o");
}
