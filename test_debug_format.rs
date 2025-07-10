use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed", "--", "debug_compact.csd"])
        .output()
        .unwrap();
    
    println!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    
    let formatted = Command::new("cargo")
        .args(&["run", "--bin", "cursed", "--", "format", "debug_compact.csd"])
        .output()
        .unwrap();
    
    println!("Formatted STDOUT:\n{}", String::from_utf8_lossy(&formatted.stdout));
    println!("Formatted STDERR:\n{}", String::from_utf8_lossy(&formatted.stderr));
}
