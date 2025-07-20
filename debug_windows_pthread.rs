// Simple test to check pthread linking for Windows cross-compilation
use std::process::Command;

fn main() {
    println!("Testing Windows pthread library linking...");
    
    // Test with minimal pthread usage
    let output = Command::new("x86_64-w64-mingw32-gcc")
        .args(&[
            "-o", "test_pthread.exe",
            "-xc", "-",
            "-lpthread"
        ])
        .arg("-")
        .output();
        
    match output {
        Ok(result) => {
            println!("Stdout: {}", String::from_utf8_lossy(&result.stdout));
            println!("Stderr: {}", String::from_utf8_lossy(&result.stderr));
            println!("Success: {}", result.status.success());
        }
        Err(e) => {
            println!("Failed to run gcc: {}", e);
        }
    }
}
