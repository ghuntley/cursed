use std::process::Command;
use std::fs;

fn main() {
    // Write the test case to a file
    let source = r#"nah x > 0 {
vibez.spill("positive")
}"#;
    
    fs::write("debug_compact.csd", source).unwrap();
    
    // Run the formatter on the file
    let output = Command::new("cargo")
        .args(&["test", "formatter::tests::tests::test_compact_config", "--", "--nocapture"])
        .output()
        .unwrap();
    
    println!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    println!("Exit code: {}", output.status);
}
