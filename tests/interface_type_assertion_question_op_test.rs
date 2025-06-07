use std::process::Command;
use std::io::{self, Write};
use std::path::Path;
use super::*;
use std::fs;

// Integration test for interface type assertions with the ? operator


#[cfg(test)]
mod tests {
    
    #[test]
    #[ignore] // Run with --ignored flag to execute this test
    fn test_interface_type_assertion_question_op() -> io::Result<()> {
        // Get the path to the example file
        let example_path = Path::new("examples/interface_type_assertion_question_op.csd");
        
        // Ensure the example file exists
        assert!(example_path.exists(), "Example file not found: {:?}", example_path);
        
        // Build the program using cargo
        let status = Command::new("cargo")
            .args(["build", "--quiet"])
            .status()?;
        
        assert!(status.success(), "Failed to build cursed compiler");
        
        // Run the example with the cursed interpreter
        let output = Command::new("./target/debug/cursed")
            .arg(example_path)
            .output()?;
        
        // Print any errors
        if !output.stderr.is_empty() {
            io::stderr().write_all(&output.stderr)?;
        }
        
        // Check that execution was successful
        assert!(output.status.success(), "Failed to execute example");
        
        // Get the output as string
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("Program output:\n{}", output_str);
        
        // Verify expected output contains successful processing
        assert!(output_str.contains("Processing Circle:"), "Missing Circle processing output");
        assert!(output_str.contains("Circle calculation result:"), "Missing successful Circle calculation result");
        assert!(output_str.contains("Processing Rectangle:"), "Missing Rectangle processing output");
        assert!(output_str.contains("Error:"), "Missing error message");
        assert!(output_str.contains("Type assertion failed"), "Missing type assertion failure message");
        assert!(output_str.contains("Shape has area:"), "Missing area calculation output");
        
        Ok(())
    }
}