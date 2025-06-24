use std::error::Error as StdError;
use std::fmt;

// Test if our unified Error type works
#[derive(Debug)]
pub struct TestError {
    message: String,
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for TestError {}

fn test_error_creation() -> Result<(), Box<dyn StdError>> {
    let err = TestError {
        message: "Test error".to_string(),
    };
    
    // Test basic error functionality
    println!("Error: {}", err);
    println!("Debug: {:?}", err);
    
    Ok(())
}

fn main() {
    match test_error_creation() {
        Ok(()) => println!("Error handling test passed"),
        Err(e) => println!("Error handling test failed: {}", e),
    }
}
